use crate::{
    statics::{REPO_COUNT, VERSION_INFO},
    templates,
};
use actix_web::{HttpResponse, ResponseError};
use std::{fmt, sync::atomic::Ordering};

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Badge(String),
    Client(reqwest::Error),
    Git(git2::Error),
    Internal,
    Io(std::io::Error),
    Log(log::SetLoggerError),
    LogBuilder(log4rs::config::Errors),
    Parse(std::num::ParseIntError),
    Serial(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Badge(s) => write!(fmt, "Badge({})", s),
            Error::Client(e) => write!(fmt, "Client({})", e),
            Error::Git(e) => write!(fmt, "Git({})", e),
            Error::Internal => write!(fmt, "Internal Error"),
            Error::Io(e) => write!(fmt, "Io({})", e),
            Error::Log(e) => write!(fmt, "Log({})", e),
            Error::LogBuilder(e) => write!(fmt, "LogBuilder({})", e),
            Error::Parse(e) => write!(fmt, "Parse({})", e),
            Error::Serial(e) => write!(fmt, "Serial({})", e),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let mut buf = Vec::new();
        templates::p500(&mut buf, VERSION_INFO, REPO_COUNT.load(Ordering::Relaxed)).unwrap();
        HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(buf)
    }

    fn render_response(&self) -> HttpResponse {
        self.error_response()
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

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error::Log(err)
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

impl From<log4rs::config::Errors> for Error {
    fn from(err: log4rs::config::Errors) -> Self {
        Error::LogBuilder(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parse(err)
    }
}
