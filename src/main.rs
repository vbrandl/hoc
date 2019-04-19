#[macro_use]
extern crate actix_web;

use actix_web::{
    error,
    http::{
        self,
        header::{CacheControl, CacheDirective, Expires},
    },
    middleware, web, App, HttpResponse, HttpServer, ResponseError,
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

type State = Arc<String>;

const INDEX: &str = include_str!("../static/index.html");
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
    #[structopt(short = "p", long = "port", default_value = "8080")]
    /// Port to listen on
    port: u16,
    #[structopt(short = "h", long = "host", default_value = "0.0.0.0")]
    /// Interface to listen on
    host: String,
}

#[derive(Debug)]
enum Error {
    Git(git2::Error),
    Io(std::io::Error),
    Badge(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Git(e) => write!(fmt, "Git({})", e),
            Error::Io(e) => write!(fmt, "Io({})", e),
            Error::Badge(s) => write!(fmt, "Badge({})", s),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Badge(s)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::Git(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

fn pull(path: impl AsRef<Path>) -> Result<(), Error> {
    let repo = Repository::open_bare(path)?;
    let mut origin = repo.find_remote("origin")?;
    origin.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
    Ok(())
}

fn hoc(repo: &str) -> Result<u64, Error> {
    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=tformat:")
        .arg("--numstat")
        .arg("--ignore-space-change")
        .arg("--ignore-all-space")
        .arg("--ignore-submodules")
        .arg("--no-color")
        .arg("--find-copies-harder")
        .arg("-M")
        .arg("--diff-filter=ACDM")
        .arg("--")
        .arg(".")
        .current_dir(repo)
        .output()?
        .stdout;
    let output = String::from_utf8_lossy(&output);
    let res: u64 = output
        .lines()
        .map(|s| {
            s.split_whitespace()
                .take(2)
                .map(str::parse::<u64>)
                .filter_map(Result::ok)
                .sum::<u64>()
        })
        .sum();

    Ok(res)
}

fn calculate_hoc(
    service: &str,
    state: web::Data<State>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let service_path = format!("{}/{}/{}", service, data.0, data.1);
    let path = format!("{}/{}", *state, service_path);
    let file = Path::new(&path);
    if !file.exists() {
        create_dir_all(file)?;
        let repo = Repository::init_bare(file)?;
        repo.remote_add_fetch("origin", "refs/heads/*:refs/heads/*")?;
        repo.remote_set_url("origin", &format!("https://{}", service_path))?;
    }
    pull(&path)?;
    let hoc = hoc(&path)?;
    let badge_opt = BadgeOptions {
        subject: "Hits-of-Code".to_string(),
        color: "#44CC11".to_string(),
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
        .streaming(rx_body.map_err(|_| error::ErrorBadRequest("bad request"))))
}

fn github(
    state: web::Data<State>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    calculate_hoc("github.com", state, data)
}

fn gitlab(
    state: web::Data<State>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    calculate_hoc("gitlab.com", state, data)
}

fn bitbucket(
    state: web::Data<State>,
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
    let (tx, rx_body) = mpsc::unbounded();
    let _ = tx.unbounded_send(Bytes::from(INDEX.as_bytes()));

    HttpResponse::Ok()
        .content_type("text/html")
        .streaming(rx_body.map_err(|_| error::ErrorBadRequest("bad request")))
}

#[get("/tacit-css.min.css")]
fn css() -> HttpResponse {
    let (tx, rx_body) = mpsc::unbounded();
    let _ = tx.unbounded_send(Bytes::from(CSS.as_bytes()));

    HttpResponse::Ok()
        .content_type("text/css")
        .streaming(rx_body.map_err(|_| error::ErrorBadRequest("bad request")))
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();
    openssl_probe::init_ssl_cert_env_vars();
    let opt = Opt::from_args();
    let interface = format!("{}:{}", opt.host, opt.port);
    let state = Arc::new(opt.outdir.display().to_string());
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
    })
    .bind(interface)?
    .run()
}
