use std::fmt;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Badge(String),
    Client(reqwest::Error),
    Git(git2::Error),
    Internal,
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Serial(serde_json::Error),
    BranchNotFound,
    UnknownPlatform(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Badge(s) => write!(fmt, "Badge({s})"),
            Error::Client(e) => write!(fmt, "Client({e})"),
            Error::Git(e) => write!(fmt, "Git({e})"),
            Error::Internal => write!(fmt, "Internal Error"),
            Error::Io(e) => write!(fmt, "Io({e})"),
            Error::Parse(e) => write!(fmt, "Parse({e})"),
            Error::Serial(e) => write!(fmt, "Serial({e})"),
            Error::BranchNotFound => write!(fmt, "Repo doesn't have master branch"),
            Error::UnknownPlatform(s) => write!(fmt, "Unknown platfom: {s}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Badge(s)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::Git(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serial(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Client(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parse(err)
    }
}
