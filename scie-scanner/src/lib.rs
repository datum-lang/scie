#[macro_use]
extern crate serde_derive;

extern crate libc;
extern crate scie_onig;

#[cfg(not(feature = "generate"))]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
pub mod scanner;
