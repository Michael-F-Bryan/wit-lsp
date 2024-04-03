//! Strongly-typed AST nodes.

#[rustfmt::skip]
mod generated;
mod hand_written;
mod traits;

use tree_sitter::Node;

pub use self::{
    generated::*,
    traits::{AstNode, HasAttr, HasIdent, HasSource, NodeExt},
};

pub(crate) fn children(parent: Node<'_>) -> impl Iterator<Item = Node<'_>> {
    std::iter::successors(parent.child(0), |last| last.next_sibling())
}
