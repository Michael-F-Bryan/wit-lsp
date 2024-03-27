//! This crate provides Wit language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [language][language func] function to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! parser.set_language(&tree_sitter_wit::language()).expect("Error loading Wit grammar");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [language func]: fn.language.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

#[cfg(test)]
mod codegen;

pub extern crate tree_sitter;

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_wit() -> Language;
}

/// Get the tree-sitter [Language][] for this grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language() -> Language {
    unsafe { tree_sitter_wit() }
}

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

// Uncomment these to include any queries that this grammar contains

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

    use quote::ToTokens;

    use crate::{codegen, NODE_TYPES};

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
                panic!("{e}");
            }
        }
    }

    #[test]
    fn generate_ast() {
        let tokens = codegen::generate_ast(NODE_TYPES);
        let src = format_rust(tokens);
        let ast_rs = project_root().join("crates/wit-compiler/src/ast/generated.rs");
        ensure_file_contents(ast_rs, src);
    }

    /// Get the root directory for this repository.
    pub fn project_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .find(|p| p.join(".git").exists())
            .unwrap()
    }

    /// Format some Rust tokens.
    ///
    /// # Panics
    ///
    /// It is assumed that the tokens would parse as a Rust file.
    pub fn format_rust(contents: impl ToTokens) -> String {
        let contents = syn::parse2(contents.to_token_stream())
            .expect("Unable to parse the tokens as a syn::File");
        prettyplease::unparse(&contents)
    }

    /// Check that a particular file has the desired contents.
    ///
    /// If the file is missing or outdated, this function will update the file and
    /// trigger a panic to fail any test this is called from.
    pub fn ensure_file_contents(path: impl AsRef<Path>, contents: impl AsRef<str>) {
        let path = path.as_ref();
        let contents = normalize_newlines(contents.as_ref());

        if let Ok(old_contents) = std::fs::read_to_string(path) {
            if contents == normalize_newlines(&old_contents) {
                // File is already up to date
                return;
            }
        }

        let display_path = path.strip_prefix(project_root()).unwrap_or(path);

        eprintln!("{} was not up-to-date, updating...", display_path.display());

        if std::env::var("CI").is_ok() {
            eprintln!("Note: run `cargo test` locally and commit the updated files");
        }

        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(path, contents).unwrap();
        panic!("some file was not up to date and has been updated. Please re-run the tests.");
    }

    fn normalize_newlines(s: &str) -> String {
        s.replace("\r\n", "\n")
    }
}
