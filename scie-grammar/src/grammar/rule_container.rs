use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::grammar::StackElement;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::{AbstractRule, CompiledRule, EmptyRule, RegExpSourceList};

#[derive(Debug, Clone, Serialize)]
pub struct RuleContainer {
    #[serde(skip_serializing)]
    pub _empty_rule: HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
    #[serde(skip_serializing)]
    pub rules: HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
}

impl Default for RuleContainer {
    fn default() -> Self {
        let mut _empty_rule = HashMap::new();

        let mut container = RuleContainer {
            _empty_rule,
            rules: Default::default(),
        };

        container
            ._empty_rule
            .insert(-2, Rc::new(RefCell::new(EmptyRule {})));
        container
    }
}

impl RuleContainer {
    pub fn get_rule(&mut self, pattern_id: i32) -> Rc<RefCell<dyn AbstractRule>> {
        let option = self
            .rules
            .get(&pattern_id)
            .unwrap_or(self._empty_rule.get_mut(&-2).unwrap());

        return option.clone();
    }

    pub fn register_rule(&mut self, result: Rc<RefCell<dyn AbstractRule>>) -> i32 {
        let id = result.borrow().id();
        self.rules.insert(id, result);
        id
    }

    pub fn compile_rule(
        &mut self,
        stack: &mut StackElement,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        let rule_id = stack.rule_id;

        println!("{:?}", rule_id);

        let rule_scanner;
        {
            let rc = self
                .rules
                .get_mut(&rule_id)
                .unwrap_or(self._empty_rule.get_mut(&-2).unwrap())
                .clone();

            let mut rule = rc.borrow_mut();
            rule_scanner = rule.compile(&mut self.rules, &stack.end_rule, allow_a, allow_g);
        }

        return rule_scanner;
    }

    pub fn collect_patterns_recursive(
        pattern_id: i32,
        rules: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
        mut out: &mut RegExpSourceList,
        is_first: bool,
    ) {
        println!("{:?}", pattern_id);
        let rc = rules.get(&pattern_id).unwrap().clone();
        let match_rule = &*rc.borrow();
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
