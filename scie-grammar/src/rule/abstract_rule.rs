use crate::grammar::Grammar;
use crate::rule::{IRuleRegistry, RegExpSourceList, CompiledRule};
use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};

pub trait AbstractRule: DynClone + erased_serde::Serialize {
    fn id(&self) -> i32;
    fn type_of(&self) -> String;
    fn has_missing_pattern(&self) -> bool {
        false
    }
    fn collect_patterns_recursive(
        &mut self,
        grammar: &mut Grammar,
        out: &mut RegExpSourceList,
        is_first: bool,
    ) {
    }
    fn compile(
        &mut self,
        grammar: &mut Grammar,
        end_regex_source: Option<String>,
        allow_a: bool,
        allow_g: bool,
    )  -> CompiledRule;
}

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AbstractRule")
    }
}

serialize_trait_object!(AbstractRule);

clone_trait_object!(AbstractRule);
