use crate::grammar::Grammar;
use crate::inter::ILocation;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::rule_factory::ICompilePatternsResult;
use crate::rule::{AbstractRule, CompiledRule, IRuleRegistry, Rule};
use crate::rule::{RegExpSource, RegExpSourceList};

#[derive(Clone, Debug, Serialize)]
pub struct BeginEndRule {
    #[serde(flatten)]
    pub rule: Rule,
    pub _begin: RegExpSource,
    // todo: refactor to RuleEnum
    pub begin_captures: Vec<Box<dyn AbstractRule>>,
    pub _end: RegExpSource,
    pub end_has_back_references: bool,
    pub end_captures: Vec<Box<dyn AbstractRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_end_pattern_last: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _cached_compiled_patterns: Option<RegExpSourceList>,
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
}

impl BeginEndRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        begin: String,
        begin_captures: Vec<Box<dyn AbstractRule>>,
        _end: String,
        end_captures: Vec<Box<dyn AbstractRule>>,
        apply_end_pattern_last: Option<bool>,
        patterns: ICompilePatternsResult,
    ) -> BeginEndRule {
        let end = RegExpSource::new(_end.clone(), -1);
        BeginEndRule {
            rule: Rule {
                _type: String::from("BeginEndRule"),
                _location: location,
                id,
                _name: name,
                _content_name: content_name,
            },
            _begin: RegExpSource::new(begin.clone(), id.clone()),
            begin_captures,
            // todo new RegExpSource(end ? end : '\uFFFF', -1);
            _end: end,
            end_has_back_references: end.has_back_references.clone(),
            end_captures,
            apply_end_pattern_last,
            has_missing_patterns: patterns.clone().has_missing_patterns,
            patterns: patterns.patterns,
            _cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for BeginEndRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }
    fn get_rule(&self) -> Rule {
        self.rule.clone()
    }

    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::BeginEndRule(self.clone())
    }

    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }
    fn patterns_length(&self) -> i32 {
        self.patterns.clone().len() as i32
    }

    fn collect_patterns_recursive(
        &mut self,
        grammar: &mut Grammar,
        mut out: &mut RegExpSourceList,
        is_first: bool,
    ) {
        if is_first {
            for x in self.patterns.clone() {
                let mut rule = grammar.get_rule(x);
                // match rule.get_rule_instance() {
                //     RuleEnum::BeginEndRule(rule) => {println!("{:?}", rule)},
                //     RuleEnum::BeginWhileRule(rule) => {println!("{:?}", rule)},
                //     RuleEnum::CaptureRule(rule) => {println!("{:?}", rule)},
                //     RuleEnum::MatchRule(rule) => {println!("{:?}", rule)},
                //     RuleEnum::EmptyRule(rule) => {println!("{:?}", rule)},
                //     RuleEnum::IncludeOnlyRule(rule) => {println!("{:?}", rule)},
                // }
                rule.collect_patterns_recursive(grammar, &mut out, false);
            }
        } else {
            &mut out.push(Box::from(self._begin.clone()));
        }
    }

    fn compile(
        &mut self,
        grammar: &mut Grammar,
        _end_regex_source: Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        let mut cached_compiled_patterns = RegExpSourceList::new();

        if let None = self._cached_compiled_patterns {
            self.collect_patterns_recursive(grammar, &mut cached_compiled_patterns, true);
            if let Some(apply_end) = self.apply_end_pattern_last {
                if apply_end {
                    cached_compiled_patterns.push(Box::new(self._end.clone()));
                } else {
                    cached_compiled_patterns.unshift(Box::new(self._end.clone()));
                }
            } else {
                cached_compiled_patterns.unshift(Box::new(self._end.clone()));
            }
        } else {
            cached_compiled_patterns = self._cached_compiled_patterns.as_ref().unwrap().clone();
        }

        // self._end.has_anchor
        // println!("todo: support for hasBackReferences");
        let compiled_rule = cached_compiled_patterns.compile(grammar, allow_a, allow_g);
        self._cached_compiled_patterns = Some(cached_compiled_patterns.clone());
        *compiled_rule
    }
}
