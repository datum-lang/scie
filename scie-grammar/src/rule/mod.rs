pub mod rule_factory;
pub mod rule_convert;

pub mod abstract_rule;
pub mod begin_end_rule;
pub mod begin_while_rule;
pub mod compiled_rule;
pub mod include_only_rule;
pub mod match_rule;
pub mod capture_rule;
pub mod empty_rule;
pub mod rule;

pub use self::rule::Rule;
pub use self::abstract_rule::AbstractRule;
pub use self::compiled_rule::CompiledRule;
pub use self::begin_end_rule::BeginEndRule;
pub use self::begin_while_rule::BeginWhileRule;
pub use self::include_only_rule::IncludeOnlyRule;
pub use self::match_rule::MatchRule;
pub use self::capture_rule::CaptureRule;
pub use self::empty_rule::EmptyRule;

use crate::inter::{ILocation, IRawGrammar, IRawRepository};
use crate::rule::rule_factory::ICompilePatternsResult;
use core::fmt;
use serde::{Serialize, Serializer};

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

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSourceList {}

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSource {
    pub source: String,
    pub rule_id: i32,
}

impl RegExpSource {
    pub fn new(reg_exp_source: String, rule_id: i32) -> RegExpSource {
        RegExpSource {
            source: reg_exp_source,
            rule_id,
        }
    }
}
