// todo: remove after finish
#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate erased_serde;

extern crate regex;

pub mod grammar;
pub mod inter;
pub mod rule;
pub mod support;
pub mod grammar_registry;
pub mod sync_register;
pub mod scope_dependency;
