use crate::grammar::rule_container::RuleContainer;
use crate::inter::ILocation;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::{AbstractRule, CompiledRule, Rule};
use crate::rule::{RegExpSource, RegExpSourceList};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Serialize)]
pub struct MatchRule {
    pub rule: Rule,
    pub _match: RegExpSource,
    #[serde(skip_serializing)]
    pub captures: Vec<Rc<RefCell<dyn AbstractRule>>>,
    pub _cached_compiled_patterns: Option<RegExpSourceList>,
}

impl MatchRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        _match: String,
        captures: Vec<Rc<RefCell<dyn AbstractRule>>>,
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
    fn get_instance(&self) -> &dyn Any {
        self
    }
    // fn collect_patterns_recursive(
    //     &mut self,
    //     _container: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
    //     out: &mut RegExpSourceList,
    //     _is_first: bool,
    // ) {
    //     out.push(self._match.clone());
    // }

    fn compile(
        &mut self,
        container: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
        _end_regex_source: &Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        if self._cached_compiled_patterns.is_none() {
            let mut cached_compiled_patterns = RegExpSourceList::new();
            // self.collect_patterns_recursive(container, &mut cached_compiled_patterns, true);
            RuleContainer::collect_patterns_recursive(
                self.id(),
                container,
                &mut cached_compiled_patterns,
                true,
            );
            self._cached_compiled_patterns = Some(cached_compiled_patterns);
        }

        return *self
            ._cached_compiled_patterns
            .as_mut()
            .unwrap()
            .compile(allow_a, allow_g);
    }
}
