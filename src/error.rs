use crate::P500;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub(crate) enum Error {
    Badge(String),
    Git(git2::Error),
    Internal,
    Io(std::io::Error),
    Serial(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // write!(fmt, "{}", P500)
        match self {
            Error::Badge(s) => write!(fmt, "Badge({})", s),
            Error::Git(e) => write!(fmt, "Git({})", e),
            Error::Internal => write!(fmt, "Internal Error"),
            Error::Io(e) => write!(fmt, "Io({})", e),
            Error::Serial(e) => write!(fmt, "Serial({})", e),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(P500)
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
