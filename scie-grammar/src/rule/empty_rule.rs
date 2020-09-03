use crate::grammar::Grammar;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::{AbstractRule, CompiledRule, RegExpSourceList, Rule};

#[derive(Clone, Debug, Serialize)]
pub struct EmptyRule {}

impl AbstractRule for EmptyRule {
    fn id(&self) -> i32 {
        0
    }
    fn type_of(&self) -> String {
        String::from("EmptyRule")
    }
    fn get_rule(&self) -> Rule {
        Rule {
            _type: "".to_string(),
            _location: None,
            id: 0,
            _name: None,
            _content_name: None,
        }
    }
    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::EmptyRule(self.clone())
    }
    fn collect_patterns_recursive(
        &mut self,
        grammar: &mut Grammar,
        out: &mut RegExpSourceList,
        is_first: bool,
    ) {
        unimplemented!()
    }

    fn compile(
        &mut self,
        grammar: &mut Grammar,
        end_regex_source: Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        unimplemented!()
    }
}
