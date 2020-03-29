use super::line::Line;
use super::warning_reason::WarningReason;
use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use url::ParseError as ParseUrlError;

#[derive(Clone, Debug)]
/// Warning of robots.txt parser about problems when parsing robots.txt file.
pub struct ParseWarning {
    line_no: usize,
    line: String,
    reason: WarningReason,
}

impl Error for ParseWarning {}

impl ParseWarning {
    /// Returns the line number in the text of the robots.txt file.
    pub fn get_line_no(&self) -> usize {
        return self.line_no;
    }

    /// Returns the text of the robots.txt file string.
    pub fn get_line_text(&self) -> &String {
        return &self.line;
    }

    /// Returns the reason of warning.
    pub fn get_reason(&self) -> &WarningReason {
        return &self.reason;
    }

    pub(crate) fn invalid_directive_format(line: &Line) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::InvalidDirectiveFormat,
        };
    }

    pub(crate) fn directive_key_is_empty(line: &Line) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::DirectiveKeyIsEmpty,
        };
    }

    pub(crate) fn unsupported_directive_key(line: &Line, key: String) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::UnsupportedDirectiveKey(key),
        };
    }

    pub(crate) fn user_agent_cannot_be_empty(line: &Line) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::UserAgentCannotBeEmpty,
        };
    }

    pub(crate) fn wrong_path_format(line: &Line) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::WrongPathFormat,
        };
    }

    pub(crate) fn directive_without_user_agent(line: &Line) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::DirectiveWithoutUserAgent,
        };
    }

    pub(crate) fn parse_crawl_delay_error(line: &Line, error: ParseFloatError) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::ParseCrawlDelayError(error),
        };
    }

    pub(crate) fn wrong_request_rate_format(line: &Line) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::WrongRequestRateFormat,
        };
    }

    pub(crate) fn parse_request_rate(line: &Line, error: ParseIntError) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::ParseRequestRate(error),
        };
    }

    pub(crate) fn parse_url(line: &Line, error: ParseUrlError) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::ParseUrl(error),
        };
    }

    pub(crate) fn wrong_clean_param_format(line: &Line) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::WrongCleanParamFormat,
        };
    }

    pub(crate) fn ignored_clean_params(line: &Line, ignored_clean_params: Vec<String>) -> ParseWarning {
        return ParseWarning {
            line_no: line.get_line_number(),
            line: line.get_line_text().into(),
            reason: WarningReason::IgnoredCleanParams(ignored_clean_params),
        };
    }
}

/// Displays text of warning.
impl fmt::Display for ParseWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Line: {}. Text: `{}`. {}", self.line_no, self.line, self.reason)
    }
}
