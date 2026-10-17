//! Strongly-typed wrappers around [`tree_sitter_wit`]'s *Concrete Syntax Tree*.

pub extern crate tree_sitter;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_iterator_outlives_node_wrapper() {
        let src = "/// Interface documentation\ninterface example {}";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_wit::language()).unwrap();
        let tree = parser.parse(src, None).unwrap();
        assert!(!tree.root_node().has_error());

        let mut attributes = {
            let interface = SourceFile::cast(tree.root_node())
                .unwrap()
                .iter_top_level_items()
                .find_map(TopLevelItem::interface_item)
                .unwrap();
            let borrowed = &interface;
            borrowed.iter_attributes()
        };

        assert_eq!(
            attributes.next().unwrap().utf8_text(src).trim(),
            "/// Interface documentation"
        );
        assert!(attributes.next().is_none());
    }
}
