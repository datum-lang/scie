pub mod factory;

use crate::inter::{ILocation, IRawGrammar, IRawRepository};
use crate::reg_exp_source::{RegExpSource, RegExpSourceList};
use crate::rule::factory::ICompilePatternsResult;
use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};

#[derive(Clone, Debug)]
pub struct Rule {
    pub location: Option<ILocation>,
    pub id: i32,
    pub name: Option<String>,
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
            location: Some(location),
            id,
            name,
            content_name,
        }
    }
}

pub trait AbstractRule: DynClone {
    fn type_of(&self) -> String;
}

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AbstractRule")
    }
}

clone_trait_object!(AbstractRule);

#[derive(Clone, Debug)]
pub struct IncludeOnlyRule {
    pub rule: Rule,
    pub captures: ICompilePatternsResult,
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
                location,
                id,
                name,
                content_name: None,
            },
            captures,
        }
    }
}

impl AbstractRule for IncludeOnlyRule {
    fn type_of(&self) -> String {
        String::from("IncludeOnlyRule")
    }
}

#[derive(Clone, Debug)]
pub struct BeginWhileRule {
    pub rule: Rule,
}

impl BeginWhileRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        begin: Option<String>,
        begin_captures: Vec<CaptureRule>,
        _while: Option<String>,
        while_captures: Vec<CaptureRule>,
        patterns: ICompilePatternsResult,
    ) -> BeginEndRule {
        BeginEndRule {
            rule: Rule {
                location,
                id,
                name,
                content_name,
            },
            _begin: RegExpSource::new(begin.unwrap().clone(), id.clone()),
            begin_captures: None,
            _end: None,
            end_captures: None,
            apply_end_pattern_last: None,
            patterns: None,
            cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for BeginWhileRule {
    fn type_of(&self) -> String {
        String::from("BeginWhileRule")
    }
}

#[derive(Clone, Debug)]
pub struct MatchRule {
    pub rule: Rule,
    pub _match: RegExpSource,
    pub captures: Vec<CaptureRule>,
}

impl MatchRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        match_s: String,
        captures: Vec<CaptureRule>,
    ) -> Self {
        MatchRule {
            rule: Rule {
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

#[derive(Clone, Debug)]
pub struct BeginEndRule {
    pub rule: Rule,
    pub _begin: RegExpSource,
    pub begin_captures: Option<Vec<CaptureRule>>,
    pub _end: Option<RegExpSource>,
    // pub endHasBackReferences: Option<bool>,
    pub end_captures: Option<Vec<CaptureRule>>,
    pub apply_end_pattern_last: Option<bool>,
    // pub hasMissingPatterns: Option<bool>,
    pub patterns: Option<i32>,
    pub cached_compiled_patterns: Option<RegExpSourceList>,
}

impl BeginEndRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        begin: String,
        begin_captures: Vec<CaptureRule>,
        _while: Option<String>,
        while_captures: Vec<CaptureRule>,
        apply_end_pattern_last: Option<bool>,
        // patterns: ICompilePatternsResult,
    ) -> BeginEndRule {
        BeginEndRule {
            rule: Rule {
                location,
                id,
                name,
                content_name,
            },
            _begin: RegExpSource::new(begin.clone(), id.clone()),
            begin_captures: None,
            _end: None,
            end_captures: None,
            apply_end_pattern_last,
            patterns: None,
            cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for BeginEndRule {
    fn type_of(&self) -> String {
        String::from("BeginEndRule")
    }
}

#[derive(Clone, Debug)]
pub struct CaptureRule {
    pub rule: Rule,
}

impl CaptureRule {
    pub fn new() -> Self {
        CaptureRule {
            rule: Rule {
                location: None,
                id: 0,
                name: None,
                content_name: None,
            },
        }
    }
}

impl AbstractRule for CaptureRule {
    fn type_of(&self) -> String {
        String::from("CaptureRule")
    }
}

#[derive(Clone, Debug)]
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
