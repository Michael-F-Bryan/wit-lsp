use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};
use wit_compiler::{
    access::ScopeIndex,
    ast::{self, AstNode, HasSource},
    queries::{metadata::HasIdent, SourceFile, Workspace},
    Db,
};

pub fn complete(
    db: &dyn Db,
    ws: Workspace,
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
    let path = file.path(db);
    let pkg = wit_compiler::queries::workspace_packages(db, ws)
        .into_iter()
        .find(|pkg| pkg.contains(db, path))
        .expect("unreachable");

    let items = wit_compiler::queries::package_items(db, pkg);

    if let Some(index) = items.enclosing_item(db, point) {
        let types = match index {
            ScopeIndex::World(index) => items.get_world(db, index).definitions(db),
            ScopeIndex::Interface(index) => items
                .get_interface(db, index)
                .items(db)
                .into_iter()
                .filter_map(|item| match item {
                    wit_compiler::queries::metadata::InterfaceItemMetadata::Func(_) => None,
                    wit_compiler::queries::metadata::InterfaceItemMetadata::Type(ty) => Some(ty),
                })
                .collect(),
        };

        let names = types
            .iter()
            .map(|t| t.ident(db).raw(db).to_string())
            .map(Completion::type_name);
        completions.extend(names);
    }

    let ast = wit_compiler::queries::parse(db, file);
    if let Some(ident) = ast.tree(db).ancestors(point).find_map(ast::Id::cast) {
        let src = file.contents(db);
        let ident = ident.utf8_text(src);
        // The user has started writing an identifier, so limit the completions
        // to whatever might match what they've written.
        completions.retain(|c| c.text.starts_with(ident));
    }

    // completions.sort_by(|left, right| left.text.cmp(&right.text));

    completions
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Completion {
    pub text: String,
    pub kind: CompletionKind,
}

impl Completion {
    pub fn new(text: impl Into<String>, kind: CompletionKind) -> Self {
        Self {
            text: text.into(),
            kind,
        }
    }

    fn type_name(text: impl Into<String>) -> Self {
        Completion::new(text, CompletionKind::TypeName)
    }
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
            .filter_map(wit_compiler::ast::Comment::cast)
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
