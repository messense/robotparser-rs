use std::fmt;

#[derive(Debug)]
pub struct RobotparserError {
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    Url(url::ParseError),
    HttpClient(reqwest::Error),
}

impl fmt::Display for RobotparserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Url(ref err) => err.fmt(f),
            ErrorKind::HttpClient(ref err) => err.fmt(f),
        }
    }
}
