//! robots.txt parser for Rust
//!
//! The robots.txt Exclusion Protocol is implemented as specified in
//! http://www.robotstxt.org/norobots-rfc.txt
//!
//! # Installation
//!
//! Add it to your ``Cargo.toml``:
//!
//! ```toml
//! [dependencies]
//! robotparser = "0.6"
//! ```
//!
//! Add ``extern crate robotparser`` to your crate root and your're good to go!
//!
//! # Examples
//!
//! ```
//! extern crate robotparser;
//!
//! use robotparser::RobotFileParser;
//!
//! fn main() {
//!     let parser = RobotFileParser::new("http://www.python.org/robots.txt");
//!     parser.read();
//!     assert!(parser.can_fetch("*", "http://www.python.org/robots.txt"));
//! }
//! ```

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", warn(cyclomatic_complexity))]

extern crate url;
extern crate hyper;

use std::io::Read;
use std::cell::{Cell, RefCell};
use std::borrow::Cow;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use url::Url;
use hyper::Client;
use hyper::header::UserAgent;
use hyper::status::StatusCode;
use hyper::client::Response;

const USER_AGENT: &'static str = "robotparser-rs (https://crates.io/crates/robotparser)";

/// A rule line is a single "Allow:" (allowance==True) or "Disallow:"
/// (allowance==False) followed by a path."""
#[derive(Debug, Eq, PartialEq, Clone)]
struct RuleLine<'a> {
    path: Cow<'a, str>,
    allowance: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RequestRate {
    pub requests: usize,
    pub seconds: usize,
}

/// An entry has one or more user-agents and zero or more rulelines
#[derive(Debug, Eq, PartialEq, Clone)]
struct Entry<'a> {
    useragents: RefCell<Vec<String>>,
    rulelines: RefCell<Vec<RuleLine<'a>>>,
    crawl_delay: Option<Duration>,
    sitemaps: Vec<Url>,
    req_rate: Option<RequestRate>,
}

/// robots.txt file parser
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RobotFileParser<'a> {
    entries: RefCell<Vec<Entry<'a>>>,
    default_entry: RefCell<Entry<'a>>,
    disallow_all: Cell<bool>,
    allow_all: Cell<bool>,
    url: Url,
    host: String,
    path: String,
    last_checked: Cell<i64>,
}


impl<'a> RuleLine<'a> {
    fn new<S>(path: S, allowance: bool) -> RuleLine<'a>
        where S: Into<Cow<'a, str>>
    {
        let path = path.into();
        let mut allow = allowance;
        if path == "" && !allowance {
            // an empty value means allow all
            allow = true;
        }
        RuleLine {
            path: path,
            allowance: allow,
        }
    }

    fn applies_to(&self, filename: &str) -> bool {
        self.path == "*" || filename.starts_with(&self.path[..])
    }
}


impl<'a> Entry<'a> {
    fn new() -> Entry<'a> {
        Entry {
            useragents: RefCell::new(vec![]),
            rulelines: RefCell::new(vec![]),
            crawl_delay: None,
            sitemaps: Vec::new(),
            req_rate: None,
        }
    }

    /// check if this entry applies to the specified agent
    fn applies_to(&self, useragent: &str) -> bool {
        let ua = useragent.split('/').nth(0).unwrap_or("").to_lowercase();
        let useragents = self.useragents.borrow();
        for agent in &*useragents {
            if agent == "*" {
                return true;
            }
            if ua.contains(agent) {
                return true;
            }
        }
        false
    }


    /// Preconditions:
    /// - our agent applies to this entry
    /// - filename is URL decoded
    fn allowance(&self, filename: &str) -> bool {
        let rulelines = self.rulelines.borrow();
        for line in &*rulelines {
            if line.applies_to(filename) {
                return line.allowance;
            }
        }
        true
    }

    fn push_useragent(&self, useragent: &str) {
        let mut useragents = self.useragents.borrow_mut();
        useragents.push(useragent.to_lowercase().to_owned());
    }

    fn push_ruleline(&self, ruleline: RuleLine<'a>) {
        let mut rulelines = self.rulelines.borrow_mut();
        rulelines.push(ruleline);
    }

    fn has_useragent(&self, useragent: &str) -> bool {
        let useragents = self.useragents.borrow();
        useragents.contains(&useragent.to_owned())
    }

    fn is_empty(&self) -> bool {
        let useragents = self.useragents.borrow();
        let rulelines = self.rulelines.borrow();
        useragents.is_empty() && rulelines.is_empty()
    }

    fn set_crawl_delay(&mut self, delay: Duration) {
        self.crawl_delay = Some(delay);
    }

    fn get_crawl_delay(&self) -> Option<Duration> {
        self.crawl_delay
    }

    fn add_sitemap(&mut self, url: &str) {
        if let Ok(url) = Url::parse(url) {
            self.sitemaps.push(url);
        }
    }

    fn get_sitemaps(&self) -> Vec<Url> {
        self.sitemaps.clone()
    }

    fn set_req_rate(&mut self, req_rate: RequestRate) {
        self.req_rate = Some(req_rate);
    }

    fn get_req_rate(&self) -> Option<RequestRate> {
        self.req_rate.clone()
    }
}


