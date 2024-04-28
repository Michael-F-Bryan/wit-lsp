//! This crate provides Wit language support for the [tree-sitter][ts] parsing
//! library.
//!
//! Typically, you will use the [`language()`] function to add this language to
//! a [`tree_sitter::Parser`], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//!   package wasi:filesystem;
//!
//!   interface wasi {
//!     enum clockid {
//!       /// The clock measuring real time. Time value zero corresponds with
//!       /// 1970-01-01T00:00:00Z.
//!       realtime,
//!       /// The store-wide monotonic clock, which is defined as a clock measuring
//!       /// real time, whose value cannot be adjusted and which cannot have negative
//!       /// clock jumps. The epoch of this clock is undefined. The absolute time
//!       /// value of this clock therefore has no meaning.
//!       monotonic,
//!     }
//!
//!     /// Timestamp in nanoseconds.
//!     type timestamp = u64;
//!   }
//! "#;
//!
//! let tree = tree_sitter_wit::parse(code);
//!
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [ts]: https://tree-sitter.github.io/

pub extern crate tree_sitter;

use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_wit() -> Language;
}

/// Get the tree-sitter [Language][lang] for this grammar.
///
/// # Exampl
///
/// [lang]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language() -> Language {
    unsafe { tree_sitter_wit() }
}

/// Parse a WIT file into a *Concrete Syntax Tree*.
///
/// This is just shorthand for creating a new [`tree_sitter::Parser`] and
/// initializing it with [`language()`].
///
/// ```
/// let mut parser = tree_sitter::Parser::new();
/// let language = tree_sitter_wit::language();
/// parser.set_language(&language).expect("Error loading Wit grammar");
///
/// let tree = parser.parse("...", None).unwrap();
/// ```
pub fn parse(src: &str) -> tree_sitter::Tree {
    let mut p = Parser::new();
    let lang = language();
    p.set_language(&lang).unwrap();
    p.parse(src, None).unwrap()
}

/// The content of the [`node-types.json`][node-types] file for this grammar.
///
/// [node-types]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");
// pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");
// pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");
// pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    use std::{
        io::ErrorKind,
        path::Path,
        process::{Command, Output},
    };

    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::language())
            .expect("Error loading Wit grammar");
    }

    #[test]
    fn run_tree_sitter_tests() {
        let parser_root = Path::new(env!("CARGO_MANIFEST_DIR"));

        let mut cmd = Command::new("tree-sitter");
        cmd.arg("test").current_dir(parser_root);

        match cmd.output() {
            Ok(Output { status, .. }) if status.success() => {}
            Ok(Output {
                status,
                stdout,
                stderr,
            }) => {
                let stdout = String::from_utf8_lossy(&stdout);
                if !stdout.is_empty() {
                    println!("==== Stdout ====");
                    println!("{stdout}");
                }
                let stderr = String::from_utf8_lossy(&stderr);
                if !stderr.is_empty() {
                    println!("==== Stderr ====");
                    println!("{stderr}");
                }

                panic!("`{cmd:?}` failed: {status}");
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                // The tree-sitter CLI isn't installed. Ignore.
            }
            Err(e) => {
                panic!("Unable to start `{cmd:?}`: {e}");
            }
        }
    }
}
