use crate::inter::ILocation;
use crate::rule::{AbstractRule, Rule, RegExpSourceList, CompiledRule};
use crate::grammar::Grammar;

#[derive(Clone, Debug, Serialize)]
pub struct CaptureRule {
    pub rule: Rule,
    pub retokenize_captured_with_rule_id: i32,
}

impl CaptureRule {
    pub fn empty() -> Self {
        CaptureRule {
            rule: Rule {
                _type: "".to_string(),
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
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }

    fn collect_patterns_recursive(&mut self, grammar: &mut Grammar, out: &mut RegExpSourceList, is_first: bool) {
        unimplemented!()
    }

    fn compile(&mut self, grammar: &mut Grammar, end_regex_source: Option<String>, allow_a: bool, allow_g: bool) -> CompiledRule {
        unimplemented!()
    }
}
