use im::Vector;
use tree_sitter::{Point, Range};

use crate::{queries::Ast, Db, Tree};

/// Calculate successively wider ranges of tokens.
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all)]
pub fn selection_ranges(db: &dyn Db, ast: Ast, cursor_location: Point) -> Option<Vector<Range>> {
    selection_ranges_impl(ast.tree(db), cursor_location)
}

fn selection_ranges_impl(tree: &Tree, cursor_location: Point) -> Option<Vector<Range>> {
    if cursor_location > tree.root_node().end_position() {
        // Out of bounds
        return None;
    }

    let mut ranges = Vector::new();

    for range in tree.ancestors(cursor_location).map(|n| n.range()) {
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
    use super::*;

    #[test]
    fn calculate_selection_ranges() {
        let src = include_str!("../../../../integration-tests/compile-pass/all-resources.wit");
        let tree = Tree::parse(src);
        let point = Point {
            row: 10,
            column: 18,
        };

        let ranges = selection_ranges_impl(&tree, point).unwrap();

        insta::assert_debug_snapshot!(ranges);
    }
}
