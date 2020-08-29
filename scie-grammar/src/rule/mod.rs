pub mod rule_factory;
pub mod rule_convert;
pub mod reg_exp_source;

pub mod abstract_rule;
pub mod begin_end_rule;
pub mod begin_while_rule;
pub mod compiled_rule;
pub mod include_only_rule;
pub mod match_rule;
pub mod capture_rule;

pub use self::abstract_rule::AbstractRule;
pub use self::compiled_rule::CompiledRule;
pub use self::begin_end_rule::BeginEndRule;
pub use self::begin_while_rule::BeginWhileRule;
pub use self::include_only_rule::IncludeOnlyRule;
pub use self::match_rule::MatchRule;
pub use self::capture_rule::CaptureRule;

use crate::inter::{ILocation, IRawGrammar, IRawRepository};
use reg_exp_source::{RegExpSource, RegExpSourceList};
use crate::rule::rule_factory::ICompilePatternsResult;
use core::fmt;
use serde::{Serialize, Serializer};

#[derive(Clone, Debug, Serialize)]
pub struct Rule {
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _location: Option<ILocation>,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
