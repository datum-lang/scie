pub mod rule_convert;
pub mod rule_factory;

pub mod abstract_rule;
pub mod begin_end_rule;
pub mod begin_while_rule;
pub mod capture_rule;
pub mod compiled_rule;
pub mod empty_rule;
pub mod include_only_rule;
pub mod match_rule;
pub mod rule;

pub use self::abstract_rule::AbstractRule;
pub use self::begin_end_rule::BeginEndRule;
pub use self::begin_while_rule::BeginWhileRule;
pub use self::capture_rule::CaptureRule;
pub use self::compiled_rule::CompiledRule;
pub use self::empty_rule::EmptyRule;
pub use self::include_only_rule::IncludeOnlyRule;
pub use self::match_rule::MatchRule;
pub use self::rule::Rule;

use crate::inter::{IRawGrammar, IRawRepository};
use serde::{Serialize};
use crate::grammar::Grammar;

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
pub struct AnchorCache {
    A0_G0: Option<String>,
    A0_G1: Option<String>,
    A1_G0: Option<String>,
    A1_G1: Option<String>,
}

impl Default for AnchorCache {
    fn default() -> Self {
        AnchorCache {
            A0_G0: None,
            A0_G1: None,
            A1_G0: None,
            A1_G1: None
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSourceList {
    pub _has_anchors: bool,
    pub _cached: Option<CompiledRule>,
    pub _anchor_cache: AnchorCache
}

impl RegExpSourceList {
    pub fn new() -> Self {
        RegExpSourceList {
            _has_anchors: false,
            _cached: None,
            _anchor_cache: Default::default()
        }
    }

    pub fn compile(&self, grammar: &mut Grammar, allow_a: bool, allow_g: bool) {

    }
}

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
