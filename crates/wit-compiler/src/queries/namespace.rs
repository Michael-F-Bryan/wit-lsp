use im::{OrdMap, Vector};

use crate::{
    Db,
    access::{InterfaceIndex, ScopeIndex},
    ast::{self, AstNode, HasSource},
    diagnostics::{Diagnostic, Diagnostics, DuplicateName, Location, UnknownName},
    hir,
    queries::{
        FilePath, SourceFile, Workspace,
        metadata::{HasDefinition, HasIdent, Ident, TopLevelItemMetadata, TypeDefinitionMetadata},
    },
};

/// Resolve a type name in the interface or world containing its use site.
/// References retain the declaration's identity rather than expanding its type.
pub fn resolve_type(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
    name: Ident,
) -> Option<hir::ItemReference> {
    visible_types(db, ws, file, point).get(&name).copied()
}

/// Find the enclosing interface or world in this file.
pub fn enclosing_scope(
    db: &dyn Db,
    file: SourceFile,
    point: tree_sitter::Point,
) -> Option<TopLevelItemMetadata> {
    crate::queries::file_items(db, file)
        .into_iter()
        .find(|item| item.definition(db).contains(point))
}

/// Type names visible at a position, including local names for imported types.
pub fn visible_types(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
) -> OrdMap<Ident, hir::ItemReference> {
    let Some(enclosing) = enclosing_scope(db, file, point) else {
        return OrdMap::new();
    };
    let scope = enclosing.index(db);
    let mut types = imported_types(db, ws, scope);
    for meta in scope_definitions(db, enclosing) {
        types.insert(meta.ident(db), type_reference(db, scope, meta));
    }
    types
}

/// Locate the declaration identified by a resolved type reference.
pub fn type_definition(
    db: &dyn Db,
    ws: Workspace,
    reference: hir::ItemReference,
) -> Option<Location> {
    let file = ws.file(db, reference.file);
    let scope = crate::queries::file_items(db, file)
        .into_iter()
        .find(|item| item.index(db) == reference.scope)?;
    scope_definitions(db, scope)
        .into_iter()
        .find(|meta| type_reference(db, reference.scope, *meta) == reference)
        .map(|meta| meta.definition(db))
}

fn scope_definitions(db: &dyn Db, scope: TopLevelItemMetadata) -> Vector<TypeDefinitionMetadata> {
    match scope {
        TopLevelItemMetadata::Interface(meta) => meta
            .items(db)
            .into_iter()
            .filter_map(|item| item.as_type())
            .collect(),
        TopLevelItemMetadata::World(meta) => meta.definitions(db),
    }
}

fn type_reference(
    db: &dyn Db,
    scope: ScopeIndex,
    meta: TypeDefinitionMetadata,
) -> hir::ItemReference {
    let kind = match meta {
        TypeDefinitionMetadata::Enum(meta) => meta.index(db).into(),
        TypeDefinitionMetadata::Record(meta) => meta.index(db).into(),
        TypeDefinitionMetadata::Resource(meta) => meta.index(db).into(),
        TypeDefinitionMetadata::Variant(meta) => meta.index(db).into(),
        TypeDefinitionMetadata::Flags(meta) => meta.index(db).into(),
        TypeDefinitionMetadata::Alias(meta) => meta.index(db).into(),
    };
    hir::ItemReference {
        file: scope.file(),
        scope,
        kind,
    }
}

/// Resolve the source name in a `use path.{name as alias}` declaration.
/// The declaration's path is authoritative even if a local name shadows its alias.
pub fn resolve_imported_type(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    path: ast::UsePath<'_>,
    name: Ident,
) -> Option<hir::ItemReference> {
    let TopLevelItemMetadata::Interface(meta) = super::resolve_use_path(db, ws, file, path)? else {
        return None;
    };
    interface_type(db, ws, meta.index(db), name)
}

/// Resolve the name that a rename should change, retaining explicit import aliases.
pub fn resolve_type_binding(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
    name: Ident,
) -> Option<Location> {
    let scope = enclosing_scope(db, file, point)?;
    type_resolution(db, ws, scope.index(db), name).map(|(_, binding)| binding)
}

