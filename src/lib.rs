//!
//! robots.txt parser for Rust
//!
//! The robots.txt Exclusion Protocol is implemented as specified in
//! http://www.robotstxt.org/norobots-rfc.txt
//!
extern crate url;
extern crate time;
extern crate hyper;

use std::io::Read;
use std::cell::{Cell, RefCell};
use url::Url;
use hyper::{Client};
use hyper::status::StatusCode;

/// A rule line is a single "Allow:" (allowance==True) or "Disallow:"
/// (allowance==False) followed by a path."""
#[derive(Debug, Eq, PartialEq, Clone)]
struct RuleLine {
    path: String,
    allowance: bool,
}

/// An entry has one or more user-agents and zero or more rulelines
#[derive(Debug, Eq, PartialEq, Clone)]
struct Entry {
    useragents: RefCell<Vec<String>>,
    rulelines: RefCell<Vec<RuleLine>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RobotFileParser {
    entries: RefCell<Vec<Entry>>,
    default_entry: RefCell<Entry>,
    disallow_all: Cell<bool>,
    allow_all: Cell<bool>,
    url: Url,
    host: String,
    path: String,
    last_checked: Cell<i64>,
}


impl RuleLine {
    fn new(path: &str, allowance: bool) -> RuleLine {
        let mut allow = allowance;
        if path == "" && !allowance {
            // an empty value means allow all
            allow = true;
        }
        RuleLine {
            path: path.to_owned(),
            allowance: allow,
        }
    }

    fn applies_to(&self, filename: &str) -> bool {
        &self.path == "*" || filename.starts_with(&self.path)
    }
}


impl Entry {
    fn new() -> Entry {
        Entry {
            useragents: RefCell::new(vec![]),
            rulelines: RefCell::new(vec![]),
        }
    }

    /// check if this entry applies to the specified agent
    fn applies_to(&self, useragent: &str) -> bool {
        let ua = useragent.split("/").nth(0).unwrap_or("");
        let useragents = self.useragents.borrow().clone();
        for agent in &useragents {
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
        let rulelines = self.rulelines.borrow().clone();
        for line in &rulelines {
            if line.applies_to(filename) {
                return line.allowance
            }
        }
        true
    }

    fn push_useragent(&self, useragent: &str) {
        let mut useragents = self.useragents.borrow_mut();
        useragents.push(useragent.to_owned());
    }

    fn push_ruleline(&self, ruleline: RuleLine) {
        let mut rulelines = self.rulelines.borrow_mut();
        rulelines.push(ruleline);
    }

    fn has_useragent(&self, useragent: &str) -> bool {
        let useragents = self.useragents.borrow().clone();
        useragents.contains(&useragent.to_owned())
    }

    fn is_empty(&self) -> bool {
        let useragents = self.useragents.borrow().clone();
        let rulelines = self.rulelines.borrow().clone();
        useragents.is_empty() && rulelines.is_empty()
    }
}


impl RobotFileParser {
    pub fn new(url: &str) -> RobotFileParser {
        let parsed_url = Url::parse(url).unwrap();
        RobotFileParser {
            entries: RefCell::new(vec![]),
            default_entry: RefCell::new(Entry::new()),
            disallow_all: Cell::new(false),
            allow_all: Cell::new(false),
            url: parsed_url.clone(),
            host: parsed_url.domain().unwrap().to_owned(),
            path: parsed_url.path().unwrap().connect("/"),
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
        use time::get_time;

        self.last_checked.set(get_time().sec);
    }

    /// Sets the URL referring to a robots.txt file.
    pub fn set_url(&mut self, url: &str) {
        let parsed_url = Url::parse(url).unwrap();
        self.url = parsed_url.clone();
        self.host = parsed_url.domain().unwrap().to_owned();
        self.path = parsed_url.path().unwrap().connect("/");
        self.last_checked.set(0i64);
    }

    /// Reads the robots.txt URL and feeds it to the parser.
    pub fn read(&self) {
        let client = Client::new();
        let mut res = match client.get(self.url.clone()).send() {
            Ok(res) => res,
            Err(_) => {
                return;
            }
        };
        match res.status {
            StatusCode::Unauthorized | StatusCode::Forbidden => {
                self.disallow_all.set(true);
            },
            status if status >= StatusCode::BadRequest && status < StatusCode::InternalServerError => {
                self.allow_all.set(true);
            },
            StatusCode::Ok => {
                let mut buf = String::new();
                res.read_to_string(&mut buf).unwrap();
                let lines: Vec<&str> = buf.split("\n").collect();
                self.parse(lines);
            },
            _ => {},
        }
    }

    fn _add_entry(&self, entry: Entry) {
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
    pub fn parse(&self, lines: Vec<&str>) {
        use url::percent_encoding::percent_decode;

        // states:
        //   0: start state
        //   1: saw user-agent line
        //   2: saw an allow or disallow line
        let mut state = 0;
        let mut entry = Entry::new();

        self.modified();
        for line in &lines {
            let mut ln = line.clone();
            if ln.is_empty() {
                match state {
                    1 => {
                        entry = Entry::new();
                        state = 0;
                    },
                    2 => {
                        self._add_entry(entry);
                        entry = Entry::new();
                        state = 0;
                    },
                    _ => {},
                }
            }
            // remove optional comment and strip line
            match ln.find("#") {
                Some(i) => {
                    ln = &ln[0..i];
                },
                None => {},
            }
            ln = ln.trim();
            if ln.is_empty() {
                continue;
            }
            let parts: Vec<&str> = ln.splitn(2, ":").collect();
            if parts.len() == 2 {
                let part0 = parts[0].trim().to_lowercase();
                let part1 = String::from_utf8(percent_decode(parts[1].trim().as_bytes())).unwrap_or("".to_owned());
                match part0 {
                    ref x if x == "user-agent" => {
                        if state == 2 {
                            self._add_entry(entry);
                            entry = Entry::new();
                        }
                        entry.push_useragent(&part1);
                        state = 1;
                    },
                    ref x if x == "disallow" => {
                        if state != 0 {
                            entry.push_ruleline(RuleLine::new(&part1, false));
                            state = 2;
                        }
                    },
                    ref x if x == "allow" => {
                        if state != 0 {
                            entry.push_ruleline(RuleLine::new(&part1, true));
                            state = 2;
                        }
                    },
                    _ => {},
                }
            }
        }
        if state == 2 {
            self._add_entry(entry);
        }
    }

    /// Using the parsed robots.txt decide if useragent can fetch url
    pub fn can_fetch(&self, useragent: &str, url: &str) -> bool {
        use url::percent_encoding::percent_decode;

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
        let decoded_url = String::from_utf8(percent_decode(url.as_bytes())).unwrap_or("".to_owned());
        let url_str = match decoded_url {
            ref u if !u.is_empty() => u.to_owned(),
            _ => "/".to_owned(),
        };
        let entries = self.entries.borrow().clone();
        for entry in &entries {
            if entry.applies_to(useragent) {
                return entry.allowance(&url_str);
            }
        }
        // try the default entry last
        let default_entry = self.default_entry.borrow().clone();
        if !default_entry.is_empty() {
            return default_entry.allowance(&url_str);
        }
        // agent not found ==> access granted
        true
    }
}
