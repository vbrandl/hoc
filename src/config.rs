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
}

pub(crate) fn init() {
    std::env::set_var("RUST_LOG", "actix_web=info,hoc=info");
    openssl_probe::init_ssl_cert_env_vars();

    tracing_subscriber::fmt().init();
}
