use url::{Origin, Url};
use std::time::Duration;
use crate::parser::parse_result::ParseResult;
use crate::model::{RobotsTxt, Rule, PathPattern, CleanParams, RequestRate};
use crate::parser::line::Line;
use crate::parser::warning::ParseWarning;
mod directive;
use self::directive::Directive;
mod group_builder;
pub use self::group_builder::GroupBuilder;

const COMMENT_BEGIN_CHAR: char = '#';
const KV_SEPARATOR: &'static str = ":";

/// Parses the text of the robots.txt file located in the specified origin.
pub fn parse(origin: Origin, input: &str) -> ParseResult<RobotsTxt> {
    let parser = Parser::new(origin);
    return parser.parse(input);
}

struct Parser {
    result: RobotsTxt,
    group_builder: GroupBuilder,
    warnings: Vec<ParseWarning>,
}

impl Parser {
    pub fn new(origin: Origin) -> Parser {
        return Parser {
            result: RobotsTxt::new(origin),
            group_builder: GroupBuilder::new(),
            warnings: Vec::new(),
        }
    }

    pub fn parse(mut self, input: &str) -> ParseResult<RobotsTxt> {
        let input = ignore_bom(input);
        let mut line_no = 0;
        for line in input.lines() {
            line_no += 1;
            let line = Line::new(line, line_no);
            match Self::parse_line(&line) {
                Ok(Some(line_value)) => {
                    self.process_line_value(&line, &line_value);
                },
                Err(warning) => {
                    self.warnings.push(warning);
                },
                _ => {},
            }
        }
        self.group_builder.fill_entries(&mut self.result);
        return ParseResult::new_with_warnings(self.result, self.warnings);
    }

