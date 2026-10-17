use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};
use wit_compiler::{
    ast::{self, AstNode, HasSource},
    queries::{SourceFile, Workspace},
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

    let types = wit_compiler::queries::visible_types(db, ws, file, point);
    completions.extend(
        types
            .keys()
            .map(|name| Completion::type_name(name.raw(db).as_str())),
    );

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

    #[test]
    fn completes_only_types_visible_in_the_current_file_and_scope() {
        for scope in ["interface", "world"] {
            let db = Compiler::default();
            let source_path = FilePath::new(&db, "file:///source.wit".into());
            let source_file = SourceFile::new(
                &db,
                source_path,
                "interface source { type original = u32; type hidden = string; }".into(),
            );
            let path = FilePath::new(&db, "file:///consumer.wit".into());
            let source = format!("{scope} consumer {{ use source.{{original as renamed}}; type local = u32; /* here */ }}");
            let point = tree_sitter::Point::new(0, source.find("/* here */").unwrap());
            let file = SourceFile::new(&db, path, source.into());
            let ws = Workspace::new(
                &db,
                [(source_path, source_file), (path, file)]
                    .into_iter()
                    .collect(),
            );
            let mut names = complete(&db, ws, file, point)
                .into_iter()
                .filter(|item| item.kind == CompletionKind::TypeName)
                .map(|item| item.text)
                .collect::<Vec<_>>();
            names.sort();
            assert_eq!(names, ["local", "renamed"]);
            for name in names {
                let ident = wit_compiler::queries::metadata::Ident::new(&db, name.into());
                assert!(wit_compiler::queries::resolve_type(&db, ws, file, point, ident).is_some());
            }
        }
    }

    #[test]
    fn imported_aliases_are_filtered_by_the_typed_prefix() {
        let db = Compiler::default();
        let path = FilePath::new(&db, "file:///types.wit".into());
        let source = "interface source { type original = u32; } interface consumer { use source.{original as renamed}; type alias = ren; }";
        let file = SourceFile::new(&db, path, source.into());
        let ws = Workspace::new(&db, [(path, file)].into_iter().collect());
        let point = tree_sitter::Point::new(0, source.find("= ren;").unwrap() + 2);
        let items = complete(&db, ws, file, point);
        assert_eq!(items, [Completion::type_name("renamed")]);
    }
}
