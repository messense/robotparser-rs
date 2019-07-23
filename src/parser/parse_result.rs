use crate::parser::warning::ParseWarning;
use std::fmt::Debug;

#[derive(Debug)]
/// The result of the robots.txt parser.
pub struct ParseResult<R> where R: Debug {
    result: R,
    warnings: Vec<ParseWarning>,
}

impl <R>ParseResult<R> where R: Debug {
    /// Creates a new structure for parser results.
    pub fn new(result: R) -> ParseResult<R>{
        return ParseResult {
            result,
            warnings: Vec::new(),
        }
    }

    /// Creates a new structure for parser results with warnings.
    pub fn new_with_warnings(result: R, warnings: Vec<ParseWarning>) -> ParseResult<R>{
        return ParseResult {
            result,
            warnings,
        }
    }

    /// Returns the result of the robots.txt parser.
    pub fn get_result(self) -> R {
        return self.result;
    }

    /// Returns the robots.txt parser warning array.
    pub fn get_warnings(&self) -> &[ParseWarning] {
        return self.warnings.as_slice();
    }

    /// Converts this structure into another type of structure.
    pub fn map<T>(self, callback: impl Fn(R) -> T) -> ParseResult<T> where T: Debug {
        return ParseResult {
            result: (callback)(self.result),
            warnings: self.warnings,
        }
    }
}