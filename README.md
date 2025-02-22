# lure

[![Crates.io Version](https://img.shields.io/crates/v/lure.svg)](https://crates.io/crates/lure)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/lure)
[![cicd](https://github.com/luander/lure/actions/workflows/ci.yaml/badge.svg)](https://github.com/luander/lure/actions/workflows/ci.yaml)

Shift left with Lure, a Rust crate that provides a macro for creating lazy [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html) instances with compile-time validation, ensuring invalid patterns fail to compile.

## Usage

This Rust crate helps prevent common pitfalls when working with regular expressions by ensuring patterns are valid at compile time and avoiding redundant compilations. It leverages the standard library `OnceCell` to compile regexes only once and uses a procedural macro for compile-time validation, improving both safety and performance.
The only dependency in the crate if `regex`

Example:
```rust
use lure::regex;

let re = regex!("[0-9a-f]+");
assert!(re.is_match("deadbeef1234"));
```

## Invalid Regex

Compilation fails if the regex is invalid. For example, the following code will not compile:

```rust
fn main() {
    let re = lure::regex!(r"/^.*(?=.{8,})(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).*$/");
    assert!(re.is_match("randomChars123!"));
}
```

Trying to compile the code above will result in the following error:

```rust
error: Invalid regex: regex parse error:
           r"/^.*(?=.{8,})(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).*$/"
                 ^^^
       error: look-around, including look-ahead and look-behind, is not supported
 --> examples/simple/src/main.rs:4:14
  |
4 |     let re = regex!(r"/^.*(?=.{8,})(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).*$/");
  |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Other examples are wrong syntax, missing closing parenthesis, and invalid escape sequences:

```rust
fn main() {
    let re = regex!(r"[0-9a-f+");
    assert!(re.is_match("1234deadbeef"));
}

```

Which prints out the error:

```rust
error: Invalid regex: regex parse error:
           r"[0-9a-f+"
             ^
       error: unclosed character class
 --> examples/simple/src/main.rs:4:14
  |
4 |     let re = regex!(r"[0-9a-f+");
  |              ^^^^^^^^^^^^^^^^^^^
```

## License

Licensed under Apache License, Version 2.0 ([LICENSE](LICENSE) or
  http://www.apache.org/licenses/LICENSE-2.0)
