use im::Vector;
use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind};
use tree_sitter::{Node, QueryCursor, StreamingIterator};
use wit_compiler::queries::Ast;

use crate::{Db, utils::Positions};

/// Implement [range folding][spec].
///
/// [spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_foldingRange
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all)]
pub fn folding_range(db: &dyn Db, ast: Ast) -> Vector<FoldingRange> {
    let db = db.as_wit();
    let root = ast.root_node(db);
    let src = ast.src(db);

    folding_range_impl(root, src)
}

fn folding_range_impl(root: Node<'_>, src: &str) -> Vector<FoldingRange> {
    let positions = Positions::for_text(src);
    let mut cursor = QueryCursor::new();
    let language = tree_sitter_wit::language();
    let query = tree_sitter::Query::new(&language, tree_sitter_wit::FOLDING_QUERY)
        .expect("The query was invalid");

    let capture_names = query.capture_names();

    let mut ranges = Vector::new();
    let mut matches = cursor.matches(&query, root, src.as_bytes());
    while let Some(m) = matches.next() {
        for capture in m.captures() {
            let kind = match capture_names[capture.index as usize] {
                "comments" => Some(FoldingRangeKind::Comment),
                "imports" => Some(FoldingRangeKind::Imports),
                "normal" => None,
                other => unreachable!(
                    "Unknown query capture name, {other:?}, expected one of {capture_names:?}"
                ),
            };
            ranges.push_back(node_range(capture.node, kind, &positions));
        }
    }
    ranges
}

fn node_range(
    node: Node<'_>,
    kind: Option<FoldingRangeKind>,
    positions: &Positions<'_>,
) -> FoldingRange {
    let range = positions.to_lsp(node.range());
    FoldingRange {
        start_line: range.start.line,
        start_character: Some(range.start.character),
        end_line: range.end.line,
        end_character: Some(range.end.character),
        kind,
        collapsed_text: None,
    }
}

#[cfg(test)]
mod tests {
    use wit_compiler::Tree;

    use super::*;

    #[test]
    fn fold_all_resources() {
        let src = include_str!("../../../../integration-tests/compile-pass/_all-resources.wit");
        let tree = Tree::parse(src);

        let got = folding_range_impl(tree.root_node(), src);

        insta::assert_yaml_snapshot!(&got);
    }
}
