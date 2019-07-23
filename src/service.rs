mod robots_txt;
mod fetched_robots_txt;
use url::Url;
use std::time::Duration;
use crate::model::RequestRate;

/// Trait that implements robots txt service.
pub trait RobotsTxtService {
    /// Using the parsed robots.txt decide if useragent can fetch url.
    fn can_fetch(&self, user_agent: &str, url: &Url) -> bool;

    /// Returns the crawl delay for this user agent as a Duration, or None if no crawl delay is defined.
    fn get_crawl_delay(&self, user_agent: &str) -> Option<Duration>;

    /// Removes the request parameters from the url that were listed in the `Clean-param` directive.
    /// This method CHECKS that the origin of the transmitted url matches the origin of robots.txt.
    /// Returns true if the operation was applied to the passed url.
    /// In other cases it returns false.
    fn normalize_url(&self, url: &mut Url) -> bool;

    /// Removes the request parameters from the url that were listed in the `Clean-param` directive.
    /// This method DOES NOT CHECK that the origin of the transmitted url coincides with the origin of robots.txt.
    fn normalize_url_ignore_origin(&self, url: &mut Url);

    /// Returns the list of URL sitemaps that have been listed in the robots.txt file.
    fn get_sitemaps(&self) -> &[Url];

    /// Returns information about the restrictions set for sending HTTP requests to the server.
    fn get_req_rate(&self, user_agent: &str) -> Option<RequestRate>;
}