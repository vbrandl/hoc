#![type_length_limit = "2257138"]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tracing;

mod cache;
pub mod config;
pub mod count;
mod error;
mod service;
mod statics;
pub mod telemetry;
mod template;

use crate::{
    cache::CacheState,
    config::Settings,
    error::{Error, Result},
    service::{Bitbucket, FormService, GitHub, Gitlab, Service, Sourcehut},
    statics::{CLIENT, CSS, FAVICON, VERSION_INFO},
    template::RepoInfo,
};
use actix_web::{
    dev::Server,
    http::header::{CacheControl, CacheDirective, Expires, LOCATION},
    middleware::{self, TrailingSlash},
    web, App, HttpResponse, HttpServer, Responder,
};
use badge::{Badge, BadgeOptions};
use git2::{BranchType, Repository};
use number_prefix::NumberPrefix;
use std::{
    borrow::Cow,
    fs::create_dir_all,
    io,
    net::TcpListener,
    path::Path,
    process::Command,
    sync::atomic::AtomicUsize,
    sync::atomic::Ordering,
    time::{Duration, SystemTime},
};
use tracing::Instrument;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[derive(Deserialize, Serialize)]
struct GeneratorForm<'a> {
    service: FormService,
    user: Cow<'a, str>,
    repo: Cow<'a, str>,
}

#[derive(Debug)]
pub(crate) struct State {
    settings: Settings,
}

impl State {
    fn repos(&self) -> String {
        self.settings.repodir.display().to_string()
    }

    fn cache(&self) -> String {
        self.settings.cachedir.display().to_string()
    }
}

#[derive(Serialize)]
struct JsonResponse<'a> {
    head: &'a str,
    branch: &'a str,
    count: u64,
    commits: u64,
}

#[derive(Deserialize, Debug)]
struct BranchQuery {
    branch: Option<String>,
}

fn pull(path: impl AsRef<Path>) -> Result<()> {
    let repo = Repository::open_bare(path)?;
    let mut origin = repo.find_remote("origin")?;
    origin.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
    Ok(())
}

fn hoc(repo: &str, repo_dir: &str, cache_dir: &str, branch: &str) -> Result<(u64, String, u64)> {
    let repo_dir = format!("{}/{}", repo_dir, repo);
    let cache_dir = format!("{}/{}.json", cache_dir, repo);
    let cache_dir = Path::new(&cache_dir);
    let repo = Repository::open_bare(&repo_dir)?;
    // TODO: do better...
    let head = repo
        .find_branch(branch, BranchType::Local)
        .map_err(|_| Error::BranchNotFound)?
        .into_reference();
    let head = format!("{}", head.target().ok_or(Error::BranchNotFound)?);
    let mut arg_commit_count = vec!["rev-list".to_string(), "--count".to_string()];
    let mut arg = vec![
        "log".to_string(),
        "--pretty=tformat:".to_string(),
        "--numstat".to_string(),
        "--ignore-space-change".to_string(),
        "--ignore-all-space".to_string(),
        "--ignore-submodules".to_string(),
        "--no-color".to_string(),
        "--find-copies-harder".to_string(),
        "-M".to_string(),
        "--diff-filter=ACDM".to_string(),
    ];
    let cache = CacheState::read_from_file(&cache_dir, branch, &head)?;
    match &cache {
        CacheState::Current { count, commits, .. } => {
            info!("Using cache");
            return Ok((*count, head, *commits));
        }
        CacheState::Old { head, .. } => {
            info!("Updating cache");
            arg.push(format!("{}..{}", head, branch));
            arg_commit_count.push(format!("{}..{}", head, branch));
        }
        CacheState::No | CacheState::NoneForBranch(..) => {
            info!("Creating cache");
            arg.push(branch.to_string());
            arg_commit_count.push(branch.to_string());
        }
    };
    arg.push("--".to_string());
    arg.push(".".to_string());
    let output = Command::new("git")
        .args(&arg)
        .current_dir(&repo_dir)
        .output()?
        .stdout;
    let output = String::from_utf8_lossy(&output);
    let output_commits = Command::new("git")
        .args(&arg_commit_count)
        .current_dir(&repo_dir)
        .output()?
        .stdout;
    let output_commits = String::from_utf8_lossy(&output_commits);
    let commits: u64 = output_commits.trim().parse()?;
    let count: u64 = output
        .lines()
        .map(|s| {
            s.split_whitespace()
                .take(2)
                .map(str::parse::<u64>)
                .filter_map(std::result::Result::ok)
                .sum::<u64>()
        })
        .sum();

    let cache = cache.calculate_new_cache(count, commits, (&head).into(), branch);
    cache.write_to_file(cache_dir)?;

    Ok((count, head, commits))
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

async fn delete_repo_and_cache<T>(
    state: web::Data<State>,
    repo_count: web::Data<AtomicUsize>,
    data: web::Path<(String, String)>,
) -> Result<impl Responder>
where
    T: Service,
{
    let data = data.into_inner();
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
        let cache_dir = format!("{}/{}.json", &state.cache(), repo);
        let repo_dir = format!("{}/{}", &state.repos(), repo);
        std::fs::remove_file(&cache_dir).or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })?;
        std::fs::remove_dir_all(&repo_dir).or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })?;
        repo_count.fetch_sub(1, Ordering::Relaxed);
        Ok(HttpResponse::TemporaryRedirect()
            .insert_header((
                LOCATION,
                format!("/{}/{}/{}/view", T::url_path(), data.0, data.1),
            ))
            .finish())
    };
    future.instrument(span).await
}

