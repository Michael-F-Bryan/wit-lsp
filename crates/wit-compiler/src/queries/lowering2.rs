use crate::{
    hir,
    queries::{Package, Workspace},
    Db,
};

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub fn lower_package(db: &dyn Db, ws: Workspace, pkg: Package) -> hir::Package {
    let _items = crate::queries::package_items(db, pkg);

    let mut lowered = hir::Package::default();

    for file in pkg.files(db) {
        let package = crate::queries::lower(db, ws, file);
        merge_packages(&mut lowered, package);
    }

    lowered
}

fn merge_packages(lowered: &mut hir::Package, package: hir::Package) {
    let hir::Package {
        decl,
        worlds,
        interfaces,
    } = package;

    if lowered.decl.is_none() {
        lowered.decl = decl;
    }

    // Note: this is probably incorrect...
    lowered.worlds.extend(worlds);
    lowered.interfaces.extend(interfaces);
}
