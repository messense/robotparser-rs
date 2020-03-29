use crate::model::robots_txt::RobotsTxt;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub(crate) enum FetchedRobotsTxtContainer {
    FetchDenied,
    FetchFailed,
    Fetched(RobotsTxt),
}

#[derive(Debug, Clone)]
/// A model of the robots.txt file that was downloaded over the network.
/// This model takes into account HTTP response codes when loading the robots.txt file.
/// To work with this model you should use the trait `robotparser::service::RobotsTxtService`.
/// To create this structure you should use the `robotparser::parser::parse_fetched_robots_txt`.
pub struct FetchedRobotsTxt {
    fetched_at: SystemTime,
    container: FetchedRobotsTxtContainer,
}

impl FetchedRobotsTxt {
    pub(crate) fn new(container: FetchedRobotsTxtContainer) -> FetchedRobotsTxt {
        FetchedRobotsTxt {
            fetched_at: SystemTime::now(),
            container,
        }
    }
    pub(crate) fn get_container(&self) -> &FetchedRobotsTxtContainer {
        return &self.container;
    }

    /// Returns the system time when the robots.txt file was downloaded over the network.
    pub fn get_fetched_at(&self) -> &SystemTime {
        return &self.fetched_at;
    }
}
