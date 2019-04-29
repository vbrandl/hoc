#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod cache;
mod error;

use crate::{cache::CacheState, error::Error};
use actix_web::{
    error::ErrorBadRequest,
    http::{
        self,
        header::{CacheControl, CacheDirective, Expires},
    },
    middleware, web, App, HttpResponse, HttpServer,
};
use badge::{Badge, BadgeOptions};
use bytes::Bytes;
use futures::{unsync::mpsc, Stream};
use git2::Repository;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    time::{Duration, SystemTime},
};
use structopt::StructOpt;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

const COMMIT: &str = env!("VERGEN_SHA_SHORT");
const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref OPT: Opt = Opt::from_args();
    static ref INDEX: Vec<u8> = {
        let mut buf = Vec::new();
        templates::index(&mut buf, COMMIT, VERSION, &OPT.domain).unwrap();
        buf
    };
    static ref P404: Vec<u8> = {
        let mut buf = Vec::new();
        templates::p404(&mut buf, COMMIT, VERSION).unwrap();
        buf
    };
    static ref P500: Vec<u8> = {
        let mut buf = Vec::new();
        templates::p500(&mut buf, COMMIT, VERSION).unwrap();
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
        CacheState::Current(res) => return Ok((*res, head)),
        CacheState::Old(cache) => {
            arg.push(format!("{}..HEAD", cache.head));
        }
        CacheState::No => {}
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

fn calculate_hoc(
    service: &str,
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let service_path = format!("{}/{}/{}", service, data.0, data.1);
    let path = format!("{}/{}", state.repos, service_path);
    let file = Path::new(&path);
    if !file.exists() {
        let url = format!("https://{}", service_path);
        if !remote_exists(&url)? {
            return Ok(p404());
        }
        create_dir_all(file)?;
        let repo = Repository::init_bare(file)?;
        repo.remote_add_fetch("origin", "refs/heads/*:refs/heads/*")?;
        repo.remote_set_url("origin", &url)?;
    }
    pull(&path)?;
    let (hoc, _) = hoc(&service_path, &state.repos, &state.cache)?;
    let badge_opt = BadgeOptions {
        subject: "Hits-of-Code".to_string(),
        color: "#007ec6".to_string(),
        status: hoc.to_string(),
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

fn github(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    calculate_hoc("github.com", state, data)
}

fn gitlab(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    calculate_hoc("gitlab.com", state, data)
}

fn bitbucket(
    state: web::Data<Arc<State>>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    calculate_hoc("bitbucket.org", state, data)
}

fn overview(_: web::Path<(String, String)>) -> HttpResponse {
    HttpResponse::TemporaryRedirect()
        .header(http::header::LOCATION, "/")
        .finish()
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
    std::env::set_var("RUST_LOG", "actix_web=warn");
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
            .service(web::resource("/github/{user}/{repo}").to(github))
            .service(web::resource("/gitlab/{user}/{repo}").to(gitlab))
            .service(web::resource("/bitbucket/{user}/{repo}").to(bitbucket))
            .service(web::resource("/view/github/{user}/{repo}").to(overview))
            .service(web::resource("/view/gitlab/{user}/{repo}").to(overview))
            .service(web::resource("/view/github/{user}/{repo}").to(overview))
            .default_service(web::resource("").route(web::get().to(p404)))
    })
    .bind(interface)?
    .run()
}
