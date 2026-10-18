use std::collections::HashSet;

use crate::{
    Db,
    ast::{self, AstNode, HasSource},
    diagnostics::Location,
    queries::{
        SourceFile, Workspace,
        metadata::{HasDefinition, Ident, TopLevelItemMetadata},
    },
};

/// Resolve an interface or world path, including file-local aliases.
/// Alias traversal stays within the referring file and terminates on cycles.
pub fn resolve_use_path<'db>(
    db: &'db dyn Db,
    ws: Workspace,
    file: SourceFile,
    path: ast::UsePath<'db>,
) -> Option<TopLevelItemMetadata> {
    resolve_path(db, ws, file, path).map(|(target, _)| target)
}

/// Keep the nearest explicit file-local alias as a distinct rename binding.
pub fn resolve_use_path_binding(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    path: ast::UsePath<'_>,
) -> Option<Location> {
    resolve_path(db, ws, file, path).map(|(_, binding)| binding)
}

fn resolve_path<'db>(
    db: &'db dyn Db,
    ws: Workspace,
    file: SourceFile,
    mut path: ast::UsePath<'db>,
) -> Option<(TopLevelItemMetadata, Location)> {
    let mut binding = None;
    let packages = super::workspace_packages(db, ws);
    let own_package = packages
        .iter()
        .copied()
        .find(|p| p.contains(db, file.path(db)))?;
    let ast = super::parse(db, file);
    let src = file.contents(db);
    let mut visited = HashSet::new();

    loop {
        if !visited.insert(path.range().start_byte) {
            return None;
        }
        if let Some(id) = path.id() {
            let name = Ident::from_ast(db, id, src);
            let alias = ast
                .source_file(db)
                .iter_top_level_items()
                .filter_map(|item| item.toplevel_use_item())
                .find_map(|item| {
                    let target = item.path()?;
                    let local = item.iter_alias().next().or_else(|| path_name(target))?;
                    (Ident::from_ast(db, local, src) == name)
                        .then_some((target, item.iter_alias().next()))
                });
            if let Some((target, alias)) = alias {
                if binding.is_none() {
                    binding = alias.map(|id| Location::new(file.path(db), id.range()));
                }
                path = target;
                continue;
            }
            return super::package_items(db, own_package)
                .items_by_name(db)
                .get(&name)
                .copied()
                .map(|target| (target, binding.unwrap_or_else(|| target.definition(db))));
        }

        let qualified = path.fully_qualified_use_path()?;
        let mut ids = qualified.iter_ids();
        let namespace = Ident::from_ast(db, ids.next()?, src);
        let package = Ident::from_ast(db, ids.next()?, src);
        let item = Ident::from_ast(db, ids.next()?, src);
        let mut package_id = format!("{}:{}", namespace.raw(db), package.raw(db));
        if let Some(version) = qualified.iter_valid_semvers().next() {
            package_id.push('@');
            package_id.push_str(version.utf8_text(src));
        }
        let package = packages
            .iter()
            .copied()
            .find(|p| p.id(db).is_some_and(|id| id.raw(db).as_str() == package_id))?;
        return super::package_items(db, package)
            .items_by_name(db)
            .get(&item)
            .copied()
            .map(|target| (target, binding.unwrap_or_else(|| target.definition(db))));
    }
}

fn path_name(path: ast::UsePath<'_>) -> Option<ast::Id<'_>> {
    path.id()
        .or_else(|| path.fully_qualified_use_path()?.iter_ids().last())
}
