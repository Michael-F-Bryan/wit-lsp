use std::path::{Path, PathBuf};

use clap::Parser;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use regex::Regex;

use crate::utils;

static ACTIONS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    utils::project_root()
        .join("crates")
        .join("wit-language-server")
        .join("src")
        .join("code_actions")
});
static ACTIONS_OUTPUT_PATH: Lazy<PathBuf> = Lazy::new(|| ACTIONS_DIR.join("all.rs"));

#[derive(Debug, Clone, Parser)]
pub struct CodeActions {
    /// The directory containing all code actions files.
    #[clap(short, long, default_value = ACTIONS_DIR.as_os_str())]
    actions: PathBuf,
    /// Where to save the generated code.
    #[clap(short, long, default_value = ACTIONS_OUTPUT_PATH.as_os_str())]
    out: PathBuf,
}

impl CodeActions {
    pub fn generate(self) -> color_eyre::Result<()> {
        let mut actions = Vec::new();

        for entry in self.actions.read_dir()? {
            let entry = entry?;
            let meta = entry.metadata()?;

            if meta.is_file() {
                let path = entry.path();
                if let Some(action) = parse_action(&path)? {
                    actions.push(action);
                }
            }
        }

        let tokens = codegen(&actions);
        let generated = crate::utils::format_rust(tokens);
        crate::utils::ensure_file_contents(self.out, generated)?;

        Ok(())
    }
}

impl Default for CodeActions {
    fn default() -> Self {
        Self {
            actions: ACTIONS_DIR.clone(),
            out: ACTIONS_OUTPUT_PATH.clone(),
        }
    }
}

fn codegen(actions: &[Action]) -> TokenStream {
    let actions = actions.iter().map(
        |Action {
             name,
             description,
             module_name,
             function,
         }| {
            quote!(CodeAction {
               name: #name,
               description: #description,
               execute: super::#module_name::#function,
            })
        },
    );

    quote! {
        //! Automatically generated code. DO NOT EDIT!
        use super::CodeAction;

        /// A list of all registered [`CodeAction`]s.
        pub const CODE_ACTIONS: &[CodeAction] = &[ #( #actions ),* ];
    }
}

#[derive(Debug)]
struct Action {
    name: String,
    description: String,
    module_name: Ident,
    function: Ident,
}

fn parse_action(path: &Path) -> color_eyre::Result<Option<Action>> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r#"(?x)
            \/\/\/ \s+ \# \s+ (?P<title>[^\n]+) \n
            (?P<docs>(?:\/\/\/[^\n]*\n)+)
            pub \s* \(crate\) \s+ fn \s+ (?P<name>[\w\d_]+_action)\(
        "#,
        )
        .unwrap()
    });

    let src = std::fs::read_to_string(path)?;

    let Some(caps) = PATTERN.captures(&src) else {
        return Ok(None);
    };

    let mut docs = String::new();

    for line in caps["docs"].lines() {
        let line = line
            .strip_prefix("/// ")
            .or_else(|| line.strip_prefix("///"))
            .unwrap_or(line);
        docs.push_str(line);
        docs.push('\n');
    }
    let module = path.file_stem().unwrap().to_str().unwrap();

    Ok(Some(Action {
        name: caps["title"].trim().to_string(),
        description: docs.trim().to_string(),
        module_name: Ident::new(module, Span::call_site()),
        function: Ident::new(&caps["name"], Span::call_site()),
    }))
}
