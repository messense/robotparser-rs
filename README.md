# robotparser-rs

[![Build Status](https://travis-ci.org/messense/robotparser-rs.svg)](https://travis-ci.org/messense/robotparser-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/usb5fo89sxq5txk3/branch/master?svg=true)](https://ci.appveyor.com/project/messense/robotparser-rs/branch/master)
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

Add ``extern crate robotparser`` to your crate root and your're good to go!


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
