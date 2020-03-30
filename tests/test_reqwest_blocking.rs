use reqwest::blocking::Client;
use robotparser::http::RobotsTxtClient;
use robotparser::service::RobotsTxtService;
use url::Url;
use url::{Host, Origin};

#[test]
fn test_reqwest_blocking() {
    let client = Client::new();
    let robots_txt_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    let robots_txt = client.fetch_robots_txt(robots_txt_url.origin()).unwrap().get_result();
    let fetch_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    assert!(robots_txt.can_fetch("*", &fetch_url));
    let fetch_url = Url::parse("https://www.python.org/webstats/").unwrap();
    assert!(!robots_txt.can_fetch("*", &fetch_url));
}

#[test]
fn test_reqwest_blocking_panic_url() {
    let client = Client::new();
    let host = Host::Domain("python.org::".into());
    let origin = Origin::Tuple("https".into(), host, 80);
    if client.fetch_robots_txt(origin).is_ok() {
        panic!()
    }
}
