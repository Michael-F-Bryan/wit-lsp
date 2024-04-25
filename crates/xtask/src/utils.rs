#![allow(dead_code)] // used during testing

use std::path::Path;

use color_eyre::eyre::{Context, Report};
use quote::ToTokens;
use xshell::Shell;

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
#[tracing::instrument(level="debug", skip_all, fields(path=%path.as_ref().display()))]
pub fn ensure_file_contents(
    path: impl AsRef<Path>,
    contents: impl AsRef<str>,
) -> Result<(), Report> {
    let path = path.as_ref();
    let contents = normalize_newlines(contents.as_ref());

    if let Ok(old_contents) = std::fs::read_to_string(path) {
        if contents == normalize_newlines(&old_contents) {
            tracing::debug!("File is already up to date");
            return Ok(());
        }
    }

    let display_path = path.strip_prefix(project_root()).unwrap_or(path);

    tracing::warn!(
        path=%display_path.display(),
        "File was not up-to-date, updating...",
    );

    if std::env::var("CI").is_ok() {
        tracing::warn!("Note: run codegen locally and commit the updated files");
    }

    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    tracing::info!(
        path=%path.display(),
        bytes_written=contents.len(),
        "Updating file",
    );
    std::fs::write(path, contents)
        .wrap_err_with(|| format!("Unable to save to \"{}\"", path.display()))?;

    color_eyre::eyre::bail!(
        "\"{}\" was not up to date and has been updated. Please re-run code generation.",
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

#[tracing::instrument(skip_all, fields(from = %from.as_ref().display(), to = %to.as_ref().display()))]
pub fn copy_dir(
    sh: &Shell,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> color_eyre::Result<()> {
    let from = from.as_ref();

    let _guard = sh.push_dir(to);

    for entry in walkdir::WalkDir::new(from) {
        let entry = entry?;
        let meta = entry.metadata()?;
        let path = entry.path();
        let dest = path.strip_prefix(from)?;

        if meta.is_dir() {
            tracing::debug!(dir=%dest.display(), "Creating directory");
            sh.create_dir(dest)?;
        } else if meta.is_file() {
            tracing::debug!(path=%dest.display(), "Copying");
            sh.copy_file(path, dest)?;
        }
    }

    Ok(())
}
