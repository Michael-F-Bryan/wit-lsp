use std::{collections::BTreeMap, path::PathBuf};

use clap::Parser;
use color_eyre::{eyre::Context, Report};
use heck::ToPascalCase;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::utils;

static AST_GENERATED_PATH: Lazy<PathBuf> = Lazy::new(|| {
    utils::project_root()
        .join("crates")
        .join("wit-syntax")
        .join("src")
        .join("generated.rs")
});

const IDENT_BLACKLIST: &[&str] = &["type"];

macro_rules! format_ident {
    ($($token:tt)*) => {{
        let ident = quote::format_ident!($($token)*);

        if IDENT_BLACKLIST.iter().any(|s| ident == s) {
            quote::format_ident!("{ident}_")
        } else {
            ident
        }
    }};
}

#[derive(Debug, Clone, Parser)]
pub struct Ast {
    #[clap(short, long)]
    node_types: Option<PathBuf>,
    #[clap(short, long, default_value = AST_GENERATED_PATH.as_os_str())]
    out: PathBuf,
}

impl Ast {
    #[tracing::instrument(skip_all)]
    pub fn generate(self) -> Result<(), Report> {
        let Ast { node_types, out } = self;

        let node_types = match node_types {
            Some(path) => {
                tracing::debug!(path=%path.display(), "Reading node-types.json");
                std::fs::read_to_string(&path)
                    .with_context(|| format!("Unable to read \"{}\"", path.display()))?
            }
            None => tree_sitter_wit::NODE_TYPES.to_string(),
        };

        tracing::trace!(%node_types);

        let tokens = generate_ast(&node_types);
        let src = utils::format_rust(tokens);

        tracing::trace!(generated = src.as_str());

        utils::ensure_file_contents(out, src)?;

        Ok(())
    }
}

impl Default for Ast {
    fn default() -> Self {
        Ast {
            node_types: None,
            out: AST_GENERATED_PATH.clone(),
        }
    }
}

pub(crate) fn generate_ast(node_types: &str) -> TokenStream {
    let node_types: Vec<NodeType> = serde_json::from_str(node_types).unwrap();

    let (nodes, tokens): (Vec<_>, Vec<_>) = node_types.iter().partition(|n| n.named);

    let ast_nodes = nodes.iter().map(|n| generate_ast_node(n));
    let (keywords, punctuation): (Vec<_>, Vec<_>) = tokens
        .iter()
        .map(|t| t.kind.as_str())
        .partition(|t| t.chars().all(|c| c.is_alphanumeric()));

    quote! {
        //! Automatically generated code. DO NOT EDIT!

        /// Keywords used by the WIT language.
        pub const KEYWORDS: &[&str] = &[ #(#keywords),* ];

        /// Symbols and punctuation used by the WIT language.
        pub const PUNCTUATION: &[&str] = &[ #(#punctuation),* ];

        #( #ast_nodes )*

    }
}

fn rust_type_name(value: &str) -> Ident {
    let blacklist = ["option", "result"];

    if blacklist.contains(&value) {
        format_ident!("{}_", value.to_pascal_case())
    } else {
        format_ident!("{}", value.to_pascal_case())
    }
}

fn ast_node_impl(ident: &Ident, kind: &str) -> TokenStream {
    quote! {
        impl<'tree> super::AstNode<'tree> for #ident<'tree> {
            const NAME: &'static str = #kind;

            fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
            where
                Self: Sized
            {
                if node.kind() == Self::NAME {
                    Some(#ident(node))
                } else {
                    None
                }
            }

            fn syntax(&self) -> tree_sitter::Node<'tree> { self.0 }
        }
    }
}

fn generate_ast_node(node: &NodeType) -> TokenStream {
    let kind = node.kind.as_str();
    let ident = rust_type_name(kind);
    let doc = format!("The `{kind}` node.");

    let ast_node_impl = ast_node_impl(&ident, kind);

    let field_getters = node
        .fields
        .iter()
        .map(|(name, field)| field_getter(field, name));
    let child_getters = node.children.iter().flat_map(|field| {
        field
            .types
            .iter()
            .map(|ty| child_getter(ty, field.multiple, field.required))
    });

    let helper_trait_impls = generate_helper_trait_impls(&ident, node);

    quote! {
        #[doc = #doc]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct #ident<'tree>(tree_sitter::Node<'tree>);

        impl<'tree> #ident<'tree> {
            #(#field_getters)*
            #(#child_getters)*
        }

        #helper_trait_impls

        #ast_node_impl
    }
}

fn generate_helper_trait_impls(ident: &Ident, node: &NodeType) -> TokenStream {
    let mut tokens = TokenStream::new();

    if let Some(Field { required: true, .. }) = node.fields.get("name") {
        tokens.extend(quote! {
            impl super::HasIdent for #ident<'_> {
                fn identifier(self, src: &str) -> Option<&str> {
                    let node = self.name()?;
                    let raw = node.0.utf8_text(src.as_bytes()).unwrap();
                    Some(crate::ident(raw))
                }
            }
        });
    }

    if node.fields.contains_key("attributes") {
        tokens.extend(quote! {
            impl<'tree> super::HasAttr<'tree> for #ident<'tree> {
                fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
                    self.iter_attributes()
                }
            }
        });
    }

    tokens
}

