use crate::{error::Result, statics::OPT};
use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
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
    #[structopt(
        short = "l",
        long = "logfile",
        parse(from_os_str),
        default_value = "./hoc.log"
    )]
    /// The logfile
    pub(crate) logfile: PathBuf,
    #[structopt(subcommand)]
    pub(crate) migrate: Option<Migration>,
}

#[derive(StructOpt, Debug, Clone, Copy)]
pub(crate) enum Migration {
    #[structopt(name = "migrate-commit-count")]
    CacheCommitCount,
}

pub(crate) fn init() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,hoc=info");
    // pretty_env_logger::init();
    openssl_probe::init_ssl_cert_env_vars();
    let stdout = ConsoleAppender::builder().build();
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build(&OPT.logfile)
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info),
        )?;
    log4rs::init_config(config)?;
    Ok(())
}
