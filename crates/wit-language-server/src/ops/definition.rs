use wit_compiler::{
    Db,
    ast::{self, AstNode},
    diagnostics::Location,
    queries::{
        self, SourceFile, Workspace,
        metadata::{HasDefinition, Ident, TopLevelItemMetadata},
    },
};

/// Find the declaration referenced by the identifier under the cursor.
pub fn goto_definition(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
) -> Option<Location> {
    definition_for_identifier(db, ws, file, identifier_at(db, file, point)?)
}

/// Find the symbol's declaration, retaining explicit import aliases.
pub fn goto_declaration(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
) -> Option<Location> {
    symbol_binding(db, ws, file, identifier_at(db, file, point)?)
}

pub(super) fn identifier_at(
    db: &dyn Db,
    file: SourceFile,
    point: tree_sitter::Point,
) -> Option<ast::Id<'_>> {
    let ast = queries::parse(db, file);
    if point >= ast.root_node(db).end_position() {
        return None;
    }
    let node = ast.root_node(db).descendant_for_point_range(point, point)?;
    let id = ancestors(node).find_map(ast::Id::cast)?;
    (point >= id.range().start_point && point < id.range().end_point).then_some(id)
}

pub(super) fn definition_for_identifier(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    id: ast::Id<'_>,
) -> Option<Location> {
    resolve_identifier(db, ws, file, id, false)
}

/// Resolve the binding being named, without expanding explicit aliases.
pub(super) fn symbol_binding(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    id: ast::Id<'_>,
) -> Option<Location> {
    resolve_identifier(db, ws, file, id, true).or_else(|| declaration_for_identifier(db, file, id))
}

fn resolve_identifier(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    id: ast::Id<'_>,
    binding: bool,
) -> Option<Location> {
    let name = Ident::from_ast(db, id, file.contents(db));
    for node in ancestors(id.syntax()) {
        if ast::Ty::cast(node).is_some() {
            if binding {
                return queries::resolve_type_binding(db, ws, file, id.range().start_point, name);
            }
            let reference = queries::resolve_type(db, ws, file, id.range().start_point, name)?;
            return queries::type_definition(db, ws, reference);
        }
        if let Some(item) = ast::UseNamesItem::cast(node) {
            let owner = ancestors(node).find_map(ast::UseItem::cast)?;
            let source = Ident::from_ast(db, item.name()?, file.contents(db));
            if binding {
                if item.alias_opt() == Some(id) {
                    return Some(Location::new(file.path(db), id.range()));
                }
                return queries::resolve_imported_type_binding(db, ws, file, owner.path()?, source);
            }
            let reference = queries::resolve_imported_type(db, ws, file, owner.path()?, source)?;
            return queries::type_definition(db, ws, reference);
        }
        if let Some(path) = ast::UsePath::cast(node) {
            let target = queries::resolve_use_path(db, ws, file, path)?;
            let parent = node.parent()?;
            if ast::IncludeItem::cast(parent).is_some() {
                if !matches!(target, TopLevelItemMetadata::World(_)) {
                    return None;
                }
            } else if ast::ToplevelUseItem::cast(parent).is_none()
                && !matches!(target, TopLevelItemMetadata::Interface(_))
            {
                return None;
            }
            return if binding {
                queries::resolve_use_path_binding(db, ws, file, path)
            } else {
                Some(target.definition(db))
            };
        }
        if let Some(item) = ast::ToplevelUseItem::cast(node) {
            if binding && item.iter_alias().any(|alias| alias == id) {
                return Some(Location::new(file.path(db), id.range()));
            }
            return queries::resolve_use_path(db, ws, file, item.path()?)
                .map(|item| item.definition(db));
        }
        if let Some(item) = ast::IncludeNamesItem::cast(node) {
            if binding && item.alias() == Some(id) {
                return Some(Location::new(file.path(db), id.range()));
            }
            let owner = ancestors(node).find_map(ast::IncludeItem::cast)?;
            let TopLevelItemMetadata::World(world) =
                queries::resolve_use_path(db, ws, file, owner.use_path()?)?
            else {
                return None;
            };
            let source = Ident::from_ast(db, item.name()?, file.contents(db));
            return world
                .imports(db)
                .into_iter()
                .find(|item| item.name(db) == Some(source))
                .map(|item| item.definition(db))
                .or_else(|| {
                    world
                        .exports(db)
                        .into_iter()
                        .find(|item| item.name(db) == Some(source))
                        .map(|item| item.definition(db))
                });
        }
    }
    None
}

pub(super) fn symbol_definition(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    id: ast::Id<'_>,
) -> Option<Location> {
    definition_for_identifier(db, ws, file, id).or_else(|| declaration_for_identifier(db, file, id))
}

