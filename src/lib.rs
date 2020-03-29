//! robots.txt parser for Rust
//!
//! The robots.txt Exclusion Protocol is implemented as specified in
//! <https://www.robotstxt.org/norobots-rfc.txt>
//!
//! # Installation
//!
//! Add it to your ``Cargo.toml``:
//!
//! ```toml
//! [dependencies]
//! robotparser = "0.11"
//! ```
//!
//!
//! # Examples
//!
//! ```rust
//! use robotparser::http::RobotsTxtClient;
//! use robotparser::service::RobotsTxtService;
//! use reqwest::blocking::Client;
//! use url::Url;
//!
//! fn main() {
//!     let client = Client::new();
//!     let robots_txt_url = Url::parse("https://www.python.org/robots.txt").unwrap();
//!     let robots_txt = client.fetch_robots_txt(robots_txt_url.origin()).unwrap().get_result();
//!     let fetch_url = Url::parse("https://www.python.org/robots.txt").unwrap();
//!     assert!(robots_txt.can_fetch("*", &fetch_url));
//! }
//! ```

/// Request builder & response parsers for other http libraries.
pub mod http;
/// Contains models of robots.txt file.
pub mod model;
/// Contains robots.txt parsers.
pub mod parser;
/// Contains robots.txt services.
pub mod service;
