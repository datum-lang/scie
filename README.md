# Scie

> Scie is a research about how to build simple code identify engine for different languages.

goal: build a better code figure engine for code refactoring.

 - scie-bingen. generate languages bindata.
 - scie-detector. detector for different frameworks & languages.
 - scie-grammar. A library that helps tokenize text using Text Mate grammars.
 - scie-infra. common infrasturcture support, like fs
 - scie-onig. Rust FFI for Oniguruma.
 - scie-model. common model of VSCode models & Miao Model.
 - scie-scanner. Wrapper Rust Oniguruma FFI api.
 scie-cli. cli part ofr Scie.

## Guideline

major issues:

 - [ ] performance
    - [ ] rule in `Grammar`
    - [ ] UTF 8 to UTF 16 in `UtfString`
    - [x] normal issue
 - [ ] unstable
    - [ ] Random test failure on OnigScanner.
    - [x] GC issues on OnigScanner.
       - GC issue seems resolved with Jemalloc.
       - Signal 6 (<cite>SIGABRT</cite>) = SIGABRT is commonly used by `libc` and other libraries to abort the program in case of critical errors. For example, `glibc` sends an SIGABRT in case of a detected double-free or other heap corruptions.
       - maybe UTF8 encoding issue

Todo:

 - [ ] replace with [fancy-regex](https://github.com/fancy-regex/fancy-regex) for pure Rust impl
 - [ ] process todo
    - [x] back references
    - [x] multiple languages
 - [ ] rewrite VSCode-textmate with Rust
    - [x] language for testing
    - [x] support others language
 - [x] benchmark
    - [ ] fast than VSCode version
 - [ ] multiple languages one project support
 - [ ] analyser
    - [ ] line counts
    - [ ] keywords map

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

## Documents

### refs

 - [Rust FFI 实践](https://blog.csdn.net/allwefantasy/article/details/89442758)
 - [Oniguruma Regular Expressions](https://github.com/kkos/oniguruma/blob/master/doc/RE)
 - [https://github.com/atom/node-oniguruma](https://github.com/atom/node-oniguruma)

License
---

[scie-grammar](scie-gramma/r) based on [vscode-textmate](https://github.com/microsoft/vscode-textmate) with MIT LICENSE see in  [scie-grammar/src/scanner/LICENSE](scie-grammar/src/scanner/LICENSE)

[onigvs](onigvs/) based on [rust-onig](https://github.com/rust-onig/rust-onig)

[![Phodal's Idea](http://brand.phodal.com/shields/idea-small.svg)](http://ideas.phodal.com/)

@ 2020 A [Phodal Huang](https://www.phodal.com)'s [Idea](http://github.com/phodal/ideas).  This code is distributed under the MPL license. See `LICENSE` in this directory.
