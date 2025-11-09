use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Badge({0})")]
    Badge(String),
    #[error("Client({0})")]
    Client(#[from] reqwest::Error),
    #[error("Git({0})")]
    Git(#[from] git2::Error),
    #[error("Internal")]
    Internal,
    #[error("Io({0})")]
    Io(#[from] std::io::Error),
    #[error("Parse({0})")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serde({0})")]
    Serial(#[from] serde_json::Error),
    #[error("BranchNotFound")]
    BranchNotFound,
    #[error("UnknownPlatform({0})")]
    UnknownPlatform(String),
    #[error("Any({0})")]
    Any(#[from] anyhow::Error),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Badge(s)
    }
}
