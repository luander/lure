# lure

[![Crates.io Version](https://img.shields.io/crates/v/lure.svg)](https://crates.io/crates/lure)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/lure)
[![cicd](https://github.com/luander/lure/actions/workflows/ci.yaml/badge.svg)](https://github.com/luander/lure/actions/workflows/ci.yaml)

Safe & Efficient Regex in Rust – Compile-Time Validation & Single Compilation

This crate contains a macro to generate a lazy
[`Regex`](https://docs.rs/regex/latest/regex/struct.Regex.html) and perform regex validation.
The code fails to compile if the regex is invalid.


## Usage

This Rust crate helps prevent common pitfalls when working with regular expressions by ensuring patterns are valid at compile time and avoiding redundant compilations. It leverages the standard library `OnceCell` to compile regexes only once and uses a procedural macro for compile-time validation, improving both safety and performance.
The only dependency in the crate if `regex`

Example:
```rust
use lure::regex;

// Password regex
let re = regex!("/^.*(?=.{8,})(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).*$/");
assert!(re.is_match("Test1234ccc#"));
```

## License

Licensed under Apache License, Version 2.0 ([LICENSE](LICENSE) or
  http://www.apache.org/licenses/LICENSE-2.0)
