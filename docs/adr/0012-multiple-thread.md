# 12. multiple thread

Date: 2020-10-15

## Status

2020-10-15 proposed

## Context

```
error[E0277]: `(dyn scie_grammar::rule::AbstractRule + 'static)` cannot be sent between threads safely
   --> scie-core/src/analyser/mod.rs:41:22
    |
41  |         let handle = thread::spawn(|| {
    |                      ^^^^^^^^^^^^^ `(dyn scie_grammar::rule::AbstractRule + 'static)` cannot be sent between threads safely
    |
   ::: /Users/fdhuang/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/src/libstd/thread/mod.rs:616:8
    |
616 |     F: Send + 'static,
    |        ---- required by this bound in `std::thread::spawn`
    |
    = help: the trait `std::marker::Send` is not implemented for `(dyn scie_grammar::rule::AbstractRule + 'static)`
    = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn scie_grammar::rule::AbstractRule + 'static)>`
    = note: required because it appears within the type `std::boxed::Box<(dyn scie_grammar::rule::AbstractRule + 'static)>`
    = note: required because it appears within the type `(i32, std::boxed::Box<(dyn scie_grammar::rule::AbstractRule + 'static)>)`
    = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(i32, std::boxed::Box<(dyn scie_grammar::rule::AbstractRule + 'static)>)>`
    = note: required because it appears within the type `hashbrown::map::HashMap<i32, std::boxed::Box<(dyn scie_grammar::rule::AbstractRule + 'static)>, std::collections::hash_map::RandomState>`
    = note: required because it appears within the type `std::collections::HashMap<i32, std::boxed::Box<(dyn scie_grammar::rule::AbstractRule + 'static)>>`
    = note: required because it appears within the type `scie_grammar::grammar::Grammar`
    = note: required because it appears within the type `(&str, scie_grammar::grammar::Grammar)`
    = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(&str, scie_grammar::grammar::Grammar)>`
    = note: required because it appears within the type `hashbrown::map::HashMap<&str, scie_grammar::grammar::Grammar, std::collections::hash_map::RandomState>`
    = note: required because it appears within the type `std::collections::HashMap<&str, scie_grammar::grammar::Grammar>`
    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::collections::HashMap<&str, scie_grammar::grammar::Grammar>`
    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut &mut std::collections::HashMap<&str, scie_grammar::grammar::Grammar>`
    = note: required because it appears within the type `[closure@scie-core/src/analyser/mod.rs:41:36: 63:10 files:std::vec::Vec<std::path::PathBuf>, grammar_map:&mut &mut std::collections::HashMap<&str, scie_grammar::grammar::Grammar>]`

error: aborting due to previous error
```

## Decision

Decision here...

## Consequences

Consequences here...
