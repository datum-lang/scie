use std::any::Any;

use scie_scanner::scanner::scie_scanner::IOnigCaptureIndex;

use crate::inter::ILocation;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::rule_factory::ICompilePatternsResult;
use crate::rule::{AbstractRule, Rule};
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
    pub apply_end_pattern_last: bool,
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

        let apply_end;
        if apply_end_pattern_last.is_none() {
            apply_end = false;
        } else {
            apply_end = apply_end_pattern_last.unwrap()
        }

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
            end_has_back_references: end.has_back_references.to_owned(),
            _end: end,
            end_captures,
            apply_end_pattern_last: apply_end,
            has_missing_patterns: patterns.to_owned().has_missing_patterns,
            patterns: patterns.patterns,
            _cached_compiled_patterns: None,
        }
    }

    pub fn get_end_with_resolved_back_references(
        &self,
        line_text: &str,
        capture_indices: Vec<IOnigCaptureIndex>,
    ) -> String {
        return self
            ._end
            .resolve_back_references(line_text, capture_indices);
    }
}

impl AbstractRule for BeginEndRule {
    fn id(&self) -> i32 {
        self.rule.id
    }

    fn type_of(&self) -> &'static str {
        "BeginEndRule"
    }

    fn get_rule(&self) -> &Rule {
        &self.rule
    }

    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::BeginEndRule(self)
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
}
