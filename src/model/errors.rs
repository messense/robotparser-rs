use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    Url(url::ParseError),
    Http(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Url(ref err) => err.fmt(f),
            ErrorKind::Http(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
