use crate::parser::warning::ParseWarning;
use std::fmt::Debug;

#[derive(Debug)]
/// The result of the robots.txt parser.
pub struct ParseResult<R>
where
    R: Debug,
{
    result: R,
    warnings: Vec<ParseWarning>,
}

impl<R> ParseResult<R>
where
    R: Debug,
{
    /// Creates a new structure for parser results.
    pub(crate) fn new(result: R) -> ParseResult<R> {
        ParseResult {
            result,
            warnings: Vec::new(),
        }
    }

    /// Creates a new structure for parser results with warnings.
    pub(crate) fn new_with_warnings(result: R, warnings: Vec<ParseWarning>) -> ParseResult<R> {
        ParseResult { result, warnings }
    }

    /// Returns the result of the robots.txt parser.
    pub fn get_result(self) -> R {
        self.result
    }

    /// Returns the robots.txt parser warning array.
    pub fn get_warnings(&self) -> &[ParseWarning] {
        self.warnings.as_slice()
    }

    /// Returns reference to result of the robots.txt parser or first warning.
    pub fn ok_ref(&self) -> Result<&R, &ParseWarning> {
        if let Some(warning) = self.warnings.first() {
            return Err(warning);
        }
        Ok(&self.result)
    }

    /// Returns the result of the robots.txt parser or first warning.
    pub fn ok(mut self) -> Result<R, ParseWarning> {
        if self.warnings.is_empty() {
            return Ok(self.result);
        }
        let first_warning = self.warnings.remove(0);
        Err(first_warning)
    }

    /// Converts this structure into another type of structure.
    pub(crate) fn map<T>(self, callback: impl Fn(R) -> T) -> ParseResult<T>
    where
        T: Debug,
    {
        ParseResult {
            result: (callback)(self.result),
            warnings: self.warnings,
        }
    }
}
