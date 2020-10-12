# 8. bindata for embed

Date: 2020-10-11

## Status

2020-10-11 proposed

## Context

For saved data for parse in process, we need to embed data in app.

for choice:

 - [https://github.com/pyros2097/rust-embed] Rust Macro which loads files into the rust binary at compile time during release and loads the file from the fs during dev.

## Decision

In the basic version, we choice servo bincode

in current:

Rust [bindata](https://github.com/servo/bincode) not support for flatten items, we need to replace flatten or bincode

issues:

 - [`#[serde(flatten)]` causes error `SequenceMustHaveLength`](https://github.com/servo/bincode/issues/245)
 - [Support serializing to Vec<u8> with unknown seq/map length](https://github.com/servo/bincode/issues/167)
 - [Responding to Serialization Errors #257](https://github.com/servo/ipc-channel/issues/257)

## Consequences

Consequences here...
