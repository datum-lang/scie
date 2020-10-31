use crate::grammar::rule_container::RuleContainer;
use crate::inter::ILocation;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::rule_factory::ICompilePatternsResult;
use crate::rule::{AbstractRule, CompiledRule, Rule};
use crate::rule::{RegExpSource, RegExpSourceList};
use std::any::Any;

#[derive(Clone, Debug, Serialize)]
pub struct BeginWhileRule {
    pub rule: Rule,
    pub _begin: RegExpSource,
    pub begin_captures: Vec<Box<dyn AbstractRule>>,
    pub _while: RegExpSource,
    pub while_captures: Vec<Box<dyn AbstractRule>>,
    pub apply_end_pattern_last: bool,
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _cached_compiled_patterns: Option<RegExpSourceList>,
}

impl BeginWhileRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        _begin: Option<String>,
        begin_captures: Vec<Box<dyn AbstractRule>>,
        _while: String,
        while_captures: Vec<Box<dyn AbstractRule>>,
        patterns: ICompilePatternsResult,
    ) -> BeginWhileRule {
        BeginWhileRule {
            rule: Rule {
                _type: String::from("BeginWhileRule"),
                _location: location,
                id,
                _name: name,
                _content_name: content_name,
            },
            _begin: RegExpSource::new(_begin.unwrap().clone(), id.clone()),
            begin_captures,
            _while: RegExpSource::new(_while, -2),
            while_captures,
            apply_end_pattern_last: false,
            has_missing_patterns: patterns.clone().has_missing_patterns,
            patterns: patterns.patterns,
            _cached_compiled_patterns: None,
        }
    }

    pub fn compile_while(
        &mut self,
        end_regex_source: Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        if self._cached_compiled_patterns.is_none() {
            let mut compiled_patterns = RegExpSourceList::new();
            compiled_patterns.push(self._while.clone());

            self._cached_compiled_patterns = Some(compiled_patterns);
        }

        if self._while.has_back_references {
            let end_regex: String;
            if end_regex_source.is_none() {
                end_regex = String::from("\u{FFFF}");
            } else {
                end_regex = end_regex_source.unwrap().clone();
            }

            self._cached_compiled_patterns
                .as_mut()
                .unwrap()
                .set_source(0, end_regex.as_str());
        }

        return *self
            ._cached_compiled_patterns
            .as_mut()
            .unwrap()
            .compile(allow_a, allow_g);
    }
}

impl AbstractRule for BeginWhileRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn type_of(&self) -> &'static str {
        "BeginWhileRule"
    }
    fn get_rule(&self) -> &Rule {
        &self.rule
    }
    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::BeginWhileRule(self)
    }
    fn get_instance(&self) -> &dyn Any {
        self
    }
    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }
    fn patterns_length(&self) -> i32 {
        self.patterns.len() as i32
    }

    fn collect_patterns_recursive(
        &mut self,
        container: &mut RuleContainer,
        out: &mut RegExpSourceList,
        is_first: bool,
    ) {
        if is_first {
            for x in self.patterns.iter() {
                let mut rule = container.get_rule(*x).clone();
                rule.collect_patterns_recursive(container, out, false);
                container.register_rule(rule);
            }
        } else {
            out.push(self._begin.clone());
        }
    }

    fn compile(
        &mut self,
        container: &mut RuleContainer,
        _end_regex_source: &Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        if self._cached_compiled_patterns.is_none() {
            let mut cached_compiled_patterns = RegExpSourceList::new();

            self.collect_patterns_recursive(container, &mut cached_compiled_patterns, true);
            self._cached_compiled_patterns = Some(cached_compiled_patterns);
        }

        return *self
            ._cached_compiled_patterns
            .as_mut()
            .unwrap()
            .compile(allow_a, allow_g);
    }
}
