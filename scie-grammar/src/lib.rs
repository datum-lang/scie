// todo: remove after finish
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate erased_serde;

extern crate regex;

pub mod grammar;
pub mod inter;
pub mod registry;
pub mod rule;
pub mod support;

use std::collections::BTreeMap as Map;

pub struct IEmbeddedLanguagesMap {
    map: Map<String, Box<i32>>,
}
