use crate::ffi::{onig_version, onig_copyright};
use libc::{c_char, puts, strlen};
use core::slice;

mod ffi;

fn main() {
    unsafe {
        let c_s = onig_copyright();
        let str = String::from_utf8_unchecked(Vec::from(slice::from_raw_parts(c_s as *const u8, (strlen(c_s) + 1))));
        println!("{:?}", str);
    };
}
