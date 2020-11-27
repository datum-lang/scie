use crate::grammar::StackElement;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::{AbstractRule, CompiledRule, EmptyRule, RegExpSourceList};
use std::cell::RefCell;
use std::collections::{HashMap as Map, HashMap};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct RuleContainer {
    pub _empty_rule: Map<i32, Box<dyn AbstractRule>>,
    pub rule_id2desc: Map<i32, Box<dyn AbstractRule>>,
    pub rules: HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
}

impl Default for RuleContainer {
    fn default() -> Self {
        let mut _empty_rule = Map::new();

        let mut container = RuleContainer {
            _empty_rule,
            rule_id2desc: Default::default(),
            rules: Default::default(),
        };

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
        self.rule_id2desc.insert(id, result);
        id
    }

    pub fn compile_rule(
        &mut self,
        stack: &mut StackElement,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        let rule_id = stack.rule_id;
        // https://stackoverflow.com/questions/44453398/how-can-i-borrow-from-a-hashmap-to-read-and-write-at-the-same-time
        let mut rule = self.get_rule(rule_id).clone();
        // let mut rule = &mut self.rules[id];
        let rule_scanner = rule.compile(&mut self.rule_id2desc, &stack.end_rule, allow_a, allow_g);

        self.register_rule(rule);

        rule_scanner
    }

    pub fn collect_patterns_recursive(
        pattern_id: i32,
        rules: &mut HashMap<i32, Box<dyn AbstractRule>>,
        mut out: &mut RegExpSourceList,
        is_first: bool,
    ) {
        let match_rule = rules.get_mut(&pattern_id).unwrap();
        match match_rule.get_rule_instance() {
            RuleEnum::BeginEndRule(rule) => {
                if is_first {
                    for id in rule.patterns.clone().iter() {
                        RuleContainer::collect_patterns_recursive(*id, rules, &mut out, false);
                    }
                } else {
                    &mut out.push(rule._begin.clone());
                }
            }
            RuleEnum::BeginWhileRule(rule) => {
                if is_first {
                    for id in rule.patterns.clone().iter() {
                        RuleContainer::collect_patterns_recursive(*id, rules, &mut out, false);
                    }
                } else {
                    out.push(rule._begin.clone());
                }
            }
            RuleEnum::CaptureRule(_) => {}
            RuleEnum::MatchRule(rule) => out.push(rule._match.clone()),
            RuleEnum::EmptyRule(_) => {}
            RuleEnum::IncludeOnlyRule(rule) => {
                for id in rule.patterns.clone().iter() {
                    RuleContainer::collect_patterns_recursive(*id, rules, &mut out, false);
                }
            }
        }
    }
}
