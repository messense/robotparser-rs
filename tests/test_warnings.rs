use robotparser::parser::{parse_robots_txt, WarningReason};
use std::convert::From;
use url::{Host, Origin};

#[derive(PartialEq, Eq, Debug, Clone)]
enum WarningReasonKind {
    InvalidDirectiveFormat,
    DirectiveKeyIsEmpty,
    UnsupportedDirectiveKey,
    UserAgentCannotBeEmpty,
    DirectiveWithoutUserAgent,
    ParseCrawlDelayError,
    WrongRequestRateFormat,
    ParseRequestRate,
    ParseUrl,
    WrongCleanParamFormat,
    IgnoredCleanParams,
    WrongPathFormat,
}

fn validate_warnings(input: &str, expected_warnings: &[WarningReasonKind]) {
    let host = Host::Domain("python.org".into());
    let origin = Origin::Tuple("http".into(), host, 80);
    let warnings = parse_robots_txt(origin, &input).get_warnings().to_vec();
    assert_eq!(warnings.len(), expected_warnings.len());
    for (warning, expected_warning) in warnings.iter().zip(expected_warnings.iter()) {
        let warning: WarningReasonKind = warning.get_reason().into();
        assert_eq!(expected_warning.clone(), warning);
    }
}

#[test]
fn test_warning_invalid_directive_format() {
    let input = "`";
    validate_warnings(input, &[WarningReasonKind::InvalidDirectiveFormat]);
    let input = " \t ` \t ";
    validate_warnings(input, &[WarningReasonKind::InvalidDirectiveFormat]);
}

#[test]
fn test_warning_directive_key_is_empty() {
    let input = ":";
    validate_warnings(input, &[WarningReasonKind::DirectiveKeyIsEmpty]);
}

#[test]
fn test_warning_supported_directive_key() {
    let input = "X-Directive:";
    validate_warnings(input, &[WarningReasonKind::UnsupportedDirectiveKey]);
    let input = "\t  X-Directive\t  :\t  ";
    validate_warnings(input, &[WarningReasonKind::UnsupportedDirectiveKey]);
}

#[test]
fn test_warning_user_agent_cannot_be_empty() {
    let input = "User-Agent:";
    validate_warnings(input, &[WarningReasonKind::UserAgentCannotBeEmpty]);
    let input = "\t  User-Agent\t  :\t  ";
    validate_warnings(input, &[WarningReasonKind::UserAgentCannotBeEmpty]);
    let input = "\t  User-Agent\t  :\t  *";
    validate_warnings(input, &[]);
}

#[test]
fn test_warning_directive_without_user_agent() {
    let input = "Crawl-Delay: 5s";
    validate_warnings(input, &[WarningReasonKind::DirectiveWithoutUserAgent]);
    let input = "User-Agent: *\nCrawl-Delay: 5";
    validate_warnings(input, &[]);
}

#[test]
fn test_warning_parse_crawl_delay_error() {
    let input = "User-Agent: *\nCrawl-Delay: ";
    validate_warnings(input, &[WarningReasonKind::ParseCrawlDelayError]);
    let input = "User-Agent: *\nCrawl-Delay: -";
    validate_warnings(input, &[WarningReasonKind::ParseCrawlDelayError]);
    let input = "User-Agent: *\nCrawl-Delay: 5h9";
    validate_warnings(input, &[WarningReasonKind::ParseCrawlDelayError]);
    let input = "User-Agent: *\nCrawl-Delay: 5";
    validate_warnings(input, &[]);
}

#[test]
fn test_warning_request_rate_format() {
    let input = "User-Agent: *\nRequest-rate: 1/5";
    validate_warnings(input, &[]);
    let input = "User-Agent: *\nRequest-rate: 1//5";
    validate_warnings(input, &[WarningReasonKind::WrongRequestRateFormat]);
    let input = "User-Agent: *\nRequest-rate: 1";
    validate_warnings(input, &[WarningReasonKind::WrongRequestRateFormat]);
}

#[test]
fn test_warning_request_rate() {
    let input = "User-Agent: *\nRequest-rate: a/b";
    validate_warnings(input, &[WarningReasonKind::ParseRequestRate]);
    let input = "User-Agent: *\nRequest-rate: a/5";
    validate_warnings(input, &[WarningReasonKind::ParseRequestRate]);
    let input = "User-Agent: *\nRequest-rate: 5/b";
    validate_warnings(input, &[WarningReasonKind::ParseRequestRate]);
    let input = "User-Agent: *\nRequest-rate: 1.0/5.0";
    validate_warnings(input, &[WarningReasonKind::ParseRequestRate]);
}

#[test]
fn test_warning_parsing_url() {
    let input = "User-Agent: *\nSitemap: https://python.org/sitemap.xml";
    validate_warnings(input, &[]);
    let input = "User-Agent: *\nSitemap: http$$$://python.org/sitemap.xml";
    validate_warnings(input, &[WarningReasonKind::ParseUrl]);
}

#[test]
fn test_wrong_clean_param() {
    let input = "User-Agent: *\nClean-param: ref ";
    validate_warnings(input, &[]);
    let input = "User-Agent: *\nClean-param: ";
    validate_warnings(input, &[WarningReasonKind::WrongCleanParamFormat]);
    let input = "User-Agent: *\nClean-param: &";
    validate_warnings(input, &[]);
    let input = "User-Agent: *\nClean-param: ?";
    validate_warnings(input, &[WarningReasonKind::IgnoredCleanParams]);
    let input = "User-Agent: *\nClean-param: abc$";
    validate_warnings(input, &[WarningReasonKind::IgnoredCleanParams]);
}

#[test]
fn test_warning_wrong_path_format() {
    let input = "User-Agent: *\nAllow: \\";
    validate_warnings(input, &[WarningReasonKind::WrongPathFormat]);
    let input = "User-Agent: *\nDisallow: \\";
    validate_warnings(input, &[WarningReasonKind::WrongPathFormat]);
}

impl From<&WarningReason> for WarningReasonKind {
    fn from(reason: &WarningReason) -> Self {
        match *reason {
            WarningReason::InvalidDirectiveFormat => WarningReasonKind::InvalidDirectiveFormat,
            WarningReason::DirectiveKeyIsEmpty => WarningReasonKind::DirectiveKeyIsEmpty,
            WarningReason::UnsupportedDirectiveKey { .. } => WarningReasonKind::UnsupportedDirectiveKey,
            WarningReason::UserAgentCannotBeEmpty => WarningReasonKind::UserAgentCannotBeEmpty,
            WarningReason::DirectiveWithoutUserAgent => WarningReasonKind::DirectiveWithoutUserAgent,
            WarningReason::ParseCrawlDelayError { .. } => WarningReasonKind::ParseCrawlDelayError,
            WarningReason::WrongRequestRateFormat => WarningReasonKind::WrongRequestRateFormat,
            WarningReason::ParseRequestRate { .. } => WarningReasonKind::ParseRequestRate,
            WarningReason::ParseUrl { .. } => WarningReasonKind::ParseUrl,
            WarningReason::WrongCleanParamFormat => WarningReasonKind::WrongCleanParamFormat,
            WarningReason::IgnoredCleanParams { .. } => WarningReasonKind::IgnoredCleanParams,
            WarningReason::WrongPathFormat => WarningReasonKind::WrongPathFormat,
        }
    }
}
