# Scie

![CI](https://github.com/charj-lang/scie/workflows/CI/badge.svg)

> Scie is a research about how to build simple code identify engine for different languages.

goal: build a better code figure engine for code refactoring.

 - scie-cli. cli part of Scie.
 - scie-bingen. generate languages bin-data.
 - scie-detector. detector for different frameworks & languages.
 - scie-grammar. A library that helps tokenize text using Text Mate grammars.
 - scie-infra. common infrastructure support, like fs
 - scie-onig. Wrapper Rust [Oniguruma](https://github.com/kkos/oniguruma) FFI api based on [rust-onig](https://github.com/rust-onig/rust-onig)
 - scie-model. common model of VSCode models & [Miao](https://github.com/phodal/miao/) Model.
 - scie-scanner. Rewrite of [VS Code Oniguruma](https://github.com/microsoft/vscode-oniguruma)

## Usage

### Scie CLI

```bash
scie-cli 0.1
Phodal HUANG<h@phodal.com>

USAGE:
    scie-cli [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -v, --verbose
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    [default: default.conf]
    -p, --path <path>        [default: .]
```

## DevSetup

1. git clone

```
git clone https://github.com/phodal/scie/
```

2. run

```
cargo run scie
```

install just

```
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to DEST
```

run tests

```
just tests
```


## Guideline

major issues:

 - [ ] performance
    - [ ] rule in `Grammar` seed in [rule_container.rs](scie-grammar/src/grammar/rule_container.rs)
    - [x] UTF 8 to UTF 16 in `UtfString`
    - [x] normal issue
 - [ ] unstable
    - [ ] Random test failure on OnigScanner.
    - [x] GC issues on OnigScanner.
       - GC issue seems resolved with Jemalloc.
       - Signal 6 (<cite>SIGABRT</cite>) = SIGABRT is commonly used by `libc` and other libraries to abort the program in case of critical errors. For example, `glibc` sends an SIGABRT in case of a detected double-free or other heap corruptions.
       - maybe UTF8 encoding issue

Todo:

 - [ ] replace with [fancy-regex](https://github.com/fancy-regex/fancy-regex) for pure Rust impl
 - [x] process todo
    - [x] back references
    - [x] multiple languages
 - [x] rewrite VSCode-textmate with Rust
    - [x] language for testing
    - [x] support others language
 - [x] benchmark
    - [ ] fast than VSCode version
 - [ ] multiple languages one project support
 - [ ] analyser
    - [ ] line counts
    - [ ] keywords map

## Documents

### refs

 - [Oniguruma Regular Expressions](https://github.com/kkos/oniguruma/blob/master/doc/RE)
 - [https://github.com/atom/node-oniguruma](https://github.com/atom/node-oniguruma)
 - [Sloc Cloc and Code - What happened on the way to faster Cloc](https://boyter.org/posts/sloc-cloc-code/)
 - [ripgrep](https://github.com/BurntSushi/ripgrep) for todo?

License
---

[scie-grammar](scie-gramma/) based on [vscode-textmate](https://github.com/microsoft/vscode-textmate) with MIT LICENSE see in  [scie-grammar/src/scanner/LICENSE](scie-grammar/src/scanner/LICENSE)

[onigvs](onigvs/) based on [rust-onig](https://github.com/rust-onig/rust-onig)

[![Phodal's Idea](http://brand.phodal.com/shields/idea-small.svg)](http://ideas.phodal.com/)

@ 2020 A [Phodal Huang](https://www.phodal.com)'s [Idea](http://github.com/phodal/ideas).  This code is distributed under the MPL license. See `LICENSE` in this directory.
