//! This crate contains a macro to generate a lazy
//! [`Regex`](https://docs.rs/regex/latest/regex/struct.Regex.html) and
//! perform regex validation.
//! The code fails to compile if the regex is invalid.
//!
//!
//! ## Usage
//!
//! This Rust crate helps prevent common pitfalls when working with regular
//! expressions by ensuring patterns are valid at compile time and avoiding
//! redundant compilations. It leverages the standard library `OnceLock` to
//! compile regexes only once and uses a procedural macro for compile-time
//! validation, improving both safety and performance.
//! The only dependency in the crate is `regex`
//!
//! Example:
//! ```rust
//! use lure::regex;
//!
//! // Password regex
//! let re = regex!("[0-9a-f]+");
//! assert!(re.is_match("Test1234ccc#"));
//! ```
//!
//! > Note: clippy already provides a lint that validates regexes.
//! > The usage of this crate is more about avoiding the overhead of creating
//! > the regex multiple with the added benefit of compile time validation.
//!

#![warn(clippy::pedantic)]
pub use lure_macros;

#[doc(hidden)]
pub type Regex = regex::Regex;
#[doc(hidden)]
pub type Lock = std::sync::OnceLock<Regex>;

/// Generates a lazy `regex::Regex` and perform regex validation.
/// The code fails to compile if the regex is invalid.
///
/// # Errors
/// Compilation fails if the regex is invalid.
///
/// # Examples
/// ```rust
/// let re = lure::regex!("[0-9a-f]+");
/// assert!(re.is_match("Test1234ccc#"));
///
/// ```
///
/// This example fails to compile because the regex is invalid.
/// ```compile_fail
/// let re = lure::regex!(r"/^.*(?=.{8,})(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).*$/");
/// ```
///
#[macro_export]
macro_rules! regex (
    // in Rust 1.45 we can now invoke proc macros in expression position
    ($re:expr) => ({
        static RE: $crate::Lock = $crate::Lock::new();
        RE.get_or_init(|| $crate::Regex::new($crate::lure_macros::expand_regex!($re)).unwrap())
    });
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex() {
        let hex = regex!("[0-9a-f]+");
        assert!(hex.is_match("1234deadbeef"));
    }
}
