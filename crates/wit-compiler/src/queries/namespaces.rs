use im::OrdMap;

use crate::{
    access::{GetAstNode, GetByIndex, ScopeIndex},
    hir,
    queries::SourceFile,
    Db, Text,
};

#[salsa::tracked]
pub fn resolve_name(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    name: Text,
) -> Option<hir::ItemReference> {
    resolve_namespace(db, file, scope).lookup(&name)
}

#[salsa::tracked]
pub fn resolve_namespace(db: &dyn Db, file: SourceFile, scope: ScopeIndex) -> Namespace {
    let defined_types = match scope {
        ScopeIndex::World(index) => file.get_by_index(db, index).items(db),
        ScopeIndex::Interface(index) => file.get_by_index(db, index).items(db),
    };

    let mut names = OrdMap::new();

    for (name, item) in defined_types.reference_kinds() {
        let reference = hir::ItemReference {
            file: file.path(db),
            scope,
            item,
        };
        names.insert(name.clone(), reference);
    }

    let ast = crate::queries::parse(db, file);
    let tree = ast.tree(db);

    let imports: Vec<_> = match scope {
        ScopeIndex::World(index) => file
            .get_by_index(db, index)
            .location(db)
            .ast_node(tree)
            .iter_items()
            .filter_map(|n| n.use_item())
            .collect(),
        ScopeIndex::Interface(index) => file
            .get_by_index(db, index)
            .location(db)
            .ast_node(tree)
            .iter_items()
            .filter_map(|n| n.use_item())
            .collect(),
    };
    if !imports.is_empty() {
        todo!("Resolve imports from other files");
    }

    Namespace(names)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namespace(pub OrdMap<Text, hir::ItemReference>);

impl Namespace {
    pub fn lookup(&self, name: &str) -> Option<hir::ItemReference> {
        self.0.get(name).cloned()
    }
}
