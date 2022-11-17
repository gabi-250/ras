# ras

[![AGPL-3.0 license][agpl-badge]][agpl-url]
[![Build Status][actions-badge]][actions-url]

[agpl-badge]: https://img.shields.io/badge/license-AGPL-purple.svg
[agpl-url]: https://github.com/gabi-250/ras/blob/master/LICENSE
[actions-badge]: https://github.com/gabi-250/ras/actions/workflows/test.yaml/badge.svg
[actions-url]: https://github.com/gabi-250/ras/actions/workflows/test.yaml?query=branch%3Amaster+workflow%3ATests

An x86-64 assembler implemented in Rust.

**This is work in progress, and known to be buggy and far from feature complete.**

## Example usage

The [examples] directory of the [ras-x86] crate contains a couple of usage
examples:
* `labels.rs` shows how to use `Assembler` as a runtime assembler (with the
  assembly program built programmatically)
* `parser.rs` shows a more traditional assembler implementation, which reads in
  a source file and outputs the corresponding object file (`ras` currently only
  supports AT&T syntax)

`ras` consists of two crates:
* [ras-x86-repr], which deals with the internal representation of x86
  instructions
* [ras-x86], which uses the [ras-x86-repr] crate and contains a build script for
  processing the instruction [csv], an AT&T assembly parser, and the instruction
  encoder

[examples]: https://github.com/gabi-250/ras/tree/master/ras-x86/examples
[csv]: https://github.com/GregoryComer/x86-csv/tree/c638bbbaa17f0c81abaa7e84a968335c985542fa
[ras-x86]: https://github.com/gabi-250/ras/tree/master/ras-x86
[ras-x86-repr]: https://github.com/gabi-250/ras/tree/master/ras-x86-repr
