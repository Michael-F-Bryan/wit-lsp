//! Strongly-typed wrappers around [`tree_sitter_wit`]'s *Concrete Syntax Tree*.

pub extern crate tree_sitter_wit;

#[rustfmt::skip]
mod generated;
mod hand_written;
mod text;
mod traits;

use tree_sitter::Node;

pub use self::{
    generated::*,
    text::Text,
    traits::{AstNode, HasAttr, HasIdent, HasSource, NodeExt},
};

pub(crate) fn children(parent: Node<'_>) -> impl Iterator<Item = Node<'_>> {
    std::iter::successors(parent.child(0), |last| last.next_sibling())
}

/// Get the underlying name an identifier is referring to. This removes
/// %-escaping.
pub(crate) fn ident(raw: &str) -> &str {
    raw.strip_prefix('%').unwrap_or(raw)
}
