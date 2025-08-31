use robotparser::parser::parse_robots_txt;
use robotparser::service::RobotsTxtService;
use std::time::Duration;
use url::Url;

const AGENT: &str = "test_robotparser";

fn robot_test(doc: &str, good_urls: Vec<&str>, bad_urls: Vec<&str>, agent: &str) {
    let url = Url::parse("https://www.baidu.com/robots.txt").unwrap();
    let parser = parse_robots_txt(url.origin(), doc).get_result();
    for url in &good_urls {
        let url = format!("https://www.baidu.com{}", url);
        let url = Url::parse(&url).unwrap();
        assert!(parser.can_fetch(agent, &url));
    }
    for url in &bad_urls {
        let url = format!("https://www.baidu.com{}", url);
        let url = Url::parse(&url).unwrap();
        assert!(!parser.can_fetch(agent, &url));
    }
}

fn robot_test_simple(doc: &str, good_urls: Vec<&str>, bad_urls: Vec<&str>) {
    robot_test(doc, good_urls, bad_urls, AGENT);
}

#[test]
fn test_robots_txt_rn_bom() {
    let doc = "\u{feff}\r\n\
    User-agent: *\r\n\
    Disallow: /cyberworld/map/ # This is an infinite virtual URL space\r\n\
    Disallow: /tmp/ # these will soon disappear\r\n\
    Disallow: /foo.html\r\n\
    ";
    let good = vec!["/", "/test.html"];
    let bad = vec!["/cyberworld/map/index.html", "/tmp/xxx", "/foo.html"];
    robot_test_simple(doc, good, bad);
}

#[test]
fn test_robots_txt_1() {
    let doc = "\n\
    User-agent: *\n\
    Disallow: /cyberworld/map/ # This is an infinite virtual URL space\n\
    Disallow: /tmp/ # these will soon disappear\n\
    Disallow: /foo.html\n\
    ";
    let good = vec!["/", "/test.html"];
    let bad = vec!["/cyberworld/map/index.html", "/tmp/xxx", "/foo.html"];
    robot_test_simple(doc, good, bad);
}

#[test]
fn test_robots_txt_2() {
    let doc = "\n\
    # robots.txt for https://www.example.com/\n\
    \n\
    User-agent: *\n\
    Disallow: /cyberworld/map/ # This is an infinite virtual URL space\n\
    \n\
    # Cybermapper knows where to go.\n\
    User-agent: cybermapper\n\
    Disallow:\n\
    \n\
    ";
    let good = vec!["/", "/test.html"];
    let bad = vec!["/cyberworld/map/index.html"];
    robot_test_simple(doc, good, bad);

    let good = vec!["/cyberworld/map/index.html"];
    robot_test(doc, good, vec![], "cybermapper");
}

#[test]
fn test_robots_txt_3() {
    let doc = "\n\
    # go away\n\
    User-agent: *\n\
    Disallow: /\n\
    ";
    let good = vec![];
    let bad = vec!["/cyberworld/map/index.html", "/", "/tmp/"];
    robot_test_simple(doc, good, bad);
}

#[test]
fn test_robots_txt_4() {
    let doc = "\n\
    User-agent: figtree\n\
    Disallow: /tmp\n\
    Disallow: /a%3cd.html\n\
    Disallow: /a%2fb.html\n\
    Disallow: /%7ejoe/index.html\n\
    ";
    let good = vec![];
    let bad = vec![
        "/tmp",
        "/tmp.html",
        "/tmp/a.html",
        "/a%3cd.html",
        "/a%3Cd.html",
        "/a%2fb.html",
        "/~joe/index.html",
    ];
    robot_test(doc, good.clone(), bad.clone(), "figtree");
    robot_test(doc, good, bad, "FigTree Robot libwww-perl/5.04");
}

