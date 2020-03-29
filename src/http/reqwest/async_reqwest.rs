use crate::http::{RobotsTxtClient, DEFAULT_USER_AGENT};
use crate::model::FetchedRobotsTxt;
use crate::model::{Error, ErrorKind};
use crate::parser::{parse_fetched_robots_txt, ParseResult};
use futures::future::ok as future_ok;
use futures::future::TryFutureExt;
use futures::task::{Context, Poll};
use futures::Future;
use reqwest::header::HeaderValue;
use reqwest::header::USER_AGENT;
use reqwest::Error as ReqwestError;
use reqwest::Method;
use reqwest::{Client, Request};
use std::pin::Pin;
use url::{Origin, Url};

type FetchFuture = Box<dyn Future<Output = Result<(ResponseInfo, String), ReqwestError>>>;

impl RobotsTxtClient for Client {
    type Result = Result<RobotsTxtResponse, Error>;
    fn fetch_robots_txt(&self, origin: Origin) -> Self::Result {
        let url = format!("{}/robots.txt", origin.unicode_serialization());
        let url = Url::parse(&url).map_err(|err| Error {
            kind: ErrorKind::Url(err),
        })?;
        let mut request = Request::new(Method::GET, url);
        let _ = request
            .headers_mut()
            .insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        let response = self.execute(request).and_then(|response| {
            let response_info = ResponseInfo {
                status_code: response.status().as_u16(),
            };
            response
                .text()
                .and_then(|response_text| future_ok((response_info, response_text)))
        });
        let response: Pin<Box<dyn Future<Output = Result<(ResponseInfo, String), ReqwestError>>>> = Box::pin(response);
        Ok(RobotsTxtResponse { origin, response })
    }
}

struct ResponseInfo {
    status_code: u16,
}

/// Future for fetching robots.txt result.
pub struct RobotsTxtResponse {
    origin: Origin,
    response: Pin<FetchFuture>,
}

impl RobotsTxtResponse {
    /// Returns origin of robots.txt
    pub fn get_origin(&self) -> &Origin {
        &self.origin
    }
}

impl Future for RobotsTxtResponse {
    type Output = Result<ParseResult<FetchedRobotsTxt>, ReqwestError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let self_mut = self.get_mut();
        let response_pin = self_mut.response.as_mut();
        match response_pin.poll(cx) {
            Poll::Ready(Ok((response_info, text))) => {
                let robots_txt = parse_fetched_robots_txt(self_mut.origin.clone(), response_info.status_code, &text);
                Poll::Ready(Ok(robots_txt))
            }
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Pending => Poll::Pending,
        }
    }
}
