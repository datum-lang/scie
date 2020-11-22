use crate::rule::{AbstractRule, EmptyRule};
use std::collections::HashMap as Map;

#[derive(Debug, Clone)]
pub struct RuleContainer {
    pub _empty_rule: Map<i32, Box<dyn AbstractRule>>,
    pub rule_id2desc: Map<i32, Box<dyn AbstractRule>>,
    pub rules: Vec<Box<dyn AbstractRule>>,
}

const RULE_SIZE: usize = 100;

impl Default for RuleContainer {
    fn default() -> Self {
        let mut _empty_rule = Map::new();

        let mut container = RuleContainer {
            _empty_rule,
            rule_id2desc: Default::default(),
            rules: vec![],
        };

        container
            .rules
            .resize_with(RULE_SIZE, || Box::new(EmptyRule {}));
        container._empty_rule.insert(-2, Box::new(EmptyRule {}));
        container
    }
}

impl RuleContainer {
    pub fn get_rule(&mut self, pattern_id: i32) -> &mut Box<dyn AbstractRule> {
        return self
            .rule_id2desc
            .get_mut(&pattern_id)
            .unwrap_or(self._empty_rule.get_mut(&-2).unwrap());
    }

    pub fn register_rule(&mut self, result: Box<dyn AbstractRule>) -> i32 {
        let id = result.id();
        self.rules
            .resize_with((id + 1) as usize, || Box::from(EmptyRule {}));
        self.rules[id as usize] = result.clone();
        self.rule_id2desc.insert(id, result);
        id
    }
}
