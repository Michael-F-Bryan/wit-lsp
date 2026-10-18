use tower_lsp::lsp_types::{DocumentHighlight, DocumentHighlightKind};
use wit_compiler::{
    Db,
    ast::AstNode,
    queries::{SourceFile, Workspace},
};

use super::{
    definition::{identifier_at, symbol_binding, symbol_definition},
    references::identifiers,
};
use crate::utils::Positions;

/// Highlight occurrences of the selected binding in this document only.
pub fn document_highlights(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
) -> Vec<DocumentHighlight> {
    let Some(id) = identifier_at(db, file, point) else {
        return Vec::new();
    };
    let ids = identifiers(db, file);
    if !ids.contains(&id) || symbol_definition(db, ws, file, id).is_none() {
        return Vec::new();
    }
    let Some(binding) = symbol_binding(db, ws, file, id) else {
        return Vec::new();
    };
    let positions = Positions::new(db, file);
    let mut highlights: Vec<_> = ids
        .into_iter()
        .filter(|id| symbol_binding(db, ws, file, *id) == Some(binding))
        .map(|id| DocumentHighlight {
            range: positions.to_lsp(id.range()),
            kind: Some(DocumentHighlightKind::TEXT),
        })
        .collect();
    highlights.sort_by_key(|highlight| (highlight.range.start, highlight.range.end));
    highlights.dedup();
    highlights
}
