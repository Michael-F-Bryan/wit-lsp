use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Context;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::utils;

static DIAGNOSTICS_DIR_PATH: Lazy<PathBuf> = Lazy::new(|| {
    utils::project_root()
        .join("crates")
        .join("wit-compiler")
        .join("src")
        .join("diagnostics")
});
static DIAGNOSTICS_MOD_PATH: Lazy<PathBuf> = Lazy::new(|| DIAGNOSTICS_DIR_PATH.join("mod.rs"));
static DIAGNOSTICS_OUTPUT_PATH: Lazy<PathBuf> = Lazy::new(|| DIAGNOSTICS_DIR_PATH.join("all.rs"));

#[derive(Debug, Clone, Parser)]
pub struct Diagnostics {
    #[clap(short, long, default_value = DIAGNOSTICS_MOD_PATH.as_os_str())]
    diagnostics: PathBuf,
    #[clap(short, long, default_value = DIAGNOSTICS_OUTPUT_PATH.as_os_str())]
    out: PathBuf,
}

impl Diagnostics {
    #[tracing::instrument(skip_all)]
    pub fn generate(self) -> color_eyre::Result<()> {
        let Diagnostics { diagnostics, out } = self;

        tracing::debug!(path=%diagnostics.display(), "Extracting all diagnostic types");
        let src = std::fs::read_to_string(&diagnostics)
            .with_context(|| format!("Unable to read \"{}\"", diagnostics.display()))?;

        let diagnostic_types = extract_diagnostic_types(&src)?;
        tracing::debug!(?diagnostic_types);

        let tokens = codegen(&diagnostic_types);
        let src = utils::format_rust(tokens);
        tracing::trace!(generated = src.as_str());

        utils::ensure_file_contents(out, src)?;

        Ok(())
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self {
            diagnostics: DIAGNOSTICS_MOD_PATH.clone(),
            out: DIAGNOSTICS_OUTPUT_PATH.clone(),
        }
    }
}

fn extract_diagnostic_types(src: &str) -> color_eyre::Result<Vec<&str>> {
    let diagnostic_types: Vec<_> = src
        .lines()
        .skip_while(|line| !line.contains("enum Diagnostic"))
        .skip(1)
        .take_while(|line| !line.contains('}'))
        .map(|line| {
            let line: Vec<_> = line.split(|c| c == '(' || c == ')').collect();
            assert_eq!(line.len(), 3, "{line:?}");
            line[1]
        })
        .collect();

    color_eyre::eyre::ensure!(!diagnostic_types.is_empty(), "No diagnostic types found");

    Ok(diagnostic_types)
}

fn codegen(diagnostic_types: &[&str]) -> TokenStream {
    let diags: Vec<Ident> = diagnostic_types
        .iter()
        .map(|s| format_ident!("{s}"))
        .collect();

    quote! {
        use super::*;

        /// Information about all know [`Diagnostic`] types.
        pub fn all_diagnostics() -> Vec<DiagnosticInfo> {
            Vec::from([
                #(
                    DiagnosticInfo::for_type::<#diags>()
                ),*
            ])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_are_up_to_date() {
        Diagnostics::default().generate().unwrap();
    }
}
