use crate::{
    cache::{Cache, Excludes, ToQuery},
    error::Result,
    hoc,
    http::{self, AppState},
    platform::Platform,
    statics::{CLIENT, VERSION_INFO},
    template::RepoInfo,
    templates,
};

use std::{
    collections::BTreeSet,
    fs::create_dir_all,
    io,
    path::Path,
    sync::{Arc, atomic::Ordering},
};

use axum::{
    Json,
    extract::{Path as ReqPath, Query, State},
    http::{
        StatusCode,
        header::{self, HeaderMap, HeaderValue},
    },
    response::{IntoResponse, Redirect},
};
use badgers::{Badge, BadgeOptions};
use git2::Repository;
use jiff::{SignedDuration, Timestamp, fmt::rfc2822};
use number_prefix::NumberPrefix;
use serde::{Deserialize, Serialize};
use tracing::{Instrument, error, info, info_span, warn};

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
        service_path: String,
    },
    NotFound,
}

pub(crate) async fn delete_repo_and_cache(
    State(state): State<Arc<AppState>>,
    ReqPath((platform, owner, repo)): ReqPath<(Platform, String, String)>,
    Query(branch): Query<BadgeQuery>,
) -> Result<impl IntoResponse> {
    let span = info_span!(
        "deleting repository and cache",
        platform = platform.domain(),
        user = owner,
        repo
    );
    let future = async {
        let repo = format!("{}/{owner}/{repo}", platform.domain());
        info!("Deleting cache and repository");
        let repo_dir = state.repos().join(&repo);
        std::fs::remove_dir_all(repo_dir).or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })?;
        state.repo_count.fetch_sub(1, Ordering::Relaxed);

        state.cache.clear(platform, &owner, &repo)?;

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
        Ok(Redirect::temporary(&format!(
            "{}/{}/{owner}/{repo}/view{query}",
            state.settings.base_url,
            platform.url_path()
        )))
    };
    future.instrument(span).await
}

async fn handle_hoc_request(
    state: &AppState,
    (platform, owner, repo): (Platform, String, String),
    excludes: BTreeSet<String>,
    branch: &str,
) -> Result<HocResult> {
    let span = info_span!(
        "handling hoc calculation",
        platform = platform.domain(),
        user = owner.as_str(),
        repo = repo.as_str(),
        branch
    );
    let future = async move {
        let slug = format!("{}/{}", owner.to_lowercase(), repo.to_lowercase());
        let service_path = format!("{}/{slug}", platform.url_path());
        let service_url = format!("{}/{slug}", platform.domain());
        let path = state.repos().join(&service_url);
        let url = format!("https://{}/{slug}", platform.domain());
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
            state.repo_count.fetch_add(1, Ordering::Relaxed);
        }
        pull(&path, branch)?;

        let (hoc, head, commits) = hoc::hoc(
            state.repos(),
            platform,
            &owner,
            repo.as_str(),
            &state.cache,
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
            service_path,
        };
        Ok(res)
    };
    future.instrument(span).await
}

pub(crate) async fn json_hoc(
    State(state): State<Arc<AppState>>,
    ReqPath(data): ReqPath<(Platform, String, String)>,
    Query(query): Query<BadgeQuery>,
) -> Result<impl IntoResponse> {
    let branch = query.branch.as_deref().unwrap_or("master");
    let exclude = query.excludes();
    let r = handle_hoc_request(&state, data, exclude, branch).await?;
    match r {
        HocResult::NotFound => Ok(http::routes::p404(State(state)).await.into_response()),
        HocResult::Hoc {
            hoc, head, commits, ..
        } => Ok(Json(JsonResponse {
            branch,
            head: &head,
            count: hoc,
            commits,
        })
        .into_response()),
    }
}

fn no_cache_headers(expires: &Timestamp) -> HeaderMap {
    const FORMATTER: rfc2822::DateTimePrinter = rfc2822::DateTimePrinter::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("image/svg+xml"),
    );
    let expires = FORMATTER.timestamp_to_rfc9110_string(expires);
    if let Err(err) = &expires {
        error!(%err, "formatting error");
    }
    // TODO: error handling
    let expires = expires.unwrap();
    info!(expires, "expires");

    let expires_value = expires.try_into();
    if let Err(err) = &expires_value {
        error!(%err, "header value error");
    }
    headers.insert(
        header::EXPIRES,
        expires_value
            // TODO: error handling
            .unwrap(),
    );
    headers.append(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.append(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    headers.append(
        header::CACHE_CONTROL,
        HeaderValue::from_static("must-revalidate"),
    );
    headers.append(header::CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
    headers
}

fn no_cache_response(body: Vec<u8>) -> impl IntoResponse {
    let expiration = Timestamp::now() + SignedDuration::from_secs(30);
    (StatusCode::OK, no_cache_headers(&expiration), body)
}

pub(crate) async fn calculate_hoc(
    State(state): State<Arc<AppState>>,
    ReqPath(data): ReqPath<(Platform, String, String)>,
    Query(query): Query<BadgeQuery>,
) -> Result<impl IntoResponse> {
    let label = query.label.clone();
    let branch = query.branch.as_deref().unwrap_or("master");
    let exclude = query.excludes();
    if let Ok(r) = handle_hoc_request(&state, data, exclude, branch).await {
        match r {
            HocResult::NotFound => Ok(http::routes::p404(State(state)).await.into_response()),
            HocResult::Hoc { hoc_pretty, .. } => {
                let badge_opt = BadgeOptions {
                    subject: label,
                    color: "#007ec6".to_string(),
                    status: hoc_pretty,
                };
                let badge = Badge::new(badge_opt)?;
                // TODO: remove clone
                let body = badge.to_svg().as_bytes().to_vec();

                Ok(no_cache_response(body).into_response())
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
        Ok(no_cache_response(body).into_response())
    }
}

pub(crate) async fn overview(
    State(state): State<Arc<AppState>>,
    ReqPath(data @ (platform, _, _)): ReqPath<(Platform, String, String)>,
    Query(query): Query<BadgeQuery>,
) -> Result<impl IntoResponse> {
    let branch = query.branch.as_deref().unwrap_or("master");
    let label = query.label.clone();
    let base_url = state.settings.base_url.clone();
    let exclude = query.excludes();
    let r = handle_hoc_request(&state, data, exclude, branch).await?;
    match r {
        HocResult::NotFound => Ok(http::routes::p404(State(state)).await.into_response()),
        HocResult::Hoc {
            hoc,
            commits,
            hoc_pretty,
            url,
            head,
            repo,
            service_path,
        } => {
            let repo_info = RepoInfo {
                commit_url: &platform.commit_url(&repo, &head),
                commits,
                base_url: &base_url,
                head: &head,
                hoc,
                hoc_pretty: &hoc_pretty,
                path: &service_path,
                url: &url,
                branch,
            };
            Ok(render!(
                templates::overview_html,
                VERSION_INFO,
                state.repo_count.load(Ordering::Relaxed),
                repo_info,
                &label,
                &query.exclude,
            )
            .into_response())
        }
    }
}
