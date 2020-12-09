use std::cell::RefCell;
use std::collections::{HashMap as Map, HashMap};
use std::rc::Rc;

use crate::grammar::StackElement;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::{
    AbstractRule, BeginEndRule, BeginWhileRule, CompiledRule, EmptyRule, IncludeOnlyRule,
    MatchRule, RegExpSourceList,
};

// todo: https://stackoverflow.com/questions/61070398/how-to-create-a-thread-local-variable-inside-of-a-rust-struct
thread_local! {
    pub static RULES: RefCell<HashMap<i32, Rc<dyn AbstractRule>>> = RefCell::new(HashMap::new());
}

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
        let rule_scanner = RuleContainer::compile(
            stack.rule_id,
            &mut self.rules,
            &stack.end_rule,
            allow_a,
            allow_g,
        );
        // let rule_id = stack.rule_id;
        // println!("{:?}", rule_id);
        //
        // let rule_scanner;
        // {
        //     let rc = self
        //         .rules
        //         .get_mut(&rule_id)
        //         .unwrap_or(self._empty_rule.get_mut(&-2).unwrap())
        //         .clone();
        //
        //     let mut rule = rc.borrow_mut();
        //     rule_scanner = rule.compile(&mut self.rules, &stack.end_rule, allow_a, allow_g);
        // }

        return rule_scanner;
    }

    pub fn compile<'rule>(
        rule_id: i32,
        container: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
        end: &Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        // todo: temp for clone
        let rc = container.get(&rule_id).unwrap().clone();
        let mut rule = &mut *rc.borrow_mut();

        let compiled;
        match rule.get_rule_instance() {
            RuleEnum::BeginEndRule(_) => {
                let begin = rule
                    .get_mut_instance()
                    .downcast_mut::<BeginEndRule>()
                    .unwrap();
                compiled =
                    RuleContainer::compile_begin_end_rule(begin, container, end, allow_a, allow_g);
            }
            RuleEnum::BeginWhileRule(_) => {
                let while_rule = rule
                    .get_mut_instance()
                    .downcast_mut::<BeginWhileRule>()
                    .unwrap();
                compiled = RuleContainer::compile_begin_while_rule(
                    while_rule, container, end, allow_a, allow_g,
                );
            }
            RuleEnum::MatchRule(_) => {
                let match_rule = rule.get_mut_instance().downcast_mut::<MatchRule>().unwrap();
                compiled =
                    RuleContainer::compile_match_rule(match_rule, container, end, allow_a, allow_g);
            }
            RuleEnum::CaptureRule(_) => unimplemented!(),
            RuleEnum::EmptyRule(_) => unimplemented!(),
            RuleEnum::IncludeOnlyRule(_) => {
                let include = rule
                    .get_mut_instance()
                    .downcast_mut::<IncludeOnlyRule>()
                    .unwrap();
                compiled =
                    RuleContainer::compile_include_only(include, container, end, allow_a, allow_g);
            }
        };

        return compiled;
    }

    fn compile_include_only(
        rule: &mut IncludeOnlyRule,
        container: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
        _end_regex_source: &Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        if rule._cached_compiled_patterns.is_none() {
            let mut cached_compiled_patterns = RegExpSourceList::new();
            RuleContainer::collect_patterns_recursive(
                rule.id(),
                container,
                &mut cached_compiled_patterns,
                true,
            );

            // self.collect_patterns_recursive(container, &mut cached_compiled_patterns, true);
            rule._cached_compiled_patterns = Some(cached_compiled_patterns);
        }

        return *rule
            ._cached_compiled_patterns
            .as_mut()
            .unwrap()
            .compile(allow_a, allow_g);
    }

    fn compile_match_rule(
        rule: &mut MatchRule,
        container: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
        _end_regex_source: &Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        if rule._cached_compiled_patterns.is_none() {
            let mut cached_compiled_patterns = RegExpSourceList::new();
            // self.collect_patterns_recursive(container, &mut cached_compiled_patterns, true);
            RuleContainer::collect_patterns_recursive(
                rule.id(),
                container,
                &mut cached_compiled_patterns,
                true,
            );
            rule._cached_compiled_patterns = Some(cached_compiled_patterns);
        }

        return *rule
            ._cached_compiled_patterns
            .as_mut()
            .unwrap()
            .compile(allow_a, allow_g);
    }

    fn compile_begin_while_rule(
        rule: &mut BeginWhileRule,
        container: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
        _end_regex_source: &Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        if rule._cached_compiled_patterns.is_none() {
            let mut cached_compiled_patterns = RegExpSourceList::new();

            RuleContainer::collect_patterns_recursive(
                rule.id(),
                container,
                &mut cached_compiled_patterns,
                true,
            );
            rule._cached_compiled_patterns = Some(cached_compiled_patterns);
        }

        return *rule
            ._cached_compiled_patterns
            .as_mut()
            .unwrap()
            .compile(allow_a, allow_g);
    }

    pub fn compile_begin_end_rule(
        rule: &mut BeginEndRule,
        container: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
        end_regex_source: &Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        if rule._cached_compiled_patterns.is_none() {
            let mut cached_compiled_patterns = RegExpSourceList::new();

            RuleContainer::collect_patterns_recursive(
                rule.id(),
                container,
                &mut cached_compiled_patterns,
                true,
            );

            if rule.apply_end_pattern_last {
                cached_compiled_patterns.push(rule._end.clone());
            } else {
                cached_compiled_patterns.unshift(rule._end.clone());
            }

            rule._cached_compiled_patterns = Some(cached_compiled_patterns);
        }

        if rule._end.has_back_references {
            if rule.apply_end_pattern_last {
                let length = rule._cached_compiled_patterns.as_ref().unwrap().length();

                rule._cached_compiled_patterns
                    .as_mut()
                    .unwrap()
                    .set_source(length - 1, end_regex_source.as_ref().unwrap())
            } else {
                rule._cached_compiled_patterns
                    .as_mut()
                    .unwrap()
                    .set_source(0, end_regex_source.as_ref().unwrap())
            }
        }

        return *rule
            ._cached_compiled_patterns
            .as_mut()
            .unwrap()
            .compile(allow_a, allow_g);
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
