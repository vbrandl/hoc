pub struct VersionInfo<'a> {
    pub commit: &'a str,
    pub version: &'a str,
}

pub(crate) const VERSION_INFO: VersionInfo = VersionInfo {
    commit: env!("VERGEN_GIT_SHA"),
    version: env!("CARGO_PKG_VERSION"),
};

lazy_static! {
    pub(crate) static ref CLIENT: reqwest::Client = reqwest::Client::new();
}