fn field_getter(field: &Field, name: &str) -> TokenStream {
    let item_types: Vec<_> = field.types.iter().filter(|ty| ty.named).collect();
    assert_eq!(item_types.len(), 1, "The \"{name}\" getter can't be generated when there are multiple named types (item_types: {item_types:?})");
    let item_ty = item_types[0].type_name();

    if field.multiple {
        let method_name = if name.ends_with('s') {
            format_ident!("iter_{name}")
        } else {
            format_ident!("iter_{name}s")
        };

        quote! {
            pub fn #method_name(&self) -> impl Iterator<Item=#item_ty<'tree>> {
                let mut cursor = self.0.walk();
                let children: Vec<_> = self.0.children_by_field_name(#name, &mut cursor)
                    .filter_map(<#item_ty as super::AstNode>::cast)
                    .collect();
                children.into_iter()
            }
        }
    } else if !field.required {
        let method_name = format_ident!("{name}_opt");

        quote! {
            pub fn #method_name(&self) -> Option<#item_ty<'tree>> {
                self.0.child_by_field_name(#name).and_then(<#item_ty as super::AstNode>::cast)
            }
        }
    } else {
        let method_name = format_ident!("{name}");

        quote! {
            pub fn #method_name(&self) -> Option<#item_ty<'tree>> {
                self.0.child_by_field_name(#name).and_then(<#item_ty as super::AstNode>::cast)
            }
        }
    }
}

fn child_getter(item_ty: &FieldType, multiple: bool, required: bool) -> TokenStream {
    let name = &item_ty.kind;
    let item_ty = rust_type_name(name);

    if multiple {
        let method_name = if name.ends_with('s') {
            format_ident!("iter_{name}")
        } else {
            format_ident!("iter_{name}s")
        };

        quote! {
            pub fn #method_name(self) -> impl Iterator<Item=#item_ty<'tree>> {
                super::children(self.0)
                    .filter_map(<#item_ty as super::AstNode<'_>>::cast)
            }
        }
    } else if !required {
        let method_name = format_ident!("{name}_opt");

        quote! {
            pub fn #method_name(self) -> Option<#item_ty<'tree>> {
                super::children(self.0)
                    .filter_map(<#item_ty as super::AstNode<'_>>::cast)
                    .next()
            }
        }
    } else {
        let method_name = format_ident!("{name}");

        quote! {
            pub fn #method_name(self) -> Option<#item_ty<'tree>> {
                super::children(self.0)
                    .filter_map(<#item_ty as super::AstNode<'_>>::cast)
                    .next()
            }
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct NodeType {
    #[serde(rename = "type")]
    kind: String,
    named: bool,
    #[serde(default)]
    fields: BTreeMap<String, Field>,
    children: Option<Field>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Field {
    multiple: bool,
    required: bool,
    types: Vec<FieldType>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FieldType {
    #[serde(rename = "type")]
    kind: String,
    named: bool,
}

impl FieldType {
    fn type_name(&self) -> Ident {
        if self.named {
            rust_type_name(&self.kind)
        } else {
            TOKENS
                .iter()
                .find(|t| t.literal == self.kind)
                .unwrap()
                .type_name()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Token {
    /// The literal as it appears in `grammar.js`.
    literal: &'static str,
    /// The human-friendly name for this token.
    name: &'static str,
    /// What type of token is it?
    kind: TokenKind,
}

impl Token {
    fn type_name(&self) -> Ident {
        let kind = self.name.to_pascal_case();

        match self.kind {
            TokenKind::Punctuation => format_ident!("{kind}Symbol"),
            TokenKind::Keyword => format_ident!("{kind}Keyword"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum TokenKind {
    Punctuation,
    Keyword,
}

const TOKENS: &[Token] = &[
    Token {
        literal: "(",
        name: "open_paren",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: ")",
        name: "close_paren",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: ",",
        name: "comma",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "->",
        name: "arrow",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: ".",
        name: "dot",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "/",
        name: "slash",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "///",
        name: "triple_slash",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: ":",
        name: "colon",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: ";",
        name: "semicolon",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "<",
        name: "left_angle_bracket",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "=",
        name: "equals",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: ">",
        name: "right_angle_bracket",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "@",
        name: "at",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "_",
        name: "underscore",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "as",
        name: "as",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "bool",
        name: "bool",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "borrow",
        name: "borrow",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "char",
        name: "char",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "constructor",
        name: "constructor",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "enum",
        name: "enum",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "export",
        name: "export",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "flags",
        name: "flags",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "float32",
        name: "float32",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "float64",
        name: "float64",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "func",
        name: "func",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "import",
        name: "import",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "include",
        name: "include",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "interface",
        name: "interface",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "list",
        name: "list",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "option",
        name: "option",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "package",
        name: "package",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "record",
        name: "record",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "resource",
        name: "resource",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "result",
        name: "result",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "s16",
        name: "s16",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "s32",
        name: "s32",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "s64",
        name: "s64",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "s8",
        name: "s8",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "static",
        name: "static",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "string",
        name: "string",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "tuple",
        name: "tuple",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "type",
        name: "type",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "u16",
        name: "u16",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "u32",
        name: "u32",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "u64",
        name: "u64",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "u8",
        name: "u8",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "use",
        name: "use",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "variant",
        name: "variant",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "with",
        name: "with",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "world",
        name: "world",
        kind: TokenKind::Keyword,
    },
    Token {
        literal: "{",
        name: "open_brace",
        kind: TokenKind::Punctuation,
    },
    Token {
        literal: "}",
        name: "close_brace",
        kind: TokenKind::Punctuation,
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn ast_is_up_to_date() {
        let tokens = generate_ast(tree_sitter_wit::NODE_TYPES);
        let src = utils::format_rust(tokens);
        utils::ensure_file_contents(&*AST_GENERATED_PATH, src).unwrap();
    }
}
