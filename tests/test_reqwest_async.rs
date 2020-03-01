use robotparser::http::RobotsTxtClient;
use robotparser::service::RobotsTxtService;
use reqwest::Client;
use url::Url;
use tokio::runtime::Runtime;
use url::{Host, Origin};

#[test]
fn test_reqwest_async() {
    let mut runtime = Runtime::new().unwrap();
    let client = Client::new();
    let robots_txt_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    let robots_txt_response = runtime.block_on(client.fetch_robots_txt(robots_txt_url.origin()));
    let robots_txt = robots_txt_response.unwrap().get_result();
    let fetch_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    assert!(robots_txt.can_fetch("*", &fetch_url));
    let fetch_url = Url::parse("http://www.python.org/webstats/").unwrap();
    assert!(!robots_txt.can_fetch("*", &fetch_url));
}

#[test]
#[should_panic]
fn test_reqwest_blocking_panic_url() {
    let client = Client::new();
    let host = Host::Domain("python.org::".into());
    let origin = Origin::Tuple("https".into(), host, 80);
    client.fetch_robots_txt(origin);
}
