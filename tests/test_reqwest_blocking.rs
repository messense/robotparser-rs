use robotparser::http::RobotsTxtClient;
use robotparser::service::RobotsTxtService;
use reqwest::blocking::Client;
use url::Url;

#[test]
fn test_reqwest_blocking() {
    let client = Client::new();
    let robots_txt_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    let robots_txt = client.fetch_robots_txt(robots_txt_url.origin()).unwrap().get_result();
    let fetch_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    assert!(robots_txt.can_fetch("*", &fetch_url));
}
