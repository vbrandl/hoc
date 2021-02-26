use crate::{config::Settings, count::count_repositories};
use std::sync::atomic::AtomicUsize;

pub struct VersionInfo<'a> {
    pub commit: &'a str,
    pub version: &'a str,
}

pub(crate) const VERSION_INFO: VersionInfo = VersionInfo {
    commit: env!("VERGEN_GIT_SHA_SHORT"),
    version: env!("CARGO_PKG_VERSION"),
};
pub(crate) const CSS: &str = include_str!("../static/tacit-css.min.css");
pub(crate) const FAVICON: &[u8] = include_bytes!("../static/favicon32.png");

lazy_static! {
    pub(crate) static ref CLIENT: reqwest::Client = reqwest::Client::new();
    pub(crate) static ref OPT: Settings = Settings::new().unwrap();
    pub(crate) static ref REPO_COUNT: AtomicUsize =
        AtomicUsize::new(count_repositories(&OPT.repodir).unwrap());
}
