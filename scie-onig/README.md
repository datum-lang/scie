## Setup

### by Auto

run test in `lib.rs` and copy ffi.rs from `target`

### by Manual

```
cargo install bindgen
```

then:

```
bindgen libonigvs/onigvs.c --with-derive-eq --no-layout-tests --distrust-clang-mangling > src/ffi.rs
```

