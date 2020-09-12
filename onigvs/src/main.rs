use libc::{c_char, puts, strlen, malloc};
use core::slice;

mod ffi;

fn main() {
    unsafe {
        malloc(8);
    };
}
