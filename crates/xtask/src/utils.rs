#![allow(dead_code)] // used during testing

use std::path::Path;

use color_eyre::eyre::{Context, Report};
use quote::ToTokens;

/// Format some Rust tokens.
///
/// # Panics
///
/// It is assumed that the tokens would parse as a Rust file.
pub fn format_rust(contents: impl ToTokens) -> String {
    let contents =
        syn::parse2(contents.to_token_stream()).expect("Unable to parse the tokens as a syn::File");
    prettyplease::unparse(&contents)
}

/// Check that a particular file has the desired contents.
///
/// If the file is missing or outdated, this function will update the file and
/// error out.
pub fn ensure_file_contents(
    path: impl AsRef<Path>,
    contents: impl AsRef<str>,
) -> Result<(), Report> {
    let path = path.as_ref();
    let contents = normalize_newlines(contents.as_ref());

    if let Ok(old_contents) = std::fs::read_to_string(path) {
        if contents == normalize_newlines(&old_contents) {
            // File is already up to date
            return Ok(());
        }
    }

    let display_path = path.strip_prefix(project_root()).unwrap_or(path);

    eprintln!(
        "\"{}\" was not up-to-date, updating...",
        display_path.display()
    );

    if std::env::var("CI").is_ok() {
        eprintln!("Note: run codegen locally and commit the updated files");
    }

    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(path, contents)
        .wrap_err_with(|| format!("Unable to save to \"{}\"", path.display()))?;

    color_eyre::eyre::bail!(
        "\"{}\" was not up to date and has been updated. Please re-run the tests.",
        display_path.display()
    );
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}

/// Get the root directory for this repository.
pub fn project_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|p| p.join(".git").exists())
        .unwrap()
}
