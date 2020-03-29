use crate::model::request_rate::RequestRate;
use crate::model::rule::Rule;
use std::time::Duration;

/// An group has one or more user-agents and zero or more rules
#[derive(Debug, Clone)]
pub struct Group {
    user_agents: Vec<String>,
    rules: Vec<Rule>,
    crawl_delay: Option<Duration>,
    req_rate: Option<RequestRate>,
}

impl Group {
    pub(crate) fn new() -> Group {
        Group {
            user_agents: vec![],
            rules: vec![],
            crawl_delay: None,
            req_rate: None,
        }
    }

    /// check if this group applies to the specified agent
    pub(crate) fn applies_to(&self, useragent: &str) -> bool {
        let ua = useragent.split('/').next().unwrap_or("").to_lowercase();
        for agent in self.user_agents.iter() {
            if ua.contains(agent) {
                return true;
            }
        }
        false
    }

    pub(crate) fn push_useragent(&mut self, useragent: &str) {
        self.user_agents.push(useragent.to_lowercase());
    }

    pub(crate) fn push_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub(crate) fn get_rules_sorted_by_path_len_desc(&self) -> Vec<&Rule> {
        let mut rules: Vec<&Rule> = self.rules.iter().collect();
        rules.sort_by(|a, b| {
            let a = a.get_path_pattern().len();
            let b = b.get_path_pattern().len();
            b.cmp(&a)
        });
        rules
    }

    pub(crate) fn contains_user_agent(&self, user_agent: &str) -> bool {
        self
            .user_agents
            .iter()
            .any(|item| {
                *item == user_agent
            })
    }

    pub(crate) fn set_crawl_delay(&mut self, delay: Duration) {
        self.crawl_delay = Some(delay);
    }

    pub(crate) fn get_crawl_delay(&self) -> Option<Duration> {
        self.crawl_delay
    }

    pub(crate) fn set_req_rate(&mut self, req_rate: RequestRate) {
        self.req_rate = Some(req_rate);
    }

    pub(crate) fn get_req_rate(&self) -> Option<RequestRate> {
        self.req_rate.clone()
    }

    pub(crate) fn is_default(&self) -> bool {
        for user_agent in self.user_agents.iter() {
            if user_agent == "*" {
                return true;
            }
        }
        false
    }
}

impl Default for Group {
    fn default() -> Group {
        Group::new()
    }
}
