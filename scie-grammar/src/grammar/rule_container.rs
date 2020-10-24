use crate::rule::AbstractRule;
use std::collections::HashMap as Map;

#[derive(Debug, Clone)]
pub struct RuleContainer {
    pub rule_id2desc: Map<i32, Box<dyn AbstractRule>>,
}

impl Default for RuleContainer {
    fn default() -> Self {
        RuleContainer {
            rule_id2desc: Default::default(),
        }
    }
}
