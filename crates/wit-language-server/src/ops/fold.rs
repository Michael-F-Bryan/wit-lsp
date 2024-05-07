use im::Vector;
use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind};
use tree_sitter::{Node, QueryCursor};
use wit_compiler::queries::Ast;

use crate::Db;

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
    let mut cursor = QueryCursor::new();
    let language = tree_sitter_wit::language();
    let query = tree_sitter::Query::new(&language, tree_sitter_wit::FOLDING_QUERY)
        .expect("The query was invalid");

    let capture_names = query.capture_names();

    cursor
        .matches(&query, root, src.as_bytes())
        .flat_map(|m| {
            let kind = match capture_names[m.pattern_index] {
                "comments" => Some(FoldingRangeKind::Comment),
                "imports" => Some(FoldingRangeKind::Imports),
                "normal" => None,
                other => unreachable!(
                    "Unknown query capture name, {other:?}, expected one of {capture_names:?}"
                ),
            };
            m.captures
                .iter()
                .map(move |cap| node_range(cap.node, kind.clone()))
        })
        .collect()
}

fn node_range(node: Node<'_>, kind: Option<FoldingRangeKind>) -> FoldingRange {
    let tree_sitter::Range {
        start_point,
        end_point,
        ..
    } = node.range();

    FoldingRange {
        start_line: start_point.row as u32,
        start_character: Some(start_point.column as u32),
        end_line: end_point.row as u32,
        end_character: Some(end_point.column as u32),
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
