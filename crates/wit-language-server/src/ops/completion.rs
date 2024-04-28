use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};
use wit_compiler::{
    access::ScopeIndex,
    ast::{self, AstNode},
    queries::{SourceFile, Workspace},
    Db,
};

pub fn complete(
    db: &dyn Db,
    _ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
) -> Vec<Completion> {
    let mut completions = Vec::new();

    // First, add all known keywords
    let keywords = wit_compiler::ast::KEYWORDS.iter().map(|kw| Completion {
        text: kw.to_string(),
        kind: CompletionKind::Keyword,
    });
    completions.extend(keywords);

    // Now, we do the hard job of figuring out which identifiers are in scope.
    // FIXME: This doesn't take imported items into account
    let items = wit_compiler::queries::file_items(db, file);
    if let Some(index) = items.enclosing_item(db, point) {
        let types = match index {
            ScopeIndex::World(index) => items.get_world(db, index).items(db),
            ScopeIndex::Interface(index) => items.get_interface(db, index).items(db),
        };

        completions.extend(types.names().map(|name| Completion {
            text: name.to_string(),
            kind: CompletionKind::TypeName,
        }));
    }

    let ast = wit_compiler::queries::parse(db, file);
    if let Some(ident) = ast
        .tree(db)
        .ancestors(point)
        .find_map(ast::Identifier::cast)
    {
        let src = file.contents(db);
        let ident = ident.value(src);
        // The user has started writing an identifier, so limit the completions
        // to whatever might match what they've written.
        completions.retain(|c| c.text.starts_with(ident));
    }

    completions
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Completion {
    pub text: String,
    pub kind: CompletionKind,
}

impl From<Completion> for CompletionItem {
    fn from(value: Completion) -> Self {
        CompletionItem {
            label: value.text,
            kind: Some(value.kind.into()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CompletionKind {
    TypeName,
    Interface,
    World,
    Keyword,
}

impl From<CompletionKind> for CompletionItemKind {
    fn from(value: CompletionKind) -> Self {
        match value {
            CompletionKind::TypeName => CompletionItemKind::STRUCT,
            CompletionKind::Interface => CompletionItemKind::INTERFACE,
            CompletionKind::World => CompletionItemKind::MODULE,
            CompletionKind::Keyword => CompletionItemKind::KEYWORD,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use im::OrdMap;
    use wit_compiler::{queries::FilePath, Compiler, Tree};

    use super::*;

    fn find_completion_location<'a>(
        tree: &Tree,
        src: &'a str,
    ) -> OrdMap<Cow<'a, str>, tree_sitter::Point> {
        tree.iter()
            .filter_map(wit_compiler::ast::BlockComment::cast)
            .map(|comment| (comment.text(src), comment.range().start_point))
            .collect()
    }

    #[test]
    fn completions_in_different_contexts() {
        let src = r#"
            interface foo {
                /* first */

                type x = /* second */ u32;
            }

            world bar {
                export /* third */;
            }
        "#;
        let db = Compiler::default();
        let path = FilePath::new(&db, "complete_builtins.wit".into());
        let file = SourceFile::new(&db, path, src.into());
        let ws = Workspace::new(&db, OrdMap::unit(path, file));
        let ast = wit_compiler::queries::parse(&db, file);
        let locations = find_completion_location(ast.tree(&db), src);

        let result: OrdMap<String, Vec<Completion>> = locations
            .into_iter()
            .map(|(k, p)| (k.to_string(), complete(&db, ws, file, p)))
            .collect();

        insta::assert_debug_snapshot!(result);
    }
}
