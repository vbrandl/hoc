use config::{Config, ConfigError, Environment, File};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Path to store cloned repositories
    pub repodir: PathBuf,
    /// Path to store cache
    pub cachedir: PathBuf,
    /// Port to listen on
    pub port: u16,
    /// Interface to listen on
    pub host: String,
    /// Base URL
    pub base_url: String,
    /// Number of worker threads
    pub workers: usize,
}

pub(crate) fn init() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,hoc=info");
    openssl_probe::init_ssl_cert_env_vars();

    tracing_subscriber::fmt().init();
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config
            .merge(File::with_name("hoc.toml").required(false))?
            .merge(Environment::with_prefix("hoc"))?
            .set_default("repodir", "./repos")?
            .set_default("cachedir", "./cache")?
            .set_default("workers", 4)?
            .set_default("port", 8080)?
            .set_default("host", "0.0.0.0")?;
        config.try_into()
    }
}
