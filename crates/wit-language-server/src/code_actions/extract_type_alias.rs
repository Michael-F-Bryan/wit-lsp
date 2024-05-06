use tree_sitter::Range;
use wit_compiler::ast::{self, AstNode, HasSource};

use crate::code_actions::{Action, CodeActionContext, TextEdit};

const TYPE_COMPLEXITY_THRESHOLD: usize = 3;

/// # Extract Type Alias
///
/// Extract a generic type (e.g. `list<tuple<string, string>>`) into a type
/// alias.
pub(crate) fn extract_type_alias_action(ctx: &CodeActionContext<'_>) -> Option<Action> {
    let ty: ast::Ty<'_> = ctx.next_ancestor()?;

    let mut cursor = ty.syntax().walk();

    let type_complexity =
        wit_compiler::traverse::cursor(&mut cursor, wit_compiler::traverse::Order::Pre)
            .filter(|&node| ast::Ty::cast(node).is_some())
            .count();

    if type_complexity < TYPE_COMPLEXITY_THRESHOLD {
        return None;
    }

    let parent_item = if let Some(item) = ctx.next_ancestor::<ast::InterfaceItems<'_>>() {
        item.syntax()
    } else if let Some(item) = ctx.next_ancestor::<ast::WorldItems<'_>>() {
        item.syntax()
    } else {
        return None;
    };

    let Range { start_byte, .. } = parent_item.range();
    let derived_name: String = type_alias_name(ty, ctx.src());
    let type_definition = ty.utf8_text(ctx.src());
    let indentation = " ".repeat(parent_item.range().start_point.column);

    let mut builder = TextEdit::builder();
    builder.insert(
        start_byte,
        format!("type {derived_name} = {type_definition};\n{indentation}"),
    );
    builder.replace(ty.syntax().byte_range(), derived_name);

    Some(Action::refactor("Extract Type Alias").modify(ctx.path(), builder.finish()))
}

fn type_alias_name(ty: ast::Ty<'_>, src: &str) -> String {
    let cursor = ty.syntax().walk();

    let mut text = String::new();

    let pieces = wit_compiler::traverse::cursor(cursor, wit_compiler::traverse::Order::Post)
        .filter(|n| !n.is_error() && !n.is_extra())
        .filter(|n| n.child_count() == 0)
        .map(|n| n.utf8_text(src.as_bytes()).unwrap())
        .flat_map(|piece| piece.split(|c: char| !c.is_alphanumeric()))
        .filter(|piece| !piece.is_empty());

    for piece in pieces {
        if !text.is_empty() && !text.ends_with('-') {
            text.push('-');
        }

        text.push_str(piece);
    }

    text.replace("--", "-")
}

#[cfg(test)]
mod tests {
    use im::OrdMap;
    use tower_lsp::lsp_types::TextDocumentIdentifier;
    use wit_compiler::{
        ast::AstNode,
        queries::{FilePath, SourceFile, Workspace},
    };

    use crate::Database;

    use super::*;

    #[test]
    fn extract_a_type_alias() {
        let before = r#"
            interface x {
                y: func(a: list< /* HERE */ list<u32>>);
            }
        "#;
        let mut db = Database::default();
        let path = FilePath::new(&db, "file:///file.wit".into());
        let mut files = OrdMap::new();
        let file = SourceFile::new(&db, path, before.into());
        files.insert(path, file);
        let ws = Workspace::new(&db, files);
        let ast = wit_compiler::queries::parse(&db, file);
        let cursor = ast
            .tree(&db)
            .iter()
            .find_map(wit_compiler::ast::BlockComment::cast)
            .unwrap();
        let ctx = CodeActionContext::from_lsp(
            &db,
            ws,
            tower_lsp::lsp_types::CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: "file:///file.wit".parse().unwrap(),
                },
                range: crate::utils::ts_to_range(cursor.range()),
                context: Default::default(),
                partial_result_params: Default::default(),
                work_done_progress_params: Default::default(),
            },
        )
        .unwrap();

        let action = extract_type_alias_action(&ctx).unwrap();

        let after = r#"
            interface x {
                type list-list-u32 = list< /* HERE */ list<u32>>;
                y: func(a: list-list-u32);
            }
        "#;
        action.apply(&mut db, ws);
        assert_eq!(file.contents(&db), after);
    }
}
