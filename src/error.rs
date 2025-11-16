use thiserror::Error;
use tokio::task::JoinError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Badge({0})")]
    Badge(#[from] badgers::Error),
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
    #[error("Serial({0})")]
    Serial(#[from] serde_json::Error),
    #[error("BranchNotFound")]
    BranchNotFound,
    #[error("UnknownPlatform({0})")]
    UnknownPlatform(String),
    #[error("Join({0})")]
    Join(#[from] JoinError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