async fn handle_hoc_request<T, F>(
    state: web::Data<State>,
    repo_count: web::Data<AtomicUsize>,
    data: web::Path<(String, String)>,
    branch: &str,
    mapper: F,
) -> Result<HttpResponse>
where
    T: Service,
    F: FnOnce(HocResult) -> Result<HttpResponse>,
{
    let data = data.into_inner();
    let span = info_span!(
        "handling hoc calculation",
        service = T::domain(),
        user = data.0.as_str(),
        repo = data.1.as_str(),
        branch
    );
    let future = async {
        let repo = format!("{}/{}", data.0.to_lowercase(), data.1.to_lowercase());
        let service_path = format!("{}/{}", T::url_path(), repo);
        let service_url = format!("{}/{}", T::domain(), repo);
        let path = format!("{}/{}", state.repos(), service_url);
        let url = format!("https://{}", service_url);
        let remote_exists = remote_exists(&url).await?;
        let file = Path::new(&path);
        if !file.exists() {
            if !remote_exists {
                warn!("Repository does not exist");
                return mapper(HocResult::NotFound);
            }
            info!("Cloning for the first time");
            create_dir_all(file)?;
            let repo = Repository::init_bare(file)?;
            repo.remote_add_fetch("origin", "refs/heads/*:refs/heads/*")?;
            repo.remote_set_url("origin", &url)?;
            repo_count.fetch_add(1, Ordering::Relaxed);
        }
        pull(&path)?;
        let (hoc, head, commits) = hoc(&service_url, &state.repos(), &state.cache(), branch)?;
        let hoc_pretty = match NumberPrefix::decimal(hoc as f64) {
            NumberPrefix::Standalone(hoc) => hoc.to_string(),
            NumberPrefix::Prefixed(prefix, hoc) => format!("{:.1}{}", hoc, prefix),
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
        mapper(res)
    };
    future.instrument(span).await
}

pub(crate) async fn json_hoc<T: Service>(
    state: web::Data<State>,
    repo_count: web::Data<AtomicUsize>,
    data: web::Path<(String, String)>,
    branch: web::Query<BranchQuery>,
) -> Result<HttpResponse> {
    let branch = branch.branch.as_deref().unwrap_or("master");
    let rc_clone = repo_count.clone();
    let mapper = move |r| match r {
        HocResult::NotFound => p404(rc_clone),
        HocResult::Hoc {
            hoc, head, commits, ..
        } => Ok(HttpResponse::Ok().json(JsonResponse {
            branch,
            head: &head,
            count: hoc,
            commits,
        })),
    };
    handle_hoc_request::<T, _>(state, repo_count, data, branch, mapper).await
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
    repo_count: web::Data<AtomicUsize>,
    data: web::Path<(String, String)>,
    branch: web::Query<BranchQuery>,
) -> HttpResponse {
    let rc_clone = repo_count.clone();
    let mapper = move |r| match r {
        HocResult::NotFound => p404(rc_clone),
        HocResult::Hoc { hoc_pretty, .. } => {
            let badge_opt = BadgeOptions {
                subject: "Hits-of-Code".to_string(),
                color: "#007ec6".to_string(),
                status: hoc_pretty,
            };
            let badge = Badge::new(badge_opt)?;
            // TODO: remove clone
            let body = badge.to_svg().as_bytes().to_vec();

            Ok(no_cache_response(body))
        }
    };
    let branch = branch.branch.as_deref().unwrap_or("master");
    let error_badge = |_| {
        let error_badge = Badge::new(BadgeOptions {
            subject: "Hits-of-Code".to_string(),
            color: "#ff0000".to_string(),
            status: "error".to_string(),
        })
        .unwrap();
        let body = error_badge.to_svg().as_bytes().to_vec();
        no_cache_response(body)
    };
    handle_hoc_request::<T, _>(state, repo_count, data, branch, mapper)
        .await
        .unwrap_or_else(error_badge)
}

async fn overview<T: Service>(
    state: web::Data<State>,
    repo_count: web::Data<AtomicUsize>,
    data: web::Path<(String, String)>,
    branch: web::Query<BranchQuery>,
) -> Result<HttpResponse> {
    let branch = branch.branch.as_deref().unwrap_or("master");
    let base_url = state.settings.base_url.clone();
    let rc_clone = repo_count.clone();
    let mapper = move |r| match r {
        HocResult::NotFound => p404(rc_clone),
        HocResult::Hoc {
            hoc,
            commits,
            hoc_pretty,
            url,
            head,
            repo,
            service_path,
        } => {
            let mut buf = Vec::new();
            let repo_info = RepoInfo {
                commit_url: &T::commit_url(&repo, &head),
                commits,
                base_url: &base_url,
                head: &head,
                hoc,
                hoc_pretty: &hoc_pretty,
                path: &service_path,
                url: &url,
                branch,
            };
            templates::overview(
                &mut buf,
                VERSION_INFO,
                rc_clone.load(Ordering::Relaxed),
                repo_info,
            )?;

            Ok(HttpResponse::Ok().content_type("text/html").body(buf))
        }
    };
    handle_hoc_request::<T, _>(state, repo_count, data, branch, mapper).await
}

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/")]
async fn index(
    state: web::Data<State>,
    repo_count: web::Data<AtomicUsize>,
) -> Result<HttpResponse> {
    let mut buf = Vec::new();
    templates::index(
        &mut buf,
        VERSION_INFO,
        repo_count.load(Ordering::Relaxed),
        &state.settings.base_url,
    )?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buf))
}

