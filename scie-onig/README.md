## Setup

### by Auto

run test in `lib.rs` and copy ffi.rs from `target`

### by Manual

```
cargo install bindgen
```

then:

```
bindgen libonigvs/src/oniguruma.h --with-derive-eq --no-layout-tests --distrust-clang-mangling > src/ffi.rs
```

