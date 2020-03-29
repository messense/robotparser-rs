use crate::model::path::Path;
use percent_encoding::percent_decode;
use std::convert::From;
use std::mem::replace;

#[derive(Debug, Clone)]
pub struct PathPattern(Vec<PathPatternToken>);

#[derive(Debug, Eq, PartialEq, Clone)]
enum PathPatternToken {
    Text(String),
    AnyString,
    TerminateString,
}

impl PathPatternToken {
    fn from_path_pattern(path: String) -> PathPatternToken {
        let path = percent_decode(path.as_bytes()).decode_utf8_lossy();
        return PathPatternToken::Text(path.to_string());
    }
}

impl PathPatternToken {
    fn len(&self) -> usize {
        return match self {
            &PathPatternToken::Text(ref text) => text.len(),
            &PathPatternToken::AnyString => 1,
            &PathPatternToken::TerminateString => 1,
        };
    }
}

impl PathPattern {
    pub fn new(path: &str) -> PathPattern {
        let mut text = String::new();
        let mut tokens = Vec::new();
        for c in path.chars() {
            let prepared_token = match c {
                '*' => Some(PathPatternToken::AnyString),
                '$' => Some(PathPatternToken::TerminateString),
                _ => {
                    text.push(c);
                    None
                }
            };
            if let Some(prepared_token) = prepared_token {
                if !text.is_empty() {
                    tokens.push(PathPatternToken::from_path_pattern(replace(&mut text, "".into())));
                }
                tokens.push(prepared_token);
            }
        }
        if !text.is_empty() {
            tokens.push(PathPatternToken::from_path_pattern(text));
        }
        if let Some(&PathPatternToken::Text(..)) = tokens.last() {
            tokens.push(PathPatternToken::AnyString);
        }
        tokens.dedup();
        return PathPattern(tokens);
    }

    pub fn all() -> PathPattern {
        return PathPattern(vec![PathPatternToken::AnyString]);
    }

    pub fn applies_to(&self, path: &Path) -> bool {
        let mut filename = path.as_str();
        for (index, token) in self.0.iter().enumerate() {
            match token {
                &PathPatternToken::Text(ref text) => {
                    if !filename.starts_with(text) {
                        return false;
                    }
                    filename = &filename[text.len()..];
                }
                &PathPatternToken::AnyString => {
                    if let Some(&PathPatternToken::Text(ref text)) = self.0.get(index + 1) {
                        while filename.len() >= 1 {
                            if filename.starts_with(text) {
                                break;
                            }
                            // Search for next unicode char.
                            if let Some((next_char_index, _)) = filename.char_indices().nth(1) {
                                filename = &filename[next_char_index..];
                            } else {
                                break;
                            }
                        }
                    } else {
                        filename = &filename[filename.len()..];
                    }
                }
                &PathPatternToken::TerminateString => {
                    if filename.len() != 0 {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        for path_token in self.0.iter() {
            length += path_token.len();
        }
        return length;
    }
}

impl From<&str> for PathPattern {
    fn from(path: &str) -> Self {
        return PathPattern::new(path);
    }
}
