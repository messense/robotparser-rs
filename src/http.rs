//! # Supported libraries
//! To enable support for the required library, you need to add this feature to your `Cargo.toml`.
//! Now only one library is supported - `reqwest`.
//! But you can also add support for other libraries.

use url::Origin;
#[cfg(feature = "reqwest")]
/// Support for reqwest library.
pub mod reqwest;

/// User agent of this crate.
pub const DEFAULT_USER_AGENT: &str = "robotparser-rs (https://crates.io/crates/robotparser)";

/// Trait to fetch and parse the robots.txt file.
/// Must be implemented on http-client.
pub trait RobotsTxtClient {
    type Result;
    fn fetch_robots_txt(&self, origin: Origin) -> Self::Result;
}
