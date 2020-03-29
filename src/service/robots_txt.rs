use crate::model::Path;
use crate::model::RequestRate;
use crate::model::RobotsTxt;
use crate::service::RobotsTxtService;
use std::time::Duration;
use url::Url;

impl RobotsTxtService for RobotsTxt {
    fn can_fetch(&self, user_agent: &str, url: &Url) -> bool {
        if url.origin() != *self.get_origin() {
            return false;
        }
        let path = Path::from_url(url);
        let rule_decision = self.find_in_group(user_agent, |group| {
            let rules = group.get_rules_sorted_by_path_len_desc();
            for rule in rules.iter() {
                if rule.applies_to(&path) {
                    return Some(rule.get_allowance());
                }
            }
            None
        });
        if let Some(rule_decision) = rule_decision {
            return rule_decision;
        }
        // Empty robots.txt allows crawling. Everything that was not denied must be allowed.
        true
    }

    fn get_crawl_delay(&self, user_agent: &str) -> Option<Duration> {
        self.find_in_group(user_agent, |group| group.get_crawl_delay())
    }

    fn normalize_url(&self, url: &mut Url) -> bool {
        if url.origin() != *self.get_origin() {
            return false;
        }
        self.normalize_url_ignore_origin(url);
        true
    }

    fn normalize_url_ignore_origin(&self, url: &mut Url) {
        if url.query().is_none() {
            return;
        }
        let mut query_params_to_filter = Vec::new();
        let path = Path::from_url(url);
        for clean_params in self.get_clean_params().iter() {
            if clean_params.get_path_pattern().applies_to(&path) {
                query_params_to_filter.extend_from_slice(clean_params.get_params())
            }
        }
        let mut pairs: Vec<(String, String)> = url
            .query_pairs()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        {
            let mut query_pairs_mut = url.query_pairs_mut();
            query_pairs_mut.clear();
            for (key, value) in pairs.drain(..) {
                if !query_params_to_filter.contains(&key) {
                    query_pairs_mut.append_pair(&key, &value);
                }
            }
        }
        if url.query() == Some("") {
            url.set_query(None);
        }
    }

    fn get_sitemaps(&self) -> &[Url] {
        self.get_sitemaps_slice()
    }

    fn get_req_rate(&self, user_agent: &str) -> Option<RequestRate> {
        self.find_in_group(user_agent, |group| group.get_req_rate())
    }
}
