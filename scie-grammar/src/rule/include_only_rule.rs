use std::any::Any;

use crate::grammar::rule_container::RuleContainer;
use crate::inter::ILocation;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::rule_factory::ICompilePatternsResult;
use crate::rule::{AbstractRule, CompiledRule, RegExpSourceList, Rule};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize)]
pub struct IncludeOnlyRule {
    #[serde(flatten)]
    pub rule: Rule,
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
    pub _cached_compiled_patterns: Option<RegExpSourceList>,
}

impl IncludeOnlyRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        captures: ICompilePatternsResult,
    ) -> Self {
        IncludeOnlyRule {
            rule: Rule {
                _type: String::from("IncludeOnlyRule"),
                _location: location,
                id,
                _name: name,
                _content_name: content_name,
            },
            patterns: captures.patterns,
            has_missing_patterns: captures.has_missing_patterns,
            _cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for IncludeOnlyRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn type_of(&self) -> &'static str {
        "IncludeOnlyRule"
    }
    fn get_rule(&self) -> &Rule {
        &self.rule
    }
    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::IncludeOnlyRule(self)
    }
    fn get_instance(&mut self) -> &mut dyn Any {
        self
    }
    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }
    fn patterns_length(&self) -> i32 {
        self.patterns.len() as i32
    }
    //
    // fn collect_patterns_recursive(
    //     &mut self,
    //     container: &mut HashMap<i32, Box<dyn AbstractRule>>,
    //     out: &mut RegExpSourceList,
    //     _is_first: bool,
    // ) {
    //     for x in self.patterns.iter() {
    //         let mut rule = container.get_rule(*x).clone();
    //         rule.collect_patterns_recursive(container, out, false);
    //         container.register_rule(rule);
    //     }
    // }

    fn compile(
        &mut self,
        container: &mut HashMap<i32, Box<dyn AbstractRule>>,
        _end_regex_source: &Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        if self._cached_compiled_patterns.is_none() {
            let mut cached_compiled_patterns = RegExpSourceList::new();
            RuleContainer::collect_patterns_recursive(
                self.id(),
                container,
                &mut cached_compiled_patterns,
                true,
            );

            // self.collect_patterns_recursive(container, &mut cached_compiled_patterns, true);
            self._cached_compiled_patterns = Some(cached_compiled_patterns);
        }

        return *self
            ._cached_compiled_patterns
            .as_mut()
            .unwrap()
            .compile(allow_a, allow_g);
    }
}
