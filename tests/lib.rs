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
    let bad = vec!["/cyberworld/map/index.html","/","/tmp/"];
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
        "/tmp", "/tmp.html", "/tmp/a.html", "/a%3cd.html", "/a%3Cd.html",
        "/a%2fb.html", "/~joe/index.html",
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
        "/tmp/", "/tmp/a.html", "/a%3cd.html", "/a%3Cd.html",
        "/a/b.html", "/%7Ejoe/index.html",
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