/// Resolve the source side of an import without crossing explicit alias bindings.
pub fn resolve_imported_type_binding(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    path: ast::UsePath<'_>,
    name: Ident,
) -> Option<Location> {
    let TopLevelItemMetadata::Interface(meta) = super::resolve_use_path(db, ws, file, path)? else {
        return None;
    };
    type_resolution(db, ws, meta.index(db).into(), name).map(|(_, binding)| binding)
}

fn interface_type(
    db: &dyn Db,
    ws: Workspace,
    index: InterfaceIndex,
    name: Ident,
) -> Option<hir::ItemReference> {
    type_resolution(db, ws, index.into(), name).map(|(reference, _)| reference)
}

fn type_resolution(
    db: &dyn Db,
    ws: Workspace,
    mut index: ScopeIndex,
    mut name: Ident,
) -> Option<(hir::ItemReference, Location)> {
    let mut binding = None;
    let mut visited = std::collections::HashSet::new();
    loop {
        if !visited.insert((index, name)) {
            return None;
        }
        let file = ws.file(db, index.file());
        let scope = super::file_items(db, file)
            .into_iter()
            .find(|item| item.index(db) == index)?;
        if let Some(item) = scope_definitions(db, scope)
            .into_iter()
            .find(|item| item.ident(db) == name)
        {
            return Some((
                type_reference(db, index, item),
                binding.unwrap_or_else(|| item.definition(db)),
            ));
        }
        let src = file.contents(db);
        let (path, source, alias) = scope_uses(db, ws, index).into_iter().find_map(|item| {
            item.names()?.iter_use_names_items().find_map(|entry| {
                let source = entry.name()?;
                let local = entry.alias_opt().unwrap_or(source);
                (Ident::from_ast(db, local, src) == name).then(|| {
                    (
                        item.path(),
                        Ident::from_ast(db, source, src),
                        entry.alias_opt(),
                    )
                })
            })
        })?;
        if binding.is_none() {
            binding = alias.map(|id| Location::new(file.path(db), id.range()));
        }
        let TopLevelItemMetadata::Interface(target) = super::resolve_use_path(db, ws, file, path?)?
        else {
            return None;
        };
        index = target.index(db).into();
        name = source;
    }
}

fn scope_uses(db: &dyn Db, ws: Workspace, scope: ScopeIndex) -> Vec<ast::UseItem<'_>> {
    let file = ws.file(db, scope.file());
    let ast = crate::queries::parse(db, file);

    match crate::queries::file_items(db, file)
        .into_iter()
        .find(|item| item.index(db) == scope)
    {
        Some(TopLevelItemMetadata::World(w)) => {
            let ptr = w.ptr(db);
            let node = ast.get(db, ptr);
            node.body()
                .into_iter()
                .flat_map(|body| body.iter_world_items())
                .filter_map(|item| item.use_item_opt())
                .collect()
        }
        Some(TopLevelItemMetadata::Interface(i)) => {
            let ptr = i.ptr(db);
            let node = ast.get(db, ptr);
            node.body()
                .into_iter()
                .flat_map(|body| body.iter_interface_items())
                .filter_map(|item| item.use_opt())
                .collect()
        }
        None => Vec::new(),
    }
}

/// Determine the items that have been imported inside a particular world or
/// interface.
#[salsa::tracked]
pub fn imported_types(
    db: &dyn Db,
    ws: Workspace,
    scope: ScopeIndex,
) -> OrdMap<Ident, hir::ItemReference> {
    let path = scope.file();
    let file = ws.file(db, path);
    let use_statements = scope_uses(db, ws, scope);

    let mut ctx = Context {
        db,
        path,
        ws,
        src: file.contents(db),
        imports: OrdMap::new(),
    };

    for stmt in use_statements {
        ctx.process(stmt);
    }

    ctx.finish()
}

/// Find all the names accessible at the top level of a file.
pub fn top_level_names(
    _db: &dyn Db,
    _ws: Workspace,
    _file: SourceFile,
) -> OrdMap<Ident, hir::ItemReference> {
    todo!();
}

struct Context<'db> {
    db: &'db dyn Db,
    src: &'db str,
    path: FilePath,
    imports: OrdMap<Ident, (hir::ItemReference, Location)>,
    ws: Workspace,
}

