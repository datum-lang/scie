use crate::grammar::Grammar;
use crate::rule::{
    BeginEndRule, BeginWhileRule, CaptureRule, CompiledRule, EmptyRule, IncludeOnlyRule, MatchRule,
    RegExpSourceList, Rule,
};
use crate::support::regex_source::RegexSource;
use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};
use scie_scanner::scanner::scie_scanner::IOnigCaptureIndex;
use std::any::Any;

pub enum RuleEnum<'r> {
    BeginEndRule(&'r BeginEndRule),
    BeginWhileRule(&'r BeginWhileRule),
    CaptureRule(&'r CaptureRule),
    MatchRule(&'r MatchRule),
    EmptyRule(&'r EmptyRule),
    IncludeOnlyRule(&'r IncludeOnlyRule),
}

pub trait AbstractRule: DynClone + erased_serde::Serialize {
    fn id(&self) -> i32;
    fn type_of(&self) -> String;
    fn display(&self) -> String {
        String::from("AbstractRule")
    }
    // todo: add support for this;
    fn get_rule(&self) -> &Rule;
    fn get_rule_instance(&self) -> RuleEnum;
    fn get_instance(&self) -> &dyn Any;
    fn get_name(
        &self,
        line_text: Option<String>,
        capture_indices: Option<&Vec<IOnigCaptureIndex>>,
    ) -> Option<String> {
        let name = self.get_rule()._name.clone();
        let has_captures = RegexSource::has_captures(name.clone());
        if let None = capture_indices {
            return name;
        }
        if !has_captures || name == None || line_text == None {
            return name;
        }

        RegexSource::replace_captures(
            String::from((self.get_rule()._name).as_ref().unwrap()),
            line_text.unwrap(),
            capture_indices.unwrap(),
        );

        return Some(String::from(""));
    }

    fn get_content_name(
        &self,
        _line_text: Option<String>,
        _capture_indices: Option<&Vec<IOnigCaptureIndex>>,
    ) -> Option<String> {
        let _content_name = self.get_rule()._content_name.clone();
        let has_captures = RegexSource::has_captures(_content_name.clone());
        if !has_captures || _content_name == None {
            return _content_name;
        }

        println!("todo: AbstractRule.get_name");
        return Some(String::from(""));
    }

    fn has_missing_pattern(&self) -> bool {
        false
    }
    fn patterns_length(&self) -> i32 {
        -1
    }
    fn collect_patterns_recursive<'a>(
        &mut self,
        grammar: &'a mut Grammar,
        out: &mut RegExpSourceList,
        is_first: bool,
    );
    fn compile(
        &mut self,
        grammar: &mut Grammar,
        end_regex_source: &Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule;
}

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

serialize_trait_object!(AbstractRule);

clone_trait_object!(AbstractRule);
