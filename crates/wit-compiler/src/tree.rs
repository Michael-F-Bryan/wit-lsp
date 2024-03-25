use std::fmt::Display;

use tree_sitter::{Node, Parser, Point};

use crate::traverse::Order;

/// A wrapper around [`tree_sitter::Tree`] that is comparable.
#[derive(Debug, Clone)]
pub struct Tree(pub tree_sitter::Tree);

impl Tree {
    pub fn parse(src: &str) -> Tree {
        let mut parser = Parser::new();
        let lang = tree_sitter_wit::language();
        parser
            .set_language(&lang)
            .expect("The tree-sitter dependencies should be in sync");

        // TODO: Use incremental parsing if we've already parsed this file before
        parser
            .parse(src.as_bytes(), None)
            .expect("We don't use cancellation")
            .into()
    }

    pub fn node_at(&self, point: Point) -> Option<Node<'_>> {
        self.ancestors(point).next()
    }

    /// Finds the node at a particular location, returning the node, followed by
    /// all parent nodes.
    #[track_caller]
    pub fn ancestors(&self, point: Point) -> impl Iterator<Item = Node<'_>> {
        let root = self.root_node();

        assert!(
            point <= root.end_position(),
            "{point} must lie within {:?}",
            root.range()
        );

        let mut cursor = root.walk();

        while cursor.goto_first_child_for_point(point).is_some() {
            // keep iterating
        }

        let innermost_node = cursor.node();
        let parents = std::iter::from_fn(move || {
            if cursor.goto_parent() {
                Some(cursor.node())
            } else {
                None
            }
        });

        std::iter::once(innermost_node).chain(parents)
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root_node())
    }
}

impl From<tree_sitter::Tree> for Tree {
    fn from(value: tree_sitter::Tree) -> Self {
        Tree(value)
    }
}

impl From<Tree> for tree_sitter::Tree {
    fn from(value: Tree) -> Self {
        value.0
    }
}

impl std::ops::Deref for Tree {
    type Target = tree_sitter::Tree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        if *self.language() != *other.language() {
            // They use different languages
            return false;
        }

        if self.included_ranges() != other.included_ranges() {
            // The trees spans different ranges
            return false;
        }

        // Looks like we need to walk the tree
        let mut left = crate::traverse::tree(&self.0, Order::Pre);
        let mut right = crate::traverse::tree(other, Order::Pre);

        loop {
            match (left.next(), right.next()) {
                (Some(lhs), Some(rhs)) => {
                    if lhs.range() != rhs.range() || lhs.kind_id() != rhs.kind_id() {
                        return false;
                    }
                }
                (None, Some(_)) | (Some(_), None) => return false,
                (None, None) => return true,
            }
        }
    }
}

impl Eq for Tree {}
