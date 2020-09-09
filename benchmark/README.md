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