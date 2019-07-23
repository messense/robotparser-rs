use url::Url;
use std::time::Duration;
use crate::service::RobotsTxtService;
use crate::model::{FetchedRobotsTxt, FetchedRobotsTxtContainer};
use crate::model::RequestRate;

impl RobotsTxtService for FetchedRobotsTxt {
    fn can_fetch(&self, user_agent: &str, url: &Url) -> bool {
        match self.get_container() {
            &FetchedRobotsTxtContainer::FetchDenied => false,
            &FetchedRobotsTxtContainer::FetchFailed => true,
            &FetchedRobotsTxtContainer::Fetched(ref robots_txt) => {
                robots_txt.can_fetch(user_agent, url)
            }
        }
    }

    fn get_crawl_delay(&self, user_agent: &str) -> Option<Duration> {
        if let &FetchedRobotsTxtContainer::Fetched(ref robots_txt) = self.get_container() {
            return robots_txt.get_crawl_delay(user_agent);
        }
        return None;
    }

    fn normalize_url(&self, url: &mut Url) -> bool {
        if let &FetchedRobotsTxtContainer::Fetched(ref robots_txt) = self.get_container() {
            return robots_txt.normalize_url(url);
        }
        return true;
    }

    fn normalize_url_ignore_origin(&self, url: &mut Url) {
        if let &FetchedRobotsTxtContainer::Fetched(ref robots_txt) = self.get_container() {
            robots_txt.normalize_url_ignore_origin(url);
        }
    }

    fn get_sitemaps(&self) -> &[Url] {
        if let &FetchedRobotsTxtContainer::Fetched(ref robots_txt) = self.get_container() {
            return robots_txt.get_sitemaps();
        }
        return &[];
    }

    fn get_req_rate(&self, user_agent: &str) -> Option<RequestRate> {
        if let &FetchedRobotsTxtContainer::Fetched(ref robots_txt) = self.get_container() {
            return robots_txt.get_req_rate(user_agent);
        }
        return None;
    }
}