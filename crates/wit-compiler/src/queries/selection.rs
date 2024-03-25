use im::Vector;
use tree_sitter::{Node, Point, Range};

use crate::{queries::Ast, Db};

/// Calculate successively wider ranges of tokens.
#[salsa::tracked]
pub fn selection_ranges(db: &dyn Db, ast: Ast, cursor_location: Point) -> Option<Vector<Range>> {
    selection_ranges_impl(ast.root_node(db), cursor_location)
}

fn selection_ranges_impl(root: Node<'_>, cursor_location: Point) -> Option<Vector<Range>> {
    if cursor_location > root.end_position() {
        // Out of bounds
        return None;
    }

    let mut cursor = root.walk();

    while cursor.goto_first_child_for_point(cursor_location).is_some() {
        // keep iterating
    }

    let mut ranges = Vector::new();
    ranges.push_back(cursor.node().range());

    while cursor.goto_parent() {
        let range = cursor.node().range();

        if ranges.last() == Some(&range) {
            // We want to ignore ranges that won't expand the selection.
            continue;
        }

        ranges.push_back(range);
    }

    Some(ranges)
}

#[cfg(test)]
mod tests {
    use crate::queries::Tree;

    use super::*;

    #[test]
    fn calculate_selection_ranges() {
        let src = include_str!("../../../../integration-tests/compile-pass/all-resources.wit");
        let tree = Tree::parse(src);
        let point = Point {
            row: 10,
            column: 18,
        };

        let ranges = selection_ranges_impl(tree.root_node(), point).unwrap();

        insta::assert_debug_snapshot!(ranges);
    }
}
