#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

#[cfg(windows)]
extern crate libc;
extern crate onig_sys;

pub mod scanner;