    fn parse_line<'a>(line: &'a Line) -> Result<Option<Directive<'a>>, ParseWarning> {
        let mut kv_part = line.get_line_text();
        if let Some(comment_separator_position) = line.get_line_text().find(COMMENT_BEGIN_CHAR) {
            kv_part = &kv_part[0..comment_separator_position];
        }
        if kv_part.is_empty() {
            return Ok(None);
        }
        let separator_index = kv_part.find(KV_SEPARATOR).ok_or_else(|| {
            return ParseWarning::invalid_directive_format(line);
        })?;
        if separator_index >= kv_part.len() {
            return Err(ParseWarning::invalid_directive_format(line));
        }
        let key = &kv_part[0..separator_index];
        let key = key.trim();
        if key.is_empty() {
            return Err(ParseWarning::directive_key_is_empty(line));
        }
        let value = &kv_part[separator_index + 1..];
        let value = value.trim();
        let result = Directive::new(key, value);
        return Ok(Some(result));
    }

    fn process_line_value(&mut self, line: &Line, directive: &Directive) {
        let key = directive.get_key_lowercase();
        match key.as_str() {
            // Group specific directives
            "user-agent" => {
                self.process_directive_user_agent(line, directive);
            },
            "allow" => {
                self.process_directive_allow(line, directive);
            },
            "disallow" => {
                self.process_directive_disallow(line, directive);
            },
            "crawl-delay" => {
                self.process_directive_crawl_delay(line, directive);
            },
            "request-rate" => {
                self.process_directive_request_rate(line, directive);
            },
            // Non-group directives
            "sitemap" => {
                self.process_directive_sitemap(line, directive);
            },
            "clean-param" => {
                self.process_directive_clean_param(line, directive);
            },
            _ => {
                self.warnings.push(ParseWarning::unsupported_directive_key(line, key));
            },
        }
    }

    fn process_directive_user_agent(&mut self, line: &Line, directive: &Directive) {
        let user_agent = directive.get_value();
        if user_agent.is_empty() {
            self.warnings.push(ParseWarning::user_agent_cannot_be_empty(line));
            return;
        }
        self.group_builder.handle_user_agent(user_agent);
    }

    fn process_directive_allow(&mut self, line: &Line, directive: &Directive) {
        if let Some(group) = self.group_builder.get_mut_active_group() {
            if directive.get_value() == "" {
                // Nothing to do. Ignoring.
            } else if directive.get_value().starts_with("*") || directive.get_value().starts_with("/") {
                group.push_rule(Rule::new(directive.get_value(), true));
            } else {
                self.warnings.push(ParseWarning::wrong_path_format(line));
            }
        } else {
            self.warnings.push(ParseWarning::directive_without_user_agent(line));
        }
    }

    fn process_directive_disallow(&mut self, line: &Line, directive: &Directive) {
        if let Some(group) = self.group_builder.get_mut_active_group() {
            if directive.get_value() == "" {
                // Allow all.
                group.push_rule(Rule::new(PathPattern::all(), true));
            } else if directive.get_value().starts_with("*") || directive.get_value().starts_with("/") {
                group.push_rule(Rule::new(directive.get_value(), false));
            } else {
                self.warnings.push(ParseWarning::wrong_path_format(line));
            }
        } else {
            self.warnings.push(ParseWarning::directive_without_user_agent(line));
        }
    }

    fn process_directive_crawl_delay(&mut self, line: &Line, directive: &Directive) {
        if let Some(group) = self.group_builder.get_mut_active_group() {
            match directive.get_value().parse::<f64>() {
                Ok(delay) => {
                    let delay_seconds = delay.trunc();
                    let delay_nanoseconds = delay.fract() * 10f64.powi(9);
                    let delay = Duration::new(delay_seconds as u64, delay_nanoseconds as u32);
                    group.set_crawl_delay(delay);
                },
                Err(error) => {
                    self.warnings.push(ParseWarning::parse_crawl_delay_error(line, error));
                },
            }
        } else {
            self.warnings.push(ParseWarning::directive_without_user_agent(line));
        }
    }

    fn process_directive_request_rate(&mut self, line: &Line, directive: &Directive) {
        if let Some(group) = self.group_builder.get_mut_active_group() {
            let numbers: Vec<&str> = directive.get_value().split('/').collect();
            if numbers.len() != 2 {
                self.warnings.push(ParseWarning::wrong_request_rate_format(line));
                return;
            }
            let requests = match numbers[0].parse::<usize>() {
                Ok(requests) => {requests},
                Err(error) => {
                    self.warnings.push(ParseWarning::parse_request_rate(line, error));
                    return;
                },
            };
            let seconds = match numbers[1].parse::<usize>() {
                Ok(seconds) => {seconds},
                Err(error) => {
                    self.warnings.push(ParseWarning::parse_request_rate(line, error));
                    return;
                },
            };
            group.set_req_rate(RequestRate{requests, seconds});
        } else {
            self.warnings.push(ParseWarning::directive_without_user_agent(line));
        }
    }

    fn process_directive_sitemap(&mut self, line: &Line, directive: &Directive) {
        match Url::parse(directive.get_value()) {
            Ok(sitemap_url) => {
                self.result.add_sitemap(sitemap_url);
            },
            Err(error) => {
                self.warnings.push(ParseWarning::parse_url(line, error));
            },
        }
    }

    fn process_directive_clean_param(&mut self, line: &Line, directive: &Directive) {
        let parts: Vec<&str> = directive.get_value().split_whitespace().collect();
        if parts.len() >= 3 || parts.len() == 0 {
            self.warnings.push(ParseWarning::wrong_clean_param_format(line));
            return;
        }
        if parts[0].len() == 0 {
            self.warnings.push(ParseWarning::wrong_clean_param_format(line));
            return;
        }
        let clean_params_path_pattern;
        let clean_params;
        if let Some(second_param) = parts.get(1) {
            if second_param.len() == 0 {
                self.warnings.push(ParseWarning::wrong_clean_param_format(line));
                return;
            }
            clean_params_path_pattern = PathPattern::new(parts[0]);
            clean_params = *second_param;
        } else {
            clean_params_path_pattern = PathPattern::all();
            clean_params = parts[0];
        }
        let (valid_clean_params, invalid_clean_params) = Self::parse_clean_params(clean_params);
        if !invalid_clean_params.is_empty() {
            self.warnings.push(ParseWarning::ignored_clean_params(line, invalid_clean_params));
        }
        self.result.add_clean_params(CleanParams::new(clean_params_path_pattern, valid_clean_params));
    }

    fn parse_clean_params(clean_params: &str) -> (Vec<String>, Vec<String>) {
        let mut valid = Vec::new();
        let mut invalid = Vec::new();
        for clean_param in clean_params.split('&') {
            if !clean_param.is_empty() {
                if Self::is_valid_clean_param(clean_param) {
                    valid.push(clean_param.into());
                } else {
                    invalid.push(clean_param.into());
                }
            }
        }
        return (valid, invalid);
    }

    fn is_valid_clean_param(clean_param: &str) -> bool {
        for c in clean_param.chars() {
            let mut is_valid = false;
            if ('A'..'Z').contains(&c) {
                is_valid = true;
            }
            if ('a'..'z').contains(&c) {
                is_valid = true;
            }
            if ('0'..'9').contains(&c) {
                is_valid = true;
            }
            if c == '.' || c == '-' || c == '_' {
                is_valid = true;
            }
            if !is_valid {
                return false;
            }
        }
        return true;
    }
}

fn ignore_bom(input: &str) -> &str {
    const BOM: &'static str = "\u{feff}";
    if input.starts_with(BOM) {
        return &input[BOM.len()..];
    }
    return input;
}