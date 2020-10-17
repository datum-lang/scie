#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate serde;

pub mod framework_detector;

pub mod facet;
pub mod dependency;
