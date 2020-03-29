//! # Supported features and directives
//!
//! * Removes BOM unicode
//! * Directive `User-Agent`
//! * Directive `Allow`
//! * Directive `Disallow`
//! * Directive `Crawl-Delay`
//! * Directive `Request-Rate`
//! * Directive `Sitemap`
//! * Directive `Clean-Param`
//!
//! # Example
//! ```rust
//! use robotparser::parser::parse_robots_txt;
//! use robotparser::service::RobotsTxtService;
//! use url::Url;
//!
//! fn main() {
//!     let robots_txt_url = Url::parse("https://google.com/robots.txt").unwrap();
//!     let robots_txt = "User-agent: *\nDisallow: /search";
//!     let robots_txt = parse_robots_txt(robots_txt_url.origin(), robots_txt);
//!     assert_eq!(robots_txt.get_warnings().len(), 0);
//!     let robots_txt = robots_txt.get_result();
//!     let good_url = Url::parse("https://google.com/test").unwrap();
//!     let bad_url = Url::parse("https://google.com/search/vvv").unwrap();
//!     assert_eq!(robots_txt.can_fetch("*", &bad_url), false);
//!     assert_eq!(robots_txt.can_fetch("*", &good_url), true);
//! }
//! ```
mod robots_txt_parser;
pub use self::robots_txt_parser::parse as parse_robots_txt;
mod warning_reason;
pub use self::warning_reason::WarningReason;
mod warning;
pub use self::warning::ParseWarning;
mod parse_result;
pub use self::parse_result::ParseResult;
mod fetched_robots_txt_parser;
pub use self::fetched_robots_txt_parser::parse as parse_fetched_robots_txt;
mod line;
