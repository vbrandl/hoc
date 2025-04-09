use std::sync::LazyLock;

#[derive(Clone, Copy)]
pub struct VersionInfo<'a> {
    pub commit: &'a str,
    pub version: &'a str,
}

pub(crate) const VERSION_INFO: VersionInfo = VersionInfo {
    commit: env!("VERGEN_GIT_SHA"),
    version: env!("CARGO_PKG_VERSION"),
};

pub(crate) static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);
