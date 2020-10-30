use crate::error::Result;
use slog::{Drain, Logger};
use slog_atomic::AtomicSwitch;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub(crate) struct Opt {
    #[structopt(
        short = "o",
        long = "outdir",
        parse(from_os_str),
        default_value = "./repos"
    )]
    /// Path to store cloned repositories
    pub(crate) outdir: PathBuf,
    #[structopt(
        short = "c",
        long = "cachedir",
        parse(from_os_str),
        default_value = "./cache"
    )]
    /// Path to store cache
    pub(crate) cachedir: PathBuf,
    #[structopt(short = "p", long = "port", default_value = "8080")]
    /// Port to listen on
    pub(crate) port: u16,
    #[structopt(short = "h", long = "host", default_value = "0.0.0.0")]
    /// Interface to listen on
    pub(crate) host: String,
    #[structopt(short = "d", long = "domain", default_value = "hitsofcode.com")]
    /// Interface to listen on
    pub(crate) domain: String,
    #[structopt(short = "w", long = "workers", default_value = "4")]
    /// Number of worker threads
    pub(crate) workers: usize,
    // #[structopt(
    //     short = "l",
    //     long = "logfile",
    //     parse(from_os_str),
    //     default_value = "./hoc.log"
    // )]
    // /// The logfile
    // pub(crate) logfile: PathBuf,
}

pub(crate) fn init() -> Logger {
    std::env::set_var("RUST_LOG", "actix_web=info,hoc=info");
    openssl_probe::init_ssl_cert_env_vars();

    let decorator = slog_term::PlainDecorator::new(std::io::stdout());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = AtomicSwitch::new(drain);

    let root = Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));

    info!(root, "Logging initialized");

    root
}
