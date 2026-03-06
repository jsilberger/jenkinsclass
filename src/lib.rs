//! Core library for the `fortune` CLI.
//!
//! This crate provides:
//! - A lazily loaded collection of fortune strings ([`FORTUNES`]).
//! - Pure formatting logic for display output ([`print_fortune`]).
//! - Testable argument parsing helpers ([`parse_name_from_args`]).
//!
//! # Examples
//!
//! Build a formatted message:
//!
//! ```
//! use fortune::print_fortune;
//!
//! let rendered = print_fortune("May your build stay green.", "Avery");
//! assert!(rendered.contains("This fortune is for Avery."));
//! ```
//!
//! Parse a name from CLI-like args:
//!
//! ```
//! use fortune::parse_name_from_args;
//!
//! assert_eq!(parse_name_from_args(["--name", "Taylor"]), "Taylor");
//! assert_eq!(parse_name_from_args(["--list"]), "you");
//! ```

use std::{env, sync::LazyLock};

/// Collection of available fortunes loaded from `data/fortunes.txt`.
///
/// The file is embedded at compile time using `include_str!` and parsed lazily
/// on first access.
///
/// # Examples
///
/// ```
/// use fortune::FORTUNES;
///
/// assert!(!FORTUNES.is_empty());
/// ```
pub static FORTUNES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    include_str!("../data/fortunes.txt")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
});

/// Parses a recipient name from CLI-like arguments.
///
/// # Behavior
///
/// - Looks for the first `--name` flag.
/// - Returns the token immediately following `--name` as the parsed name.
/// - Returns `"you"` when `--name` is missing or has no value.
///
/// # Arguments
///
/// - `args`: Any iterator of argument-like values, such as `&str` or `String`.
///
/// # Returns
///
/// The parsed recipient name or the default `"you"`.
///
/// # Examples
///
/// ```
/// use fortune::parse_name_from_args;
///
/// assert_eq!(parse_name_from_args(["--name", "Avery"]), "Avery");
/// assert_eq!(parse_name_from_args(["--verbose"]), "you");
/// assert_eq!(parse_name_from_args(["--name"]), "you");
/// ```
#[must_use]
pub fn parse_name_from_args<I, S>(args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        if arg.as_ref() == "--name" {
            return args
                .next()
                .map_or_else(|| String::from("you"), |value| value.as_ref().to_string());
        }
    }

    String::from("you")
}

/// Builds the formatted fortune message for a recipient.
///
/// This function is pure: it only assembles and returns the output string.
/// Printing is handled by the binary (`main`).
///
/// # Arguments
///
/// - `fortune`: The fortune text to place in quotes.
/// - `name`: The recipient name shown in the greeting line.
///
/// # Returns
///
/// A multiline, display-ready fortune message.
///
/// # Examples
///
/// ```
/// use fortune::print_fortune;
///
/// let output = print_fortune("Stay curious.", "Kai");
///
/// assert!(output.contains("This fortune is for Kai."));
/// assert!(output.contains("\"Stay curious.\""));
/// ```
#[must_use]
pub fn print_fortune(fortune: &str, name: &str) -> String {
    format!(
        "🥠 Fortune Cookie 🥠\nThis fortune is for {name}.\n\n\"{fortune}\"\n\n✨ May your fortune come true! ✨"
    )
}

/// Parses the recipient name from process arguments.
///
/// This is a thin wrapper around [`parse_name_from_args`] that reads from
/// `std::env::args()` and skips the executable name.
///
/// # Returns
///
/// The parsed recipient name or `"you"` when no name is provided.
#[must_use]
pub fn get_name_from_args() -> String {
    parse_name_from_args(env::args().skip(1))
}
