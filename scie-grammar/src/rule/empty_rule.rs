use std::any::Any;

use crate::rule::abstract_rule::RuleEnum;
use crate::rule::{AbstractRule, Rule};

#[derive(Clone, Debug, Serialize)]
pub struct EmptyRule {}

impl EmptyRule {
    pub fn new() -> Self {
        EmptyRule {}
    }
}

lazy_static! {
    static ref EMPTY_RULE: Rule = Rule {
        _type: "".to_string(),
        _location: None,
        id: 0,
        _name: None,
        _content_name: None,
    };
}

impl AbstractRule for EmptyRule {
    fn id(&self) -> i32 {
        0
    }
    fn type_of(&self) -> &'static str {
        "EmptyRule"
    }
    fn get_rule(&self) -> &Rule {
        &EMPTY_RULE
    }
    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::EmptyRule(self)
    }
    fn get_instance(&mut self) -> &mut dyn Any {
        self
    }
}
