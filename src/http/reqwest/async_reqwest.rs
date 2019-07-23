use reqwest::r#async::{Client, Request};
use reqwest::{Method, Error};
use reqwest::header::HeaderValue;
use url::{Origin, Url};
use reqwest::header::USER_AGENT;
use crate::http::{RobotsTxtClient, DEFAULT_USER_AGENT};
use crate::parser::{ParseResult, parse_fetched_robots_txt};
use crate::model::FetchedRobotsTxt;
use futures::{Future, Async};
use futures::future::ok as future_ok;

type FetchFuture = Box<dyn Future<Item=(ResponseInfo, String), Error=Error>>;

impl RobotsTxtClient for Client {
    type Result = RobotsTxtResponse;
    fn fetch_robots_txt(&self, origin: Origin) -> Self::Result {
        let url = format!("{}/robots.txt", origin.unicode_serialization());
        let url = Url::parse(&url).expect("Unable to parse robots.txt url");
        let mut request = Request::new(Method::GET, url);
        let _ = request.headers_mut().insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        let response = self
            .execute(request)
            .and_then(|mut response| {
                let response_info = ResponseInfo {status_code: response.status().as_u16()};
                return future_ok(response_info).join(response.text());
            });
        let response: FetchFuture = Box::new(response);
        return RobotsTxtResponse {
            origin,
            response,
        }
    }
}

struct ResponseInfo {
    status_code: u16,
}

/// Future for fetching robots.txt result.
pub struct RobotsTxtResponse {
    origin: Origin,
    response: FetchFuture,
}

impl RobotsTxtResponse {
    /// Returns origin of robots.txt
    pub fn get_origin(&self) -> &Origin {
        return &self.origin;
    }
}

impl Future for RobotsTxtResponse {
    type Item = ParseResult<FetchedRobotsTxt>;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.response.poll()? {
            Async::Ready((response_info, text)) => {
                let robots_txt = parse_fetched_robots_txt(self.origin.clone(), response_info.status_code, &text);
                return Ok(Async::Ready(robots_txt));
            },
            Async::NotReady => {
                return Ok(Async::NotReady);
            },
        }
    }
}