extern crate robotparser;

use robotparser::RobotFileParser;

const AGENT: &'static str = "test_robotparser";


fn robot_test(doc: &str, good_urls: Vec<&str>, bad_urls: Vec<&str>, agent: &str) {
    let parser = RobotFileParser::new("http://www.baidu.com/robots.txt");
    let lines: Vec<&str> = doc.split("\n").collect();
    parser.parse(lines);
    for url in &good_urls {
        assert!(parser.can_fetch(agent, url));
    }
    for url in &bad_urls {
        assert!(!parser.can_fetch(agent, url));
    }
}


fn robot_test_simple(doc: &str, good_urls: Vec<&str>, bad_urls: Vec<&str>) {
    robot_test(doc, good_urls, bad_urls, AGENT);
}


#[test]
fn test_robots_txt_1() {
    let doc = "\n\
    User-agent: *\n\
    Disallow: /cyberworld/map/ # This is an infinite virtual URL space\n\
    Disallow: /tmp/ # these will soon disappear\n\
    Disallow: /foo.html\n\
    ";
    let good = vec!["/","/test.html"];
    let bad = vec!["/cyberworld/map/index.html","/tmp/xxx","/foo.html"];
    robot_test_simple(doc, good, bad);
}


#[test]
fn test_robots_txt_2() {
    let doc = "\n\
    # robots.txt for http://www.example.com/\n\
    \n\
    User-agent: *\n\
    Disallow: /cyberworld/map/ # This is an infinite virtual URL space\n\
    \n\
    # Cybermapper knows where to go.\n\
    User-agent: cybermapper\n\
    Disallow:\n\
    \n\
    ";
    let good = vec!["/","/test.html"];
    let bad = vec!["/cyberworld/map/index.html"];
    robot_test_simple(doc, good, bad);
}
