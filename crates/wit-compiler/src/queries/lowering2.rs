use crate::{
    hir,
    queries::{Package, Workspace},
    Db,
};

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub fn lower_package(db: &dyn Db, _ws: Workspace, pkg: Package) -> hir::Package {
    let _items = crate::queries::package_items(db, pkg);

    hir::Package {
        ..Default::default()
    }
}