impl Context<'_> {
    fn finish(self) -> OrdMap<Ident, hir::ItemReference> {
        self.imports.into_iter().map(|(k, (v, _))| (k, v)).collect()
    }

    fn process(&mut self, node: ast::UseItem<'_>) {
        self._process(node);
    }

    fn _process(&mut self, node: ast::UseItem<'_>) -> Option<()> {
        let path = node.path()?;
        let interface = self.resolve_path(path)?;

        for use_name in node.names()?.iter_use_names_items() {
            if let Some((name, item, node)) = self.process_use_name(interface, use_name) {
                self.insert(name, item, node);
            }
        }

        Some(())
    }

    fn resolve_path(&self, path: ast::UsePath<'_>) -> Option<InterfaceIndex> {
        let file = self.ws.file(self.db, self.path);
        if let Some(TopLevelItemMetadata::Interface(meta)) =
            crate::queries::resolve_use_path(self.db, self.ws, file, path)
        {
            return Some(meta.index(self.db));
        }
        let name = path
            .id()
            .or_else(|| path.fully_qualified_use_path()?.iter_ids().last())?;
        self.emit(UnknownName {
            name: name.utf8_text(self.src).into(),
            location: Location::new(self.path, name.range()),
        });
        None
    }

    fn emit(&self, diag: impl Into<Diagnostic>) {
        Diagnostics::push(self.db, diag.into());
    }

    fn process_use_name(
        &self,
        index: InterfaceIndex,
        use_name: ast::UseNamesItem<'_>,
    ) -> Option<(Ident, hir::ItemReference, Location)> {
        let source_node = use_name.name()?;
        let source_name = source_node.utf8_text(self.src);
        let source_ident = Ident::from_ast(self.db, source_node, self.src);
        let ident_node = use_name.alias_opt().unwrap_or(source_node);
        let ident = Ident::from_ast(self.db, ident_node, self.src);

        let Some(reference) = interface_type(self.db, self.ws, index, source_ident) else {
            self.emit(UnknownName {
                name: source_name.into(),
                location: Location::new(self.path, source_node.range()),
            });
            return None;
        };

        let location = Location::new(self.path, ident_node.range());

        Some((ident, reference, location))
    }

    fn insert(&mut self, name: Ident, item: hir::ItemReference, location: Location) {
        match self.imports.entry(name) {
            im::ordmap::Entry::Occupied(entry) => {
                let original_definition = entry.get().1;
                self.emit(DuplicateName {
                    name: name.raw(self.db).clone(),
                    location,
                    original_definition,
                });
            }
            im::ordmap::Entry::Vacant(entry) => {
                entry.insert((item, location));
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        Compiler, diagnostics,
        queries::{FilePath, metadata::InterfaceItemMetadata},
    };
    use im::OrdMap;

    use super::*;

    macro_rules! workspace {
        ($db:ident, { $( $path:expr => $contents:expr),* $(,)? }) => {{
            let mut files = OrdMap::new();
            $(
                let path = FilePath::new(&$db, $path.into());
                let f = SourceFile::new(&$db, path, $contents.into());
                files.insert(path, f);
            )*
            Workspace::new(&$db, files)
        }};
    }

    #[test]
    fn imports_in_different_contexts() {
        let db = Compiler::default();
        let ws = workspace!(db, {
            "empty.wit" => "interface empty { }",
            "use-empty.wit" => "interface use-empty { use empty.{}; }",
            "use-local.wit" => r#"
                interface local {
                    type x = u32;
                }
                interface use-local {
                    use local.{x};
                }
            "#,
            "world-using-local.wit" => r#"
                interface local2 {
                    type x = u32;
                }
                world world-using-local {
                    use local2.{x};
                }
            "#,
            "use-across-files.wit" => r#"
                interface use-across-file {
                    use local.{x};
                }
            "#,
        });

        let all_imports: OrdMap<String, OrdMap<&str, hir::ItemReference>> =
            crate::queries::workspace_packages(&db, ws)
                .into_iter()
                .flat_map(|pkg| crate::queries::package_items(&db, pkg).items_by_name(&db))
                .map(|(ident, meta)| {
                    let ident = ident.raw(&db).to_string();
                    let scope = meta.index(&db);
                    let imports: OrdMap<&str, hir::ItemReference> = imported_types(&db, ws, scope)
                        .into_iter()
                        .map(|(k, v)| (k.raw(&db).as_str(), v))
                        .collect();
                    (ident, imports)
                })
                .collect();

        insta::assert_debug_snapshot!(all_imports);
    }

    fn workspace(db: &Compiler, files: &[(&str, &str)]) -> Workspace {
        Workspace::new(
            db,
            files
                .iter()
                .map(|(path, text)| {
                    let path = FilePath::new(db, (*path).into());
                    (path, SourceFile::new(db, path, (*text).into()))
                })
                .collect(),
        )
    }

    #[test]
    fn lowers_local_types_and_forward_references() {
        let db = Compiler::default();
        let ws = workspace(
            &db,
            &[(
                "local.wit",
                r#"
            interface example {
                record span { start: position, end: option<position> }
                record position { line: u32, column: u32 }
                enum color { red, blue }
                flags permissions { read, write }
                variant value { point(position), color(color) }
                resource file {
                    read: func(at: position) -> value;
                }
                type points = list<position>;
                inspect: func(p: points, c: color, f: borrow<file>, flags: permissions)
                    -> result<tuple<span, value>, string>;
            }
            world example-world {
                type count = u32;
                type counts = list<count>;
            }
        "#,
            )],
        );
        assert!(diagnostics::check_all(&db, ws).is_empty());

        let file = ws.lookup_by_path(&db, "local.wit").unwrap();
        let meta = crate::queries::file_items(&db, file)
            .into_iter()
            .find_map(|item| match item {
                TopLevelItemMetadata::Interface(meta) => Some(meta),
                _ => None,
            })
            .unwrap();
        let pkg = crate::queries::workspace_packages(&db, ws)[0];
        let lowered = crate::queries::lower_package(&db, ws, pkg);
        let span = lowered.interfaces[&meta.index(&db)]
            .items
            .iter()
            .find_map(|item| match item {
                hir::InterfaceItem::Type(hir::TypeDefinition::Record(record))
                    if record.name.raw(&db) == "span" =>
                {
                    Some(record)
                }
                _ => None,
            })
            .unwrap();
        // A successful lookup must produce a real declaration reference, not
        // merely suppress the diagnostic and leave an error type behind.
        let position = meta
            .items(&db)
            .into_iter()
            .find_map(|item| match item {
                InterfaceItemMetadata::Type(TypeDefinitionMetadata::Record(meta))
                    if meta.name(&db).raw(&db) == "position" =>
                {
                    Some(meta)
                }
                _ => None,
            })
            .unwrap();
        let expected = hir::Type::UserDefinedType(type_reference(
            &db,
            meta.index(&db).into(),
            TypeDefinitionMetadata::Record(position),
        ));
        assert_eq!(span.fields[0].ty, expected);
    }

    #[test]
    fn resolves_renamed_imports_across_files() {
        let db = Compiler::default();
        let ws = workspace(
            &db,
            &[
                (
                    "types.wit",
                    "interface types { record position { line: u32 } }",
                ),
                (
                    "consumer.wit",
                    r#"
                interface consumer {
                    use types.{position as location};
                    record span { start: location, end: option<location> }
                }
                world consumer-world {
                    use types.{position};
                    type positions = list<position>;
                }
            "#,
                ),
            ],
        );
        assert!(diagnostics::check_all(&db, ws).is_empty());
        let file = ws.lookup_by_path(&db, "consumer.wit").unwrap();
        let reference = resolve_type(
            &db,
            ws,
            file,
            tree_sitter::Point::new(3, 40),
            Ident::new(&db, "location".into()),
        )
        .unwrap();
        assert_eq!(reference.file.raw_path(&db), "types.wit");
        assert!(matches!(reference.kind, hir::ItemReferenceKind::Record(_)));
    }

    #[test]
    fn does_not_resolve_types_from_other_scopes_or_functions() {
        let db = Compiler::default();
        let ws = workspace(
            &db,
            &[
                ("a.wit", "interface first { type hidden = u32; }"),
                ("b.wit", "interface second { type invalid = hidden; }"),
                (
                    "c.wit",
                    "interface third { function: func(); type invalid = function; }",
                ),
            ],
        );
        let diags = diagnostics::check_all(&db, ws);
        let mut unknown = diags
            .iter()
            .filter_map(|diag| match diag {
                Diagnostic::UnknownName(diag) => Some((
                    diag.location.filename.raw_path(&db).as_str(),
                    diag.name.as_str(),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        unknown.sort();
        assert_eq!(unknown, [("b.wit", "hidden"), ("c.wit", "function")]);
    }

    #[test]
    fn invalid_imports_report_errors_without_panicking() {
        for declaration in [
            "world source {}",
            "interface source { value: func(); }",
            "interface source { type other = u32; }",
        ] {
            let db = Compiler::default();
            let ws = workspace(
                &db,
                &[
                    ("source.wit", declaration),
                    (
                        "consumer.wit",
                        "interface consumer { use source.{value as renamed}; type alias = renamed; }",
                    ),
                ],
            );
            let diags = diagnostics::check_all(&db, ws);
            assert!(!diags.is_empty(), "{declaration}");
            for diag in diags {
                assert!(matches!(diag, Diagnostic::UnknownName(_)), "{diag:?}");
                assert_eq!(diag.location().filename.raw_path(&db), "consumer.wit");
            }
        }
    }

    #[test]
    fn qualified_and_transitive_imports_share_type_resolution() {
        let db = Compiler::default();
        let ws = workspace(
            &db,
            &[
                (
                    "dep/types.wit",
                    "package example:remote@1.2.3; interface types { type %type = u32; }",
                ),
                (
                    "bridge/bridge.wit",
                    "package example:bridge; use example:remote/types@1.2.3 as remote-types; interface bridge { use remote-types.{%type as value}; }",
                ),
                (
                    "app/main.wit",
                    "use example:bridge/bridge; interface consumer { use bridge.{value as local}; type alias = local; }",
                ),
            ],
        );
        assert!(diagnostics::check_all(&db, ws).is_empty());
        let file = ws.lookup_by_path(&db, "app/main.wit").unwrap();
        let point = tree_sitter::Point::new(0, file.contents(&db).find("type alias").unwrap());
        let reference =
            resolve_type(&db, ws, file, point, Ident::new(&db, "local".into())).unwrap();
        assert_eq!(reference.file.raw_path(&db), "dep/types.wit");
        let location = type_definition(&db, ws, reference).unwrap();
        let source = ws.file(&db, location.filename).contents(&db);
        assert_eq!(
            &source[location.range.start_byte..location.range.end_byte],
            "type %type = u32;"
        );
    }

    #[test]
    fn cyclic_imports_report_errors_without_recursing() {
        let db = Compiler::default();
        let ws = workspace(
            &db,
            &[(
                "cycles.wit",
                "use second as first; use first as second; interface a { use first.{missing}; use b.{value}; type x = value; } interface b { use a.{value}; type x = value; }",
            )],
        );
        let diags = diagnostics::check_all(&db, ws);
        assert!(!diags.is_empty());
        assert!(
            diags
                .iter()
                .all(|diag| matches!(diag, Diagnostic::UnknownName(_)))
        );
    }

    #[test]
    fn updates_resolution_after_editing_a_declaration() {
        let mut db = Compiler::default();
        let ws = workspace(
            &db,
            &[(
                "edit.wit",
                "interface example { type value = u32; type alias = value; }",
            )],
        );
        assert!(diagnostics::check_all(&db, ws).is_empty());
        ws.update(
            &mut db,
            "edit.wit",
            "interface example { type renamed = u32; type alias = value; }",
        );
        let diags = diagnostics::check_all(&db, ws);
        assert!(matches!(&diags[..], [Diagnostic::UnknownName(diag)] if diag.name == "value"));
        ws.update(
            &mut db,
            "edit.wit",
            "interface example { type value = u32; type alias = value; }",
        );
        assert!(diagnostics::check_all(&db, ws).is_empty());
    }
}
