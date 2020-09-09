# Setup Profile

[Intro to rustc's self profiler](https://blog.rust-lang.org/inside-rust/2020/02/25/intro-rustc-self-profile.html)

```
$ cargo install --git https://github.com/rust-lang/measureme crox flamegraph summarize
```

```
rustup override set nightly
```


## Profile

```bash
cargo rustc -- -Zself-profile
```

```
{crate name}-{rustc process id}.{events,string_data,string_index}.
```

then

```bash
summarize summarize regex-17088
```

## Notes

Node version:

```
JSON
TOKENIZING 100210 lines using grammar source.js
Oniguruma: 216 ms., Onigasm: 117 ms. (1.8x faster)
```

we used:

```
➜  benchmark git:(master) ✗ cargo run benchmark
   Compiling benchmark v0.1.0 (/Users/fdhuang/repractise/scie/benchmark)
    Finished dev [unoptimized + debuginfo] target(s) in 1.26s
     Running `/Users/fdhuang/repractise/scie/target/debug/benchmark benchmark`


TOKENIZING 100210 length using grammar source.js 107204 ms
```

with release version:

```
/Users/fdhuang/repractise/scie/target/release/benchmark

TOKENIZING 100210 length using grammar source.js 10306 ms
```
