use crate::rule::{IRuleRegistry, RegExpSourceList};
use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};

pub trait AbstractRule: DynClone + erased_serde::Serialize {
    fn id(&self) -> i32;
    fn type_of(&self) -> String;
    fn has_missing_pattern(&self) -> bool {
        false
    }
    fn collect_patterns_recursive(
        &self,
        grammar: Box<dyn IRuleRegistry>,
        out: RegExpSourceList,
        is_first: bool,
    ) {
    }
    fn compile(
        &self,
        grammar: Box<dyn IRuleRegistry>,
        end_regex_source: Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) {
    }
}

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AbstractRule")
    }
}

serialize_trait_object!(AbstractRule);

clone_trait_object!(AbstractRule);
