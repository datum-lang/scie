extern crate cmake;

use cmake::Config;
use std::env;

fn main()
{
    let dst = Config::new("libonigvs").build();

    // println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=onigvs");

    // C++ is bit more complicated, since platform specifics come to play
    let target  = env::var("TARGET").unwrap();
    if target.contains("apple")
    {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=dylib=libonig");
    }
    else if target.contains("linux")
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=libonig");
    }
    else
    {
        unimplemented!();
    }
}