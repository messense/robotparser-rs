# robotparser-rs

[![Build Status](https://travis-ci.org/messense/robotparser-rs.svg)](https://travis-ci.org/messense/robotparser-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/usb5fo89sxq5txk3/branch/master?svg=true)](https://ci.appveyor.com/project/messense/robotparser-rs/branch/master)
[![Coverage Status](https://coveralls.io/repos/messense/robotparser-rs/badge.svg)](https://coveralls.io/r/messense/robotparser-rs)
[![Crates.io](https://img.shields.io/crates/v/robotparser.svg)](https://crates.io/crates/robotparser)

robots.txt parser for Rust.


## Installation

Add it to your ``Cargo.toml``:

```toml
[dependencies]
robotparser = "0.7"
```

Add ``extern crate robotparser`` to your crate root and your're good to go!


## Examples

```rust
extern crate robotparser;

use robotparser::RobotFileParser;

fn main() {
    let parser = RobotFileParser::new("http://www.python.org/robots.txt");
    parser.read();
    assert!(parser.can_fetch("*", "http://www.python.org/robots.txt"));
}
```


## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
