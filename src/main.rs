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

#[cfg(test)]
mod tests;

use crate::{
    cache::CacheState,
    error::{Error, Result},
    service::{Bitbucket, FormService, GitHub, Gitlab, Service},
    statics::{CLIENT, CSS, FAVICON, OPT, REPO_COUNT, VERSION_INFO},
    template::RepoInfo,
};
use actix_web::{
    http::header::{CacheControl, CacheDirective, Expires, LOCATION},
    middleware, web, App, HttpResponse, HttpServer, Responder,
};
use badge::{Badge, BadgeOptions};
use git2::Repository;
use number_prefix::NumberPrefix;
use std::{
    borrow::Cow,
    fs::create_dir_all,
    io,
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

#[derive(Debug)]
pub(crate) struct State {
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
    // TODO: do better...
    let head = match repo.head() {
        Ok(v) => v,
        Err(_) => return Err(Error::GitNoMaster),
    };
    let head = format!("{}", head.target().ok_or(Error::Internal)?);
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
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<impl Responder>
where
    T: Service,
{
    let repo = format!(
        "{}/{}/{}",
        T::domain(),
        data.0.to_lowercase(),
        data.1.to_lowercase()
    );
    info!("Deleting cache and repository for {}", repo);
    let cache_dir = dbg!(format!("{}/{}.json", &state.cache, repo));
    let repo_dir = dbg!(format!("{}/{}", &state.repos, repo));
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
    REPO_COUNT.fetch_sub(1, Ordering::Relaxed);
    Ok(HttpResponse::TemporaryRedirect()
        .header(
            LOCATION,
            format!("/view/{}/{}/{}", T::url_path(), data.0, data.1),
        )
        .finish())
}

async fn handle_hoc_request<T, F>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
    mapper: F,
) -> Result<HttpResponse>
where
    T: Service,
    F: Fn(HocResult) -> Result<HttpResponse>,
{
    let repo = format!("{}/{}", data.0.to_lowercase(), data.1.to_lowercase());
    let service_path = format!("{}/{}", T::url_path(), repo);
    let service_url = format!("{}/{}", T::domain(), repo);
    let path = format!("{}/{}", state.repos, service_url);
    let url = format!("https://{}", service_url);
    error!("{}", url);
    let remote_exists = remote_exists(&url).await?;
    let file = Path::new(&path);
    if !file.exists() {
        if !remote_exists {
            warn!("Repository does not exist: {}", url);
            return mapper(HocResult::NotFound);
        }
        info!("Cloning {} for the first time", url);
        create_dir_all(file)?;
        let repo = Repository::init_bare(file)?;
        repo.remote_add_fetch("origin", "refs/heads/*:refs/heads/*")?;
        repo.remote_set_url("origin", &url)?;
        REPO_COUNT.fetch_add(1, Ordering::Relaxed);
    }
    pull(&path)?;
    let (hoc, head, commits) = hoc(&service_url, &state.repos, &state.cache)?;
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
}

pub(crate) async fn json_hoc<T: Service>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse> {
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
    handle_hoc_request::<T, _>(state, data, mapper).await
}

pub(crate) async fn calculate_hoc<T: Service>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let mapper = move |r| match r {
        HocResult::NotFound => p404(),
        HocResult::Hoc { hoc_pretty, .. } => {
            let badge_opt = BadgeOptions {
                subject: "Hits-of-Code".to_string(),
                color: "#007ec6".to_string(),
                status: hoc_pretty,
            };
            let badge = Badge::new(badge_opt)?;
            // TODO: remove clone
            let body = badge.to_svg().as_bytes().to_vec();

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
                .body(body))
        }
    };
    handle_hoc_request::<T, _>(state, data, mapper).await
}

async fn overview<T: Service>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse> {
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
                repo_info,
            )?;

            Ok(HttpResponse::Ok().content_type("text/html").body(buf))
        }
    };
    handle_hoc_request::<T, _>(state, data, mapper).await
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
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
async fn generate(params: web::Form<GeneratorForm<'_>>) -> Result<HttpResponse> {
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

    Ok(HttpResponse::Ok().content_type("text/html").body(buf))
}

fn p404() -> Result<HttpResponse> {
    let mut buf = Vec::new();
    templates::p404(&mut buf, VERSION_INFO, REPO_COUNT.load(Ordering::Relaxed))?;
    Ok(HttpResponse::NotFound().content_type("text/html").body(buf))
}

async fn async_p404() -> Result<HttpResponse> {
    p404()
}

#[get("/tacit-css.min.css")]
fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(CSS)
}

#[get("/favicon.ico")]
fn favicon32() -> HttpResponse {
    HttpResponse::Ok().content_type("image/png").body(FAVICON)
}

async fn start_server() -> std::io::Result<()> {
    let interface = format!("{}:{}", OPT.host, OPT.port);
    let state = Arc::new(State {
        repos: OPT.outdir.display().to_string(),
        cache: OPT.cachedir.display().to_string(),
    });
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath)
            .service(index)
            .service(css)
            .service(favicon32)
            .service(generate)
            .service(web::resource("/github/{user}/{repo}").to(calculate_hoc::<GitHub>))
            .service(web::resource("/gitlab/{user}/{repo}").to(calculate_hoc::<Gitlab>))
            .service(web::resource("/bitbucket/{user}/{repo}").to(calculate_hoc::<Bitbucket>))
            .service(
                web::resource("/github/{user}/{repo}/delete")
                    .route(web::post().to(delete_repo_and_cache::<GitHub>)),
            )
            .service(
                web::resource("/gitlab/{user}/{repo}/delete")
                    .route(web::post().to(delete_repo_and_cache::<Gitlab>)),
            )
            .service(
                web::resource("/bitbucket/{user}/{repo}/delete")
                    .route(web::post().to(delete_repo_and_cache::<Bitbucket>)),
            )
            .service(web::resource("/github/{user}/{repo}/json").to(json_hoc::<GitHub>))
            .service(web::resource("/gitlab/{user}/{repo}/json").to(json_hoc::<Gitlab>))
            .service(web::resource("/bitbucket/{user}/{repo}/json").to(json_hoc::<Bitbucket>))
            .service(web::resource("/view/github/{user}/{repo}").to(overview::<GitHub>))
            .service(web::resource("/view/gitlab/{user}/{repo}").to(overview::<Gitlab>))
            .service(web::resource("/view/bitbucket/{user}/{repo}").to(overview::<Bitbucket>))
            .default_service(web::resource("").route(web::get().to(async_p404)))
    })
    .workers(OPT.workers)
    .bind(interface)?
    .run()
    .await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    config::init().await.unwrap();
    start_server().await
}
