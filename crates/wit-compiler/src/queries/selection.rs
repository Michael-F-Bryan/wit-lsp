use im::Vector;
use tree_sitter::{Point, Range};

use crate::{Db, Tree, queries::Ast};

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
        let src = include_str!("../../../../integration-tests/compile-pass/_all-resources.wit");
        let tree = Tree::parse(src);
        let point = Point {
            row: 10,
            column: 18,
        };

        let ranges = selection_ranges_impl(&tree, point).unwrap();

        insta::assert_debug_snapshot!(ranges);
    }

    #[test]
    fn selections_contain_the_cursor_at_token_boundaries_whitespace_and_eof() {
        let src = "interface test { type sample = u32;   type other = string; }\n";
        let tree = Tree::parse(src);

        for offset in 0..=src.len() {
            let prefix = &src[..offset];
            let point = Point::new(
                prefix.bytes().filter(|&b| b == b'\n').count(),
                prefix.rsplit('\n').next().unwrap().len(),
            );
            let ranges = selection_ranges_impl(&tree, point).unwrap();
            assert!(!ranges.is_empty(), "no selection at {point:?}");
            for range in &ranges {
                assert!(
                    range.start_point <= point && point <= range.end_point,
                    "selection {range:?} excludes cursor {point:?}"
                );
            }
        }
    }
}
