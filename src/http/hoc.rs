use crate::{
    State,
    cache::{Cache, Excludes, Persist, ToQuery},
    error::Result,
    hoc, http,
    service::Service,
    statics::{CLIENT, VERSION_INFO},
    template::RepoInfo,
    templates,
};

use std::{
    collections::BTreeSet,
    fs::create_dir_all,
    io,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, SystemTime},
};

use actix_web::{
    HttpResponse, Responder,
    http::header::{CacheControl, CacheDirective, Expires, LOCATION},
    web,
};
use badgers::{Badge, BadgeOptions};
use git2::Repository;
use number_prefix::NumberPrefix;
use serde::{Deserialize, Serialize};
use tracing::{Instrument, info, info_span, warn};

#[derive(Serialize)]
struct JsonResponse<'a> {
    head: &'a str,
    branch: &'a str,
    count: u64,
    commits: u64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct BadgeQuery {
    branch: Option<String>,
    #[serde(default = "String::new")]
    exclude: String,
    #[serde(default = "default_label")]
    label: String,
}

impl BadgeQuery {
    fn excludes(&self) -> Excludes {
        self.exclude
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }
}

fn default_label() -> String {
    "Hits-of-Code".to_string()
}

fn pull(path: impl AsRef<Path>, branch: &str) -> Result<()> {
    let repo = Repository::open_bare(path)?;
    let mut origin = repo.find_remote("origin")?;
    origin.fetch(&[branch], None, None)?;
    Ok(())
}

async fn remote_exists(url: &str) -> Result<bool> {
    let resp = CLIENT.head(url).send().await?;
    Ok(resp.status() == reqwest::StatusCode::OK)
}

enum HocResult {
    Hoc {
        hoc: u64,
        commits: u64,
        hoc_pretty: String,
        head: String,
        url: String,
        repo: String,
        owner: String,
        service_path: String,
    },
    NotFound,
}

pub(crate) async fn delete_repo_and_cache<T>(
    state: web::Data<State>,
    cache: web::Data<Persist>,
    repo_count: web::Data<AtomicUsize>,
    data: web::Path<(String, String)>,
    branch: web::Query<BadgeQuery>,
) -> Result<impl Responder>
where
    T: Service,
{
    let data = data.into_inner();
    let cache = cache.into_inner();
    let span = info_span!(
        "deleting repository and cache",
        service = T::domain(),
        user = data.0.as_str(),
        repo = data.1.as_str()
    );
    let future = async {
        let repo = format!(
            "{}/{}/{}",
            T::domain(),
            data.0.to_lowercase(),
            data.1.to_lowercase()
        );
        info!("Deleting cache and repository");
        let repo_dir = state.repos().join(&repo);
        std::fs::remove_dir_all(repo_dir).or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })?;
        repo_count.fetch_sub(1, Ordering::Relaxed);

        cache.clear(T::form_value(), data.0.as_str(), data.1.as_str())?;

        let branch_query = branch.branch.as_ref().map(|b| format!("branch={b}"));

        let excludes = branch.excludes();
        let exclude_query = if excludes.is_empty() {
            None
        } else {
            Some(excludes.to_query())
        };

        let label_query = Some(format!("label={}", branch.label));

        let query: Vec<_> = [branch_query, exclude_query, label_query]
            .into_iter()
            .flatten()
            .collect();
        let query = query.join("&");
        let query = if query.is_empty() {
            query
        } else {
            format!("?{query}")
        };
        Ok(HttpResponse::TemporaryRedirect()
            .insert_header((
                LOCATION,
                format!("/{}/{}/{}/view{query}", T::url_path(), data.0, data.1),
            ))
            .finish())
    };
    future.instrument(span).await
}

async fn handle_hoc_request<T>(
    state: web::Data<State>,
    cache: web::Data<Persist>,
    repo_count: &AtomicUsize,
    data: web::Path<(String, String)>,
    excludes: BTreeSet<String>,
    branch: &str,
) -> Result<HocResult>
where
    T: Service,
{
    let (owner, repo) = data.into_inner();
    let span = info_span!(
        "handling hoc calculation",
        service = T::domain(),
        user = owner.as_str(),
        repo = repo.as_str(),
        branch
    );
    let future = async move {
        let slug = format!("{}/{}", owner.to_lowercase(), repo.to_lowercase());
        let service_path = format!("{}/{slug}", T::url_path());
        let service_url = format!("{}/{slug}", T::domain());
        let path = state.repos().join(&service_url);
        let url = format!("https://{service_url}");
        let remote_exists = remote_exists(&url).await?;
        let file = Path::new(&path);
        if !file.exists() {
            if !remote_exists {
                warn!("Repository does not exist");
                return Ok(HocResult::NotFound);
            }
            info!("Cloning for the first time");
            create_dir_all(file)?;
            let repo = Repository::init_bare(file)?;
            repo.remote_add_fetch("origin", "refs/heads/*:refs/heads/*")?;
            repo.remote_set_url("origin", &url)?;
            repo_count.fetch_add(1, Ordering::Relaxed);
        }
        pull(&path, branch)?;

        let (hoc, head, commits) = hoc::hoc(
            state.repos(),
            T::form_value(),
            &owner,
            repo.as_str(),
            &cache,
            branch,
            excludes,
        )?;

        #[allow(clippy::cast_precision_loss)]
        let hoc_pretty = match NumberPrefix::decimal(hoc as f64) {
            NumberPrefix::Standalone(hoc) => hoc.to_string(),
            NumberPrefix::Prefixed(prefix, hoc) => format!("{hoc:.1}{prefix}"),
        };
        let res = HocResult::Hoc {
            hoc,
            commits,
            hoc_pretty,
            head,
            url,
            repo,
            owner,
            service_path,
        };
        Ok(res)
    };
    future.instrument(span).await
}

