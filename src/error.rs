use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub(crate) enum Error {
    Git(git2::Error),
    Io(std::io::Error),
    Badge(String),
    ParseColor,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Git(e) => write!(fmt, "Git({})", e),
            Error::Io(e) => write!(fmt, "Io({})", e),
            Error::Badge(s) => write!(fmt, "Badge({})", s),
            Error::ParseColor => write!(fmt, "Parse error"),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
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
