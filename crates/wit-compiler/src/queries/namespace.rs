use im::{OrdMap, Vector};

use crate::{
    access::{InterfaceIndex, ScopeIndex},
    ast::{self, AstNode, HasSource},
    diagnostics::{Diagnostic, Diagnostics, DuplicateName, Location, UnknownName, UnknownPackage},
    hir,
    queries::{
        metadata::{
            GetByIndex, HasDefinition, HasIdent, Ident, PackageMetadata, TopLevelItemMetadata,
            TypeDefinitionMetadata,
        },
        FilePath, Package, PackageId, SourceFile, Workspace,
    },
    Db,
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
    // Restrict the search to this file: positions alone are not unique across
    // the files in a package.
    let enclosing = crate::queries::file_items(db, file)
        .into_iter()
        .find(|item| item.definition(db).contains(point))?;
    let scope = enclosing.index(db);
    let definitions = match enclosing {
        TopLevelItemMetadata::Interface(meta) => meta
            .items(db)
            .into_iter()
            .filter_map(|item| item.as_type())
            .collect(),
        TopLevelItemMetadata::World(meta) => meta.definitions(db),
    };

    if let Some(meta) = definitions.into_iter().find(|meta| meta.ident(db) == name) {
        return Some(type_reference(db, scope, meta));
    }

    imported_types(db, ws, scope).get(&name).copied()
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

/// Determine the items that have been imported inside a particular world or
/// interface.
#[salsa::tracked]
pub fn imported_types(
    db: &dyn Db,
    ws: Workspace,
    scope: ScopeIndex,
) -> OrdMap<Ident, hir::ItemReference> {
    let packages = crate::queries::workspace_packages(db, ws);
    let path = scope.file();

    let pkg = packages
        .iter()
        .copied()
        .find(|pkg| pkg.contains(db, path))
        .unwrap();
    let meta = crate::queries::package_items(db, pkg);

    let file = ws.file(db, path);
    let ast = crate::queries::parse(db, file);

    let use_statements: Vec<_> = match scope {
        ScopeIndex::World(w) => {
            let ptr = meta.get_by_index(db, w).ptr(db);
            let node = ast.get(db, ptr);
            node.body()
                .into_iter()
                .flat_map(|body| body.iter_world_items())
                .filter_map(|item| item.use_item_opt())
                .collect()
        }
        ScopeIndex::Interface(i) => {
            let ptr = meta.get_by_index(db, i).ptr(db);
            let node = ast.get(db, ptr);
            node.body()
                .into_iter()
                .flat_map(|body| body.iter_interface_items())
                .filter_map(|item| item.use_opt())
                .collect()
        }
    };

    let mut ctx = Context {
        db,
        packages,
        path,
        meta,
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
    packages: Vector<Package>,
    meta: PackageMetadata,
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
        let (meta, item_name_node) = if let Some(bare_ident) = path.id() {
            // TODO: Also check for top-level `use` statements that refer to
            // this package.
            let raw = bare_ident.utf8_text(self.src);
            let ident = Ident::new(self.db, raw.into());
            let item = self.meta.items_by_name(self.db).get(&ident).copied();

            (item, bare_ident)
        } else if let Some(fully_qualified) = path.fully_qualified_use_path() {
            let pkg_id = self.resolve_package_id(fully_qualified)?;
            let pkg = self.find_pkg(pkg_id, fully_qualified)?;

            let item_name = fully_qualified.iter_ids().last()?;
            let raw = item_name.utf8_text(self.src);
            let ident = Ident::new(self.db, raw.into());

            let meta = crate::queries::package_items(self.db, pkg);
            let meta = meta.items_by_name(self.db).get(&ident).copied();

            (meta, item_name)
        } else {
            return None;
        };

        match meta {
            Some(TopLevelItemMetadata::Interface(i)) => Some(i.index(self.db)),
            Some(TopLevelItemMetadata::World(_)) | None => {
                self.emit(crate::diagnostics::UnknownName {
                    name: item_name_node.utf8_text(self.src).into(),
                    location: Location::new(self.path, item_name_node.range()),
                });
                None
            }
        }
    }

    fn resolve_package_id(
        &self,
        package_name: ast::FullyQualifiedUsePath<'_>,
    ) -> Option<PackageId> {
        let raw = package_name.utf8_text(self.src);

        self.packages.iter().find_map(|pkg| {
            let pkg_id = pkg.id(self.db)?;
            if pkg_id.raw(self.db) == raw {
                Some(pkg_id)
            } else {
                None
            }
        })
    }

    fn find_pkg(&self, pkg_id: PackageId, node: ast::FullyQualifiedUsePath<'_>) -> Option<Package> {
        let pkg = self
            .packages
            .iter()
            .copied()
            .find(|pkg| pkg.id(self.db) == Some(pkg_id));

        if pkg.is_none() {
            self.emit(UnknownPackage {
                package_id: pkg_id.raw(self.db).clone(),
                location: Location::new(self.path, node.range()),
            });
        };

        pkg
    }

    fn emit(&self, diag: impl Into<Diagnostic>) {
        Diagnostics::push(self.db, diag.into());
    }

    fn process_use_name(
        &self,
        index: InterfaceIndex,
        use_name: ast::UseNamesItem<'_>,
    ) -> Option<(Ident, hir::ItemReference, Location)> {
        let path = index.file();
        let packages = crate::queries::workspace_packages(self.db, self.ws);
        let pkg = packages
            .iter()
            .copied()
            .find(|pkg| pkg.contains(self.db, path))
            .unwrap();

        let meta = crate::queries::package_items(self.db, pkg);
        let interface_meta = meta.get_by_index(self.db, index);

        let source_node = use_name.name()?;
        let source_name = source_node.utf8_text(self.src);
        let source_ident = Ident::new(self.db, source_name.into());
        let ident_node = use_name.alias_opt().unwrap_or(source_node);
        let ident = Ident::new(self.db, ident_node.utf8_text(self.src).into());

        let Some(item_meta) = interface_meta
            .items(self.db)
            .into_iter()
            .filter_map(|item| item.as_type())
            .find(|item| item.ident(self.db) == source_ident)
        else {
            self.emit(UnknownName {
                name: source_name.into(),
                location: Location::new(self.path, source_node.range()),
            });
            return None;
        };

        let reference = type_reference(self.db, index.into(), item_meta);
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
        diagnostics,
        queries::{metadata::InterfaceItemMetadata, FilePath},
        Compiler,
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
            let ws = workspace(&db, &[
                ("source.wit", declaration),
                ("consumer.wit", "interface consumer { use source.{value as renamed}; type alias = renamed; }"),
            ]);
            let diags = diagnostics::check_all(&db, ws);
            assert!(!diags.is_empty(), "{declaration}");
            for diag in diags {
                assert!(matches!(diag, Diagnostic::UnknownName(_)), "{diag:?}");
                assert_eq!(diag.location().filename.raw_path(&db), "consumer.wit");
            }
        }
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
