extern crate actix_web;
extern crate badge;
extern crate git2;
extern crate openssl_probe;
extern crate pretty_env_logger;
extern crate structopt;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, ResponseError};
use badge::{Badge, BadgeOptions};
use git2::{Repository, ResetType};
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};
use structopt::StructOpt;

type State = Arc<String>;

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
    Internal,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Git(e) => write!(fmt, "Git({})", e),
            Error::Io(e) => write!(fmt, "Io({})", e),
            Error::Badge(s) => write!(fmt, "Basge({})", s),
            Error::Internal => write!(fmt, "Internal"),
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
    let repo = Repository::open(path)?;
    let mut origin = repo.find_remote("origin")?;
    origin.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
    let head = repo.head()?.target().ok_or(Error::Internal)?;
    let obj = repo.find_object(head, None)?;
    Ok(repo.reset(&obj, ResetType::Hard, None)?)
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

fn github(
    state: web::Data<State>,
    data: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let path = format!("{}/github.com/{}/{}", *state, data.0, data.1);
    let file = Path::new(&path);
    if !file.exists() {
        Repository::clone(&format!("https://{}", path), file)?;
    }
    pull(&path)?;
    let hoc = hoc(&path)?;
    let badge_opt = BadgeOptions {
        subject: "Hits-of-Code".to_string(),
        color: "#44CC11".to_string(),
        status: hoc.to_string(),
    };
    let badge = Badge::new(badge_opt)?;
    Ok(HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(badge.to_svg()))
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
            .service(web::resource("/github/{user}/{repo}").to(github))
    })
    .bind(interface)?
    .run()
}
