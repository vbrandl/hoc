#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod cache;
mod error;
mod service;

use crate::{
    cache::CacheState,
    error::Error,
    service::{Bitbucket, GitHub, Gitlab, Service},
};
use actix_web::{
    error::ErrorBadRequest,
    http::header::{CacheControl, CacheDirective, Expires},
    middleware, web, App, HttpResponse, HttpServer,
};
use badge::{Badge, BadgeOptions};
use bytes::Bytes;
use futures::{unsync::mpsc, Stream};
use git2::Repository;
use number_prefix::{NumberPrefix, Prefixed, Standalone};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    time::{Duration, SystemTime},
};
use structopt::StructOpt;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

pub struct VersionInfo<'a> {
    pub commit: &'a str,
    pub version: &'a str,
}

const VERSION_INFO: VersionInfo = VersionInfo {
    commit: env!("VERGEN_SHA_SHORT"),
    version: env!("CARGO_PKG_VERSION"),
};

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref OPT: Opt = Opt::from_args();
    static ref INDEX: Vec<u8> = {
        let mut buf = Vec::new();
        templates::index(&mut buf, VERSION_INFO, &OPT.domain).unwrap();
        buf
    };
    static ref P404: Vec<u8> = {
        let mut buf = Vec::new();
        templates::p404(&mut buf, VERSION_INFO).unwrap();
        buf
    };
    static ref P500: Vec<u8> = {
        let mut buf = Vec::new();
        templates::p500(&mut buf, VERSION_INFO).unwrap();
        buf
    };
}

struct State {
    repos: String,
    cache: String,
}

const CSS: &str = include_str!("../static/tacit-css.min.css");

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(
        short = "o",
        long = "outdir",
        parse(from_os_str),
        default_value = "./repos"
    )]
    /// Path to store cloned repositories
    outdir: PathBuf,
    #[structopt(
        short = "c",
        long = "cachedir",
        parse(from_os_str),
        default_value = "./cache"
    )]
    /// Path to store cache
    cachedir: PathBuf,
    #[structopt(short = "p", long = "port", default_value = "8080")]
    /// Port to listen on
    port: u16,
    #[structopt(short = "h", long = "host", default_value = "0.0.0.0")]
    /// Interface to listen on
    host: String,
    #[structopt(short = "d", long = "domain", default_value = "hitsofcode.com")]
    /// Interface to listen on
    domain: String,
}

fn pull(path: impl AsRef<Path>) -> Result<(), Error> {
    let repo = Repository::open_bare(path)?;
    let mut origin = repo.find_remote("origin")?;
    origin.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
    Ok(())
}

fn hoc(repo: &str, repo_dir: &str, cache_dir: &str) -> Result<(u64, String), Error> {
    let repo_dir = format!("{}/{}", repo_dir, repo);
    let cache_dir = format!("{}/{}.json", cache_dir, repo);
    let cache_dir = Path::new(&cache_dir);
    let head = format!(
        "{}",
        Repository::open_bare(&repo_dir)?
            .head()?
            .target()
            .ok_or(Error::Internal)?
    );
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
        CacheState::Current(res) => {
            info!("Using cache for {}", repo_dir);
            return Ok((*res, head));
        }
        CacheState::Old(cache) => {
            info!("Updating cache for {}", repo_dir);
            arg.push(format!("{}..HEAD", cache.head));
        }
        CacheState::No => {
            info!("Creating cache for {}", repo_dir);
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
    let count: u64 = output
        .lines()
        .map(|s| {
            s.split_whitespace()
                .take(2)
                .map(str::parse::<u64>)
                .filter_map(Result::ok)
                .sum::<u64>()
        })
        .sum();

    let cache = cache.calculate_new_cache(count, (&head).into());
    cache.write_to_file(cache_dir)?;

    Ok((cache.count, head))
}

fn remote_exists(url: &str) -> Result<bool, Error> {
    Ok(CLIENT.head(url).send()?.status() == reqwest::StatusCode::OK)
}

enum HocResult {
    Hoc {
        hoc: u64,
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
) -> Result<HttpResponse, Error>
where
    T: Service,
    F: Fn(HocResult) -> Result<HttpResponse, Error>,
{
    hoc_request::<T>(state, data).and_then(mapper)
}

fn hoc_request<T: Service>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<HocResult, Error> {
    let repo = format!("{}/{}", data.0, data.1);
    let service_path = format!("{}/{}", T::domain(), repo);
    let path = format!("{}/{}", state.repos, service_path);
    let file = Path::new(&path);
    let url = format!("https://{}", service_path);
    if !file.exists() {
        if !remote_exists(&url)? {
            warn!("Repository does not exist: {}", url);
            return Ok(HocResult::NotFound);
        }
        info!("Cloning {} for the first time", url);
        create_dir_all(file)?;
        let repo = Repository::init_bare(file)?;
        repo.remote_add_fetch("origin", "refs/heads/*:refs/heads/*")?;
        repo.remote_set_url("origin", &url)?;
    }
    pull(&path)?;
    let (hoc, head) = hoc(&service_path, &state.repos, &state.cache)?;
    let hoc_pretty = match NumberPrefix::decimal(hoc as f64) {
        Standalone(hoc) => hoc.to_string(),
        Prefixed(prefix, hoc) => format!("{:.1}{}", hoc, prefix),
    };
    Ok(HocResult::Hoc {
        hoc,
        hoc_pretty,
        head,
        url,
        repo,
        service_path,
    })
}

fn calculate_hoc<T: Service>(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let mapper = |r| match r {
        HocResult::NotFound => Ok(p404()),
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
) -> Result<HttpResponse, Error> {
    let mapper = |r| match r {
        HocResult::NotFound => Ok(p404()),
        HocResult::Hoc {
            hoc,
            hoc_pretty,
            url,
            head,
            repo,
            service_path,
        } => {
            let mut buf = Vec::new();
            templates::overview(
                &mut buf,
                VERSION_INFO,
                &OPT.domain,
                &service_path,
                &url,
                hoc,
                &hoc_pretty,
                &head,
                &T::commit_url(&repo, &head),
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
fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(INDEX.as_slice())
}

fn p404() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(P404.as_slice())
}

#[get("/tacit-css.min.css")]
fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(CSS)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,hoc=info");
    pretty_env_logger::init();
    openssl_probe::init_ssl_cert_env_vars();
    let interface = format!("{}:{}", OPT.host, OPT.port);
    let state = Arc::new(State {
        repos: OPT.outdir.display().to_string(),
        cache: OPT.cachedir.display().to_string(),
    });
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(css)
            .service(web::resource("/github/{user}/{repo}").to(calculate_hoc::<GitHub>))
            .service(web::resource("/gitlab/{user}/{repo}").to(calculate_hoc::<Gitlab>))
            .service(web::resource("/bitbucket/{user}/{repo}").to(calculate_hoc::<Bitbucket>))
            .service(web::resource("/view/github/{user}/{repo}").to(overview::<GitHub>))
            .service(web::resource("/view/gitlab/{user}/{repo}").to(overview::<Gitlab>))
            .service(web::resource("/view/bitbucket/{user}/{repo}").to(overview::<Bitbucket>))
            .default_service(web::resource("").route(web::get().to(p404)))
    })
    .bind(interface)?
    .run()
}