#[test]
fn test_robots_txt_5() {
    let doc = "\n\
    User-agent: *\n\
    Disallow: /tmp/\n\
    Disallow: /a%3Cd.html\n\
    Disallow: /a/b.html\n\
    Disallow: /%7ejoe/index.html\n\
    ";
    let good = vec!["/tmp"];
    let bad = vec![
        "/tmp/",
        "/tmp/a.html",
        "/a%3cd.html",
        "/a%3Cd.html",
        "/a/b.html",
        "/%7Ejoe/index.html",
    ];
    robot_test_simple(doc, good, bad);
}

#[test]
fn test_robots_txt_6() {
    let doc = "\n\
    User-Agent: *\n\
    Disallow: /.\n\
    ";
    let good = vec!["/foo.html"];
    let bad = vec![];
    robot_test_simple(doc, good, bad);
}

#[test]
fn test_robots_txt_7() {
    let doc = "\n\
    User-agent: Googlebot\n\
    Allow: /folder1/myfile.html\n\
    Disallow: /folder1/\n\
    ";
    let good = vec!["/folder1/myfile.html"];
    let bad = vec!["/folder1/anotherfile.html"];
    robot_test(doc, good, bad, "Googlebot");
}

/// This file is incorrect because "Googlebot" is a substring of "Googlebot-Mobile"
#[test]
fn test_robots_txt_8() {
    let doc = "\n\
    User-agent: Googlebot\n\
    Disallow: /\n\
    \n\
    User-agent: Googlebot-Mobile\n\
    Allow: /\n\
    ";
    let good = vec![];
    let bad = vec!["/something.jpg"];
    robot_test(doc, good.clone(), bad.clone(), "Googlebot");
    robot_test(doc, good, bad, "Googlebot-Mobile");
}

#[test]
fn test_robots_txt_9() {
    let doc = "\n\
    User-agent: Googlebot-Mobile\n\
    Allow: /\n\
    \n\
    User-agent: Googlebot\n\
    Disallow: /\n\
    ";
    let good = vec![];
    let bad = vec!["/something.jpg"];
    robot_test(doc, good.clone(), bad.clone(), "Googlebot");
    robot_test(doc, bad, good, "Googlebot-Mobile");
}

#[test]
fn test_robots_txt_10() {
    let doc = "\n\
    User-agent: Googlebot\n\
    Allow: /folder1/myfile.html\n\
    Disallow: /folder1/\n\
    ";
    let good = vec!["/folder1/myfile.html"];
    let bad = vec!["/folder1/anotherfile.html"];
    robot_test(doc, good, bad, "googlebot");
}

/// query string support
#[test]
fn test_robots_txt_11() {
    let doc = "\n\
    User-agent: *\n\
    Disallow: /some/path?name=value\n\
    ";
    let good = vec!["/some/path"];
    let bad = vec!["/some/path?name=value"];
    robot_test_simple(doc, good, bad);
}

/// obey first * entry
#[test]
fn test_robots_txt_12() {
    let doc = "\n\
    User-agent: *\n\
    Disallow: /some/path\n\
    \n\
    User-agent: *\n\
    Disallow: /another/path\n\
    ";
    let good = vec!["/another/path"];
    let bad = vec!["/some/path"];
    robot_test_simple(doc, good, bad);
}

/// Empty query. Normalizing the url first.
#[test]
fn test_robots_txt_13() {
    let doc = "\n\
    User-agent: *\n\
    Allow: /some/path?\n\
    Disallow: /another/path?\n\
    ";
    let good = vec!["/some/path?"];
    let bad = vec!["/another/path?"];
    robot_test_simple(doc, good, bad);
}

/// Using patterns with `*` and `$` symbols.
#[test]
fn test_robots_txt_14() {
    let doc = "\n\
    User-agent: *\n
    Allow: /*video.html\n
    Allow: */?amp*\n
    Disallow: */rss$\n
    Disallow: */rss/$\n
    Disallow: /rate/\n
    ";
    let good = vec!["/rss/test", "/sdfvsdvs-sdfvsdv-video.html", "/rate"];
    let bad = vec!["/rss", "/rss/", "/rate/", "/rate/0/9"];
    robot_test_simple(doc, good, bad);
}

