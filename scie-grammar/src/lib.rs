// todo: remove after finish
#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate erased_serde;

extern crate regex;

pub mod grammar;
pub mod inter;
pub mod matcher;
pub mod rule;
pub mod support;
