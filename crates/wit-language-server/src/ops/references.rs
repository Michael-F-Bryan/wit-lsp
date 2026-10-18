use tower_lsp::lsp_types::{Location, Url};
use wit_compiler::{
    Db,
    ast::{self, AstNode},
    queries::{self, SourceFile, Workspace},
};

use super::definition::{declaration_for_identifier, identifier_at, symbol_definition};
use crate::utils::Positions;

/// Find identifier occurrences that resolve to the same declaration.
pub fn find_references(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
    include_declaration: bool,
) -> Vec<Location> {
    let Some(target) =
        identifier_at(db, file, point).and_then(|id| symbol_definition(db, ws, file, id))
    else {
        return Vec::new();
    };
    let mut references = Vec::new();
    for (path, file) in ws.files(db) {
        let Ok(uri) = path.raw_path(db).parse::<Url>() else {
            continue;
        };
        let positions = Positions::new(db, file);
        for id in identifiers(db, file) {
            let declaration = declaration_for_identifier(db, file, id);
            if !include_declaration && (declaration.is_some() || is_alias(id)) {
                continue;
            }
            if declaration.or_else(|| symbol_definition(db, ws, file, id)) == Some(target) {
                references.push(Location {
                    uri: uri.clone(),
                    range: positions.to_lsp(id.range()),
                });
            }
        }
    }

    references.sort_by(|left, right| {
        left.uri
            .cmp(&right.uri)
            .then(left.range.start.cmp(&right.range.start))
            .then(left.range.end.cmp(&right.range.end))
    });
    references.dedup();
    references
}

/// Identifier occurrences, excluding package segments and parser recovery placeholders.
pub(super) fn identifiers(db: &dyn Db, file: SourceFile) -> Vec<ast::Id<'_>> {
    let ast = queries::parse(db, file);
    let mut ids = Vec::new();
    let mut pending = vec![ast.root_node(db)];
    while let Some(node) = pending.pop() {
        if let Some(id) = ast::Id::cast(node) {
            if node.is_missing() || node.start_byte() == node.end_byte() {
                continue;
            }
            if let Some(path) = node.parent().and_then(ast::FullyQualifiedUsePath::cast)
                && path.iter_ids().last() != Some(id)
            {
                continue;
            }
            ids.push(id);
        } else {
            let mut cursor = node.walk();
            pending.extend(node.named_children(&mut cursor));
        }
    }
    ids
}

pub(super) fn is_alias(id: ast::Id<'_>) -> bool {
    let Some(parent) = id.syntax().parent() else {
        return false;
    };
    if let Some(item) = ast::ToplevelUseItem::cast(parent) {
        return item.iter_alias().any(|alias| alias == id);
    }
    matches!(parent.kind(), "use_names_item" | "include_names_item")
        && parent.child_by_field_name("alias") == Some(id.syntax())
}
