pub mod rule_factory;

use crate::inter::{ILocation, IRawGrammar, IRawRepository};
use crate::reg_exp_source::{RegExpSource, RegExpSourceList};
use crate::rule::rule_factory::ICompilePatternsResult;
use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};
use serde::{Serialize, Serializer};

#[derive(Clone, Debug, Serialize)]
pub struct Rule {
    pub _type: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<ILocation>,
    pub id: i32,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_name: Option<String>,
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
            location: Some(location),
            id,
            name,
            content_name,
        }
    }
}

pub trait AbstractRule: DynClone + erased_serde::Serialize {
    fn type_of(&self) -> String;
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
                location,
                id,
                name,
                content_name: None,
            },
            patterns: captures.patterns,
        }
    }
}

impl AbstractRule for IncludeOnlyRule {
    fn type_of(&self) -> String {
        String::from("IncludeOnlyRule")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BeginWhileRule {
    pub rule: Rule,

    pub begin: RegExpSource,
    pub begin_captures: Vec<Box<dyn AbstractRule>>,

    pub while_s: Option<String>,
    pub while_captures: Vec<Box<dyn AbstractRule>>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub apply_end_pattern_last: Option<bool>,

    pub patterns: ICompilePatternsResult,
    #[serde(skip_serializing_if="Option::is_none")]
    pub cached_compiled_patterns: Option<RegExpSourceList>,
}

impl BeginWhileRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        begin: Option<String>,
        begin_captures: Vec<Box<dyn AbstractRule>>,
        _while: Option<String>,
        while_captures: Vec<Box<dyn AbstractRule>>,
        patterns: ICompilePatternsResult,
    ) -> BeginWhileRule {
        BeginWhileRule {
            rule: Rule {
                _type: String::from("BeginEndRule"),
                location,
                id,
                name,
                content_name,
            },
            begin: RegExpSource::new(begin.unwrap().clone(), id.clone()),
            begin_captures,
            while_s: _while,
            while_captures,
            apply_end_pattern_last: None,
            patterns,
            cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for BeginWhileRule {
    fn type_of(&self) -> String {
        String::from("BeginWhileRule")
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
        match_s: String,
        captures: Vec<Box<dyn AbstractRule>>,
    ) -> Self {
        MatchRule {
            rule: Rule {
                _type: String::from("MatchRule"),
                location,
                id,
                name,
                content_name: None,
            },
            _match: RegExpSource::new(match_s, id),
            captures,
        }
    }
}

impl AbstractRule for MatchRule {
    fn type_of(&self) -> String {
        String::from("MatchRule")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BeginEndRule {
    #[serde(flatten)]
    pub rule: Rule,
    pub begin: RegExpSource,
    pub begin_captures: Vec<Box<dyn AbstractRule>>,
    pub end: RegExpSource,
    // pub endHasBackReferences: Option<bool>,
    pub end_captures: Vec<Box<dyn AbstractRule>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub apply_end_pattern_last: Option<bool>,
    // pub hasMissingPatterns: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub cached_compiled_patterns: Option<RegExpSourceList>,
    pub patterns: Vec<i32>,
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
        patterns: Vec<i32>,
    ) -> BeginEndRule {
        BeginEndRule {
            rule: Rule {
                _type: String::from("BeginEndRule"),
                location,
                id,
                name,
                content_name,
            },
            begin: RegExpSource::new(begin.clone(), id.clone()),
            begin_captures,
            end: RegExpSource::new(_end.clone(), id.clone()),
            end_captures,
            apply_end_pattern_last,
            patterns,
            cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for BeginEndRule {
    fn type_of(&self) -> String {
        String::from("BeginEndRule")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CaptureRule {
    pub rule: Rule,
}

impl CaptureRule {
    pub fn empty() -> Self {
        CaptureRule {
            rule: Rule {
                _type: "".to_string(),
                location: None,
                id: 0,
                name: None,
                content_name: None
            }
        }
    }
    pub fn new(location: Option<ILocation>, id: i32, name: Option<String>, content_name: Option<String>) -> Self {
        CaptureRule {
            rule: Rule {
                _type: String::from("CaptureRule"),
                location,
                id,
                name,
                content_name,
            },
        }
    }
}

impl AbstractRule for CaptureRule {
    fn type_of(&self) -> String {
        String::from("CaptureRule")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct NoneRule {}

impl AbstractRule for NoneRule {
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