pub(crate) async fn json_hoc<T: Service>(
    state: web::Data<State>,
    cache: web::Data<Persist>,
    repo_count: web::Data<AtomicUsize>,
    data: web::Path<(String, String)>,
    query: web::Query<BadgeQuery>,
) -> Result<HttpResponse> {
    let query = query.into_inner();
    let branch = query.branch.as_deref().unwrap_or("master");
    let exclude = query.excludes();
    let repo_count = repo_count.into_inner();
    let r = handle_hoc_request::<T>(state, cache, &repo_count, data, exclude, branch).await?;
    match r {
        HocResult::NotFound => http::p404(&repo_count),
        HocResult::Hoc {
            hoc, head, commits, ..
        } => Ok(HttpResponse::Ok().json(JsonResponse {
            branch,
            head: &head,
            count: hoc,
            commits,
        })),
    }
}

fn no_cache_response(body: Vec<u8>) -> HttpResponse {
    let expiration = SystemTime::now() + Duration::from_secs(30);
    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .insert_header(Expires(expiration.into()))
        .insert_header(CacheControl(vec![
            CacheDirective::MaxAge(0u32),
            CacheDirective::MustRevalidate,
            CacheDirective::NoCache,
            CacheDirective::NoStore,
        ]))
        .body(body)
}

pub(crate) async fn calculate_hoc<T: Service>(
    state: web::Data<State>,
    cache: web::Data<Persist>,
    repo_count: web::Data<AtomicUsize>,
    data: web::Path<(String, String)>,
    query: web::Query<BadgeQuery>,
) -> Result<HttpResponse> {
    let query = query.into_inner();
    let repo_count = repo_count.into_inner();
    let label = query.label.clone();
    let branch = query.branch.as_deref().unwrap_or("master");
    let exclude = query.excludes();
    if let Ok(r) = handle_hoc_request::<T>(state, cache, &repo_count, data, exclude, branch).await {
        match r {
            HocResult::NotFound => http::p404(&repo_count),
            HocResult::Hoc { hoc_pretty, .. } => {
                let badge_opt = BadgeOptions {
                    subject: label,
                    color: "#007ec6".to_string(),
                    status: hoc_pretty,
                };
                let badge = Badge::new(badge_opt)?;
                // TODO: remove clone
                let body = badge.to_svg().as_bytes().to_vec();

                Ok(no_cache_response(body))
            }
        }
    } else {
        let error_badge = Badge::new(BadgeOptions {
            subject: query.label.clone(),
            color: "#ff0000".to_string(),
            status: "error".to_string(),
        })
        .unwrap();
        let body = error_badge.to_svg().as_bytes().to_vec();
        Ok(no_cache_response(body))
    }
}

pub(crate) async fn overview<T: Service>(
    state: web::Data<State>,
    cache: web::Data<Persist>,
    repo_count: web::Data<AtomicUsize>,
    data: web::Path<(String, String)>,
    query: web::Query<BadgeQuery>,
) -> Result<HttpResponse> {
    let query = query.into_inner();
    let branch = query.branch.as_deref().unwrap_or("master");
    let repo_count = repo_count.into_inner();
    let label = query.label.clone();
    let base_url = state.settings.base_url.clone();
    let exclude = query.excludes();
    let r = handle_hoc_request::<T>(state, cache, &repo_count, data, exclude, branch).await?;
    match r {
        HocResult::NotFound => http::p404(&repo_count),
        HocResult::Hoc {
            hoc,
            commits,
            hoc_pretty,
            url,
            head,
            owner,
            repo,
            service_path,
        } => {
            let mut buf = Vec::new();
            let repo_info = RepoInfo {
                commit_url: &T::commit_url(&owner, &repo, &head),
                commits,
                base_url: &base_url,
                head: &head,
                hoc,
                hoc_pretty: &hoc_pretty,
                path: &service_path,
                url: &url,
                branch,
            };
            templates::overview_html(
                &mut buf,
                VERSION_INFO,
                repo_count.load(Ordering::Relaxed),
                repo_info,
                &label,
                &query.exclude,
            )?;

            Ok(HttpResponse::Ok().content_type("text/html").body(buf))
        }
    }
}
