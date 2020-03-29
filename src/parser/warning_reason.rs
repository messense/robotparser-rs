use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use url::ParseError as ParseUrlError;

#[derive(Clone, Debug)]
/// Warning reason of robots.txt parser about problems when parsing robots.txt file.
pub enum WarningReason {
    /// Invalid directive format. Invalid directive example: `:`
    InvalidDirectiveFormat,
    /// Directive key is empty. Invalid directive example: `: <Value>`
    DirectiveKeyIsEmpty,
    /// Directive key is not suppored by this parser.
    UnsupportedDirectiveKey(String),
    /// Passed directive key is `User-Agent` and passed value is empty.
    UserAgentCannotBeEmpty,
    /// It is impossible to process this directive before the `User-Agent` directive has not been processed.
    DirectiveWithoutUserAgent,
    /// It is impossible to process the `Crawl-Delay` directive because of an error when parsing a floating point number.
    ParseCrawlDelayError(ParseFloatError),
    /// Incorrect format of the `Request-Rate` directive. Example of the correct format: `Request-rate: 1/5`
    WrongRequestRateFormat,
    /// Incorrect format of the `Request-Rate` directive. Example of the correct format: `Request-rate: 1/5`
    ParseRequestRate(ParseIntError),
    /// Parsing URL error.
    ParseUrl(ParseUrlError),
    /// Incorrect format of the `Clean-Param` directive.
    /// Parameters must be matched to regular expression: `A-Za-z0-9.-_`.
    /// Example of the correct format: `Clean-param: ref1&ref2 /some_dir/get_book.pl`
    WrongCleanParamFormat,
    /// Some parameters of the `Clean-Param` directive has wrong symbols.
    /// Parameters must be matched to regular expression: `A-Za-z0-9.-_`.
    /// Example of the correct format: `Clean-param: ref1&ref2 /some_dir/get_book.pl`
    IgnoredCleanParams(Vec<String>),
    /// Error in URL path format.
    WrongPathFormat,
}

/// Displays text of warning reason.
impl fmt::Display for WarningReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::InvalidDirectiveFormat => {
                write!(f, "Invalid directive format.")
            },
            Self::DirectiveKeyIsEmpty => {
                write!(f, "Directive key is empty.")
            },
            Self::UnsupportedDirectiveKey(key) => {
                write!(f, "Directive key `{}` is not suppored by this parser.", key)
            },
            Self::UserAgentCannotBeEmpty => {
                write!(f, "Passed directive key is `User-Agent` and passed value is empty.")
            },
            Self::DirectiveWithoutUserAgent => {
                write!(f, "It is impossible to process this directive before `User-Agent` directive has not been processed.")
            },
            Self::ParseCrawlDelayError(err) => {
                write!(f, "It is impossible to process the `Crawl-Delay` directive because of an error when parsing a floating point number: {}", err)
            },
            Self::WrongRequestRateFormat => {
                write!(f, "Incorrect format of the `Request-Rate` directive")
            },
            Self::ParseRequestRate(err) => {
                write!(f, "Incorrect format of the `Request-Rate` directive: {}", err)
            },
            Self::ParseUrl(err) => {
                write!(f, "Parsing URL error: {}", err)
            },
            Self::WrongCleanParamFormat => {
                write!(f, "Incorrect format of the `Clean-Param` directive.")
            },
            Self::IgnoredCleanParams(ref params) => {
                write!(f, "Directive `Clean-Param` directive has incorrect parameters: {:?}", params)
            },
            Self::WrongPathFormat => {
                write!(f, "Error in URL path format.")
            },
        }
    }
}
