use im::{OrdMap, Vector};

use crate::{
    access::{InterfaceIndex, ScopeIndex},
    ast::{self, AstNode, HasSource},
    diagnostics::{Diagnostic, Diagnostics, DuplicateName, Location, UnknownName, UnknownPackage},
    hir,
    queries::{
        metadata::{
            GetByIndex, HasIdent, Ident, InterfaceItemMetadata, PackageMetadata,
            TopLevelItemMetadata, TypeDefinitionMetadata,
        },
        FilePath, Package, PackageId, SourceFile, Workspace,
    },
    Db,
};

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
            Some(TopLevelItemMetadata::World(_)) => todo!(),
            None => {
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

        let ident_node = use_name.alias_opt().or_else(|| use_name.name())?;
        let name = ident_node.utf8_text(self.src);
        let ident = Ident::new(self.db, name.into());

        let Some(item_meta) = interface_meta
            .items(self.db)
            .into_iter()
            .find(|item| item.ident(self.db) == ident)
        else {
            self.emit(UnknownName {
                name: name.into(),
                location: Location::new(path, ident_node.range()),
            });
            return None;
        };

        let kind: hir::ItemReferenceKind = match item_meta {
            InterfaceItemMetadata::Func(_) => todo!("Can you import a function?"),
            InterfaceItemMetadata::Type(TypeDefinitionMetadata::Enum(meta)) => {
                meta.index(self.db).into()
            }
            InterfaceItemMetadata::Type(TypeDefinitionMetadata::Record(meta)) => {
                meta.index(self.db).into()
            }
            InterfaceItemMetadata::Type(TypeDefinitionMetadata::Resource(meta)) => {
                meta.index(self.db).into()
            }
            InterfaceItemMetadata::Type(TypeDefinitionMetadata::Variant(meta)) => {
                meta.index(self.db).into()
            }
            InterfaceItemMetadata::Type(TypeDefinitionMetadata::Flags(meta)) => {
                meta.index(self.db).into()
            }
            InterfaceItemMetadata::Type(TypeDefinitionMetadata::Alias(meta)) => {
                meta.index(self.db).into()
            }
        };

        let reference = hir::ItemReference {
            file: path,
            scope: index.into(),
            kind,
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

    use crate::{queries::FilePath, Compiler};
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
}
