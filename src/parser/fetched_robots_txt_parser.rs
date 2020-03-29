use crate::model::{FetchedRobotsTxt, FetchedRobotsTxtContainer};
use crate::parser::parse_robots_txt;
use crate::parser::ParseResult;
use url::Origin;

const UNAUTHORIZED: u16 = 401;
const FORBIDDEN: u16 = 403;
const OK: u16 = 200;

/// Parses the text of the robots.txt file located in the specified place of origin,
/// taking into account the response status code of the HTTP-request.
/// **IMPORTANT NOTE**: origin must point to robots.txt url **before redirects**.
pub fn parse(origin: Origin, status_code: u16, input: &str) -> ParseResult<FetchedRobotsTxt> {
    match status_code {
        UNAUTHORIZED | FORBIDDEN => {
            ParseResult::new(FetchedRobotsTxt::new(FetchedRobotsTxtContainer::FetchDenied))
        }
        OK => {
            parse_robots_txt(origin, input).map(|result| {
                FetchedRobotsTxt::new(FetchedRobotsTxtContainer::Fetched(result))
            })
        }
        _ => {
            ParseResult::new(FetchedRobotsTxt::new(FetchedRobotsTxtContainer::FetchFailed))
        }
    }
}
