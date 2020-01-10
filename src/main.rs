#![type_length_limit = "2257138"]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod cache;
mod config;
mod count;
mod error;
mod service;
mod statics;
mod template;

use crate::{
    cache::CacheState,
    error::{Error, Result},
    service::{Bitbucket, FormService, GitHub, Gitlab, Service},
    statics::{CLIENT, CSS, FAVICON, OPT, REPO_COUNT, VERSION_INFO},
    template::RepoInfo,
};
use actix_web::{
    error::ErrorBadRequest,
    http::header::{CacheControl, CacheDirective, Expires},
    middleware, web, App, HttpResponse, HttpServer,
};
use badge::{Badge, BadgeOptions};
use bytes::Bytes;
use futures::{unsync::mpsc, Future, Stream};
use git2::Repository;
use number_prefix::{NumberPrefix, Prefixed, Standalone};
use std::{
    borrow::Cow,
    fs::create_dir_all,
    path::Path,
    process::Command,
    sync::atomic::Ordering,
    sync::Arc,
    time::{Duration, SystemTime},
};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[derive(Deserialize, Serialize)]
struct GeneratorForm<'a> {
    service: FormService,
    user: Cow<'a, str>,
    repo: Cow<'a, str>,
}

struct State {
    repos: String,
    cache: String,
}

#[derive(Serialize)]
struct JsonResponse<'a> {
    head: &'a str,
    count: u64,
    commits: u64,
}

fn pull(path: impl AsRef<Path>) -> Result<()> {
    let repo = Repository::open_bare(path)?;
    let mut origin = repo.find_remote("origin")?;
    origin.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
    Ok(())
}

fn hoc(repo: &str, repo_dir: &str, cache_dir: &str) -> Result<(u64, String, u64)> {
    let repo_dir = format!("{}/{}", repo_dir, repo);
    let cache_dir = format!("{}/{}.json", cache_dir, repo);
    let cache_dir = Path::new(&cache_dir);
    let repo = Repository::open_bare(&repo_dir)?;
    let head = format!("{}", repo.head()?.target().ok_or(Error::Internal)?);
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
    let cache = CacheState::read_from_file(&cache_dir, &head)?;
    match &cache {
        CacheState::Current { count, commits } => {
            info!("Using cache for {}", repo_dir);
            return Ok((*count, head, *commits));
        }
        CacheState::Old(cache) => {
            info!("Updating cache for {}", repo_dir);
            arg.push(format!("{}..HEAD", cache.head));
            arg_commit_count.push(format!("{}..HEAD", cache.head));
        }
        CacheState::No => {
            info!("Creating cache for {}", repo_dir);
            arg_commit_count.push("HEAD".to_string());
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

    let cache = cache.calculate_new_cache(count, commits, (&head).into());
    cache.write_to_file(cache_dir)?;

    Ok((cache.count, head, commits))
}

fn remote_exists(url: &str) -> impl Future<Item = bool, Error = Error> {
    CLIENT
        .head(url)
        .send()
        .map(|resp| resp.status() == reqwest::StatusCode::OK)
        .from_err()
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

fn handle_hoc_request<T, F>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
    mapper: F,
) -> impl Future<Item = HttpResponse, Error = Error>
where
    T: Service,
    F: Fn(HocResult) -> Result<HttpResponse>,
{
    let repo = format!("{}/{}", data.0.to_lowercase(), data.1.to_lowercase());
    let service_path = format!("{}/{}", T::domain(), repo);
    let path = format!("{}/{}", state.repos, service_path);
    let url = format!("https://{}", service_path);
    remote_exists(&url)
        .and_then(move |remote_exists| {
            let file = Path::new(&path);
            if !file.exists() {
                if !remote_exists {
                    warn!("Repository does not exist: {}", url);
                    return Ok(HocResult::NotFound);
                }
                info!("Cloning {} for the first time", url);
                create_dir_all(file)?;
                let repo = Repository::init_bare(file)?;
                repo.remote_add_fetch("origin", "refs/heads/*:refs/heads/*")?;
                repo.remote_set_url("origin", &url)?;
                REPO_COUNT.fetch_add(1, Ordering::Relaxed);
            }
            pull(&path)?;
            let (hoc, head, commits) = hoc(&service_path, &state.repos, &state.cache)?;
            let hoc_pretty = match NumberPrefix::decimal(hoc as f64) {
                Standalone(hoc) => hoc.to_string(),
                Prefixed(prefix, hoc) => format!("{:.1}{}", hoc, prefix),
            };
            Ok(HocResult::Hoc {
                hoc,
                commits,
                hoc_pretty,
                head: head,
                url,
                repo,
                service_path,
            })
        })
        .and_then(mapper)
}

fn json_hoc<T: Service>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mapper = |r| match r {
        HocResult::NotFound => p404(),
        HocResult::Hoc {
            hoc, head, commits, ..
        } => Ok(HttpResponse::Ok().json(JsonResponse {
            head: &head,
            count: hoc,
            commits,
        })),
    };
    handle_hoc_request::<T, _>(state, data, mapper)
}

fn calculate_hoc<T: Service>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mapper = |r| match r {
        HocResult::NotFound => p404(),
        HocResult::Hoc { hoc_pretty, .. } => {
            let badge_opt = BadgeOptions {
                subject: "Hits-of-Code".to_string(),
                color: "#007ec6".to_string(),
                status: hoc_pretty,
            };
            let badge = Badge::new(badge_opt)?;

            let (tx, rx_body) = mpsc::unbounded();
            let _ = tx.unbounded_send(Bytes::from(badge.to_svg().as_bytes()));

            let expiration = SystemTime::now() + Duration::from_secs(30);
            Ok(HttpResponse::Ok()
                .content_type("image/svg+xml")
                .set(Expires(expiration.into()))
                .set(CacheControl(vec![
                    CacheDirective::MaxAge(0u32),
                    CacheDirective::MustRevalidate,
                    CacheDirective::NoCache,
                    CacheDirective::NoStore,
                ]))
                .streaming(rx_body.map_err(|_| ErrorBadRequest("bad request"))))
        }
    };
    handle_hoc_request::<T, _>(state, data, mapper)
}

