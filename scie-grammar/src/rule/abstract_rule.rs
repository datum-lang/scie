use dyn_clone::{clone_trait_object, DynClone};
use core::fmt;

pub trait AbstractRule: DynClone + erased_serde::Serialize {
    fn id(&self) -> i32;
    fn type_of(&self) -> String;
    fn has_missing_pattern(&self) -> bool {
        false
    }
}

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AbstractRule")
    }
}

serialize_trait_object!(AbstractRule);

clone_trait_object!(AbstractRule);

