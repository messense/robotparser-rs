mod path_pattern;
pub(crate) use self::path_pattern::PathPattern;
mod group;
pub(crate) use self::group::Group;
mod rule;
pub(crate) use self::rule::Rule;
mod clean_params;
pub(crate) use self::clean_params::CleanParams;
mod request_rate;
pub use self::request_rate::RequestRate;
mod robots_txt;
pub use self::fetched_robots_txt::FetchedRobotsTxt;
pub(crate) use self::fetched_robots_txt::FetchedRobotsTxtContainer;
mod fetched_robots_txt;
pub use self::robots_txt::RobotsTxt;
mod path;
pub(crate) use self::path::Path;
#[cfg(feature = "reqwest")]
mod errors;
#[cfg(feature = "reqwest")]
pub use self::errors::{Error, ErrorKind};
