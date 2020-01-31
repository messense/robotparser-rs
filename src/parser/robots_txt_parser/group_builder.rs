use crate::model::{Group, RobotsTxt};
enum State {
    WaitingForNewGroup,
    WaitingForAdditionalUserAgent,
}

pub struct GroupBuilder {
    state: State,
    active_group: Option<usize>,
    groups: Vec<Group>,
}

impl GroupBuilder {
    pub fn new() -> GroupBuilder {
        return GroupBuilder {
            state: State::WaitingForNewGroup,
            active_group: None,
            groups: Vec::new(),
        }
    }

    pub fn handle_user_agent(&mut self, user_agent: &str) {
        match self.state {
            State::WaitingForNewGroup => {
                let mut group = Group::new();
                group.push_useragent(user_agent);
                self.groups.push(group);
                self.active_group = Some(self.groups.len() - 1);
                self.state = State::WaitingForAdditionalUserAgent;
            },
            State::WaitingForAdditionalUserAgent => {
                let active_group = self.active_group.expect("Unable to get active group");
                let group = self.groups.get_mut(active_group).expect("Unable to get group index");
                if !group.contains_user_agent(user_agent) {
                    group.push_useragent(user_agent);
                }
            },
        }
    }

    pub fn get_mut_active_group(&mut self) -> Option<&mut Group> {
        self.state = State::WaitingForNewGroup;
        if let Some(active_group) = self.active_group {
            return self.groups.get_mut(active_group);
        }
        return None;
    }

    pub fn fill_entries(mut self, robots_txt: &mut RobotsTxt) {
        for group in self.groups.drain(..) {
            robots_txt.add_group(group);
        }
    }
}