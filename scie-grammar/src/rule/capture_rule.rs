use crate::grammar::Grammar;
use crate::inter::ILocation;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::{AbstractRule, CompiledRule, RegExpSourceList, Rule};
use std::any::Any;

#[derive(Clone, Debug, Serialize)]
pub struct CaptureRule {
    pub rule: Rule,
    pub retokenize_captured_with_rule_id: i32,
}

impl CaptureRule {
    pub fn empty() -> Self {
        CaptureRule {
            rule: Rule {
                _type: "CaptureRule".to_string(),
                _location: None,
                id: 0,
                _name: None,
                _content_name: None,
            },
            retokenize_captured_with_rule_id: 0,
        }
    }
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        retokenize_captured_with_rule_id: i32,
    ) -> Self {
        CaptureRule {
            rule: Rule {
                _type: String::from("CaptureRule"),
                _location: location,
                id,
                _name: name,
                _content_name: content_name,
            },
            retokenize_captured_with_rule_id,
        }
    }
}

impl AbstractRule for CaptureRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn type_of(&self) -> &'static str {
        "CaptureRule"
    }
    fn get_rule(&self) -> &Rule {
        &self.rule
    }
    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::CaptureRule(self)
    }
    fn get_instance(&self) -> &dyn Any {
        self
    }
    fn collect_patterns_recursive(
        &mut self,
        _grammar: &mut Grammar,
        _out: &mut RegExpSourceList,
        _is_first: bool,
    ) {
        unimplemented!()
    }

    fn compile(
        &mut self,
        _grammar: &mut Grammar,
        _end_regex_source: &Option<String>,
        _allow_a: bool,
        _allow_g: bool,
    ) -> CompiledRule {
        unimplemented!()
    }
}
