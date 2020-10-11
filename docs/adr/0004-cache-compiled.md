# 4. cache compiled

Date: 2020-09-09

## Status

2020-09-09 proposed

2020-10-11 done

## Context

For now, we don't implement all cached logic from vscode-textmate, if we want to do better on it, we need this.

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

## Decision

Decision here...

## Consequences

Consequences here...
