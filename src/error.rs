use crate::{statics::VERSION_INFO, templates};

use std::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

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
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::BranchNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut buf = Vec::new();
        if let Error::BranchNotFound = self {
            templates::p404_no_master_html(&mut buf, VERSION_INFO, 0).unwrap();
            HttpResponse::NotFound().content_type("text/html").body(buf)
        } else {
            templates::p500_html(&mut buf, VERSION_INFO, 0).unwrap();
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(buf)
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
