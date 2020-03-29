use crate::model::clean_params::CleanParams;
use crate::model::group::Group;
use url::{Origin, Url};

#[derive(Debug, Clone)]
/// The robots.txt model that was obtained after parsing the text of the robots.txt file.
/// To work with this model you should use the trait `robotparser::service::RobotsTxtService`.
/// To create this structure you should use the `robotparser::parser::parse_robots_txt`.
pub struct RobotsTxt {
    origin: Origin,
    groups: Vec<Group>,
    sitemaps: Vec<Url>,
    clean_params: Vec<CleanParams>,
}

impl RobotsTxt {
    pub(crate) fn new(origin: Origin) -> RobotsTxt {
        return RobotsTxt {
            origin,
            groups: Vec::new(),
            sitemaps: Vec::new(),
            clean_params: Vec::new(),
        };
    }

    pub(crate) fn add_sitemap(&mut self, url: Url) {
        self.sitemaps.push(url);
    }

    pub(crate) fn get_sitemaps_slice(&self) -> &[Url] {
        return self.sitemaps.as_slice();
    }

    pub(crate) fn add_clean_params(&mut self, clean_params: CleanParams) {
        self.clean_params.push(clean_params);
    }

    pub(crate) fn get_clean_params(&self) -> &[CleanParams] {
        return self.clean_params.as_slice();
    }

    pub(crate) fn add_group(&mut self, group: Group) {
        self.groups.push(group);
    }

    pub(crate) fn get_origin(&self) -> &Origin {
        return &self.origin;
    }

    pub(crate) fn find_in_group<'a, T>(
        &'a self,
        user_agent: &str,
        callback: impl Fn(&'a Group) -> Option<T>,
    ) -> Option<T> {
        // Search by user agents
        for group in self.groups.iter() {
            if group.applies_to(user_agent) {
                if let Some(output) = (callback)(group) {
                    return Some(output);
                }
            }
        }
        if let Some(group) = self.get_default_group() {
            if let Some(output) = (callback)(group) {
                return Some(output);
            }
        }
        return None;
    }

    pub(crate) fn get_default_group(&self) -> Option<&Group> {
        for group in self.groups.iter() {
            if group.is_default() {
                return Some(group);
            }
        }
        return None;
    }
}
