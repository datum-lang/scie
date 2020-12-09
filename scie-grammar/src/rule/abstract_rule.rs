use core::fmt;
use std::any::Any;

use dyn_clone::{clone_trait_object, DynClone};

use scie_scanner::scanner::scie_scanner::IOnigCaptureIndex;

use crate::rule::{
    BeginEndRule, BeginWhileRule, CaptureRule, EmptyRule, IncludeOnlyRule, MatchRule, Rule,
};
use crate::support::regex_source::RegexSource;

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
    fn type_of(&self) -> &'static str {
        "AbstractRule"
    }
    fn get_rule(&self) -> &Rule;
    fn get_rule_instance(&self) -> RuleEnum;
    fn get_mut_instance(&mut self) -> &mut dyn Any;
    fn get_instance(&self) -> &dyn Any;
    fn get_name(
        &self,
        line_text: Option<String>,
        capture_indices: Option<&Vec<IOnigCaptureIndex>>,
    ) -> Option<String> {
        let name = self.get_rule()._name.clone();
        let has_captures = RegexSource::has_captures(&name);
        if let None = capture_indices {
            return name;
        }

        if !has_captures || name == None || line_text == None {
            return name;
        }

        let string = RegexSource::replace_captures(
            String::from((self.get_rule()._name).as_ref().unwrap()),
            line_text.unwrap(),
            capture_indices.unwrap(),
        );

        return Some(string);
    }

    fn get_content_name(
        &self,
        _line_text: Option<String>,
        _capture_indices: Option<&Vec<IOnigCaptureIndex>>,
    ) -> Option<String> {
        let _content_name = self.get_rule()._content_name.clone();
        let has_captures = RegexSource::has_captures(&_content_name);
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
}

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

serialize_trait_object!(AbstractRule);

clone_trait_object!(AbstractRule);
