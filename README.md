# robotparser-rs

[![GitHub Actions](https://github.com/messense/robotparser-rs/workflows/CI/badge.svg)](https://github.com/messense/robotparser-rs/actions?query=workflow%3ACI)
[![Coverage Status](https://coveralls.io/repos/messense/robotparser-rs/badge.svg)](https://coveralls.io/r/messense/robotparser-rs)
[![Crates.io](https://img.shields.io/crates/v/robotparser.svg)](https://crates.io/crates/robotparser)
[![Dependency status](https://deps.rs/repo/github/messense/robotparser-rs/status.svg)](https://deps.rs/repo/github/messense/robotparser-rs)

robots.txt parser for Rust.


## Installation

Add it to your ``Cargo.toml``:

```toml
[dependencies]
robotparser = "0.11"
```


## Examples

```rust
use robotparser::http::RobotsTxtClient;
use robotparser::service::RobotsTxtService;
use reqwest::Client;
use url::Url;

fn main() {
    let client = Client::new();
    let robots_txt_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    let robots_txt = client.fetch_robots_txt(robots_txt_url.origin()).unwrap().get_result();
    let fetch_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    assert!(robots_txt.can_fetch("*", &fetch_url));
}
```


## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
