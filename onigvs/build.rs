extern crate cmake;

use cmake::Config;
use std::env;

fn main()
{
    let dst = Config::new("libfoo++").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo++");

    let target  = env::var("TARGET").unwrap();
    if target.contains("apple")
    {
        println!("cargo:rustc-link-lib=dylib=c++");
    }
    else if target.contains("linux")
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    else
    {
        unimplemented!();
    }
}