impl<'a> Default for Entry<'a> {
    fn default() -> Entry<'a> {
        Entry::new()
    }
}


impl<'a> RobotFileParser<'a> {
    pub fn new<T: AsRef<str>>(url: T) -> RobotFileParser<'a> {
        let parsed_url = Url::parse(url.as_ref()).unwrap();
        RobotFileParser {
            entries: RefCell::new(vec![]),
            default_entry: RefCell::new(Entry::new()),
            disallow_all: Cell::new(false),
            allow_all: Cell::new(false),
            url: parsed_url.clone(),
            host: parsed_url.domain().unwrap().to_owned(),
            path: parsed_url.path().to_owned(),
            last_checked: Cell::new(0i64),
        }
    }

    /// Returns the time the robots.txt file was last fetched.
    ///
    /// This is useful for long-running web spiders that need to
    /// check for new robots.txt files periodically.
    pub fn mtime(&self) -> i64 {
        self.last_checked.get()
    }

    /// Sets the time the robots.txt file was last fetched to the
    /// current time.
    pub fn modified(&self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        self.last_checked.set(now);
    }

    /// Sets the URL referring to a robots.txt file.
    pub fn set_url<T: AsRef<str>>(&mut self, url: T) {
        let parsed_url = Url::parse(url.as_ref()).unwrap();
        self.url = parsed_url.clone();
        self.host = parsed_url.domain().unwrap().to_owned();
        self.path = parsed_url.path().to_owned();
        self.last_checked.set(0i64);
    }

    /// Reads the robots.txt URL and feeds it to the parser.
    pub fn read(&self) {
        let client = Client::new();
        let request = client.get(self.url.clone())
            .header(UserAgent(USER_AGENT.to_owned()));
        let mut res = match request.send() {
            Ok(res) => res,
            Err(_) => {
                return;
            }
        };
        match res.status {
            StatusCode::Unauthorized | StatusCode::Forbidden => {
                self.disallow_all.set(true);
            }
            status if status >= StatusCode::BadRequest && status < StatusCode::InternalServerError => {
                self.allow_all.set(true);
            }
            StatusCode::Ok => self.from_response(&mut res),
            _ => {}
        }
    }

    /// Reads the HTTP response and feeds it to the parser.
    pub fn from_response(&self, response: &mut Response) {
        let mut buf = String::new();
        response.read_to_string(&mut buf).unwrap();
        let lines: Vec<&str> = buf.split('\n').collect();
        self.parse(&lines);
    }

    fn _add_entry(&self, entry: Entry<'a>) {
        if entry.has_useragent("*") {
            // the default entry is considered last
            let mut default_entry = self.default_entry.borrow_mut();
            if default_entry.is_empty() {
                // the first default entry wins
                *default_entry = entry;
            }
        } else {
            let mut entries = self.entries.borrow_mut();
            entries.push(entry);
        }
    }

    ///
    /// Parse the input lines from a robots.txt file
    ///
    /// We allow that a user-agent: line is not preceded by
    /// one or more blank lines.
    ///
    pub fn parse<T: AsRef<str>>(&self, lines: &[T]) {
        use url::percent_encoding::percent_decode;

        // states:
        //   0: start state
        //   1: saw user-agent line
        //   2: saw an allow or disallow line
        let mut state = 0;
        let mut entry = Entry::new();

        self.modified();
        for line in lines {
            let mut ln = line.as_ref();
            if ln.is_empty() {
                match state {
                    1 => {
                        entry = Entry::new();
                        state = 0;
                    }
                    2 => {
                        self._add_entry(entry);
                        entry = Entry::new();
                        state = 0;
                    }
                    _ => {}
                }
            }
            // remove optional comment and strip line
            if let Some(i) = ln.find('#') {
                ln = &ln[0..i];
            }
            ln = ln.trim();
            if ln.is_empty() {
                continue;
            }
            let parts: Vec<&str> = ln.splitn(2, ':').collect();
            if parts.len() == 2 {
                let part0 = parts[0].trim().to_lowercase();
                let part1 = String::from_utf8(percent_decode(parts[1].trim().as_bytes()).collect())
                    .unwrap_or("".to_owned());
                match part0 {
                    ref x if x == "user-agent" => {
                        if state == 2 {
                            self._add_entry(entry);
                            entry = Entry::new();
                        }
                        entry.push_useragent(&part1);
                        state = 1;
                    }
                    ref x if x == "disallow" => {
                        if state != 0 {
                            entry.push_ruleline(RuleLine::new(part1, false));
                            state = 2;
                        }
                    }
                    ref x if x == "allow" => {
                        if state != 0 {
                            entry.push_ruleline(RuleLine::new(part1, true));
                            state = 2;
                        }
                    }
                    ref x if x == "crawl-delay" => {
                        if state != 0 {
                            if let Ok(delay) = part1.parse::<f64>() {
                                let delay_seconds = delay.trunc();
                                let delay_nanoseconds = delay.fract() * 10f64.powi(9);
                                let delay = Duration::new(delay_seconds as u64, delay_nanoseconds as u32);
                                entry.set_crawl_delay(delay);
                            }
                            state = 2;
                        }
                    }
                    ref x if x == "sitemap" => {
                        if state != 0 {
                            entry.add_sitemap(&part1);
                            state = 2;
                        }
                    }
                    ref x if x == "request-rate" => {
                        if state != 0 {
                            let numbers: Vec<Result<usize, _>> = part1.split('/').map(|x| x.parse::<usize>()).collect();
                            if numbers.len() == 2 && numbers[0].is_ok() && numbers[1].is_ok() {
                                let req_rate = RequestRate {
                                    requests: numbers[0].clone().unwrap(),
                                    seconds: numbers[1].clone().unwrap(),
                                };
                                entry.set_req_rate(req_rate);
                            }
                            state = 2;
                        }
                    }
                    _ => {}
                }
            }
        }
        if state == 2 {
            self._add_entry(entry);
        }
    }

    /// Using the parsed robots.txt decide if useragent can fetch url
    pub fn can_fetch<T: AsRef<str>>(&self, useragent: T, url: T) -> bool {
        use url::percent_encoding::percent_decode;

        let useragent = useragent.as_ref();
        let url = url.as_ref();

        if self.disallow_all.get() {
            return false;
        }
        if self.allow_all.get() {
            return true;
        }
        // Until the robots.txt file has been read or found not
        // to exist, we must assume that no url is allowable.
        // This prevents false positives when a user erronenously
        // calls can_fetch() before calling read().
        if self.last_checked.get() == 0 {
            return false;
        }
        // search for given user agent matches
        // the first match counts
        let decoded_url = String::from_utf8(percent_decode(url.trim().as_bytes()).collect()).unwrap_or("".to_owned());
        let url_str = match decoded_url {
            ref u if !u.is_empty() => u.to_owned(),
            _ => "/".to_owned(),
        };
        let entries = self.entries.borrow();
        for entry in &*entries {
            if entry.applies_to(useragent) {
                return entry.allowance(&url_str);
            }
        }
        // try the default entry last
        let default_entry = self.default_entry.borrow();
        if !default_entry.is_empty() {
            return default_entry.allowance(&url_str);
        }
        // agent not found ==> access granted
        true
    }

    /// Returns the crawl delay for this user agent as a `Duration`, or None if no crawl delay is defined.
    pub fn get_crawl_delay<T: AsRef<str>>(&self, useragent: T) -> Option<Duration> {
        let useragent = useragent.as_ref();
        if self.last_checked.get() == 0 {
            return None;
        }
        let entries = self.entries.borrow();
        for entry in &*entries {
            if entry.applies_to(useragent) {
                return entry.get_crawl_delay();
            }
        }
        None
    }

    /// Returns the sitemaps for this user agent as a `Vec<Url>`.
    pub fn get_sitemaps<T: AsRef<str>>(&self, useragent: T) -> Vec<Url> {
        let useragent = useragent.as_ref();
        if self.last_checked.get() == 0 {
            return Vec::new();
        }
        let entries = self.entries.borrow();
        for entry in &*entries {
            if entry.applies_to(useragent) {
                return entry.get_sitemaps();
            }
        }
        vec![]
    }

    /// Returns the request rate for this user agent as a `RequestRate`, or None if not request rate is defined
    pub fn get_req_rate<T: AsRef<str>>(&self, useragent: T) -> Option<RequestRate> {
        let useragent = useragent.as_ref();
        if self.last_checked.get() == 0 {
            return None;
        }
        let entries = self.entries.borrow();
        for entry in &*entries {
            if entry.applies_to(useragent) {
                return entry.get_req_rate();
            }
        }
        None
    }
}
