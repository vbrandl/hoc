use crate::{config::Opt, templates};
use structopt::StructOpt;

pub struct VersionInfo<'a> {
    pub commit: &'a str,
    pub version: &'a str,
}

pub(crate) const VERSION_INFO: VersionInfo = VersionInfo {
    commit: env!("VERGEN_SHA_SHORT"),
    version: env!("CARGO_PKG_VERSION"),
};
pub(crate) const CSS: &str = include_str!("../static/tacit-css.min.css");
pub(crate) const FAVICON: &[u8] = include_bytes!("../static/favicon32.png");

lazy_static! {
    pub(crate) static ref CLIENT: reqwest::Client = reqwest::Client::new();
    pub(crate) static ref OPT: Opt = Opt::from_args();
    pub(crate) static ref INDEX: Vec<u8> = {
        let mut buf = Vec::new();
        templates::index(&mut buf, VERSION_INFO, &OPT.domain).unwrap();
        buf
    };
    pub(crate) static ref P404: Vec<u8> = {
        let mut buf = Vec::new();
        templates::p404(&mut buf, VERSION_INFO).unwrap();
        buf
    };
    pub(crate) static ref P500: Vec<u8> = {
        let mut buf = Vec::new();
        templates::p500(&mut buf, VERSION_INFO).unwrap();
        buf
    };
}
