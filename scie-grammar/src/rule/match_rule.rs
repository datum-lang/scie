use std::any::Any;

use crate::inter::ILocation;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::{AbstractRule, Rule};
use crate::rule::{RegExpSource, RegExpSourceList};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Serialize)]
pub struct MatchRule {
    pub rule: Rule,
    pub _match: RegExpSource,
    #[serde(skip_serializing)]
    pub captures: Vec<Rc<dyn AbstractRule>>,
    pub _cached_compiled_patterns: Option<RegExpSourceList>,
}

impl MatchRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        _match: String,
        captures: Vec<Rc<dyn AbstractRule>>,
    ) -> Self {
        MatchRule {
            rule: Rule {
                _type: String::from("MatchRule"),
                _location: location,
                id,
                _name: name,
                _content_name: None,
            },
            _match: RegExpSource::new(_match, id),
            captures,
            _cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for MatchRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn type_of(&self) -> &'static str {
        "MatchRule"
    }
    fn get_rule(&self) -> &Rule {
        &self.rule
    }
    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::MatchRule(self)
    }
    fn get_mut_instance(&mut self) -> &mut dyn Any {
        self
    }
    fn get_instance(&self) -> &dyn Any {
        self
    }
}
