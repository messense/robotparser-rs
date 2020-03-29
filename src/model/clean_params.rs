use crate::model::PathPattern;

#[derive(Debug, Clone)]
pub struct CleanParams {
    path_pattern: PathPattern,
    params: Vec<String>,
}

impl CleanParams {
    pub fn new(path_pattern: PathPattern, params: Vec<String>) -> CleanParams {
        CleanParams { path_pattern, params }
    }

    pub fn get_path_pattern(&self) -> &PathPattern {
        &self.path_pattern
    }

    pub fn get_params(&self) -> &Vec<String> {
        &self.params
    }
}