#[cfg(feature = "reqwest")]
#[test]
fn test_robots_txt_read() {
    use reqwest::{Client, Request};
    use robotparser::http::{CreateRobotsTxtRequest, ParseRobotsTxtResponse};
    let http_client = Client::new();
    let url = Url::parse("https://www.python.org/robots.txt").unwrap();
    let request = Request::create_robots_txt_request(url.origin());
    let mut response = http_client.execute(request).unwrap();
    let parser = response.parse_robots_txt_response().unwrap().get_result();
    assert!(parser.can_fetch("*", &url));
}

#[test]
fn test_robots_text_crawl_delay() {
    let robots_txt_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    let doc = "User-agent: Yandex\n\
    Crawl-delay: 2.35\n\
    Disallow: /search/\n";
    let parser = parse_robots_txt(robots_txt_url.origin(), doc).get_result();
    assert_eq!(
        Duration::new(2, 350 * 1000 * 1000),
        parser.get_crawl_delay("Yandex").unwrap()
    );
}

#[test]
fn test_robots_text_sitemaps() {
    let robots_txt_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    let doc = "User-agent: Yandex\n\
    Sitemap    \t  :  https://example.com/sitemap1.xml\n
    Sitemap:  https://example.com/sitemap2.xml\n
    Sitemap:  https://example.com/sitemap3.xml\n
    Disallow: /search/\n";
    let parser = parse_robots_txt(robots_txt_url.origin(), doc).get_result();
    assert_eq!(
        &[
            Url::parse("https://example.com/sitemap1.xml").unwrap(),
            Url::parse("https://example.com/sitemap2.xml").unwrap(),
            Url::parse("https://example.com/sitemap3.xml").unwrap()
        ],
        parser.get_sitemaps()
    );
}

#[test]
fn test_robots_text_request_rate() {
    let robots_txt_url = Url::parse("https://www.python.org/robots.txt").unwrap();
    let doc = "User-agent: Yandex\n\
        Request-rate: 3/15\n\
        Disallow: /search/\n";
    let parser = parse_robots_txt(robots_txt_url.origin(), doc).get_result();
    let req_rate = parser.get_req_rate("Yandex").unwrap();
    assert_eq!(3, req_rate.requests);
    assert_eq!(15, req_rate.seconds);

    let req_rate = parser.get_req_rate("Google");
    assert!(req_rate.is_none());
}

#[test]
fn test_robots_text_clean_params() {
    let doc = "\
User-Agent: *\n\
Clean-param: mode\n\
Clean-param: from\n\
Clean-param: pid\n\
Clean-param: gid\n\
Clean-param: tm\n\
Clean-param: amp\n\
    ";
    let url = Url::parse("https://www.baidu.com/robots.txt").unwrap();
    let parser = parse_robots_txt(url.origin(), doc).get_result();
    let mut site_url =
        Url::parse("https://www.baidu.com/test?post_id=7777&mode=99&from=google&pid=99&gid=88&tm=777&amp=1").unwrap();
    let was_updated = parser.normalize_url(&mut site_url);
    assert!(was_updated);
    assert_eq!(site_url.as_str(), "https://www.baidu.com/test?post_id=7777");

    let mut site_url =
        Url::parse("https://www.google.com/test?post_id=7777&mode=99&from=google&pid=99&gid=88&tm=777&amp=1").unwrap();
    let was_updated = parser.normalize_url(&mut site_url);
    assert!(!was_updated);
    assert_eq!(
        site_url.as_str(),
        "https://www.google.com/test?post_id=7777&mode=99&from=google&pid=99&gid=88&tm=777&amp=1"
    );
}