pub(super) fn declaration_for_identifier(
    db: &dyn Db,
    file: SourceFile,
    id: ast::Id<'_>,
) -> Option<Location> {
    let parent = id.syntax().parent()?;
    let field = match parent.kind() {
        "type_item" => "alias",
        "interface_item"
        | "world_item"
        | "record_item"
        | "resource_item"
        | "enum_items"
        | "flags_items"
        | "variant_items"
        | "func_item"
        | "static_resource_method"
        | "record_field"
        | "enum_case"
        | "flags_case"
        | "variant_case"
        | "named_type"
        | "import_item"
        | "export_item" => "name",
        _ => return None,
    };
    (parent.child_by_field_name(field) == Some(id.syntax()))
        .then(|| Location::new(file.path(db), parent.range()))
}

fn ancestors(node: tree_sitter::Node<'_>) -> impl Iterator<Item = tree_sitter::Node<'_>> {
    std::iter::successors(Some(node), |node| node.parent())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wit_compiler::{Compiler, queries::FilePath};

    fn marked_source(source: &str) -> (String, tree_sitter::Point) {
        let offset = source.find('|').unwrap();
        let prefix = &source[..offset];
        let point = tree_sitter::Point::new(
            prefix.bytes().filter(|b| *b == b'\n').count(),
            prefix.rsplit('\n').next().unwrap().len(),
        );
        (source.replacen('|', "", 1), point)
    }

    #[test]
    fn resolves_type_uses_to_their_declarations() {
        for (source, expected) in [
            (
                "interface i { record point { x: u32 } type points = list<|point>; }",
                "record point { x: u32 }",
            ),
            (
                "interface i { type points = list<|point>; record point { x: u32 } }",
                "record point { x: u32 }",
            ),
            (
                "world w { type number = u32; type numbers = list<|number>; }",
                "type number = u32;",
            ),
            (
                "interface i { resource file; read: func(f: borrow<|file>); }",
                "resource file;",
            ),
            (
                "interface i { resource file; read: func(f: own<|file>); }",
                "resource file;",
            ),
            (
                "interface i { type count = u32; record r { field: |count } }",
                "type count = u32;",
            ),
        ] {
            let db = Compiler::default();
            let (source, point) = marked_source(source);
            let path = FilePath::new(&db, "file:///types.wit".into());
            let file = SourceFile::new(&db, path, source.clone().into());
            let ws = Workspace::new(&db, [(path, file)].into_iter().collect());
            let location = goto_definition(&db, ws, file, point).unwrap();
            assert_eq!(location.filename, path);
            assert_eq!(
                &source[location.range.start_byte..location.range.end_byte],
                expected
            );
        }
    }

    #[test]
    fn follows_renamed_import_to_original_file() {
        let db = Compiler::default();
        let (source, point) = marked_source(
            "interface consumer { use types.{point as location}; type x = option<|location>; }",
        );
        let source_path = FilePath::new(&db, "file:///types.wit".into());
        let target_path = FilePath::new(&db, "file:///consumer.wit".into());
        let definitions = "interface types { record point { x: u32 } }";
        let source_file = SourceFile::new(&db, source_path, definitions.into());
        let target_file = SourceFile::new(&db, target_path, source.into());
        let ws = Workspace::new(
            &db,
            [(source_path, source_file), (target_path, target_file)]
                .into_iter()
                .collect(),
        );
        let location = goto_definition(&db, ws, target_file, point).unwrap();
        assert_eq!(location.filename, source_path);
        assert_eq!(
            &definitions[location.range.start_byte..location.range.end_byte],
            "record point { x: u32 }"
        );
    }

    fn import_target(source: &str) -> Option<(String, String)> {
        let db = Compiler::default();
        let (source, point) = marked_source(source);
        let fixtures = [
            (
                "file:///types.wit",
                "interface types { record point { x: u32 } type %type = u32; } interface bridge { use types.{point as other}; } world base { import log: func(); export run: func(); }",
            ),
            (
                "file:///dep/types.wit",
                "package example:remote@1.2.3; interface types { type point = string; } world base {}",
            ),
            ("file:///other.wit", "use types as hidden;"),
            ("file:///consumer.wit", source.as_str()),
        ];
        let ws = Workspace::new(
            &db,
            fixtures
                .into_iter()
                .map(|(path, source)| {
                    let path = FilePath::new(&db, path.into());
                    (path, SourceFile::new(&db, path, source.into()))
                })
                .collect(),
        );
        let file = ws.lookup_by_path(&db, "file:///consumer.wit").unwrap();
        let location = goto_definition(&db, ws, file, point)?;
        let path = location.filename.raw_path(&db).to_string();
        let target = ws.lookup_by_path(&db, &path).unwrap();
        Some((
            path,
            target.contents(&db)[location.range.start_byte..location.range.end_byte].to_owned(),
        ))
    }

    #[test]
    fn navigates_import_names_aliases_and_paths() {
        let interface = "interface types { record point { x: u32 } type %type = u32; }";
        for (source, expected) in [
            ("interface i { use |types.{point}; }", interface),
            (
                "interface i { use types.{|point as location}; }",
                "record point { x: u32 }",
            ),
            (
                "interface i { use types.{point as |location}; }",
                "record point { x: u32 }",
            ),
            ("world w { use types.{|point}; }", "record point { x: u32 }"),
            (
                "interface i { use types.{point as |local}; type local = bool; }",
                "record point { x: u32 }",
            ),
            ("use |types as local;", interface),
            ("use types as |local;", interface),
            (
                "use types as local; interface i { use |local.{point}; }",
                interface,
            ),
            (
                "use types as local; interface i { use local.{|point}; }",
                "record point { x: u32 }",
            ),
            ("world w { import |types; }", interface),
            ("world w { export |types; }", interface),
            (
                "world w { include |base; }",
                "world base { import log: func(); export run: func(); }",
            ),
            (
                "world w { include base with { |log as logger }; }",
                "import log: func();",
            ),
            (
                "world w { include base with { log as |logger }; }",
                "import log: func();",
            ),
            (
                "world w { include base with { run as |runner }; }",
                "export run: func();",
            ),
            (
                "interface i { use bridge.{|other as local}; }",
                "record point { x: u32 }",
            ),
            (
                "interface i { use bridge.{other as local}; type result-type = |local; }",
                "record point { x: u32 }",
            ),
            (
                "interface i { use types.{|%type as local}; }",
                "type %type = u32;",
            ),
            (
                "interface i { use types.{%type}; type result-type = |%type; }",
                "type %type = u32;",
            ),
        ] {
            assert_eq!(
                import_target(source),
                Some(("file:///types.wit".into(), expected.into())),
                "{source}"
            );
        }
    }

    #[test]
    fn navigates_qualified_paths_and_file_local_aliases() {
        for (source, expected) in [
            (
                "interface i { use example:remote/|types@1.2.3.{point}; }",
                "interface types { type point = string; }",
            ),
            (
                "interface i { use example:remote/types@1.2.3.{|point}; }",
                "type point = string;",
            ),
            (
                "use example:remote/types@1.2.3 as |remote-types;",
                "interface types { type point = string; }",
            ),
            (
                "use example:remote/types@1.2.3; interface i { use |types.{point}; }",
                "interface types { type point = string; }",
            ),
            (
                "use example:remote/types@1.2.3 as remote-types; interface i { use remote-types.{point as |local}; }",
                "type point = string;",
            ),
            (
                "use example:remote/types@1.2.3 as remote-types; interface i { use remote-types.{point}; type alias = |point; }",
                "type point = string;",
            ),
            (
                "world w { import example:remote/|types@1.2.3; }",
                "interface types { type point = string; }",
            ),
            (
                "world w { include example:remote/|base@1.2.3; }",
                "world base {}",
            ),
        ] {
            assert_eq!(
                import_target(source),
                Some(("file:///dep/types.wit".into(), expected.into())),
                "{source}"
            );
        }
    }

    #[test]
    fn invalid_imports_do_not_navigate_or_panic() {
        for source in [
            "interface i { use types.{|missing}; }",
            "interface i { use |missing.{point}; }",
            "interface i { use |hidden.{point}; }",
            "interface i { use types.{point |as local}; }",
            "interface i { use types.{point}|; }",
            "interface i { use example:remote/types@9.9.9.{|point}; }",
            "interface i { use example:missing/|types.{point}; }",
            "interface i { use |base.{point}; }",
            "world w { import |base; }",
            "world w { include |types; }",
            "world w { include base with { |missing as local }; }",
            "use second as first; use first as second; interface i { use |first.{point}; }",
            "interface first { use second.{point}; } interface second { use first.{point}; } interface i { use first.{|point}; }",
        ] {
            assert_eq!(import_target(source), None, "{source}");
        }
    }

    #[test]
    fn does_not_navigate_from_unrelated_identifiers_or_missing_types() {
        for source in [
            "interface i { type point = u32; record r { |point: u32 } }",
            "interface i { type point = u32; |point: func(); }",
            "interface i { type point = u32; type x = |list<point>; }",
            "interface i { type x = |missing; }",
            "interface i { type x = |u32; }",
            "interface i { /* |comment */ }",
            "interface i {}|",
        ] {
            let db = Compiler::default();
            let (source, point) = marked_source(source);
            let path = FilePath::new(&db, "file:///types.wit".into());
            let file = SourceFile::new(&db, path, source.into());
            let ws = Workspace::new(&db, [(path, file)].into_iter().collect());
            assert!(goto_definition(&db, ws, file, point).is_none());
        }
    }
}
