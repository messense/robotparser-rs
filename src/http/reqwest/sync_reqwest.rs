use reqwest::blocking::{Client, Request};
use reqwest::Method;
use reqwest::header::HeaderValue;
use url::{Origin, Url};
use reqwest::header::USER_AGENT;
use crate::http::{RobotsTxtClient, DEFAULT_USER_AGENT};
use crate::parser::{ParseResult, parse_fetched_robots_txt};
use crate::model::FetchedRobotsTxt;
use crate::model::{Error, ErrorKind};

impl RobotsTxtClient for Client {
    type Result = Result<ParseResult<FetchedRobotsTxt>, Error>;
    fn fetch_robots_txt(&self, origin: Origin) -> Self::Result {
        let url = format!("{}/robots.txt", origin.unicode_serialization());
        let url = Url::parse(&url).map_err(|err| Error {kind: ErrorKind::Url(err)})?;
        let mut request = Request::new(Method::GET, url);
        let _ = request.headers_mut().insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        let response = self.execute(request).map_err(|err| Error {kind: ErrorKind::Http(err)})?;
        let status_code = response.status().as_u16();
        let text = response.text().map_err(|err| Error {kind: ErrorKind::Http(err)})?;
        let robots_txt = parse_fetched_robots_txt(origin, status_code, &text);
        return Ok(robots_txt);
    }
}
