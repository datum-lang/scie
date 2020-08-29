pub mod rule_factory;
pub mod rule_convert;
pub mod reg_exp_source;

use crate::inter::{ILocation, IRawGrammar, IRawRepository};
use reg_exp_source::{RegExpSource, RegExpSourceList};
use crate::rule::rule_factory::ICompilePatternsResult;
use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};
use serde::{Serialize, Serializer};

#[derive(Clone, Debug, Serialize)]
pub struct Rule {
    pub _type: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub _location: Option<ILocation>,
    pub id: i32,
    #[serde(skip_serializing_if="Option::is_none")]
    pub _name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub _content_name: Option<String>,
}

impl Rule {
    pub fn new(
        location: ILocation,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
    ) -> Self {
        Rule {
            _type: "".to_string(),
            _location: Some(location),
            id,
            _name: name,
            _content_name: content_name,
        }
    }
}

pub trait AbstractRule: DynClone + erased_serde::Serialize {
    fn id(&self) -> i32;
    fn type_of(&self) -> String;
    fn has_missing_pattern(&self) -> bool {
        false
    }
}

serialize_trait_object!(AbstractRule);

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AbstractRule")
    }
}

clone_trait_object!(AbstractRule);

#[derive(Clone, Debug, Serialize)]
pub struct IncludeOnlyRule {
    #[serde(flatten)]
    pub rule: Rule,
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool
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
            has_missing_patterns: captures.has_missing_patterns
        }
    }
}

impl AbstractRule for IncludeOnlyRule {
    fn id(&self) -> i32 { self.rule.id }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }

    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BeginWhileRule {
    pub rule: Rule,

    pub _begin: RegExpSource,
    pub begin_captures: Vec<Box<dyn AbstractRule>>,

    pub _while: Option<String>,
    pub while_captures: Vec<Box<dyn AbstractRule>>,

    pub apply_end_pattern_last: bool,

    pub patterns: ICompilePatternsResult,
    pub has_missing_patterns: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub cached_compiled_patterns: Option<RegExpSourceList>,
}

impl BeginWhileRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        _begin: Option<String>,
        begin_captures: Vec<Box<dyn AbstractRule>>,
        _while: Option<String>,
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
            _while,
            while_captures,
            apply_end_pattern_last: false,
            has_missing_patterns: patterns.clone().has_missing_patterns,
            patterns,
            cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for BeginWhileRule {
    fn id(&self) -> i32 { self.rule.id }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }
    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MatchRule {
    pub rule: Rule,
    pub _match: RegExpSource,
    pub captures: Vec<Box<dyn AbstractRule>>,
}

impl MatchRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        _match: String,
        captures: Vec<Box<dyn AbstractRule>>,
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
        }
    }
}

impl AbstractRule for MatchRule {
    fn id(&self) -> i32 { self.rule.id }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BeginEndRule {
    #[serde(flatten)]
    pub rule: Rule,
    pub _begin: RegExpSource,
    pub begin_captures: Vec<Box<dyn AbstractRule>>,
    pub _end: RegExpSource,
    // pub endHasBackReferences: Option<bool>,
    pub end_captures: Vec<Box<dyn AbstractRule>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub apply_end_pattern_last: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
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
            _end: RegExpSource::new(_end.clone(), id.clone()),
            end_captures,
            apply_end_pattern_last,
            has_missing_patterns: patterns.clone().has_missing_patterns,
            patterns: patterns.patterns,
            _cached_compiled_patterns: None
        }
    }
}

impl AbstractRule for BeginEndRule {
    fn id(&self) -> i32 { self.rule.id }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }
    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CaptureRule {
    pub rule: Rule,
    pub retokenize_captured_with_rule_id: i32
}

impl CaptureRule {
    pub fn empty() -> Self {
        CaptureRule {
            rule: Rule {
                _type: "".to_string(),
                _location: None,
                id: 0,
                _name: None,
                _content_name: None
            },
            retokenize_captured_with_rule_id: 0
        }
    }
    pub fn new(location: Option<ILocation>, id: i32, name: Option<String>, content_name: Option<String>, retokenize_captured_with_rule_id: i32) -> Self {
        CaptureRule {
            rule: Rule {
                _type: String::from("CaptureRule"),
                _location: location,
                id,
                _name: name,
                _content_name: content_name,
            },
            retokenize_captured_with_rule_id
        }
    }
}

impl AbstractRule for CaptureRule {
    fn id(&self) -> i32 { self.rule.id }
    fn type_of(&self) -> String { String::from(self.rule.clone()._type) }
}

#[derive(Clone, Debug, Serialize)]
pub struct NoneRule {}

impl AbstractRule for NoneRule {
    fn id(&self) -> i32 { 0 }
    fn type_of(&self) -> String {
        String::from("NoneRule")
    }
}

// todo: trait with types
// https://users.rust-lang.org/t/impl-trait-with-generic-function-for-generic-struct/27083/2
pub trait IRuleRegistry {
    // type Output;
    // fn method(&self) -> Self::Output;
    fn register_id(&mut self) -> i32;
    fn get_rule(&self, pattern_id: i32) -> Box<dyn AbstractRule>;
    fn register_rule(&mut self, result: Box<dyn AbstractRule>) -> Box<dyn AbstractRule>;
}

pub trait IGrammarRegistry {
    fn get_external_grammar(
        &self,
        scope_name: String,
        repository: IRawRepository,
    ) -> Option<IRawGrammar>;
}

pub trait IRuleFactoryHelper: IGrammarRegistry + IRuleRegistry {}
