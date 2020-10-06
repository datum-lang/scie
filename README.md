# Scie

> Scie is a research about how to build simple code identify engine for different languages.

goal: build a better code figure engine for code refactoring.

Todo:

 - [ ] replace with [fancy-regex](https://github.com/fancy-regex/fancy-regex)
 - [ ] process todo
    - [x] back references
    - [ ] multiple languages
 - [ ] rewrite VSCode-textmate with Rust
    - [x] language for testing
       - [x] makefile
       - [x] json
       - [x] html
    - [ ] languages main support
       - [ ] lua
       - [x] javascript
       - [x] java
       - [ ] typescript
       - [ ] groovy
       - [ ] kotlin
       - [ ] go
       - [ ] rust
       - [ ] python
       - [ ] php
       - [ ] swift
       - [ ] c/c++
    - [ ] support others language
 - [x] benchmark
    - [ ] fast than VSCode version
 - [ ] multiple languages one project support
 - [ ] analyser
    - [ ] line counts
    - [ ] keywords map

## DevSetup


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