#[post("/generate")]
async fn generate(
    params: web::Form<GeneratorForm<'_>>,
    state: web::Data<State>,
    repo_count: web::Data<AtomicUsize>,
) -> Result<HttpResponse> {
    let repo = format!("{}/{}", params.user, params.repo);
    let mut buf = Vec::new();
    templates::generate(
        &mut buf,
        VERSION_INFO,
        repo_count.load(Ordering::Relaxed),
        &state.settings.base_url,
        params.service.url(),
        params.service.service(),
        &repo,
    )?;

    Ok(HttpResponse::Ok().content_type("text/html").body(buf))
}

fn p404(repo_count: web::Data<AtomicUsize>) -> Result<HttpResponse> {
    let mut buf = Vec::new();
    templates::p404(&mut buf, VERSION_INFO, repo_count.load(Ordering::Relaxed))?;
    Ok(HttpResponse::NotFound().content_type("text/html").body(buf))
}

async fn async_p404(repo_count: web::Data<AtomicUsize>) -> Result<HttpResponse> {
    p404(repo_count)
}

#[get("/tacit-css.min.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(CSS)
}

#[get("/favicon.ico")]
async fn favicon32() -> HttpResponse {
    HttpResponse::Ok().content_type("image/png").body(FAVICON)
}

async fn start_server(listener: TcpListener, settings: Settings) -> std::io::Result<Server> {
    let workers = settings.workers;
    let repo_count =
        // TODO: errorhandling
        web::Data::new(AtomicUsize::new(count::count_repositories(&settings.repodir).unwrap()));
    let state = web::Data::new(State { settings });
    Ok(HttpServer::new(move || {
        let app = App::new()
            .app_data(state.clone())
            .app_data(repo_count.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .service(index)
            .service(health_check)
            .service(css)
            .service(favicon32)
            .service(generate)
            .default_service(web::to(async_p404));
        let app = GitHub::register_service(app);
        let app = Gitlab::register_service(app);
        let app = Bitbucket::register_service(app);
        Sourcehut::register_service(app)
    })
    .workers(workers)
    .listen(listener)?
    .run())
}

pub async fn run(listener: TcpListener, settings: Settings) -> std::io::Result<Server> {
    let span = info_span!("hoc", version = env!("CARGO_PKG_VERSION"));
    let _ = span.enter();
    start_server(listener, settings).instrument(span).await
}
