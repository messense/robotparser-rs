use crate::model::path_pattern::PathPattern;
use crate::model::path::Path;

/// A rule line is a single "Allow:" (allowance==True) or "Disallow:"
/// (allowance==False) followed by a path."""
#[derive(Debug, Clone)]
pub struct Rule {
    path_pattern: PathPattern,
    allowance: bool,
}

impl Rule {
    pub fn new(path_pattern: impl Into<PathPattern>, allowance: bool) -> Rule {
        Rule {
            path_pattern: path_pattern.into(),
            allowance,
        }
    }

    pub (crate) fn applies_to(&self, path: &Path) -> bool {
        return self.path_pattern.applies_to(path);
    }

    pub (crate) fn get_allowance(&self) -> bool {
        return self.allowance;
    }

    pub (crate) fn get_path_pattern(&self) -> &PathPattern {
        return &self.path_pattern;
    }
}