fn overview<T: Service>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mapper = |r| match r {
        HocResult::NotFound => p404(),
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
                domain: &OPT.domain,
                head: &head,
                hoc,
                hoc_pretty: &hoc_pretty,
                path: &service_path,
                url: &url,
            };
            templates::overview(
                &mut buf,
                VERSION_INFO,
                REPO_COUNT.load(Ordering::Relaxed),
                repo_info
                // &OPT.domain,
                // &service_path,
                // &url,
                // hoc,
                // &hoc_pretty,
                // &head,
                // &T::commit_url(&repo, &head),
                // commits,
            )?;

            let (tx, rx_body) = mpsc::unbounded();
            let _ = tx.unbounded_send(Bytes::from(buf));

            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .streaming(rx_body.map_err(|_| ErrorBadRequest("bad request"))))
        }
    };
    handle_hoc_request::<T, _>(state, data, mapper)
}

#[get("/")]
fn index() -> Result<HttpResponse> {
    let mut buf = Vec::new();
    templates::index(
        &mut buf,
        VERSION_INFO,
        REPO_COUNT.load(Ordering::Relaxed),
        &OPT.domain,
    )?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buf))
}

#[post("/generate")]
fn generate(params: web::Form<GeneratorForm>) -> Result<HttpResponse> {
    let repo = format!("{}/{}", params.user, params.repo);
    let mut buf = Vec::new();
    templates::generate(
        &mut buf,
        VERSION_INFO,
        REPO_COUNT.load(Ordering::Relaxed),
        &OPT.domain,
        params.service.url(),
        params.service.service(),
        &repo,
    )?;
    let (tx, rx_body) = mpsc::unbounded();
    let _ = tx.unbounded_send(Bytes::from(buf));

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .streaming(rx_body.map_err(|_| ErrorBadRequest("bad request"))))
}

fn p404() -> Result<HttpResponse> {
    let mut buf = Vec::new();
    templates::p404(&mut buf, VERSION_INFO, REPO_COUNT.load(Ordering::Relaxed))?;
    Ok(HttpResponse::NotFound().content_type("text/html").body(buf))
}

#[get("/tacit-css.min.css")]
fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(CSS)
}

#[get("/favicon.ico")]
fn favicon32() -> HttpResponse {
    HttpResponse::Ok().content_type("image/png").body(FAVICON)
}

fn start_server() -> Result<()> {
    let interface = format!("{}:{}", OPT.host, OPT.port);
    let state = Arc::new(State {
        repos: OPT.outdir.display().to_string(),
        cache: OPT.cachedir.display().to_string(),
    });
    Ok(HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath)
            .service(index)
            .service(css)
            .service(favicon32)
            .service(generate)
            .service(web::resource("/github/{user}/{repo}").to_async(calculate_hoc::<GitHub>))
            .service(web::resource("/gitlab/{user}/{repo}").to_async(calculate_hoc::<Gitlab>))
            .service(web::resource("/bitbucket/{user}/{repo}").to_async(calculate_hoc::<Bitbucket>))
            .service(web::resource("/github/{user}/{repo}/json").to_async(json_hoc::<GitHub>))
            .service(web::resource("/gitlab/{user}/{repo}/json").to_async(json_hoc::<Gitlab>))
            .service(web::resource("/bitbucket/{user}/{repo}/json").to_async(json_hoc::<Bitbucket>))
            .service(web::resource("/view/github/{user}/{repo}").to_async(overview::<GitHub>))
            .service(web::resource("/view/gitlab/{user}/{repo}").to_async(overview::<Gitlab>))
            .service(web::resource("/view/bitbucket/{user}/{repo}").to_async(overview::<Bitbucket>))
            .default_service(web::resource("").route(web::get().to_async(p404)))
    })
    .workers(OPT.workers)
    .bind(interface)?
    .run()?)
}

fn main() -> Result<()> {
    config::init()?;
    start_server()
}
