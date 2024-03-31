use im::{OrdMap, Vector};
use tree_sitter::Range;

use crate::{
    ast::{self, AstNode},
    diagnostics::{Diagnostic, Diagnostics, Location},
    hir,
    queries::{SourceFile, Workspace},
    Db, Text,
};

/// Parse a file and lower it from its [`crate::ast`] representation to the
/// [`crate::hir`] representation.
///
/// Nodes that contain syntactic or semantic errors will be ignored and a
/// corresponding [`Diagnostic`] will be emitted.
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(file = %file.path(db)))]
pub fn lower(db: &dyn Db, _ws: Workspace, file: SourceFile) -> Items {
    let ast = crate::queries::parse(db, file);
    let src = file.contents(db);
    let root = ast.source_file(db);

    let mut named_items: OrdMap<&str, NamedItem<'_>> = OrdMap::new();

    for item in root.iter_top_level_items().flat_map(named_item) {
        let ident = item.ident();
        let name = ident.syntax().utf8_text(src.as_bytes()).unwrap();

        match named_items.entry(name) {
            im::ordmap::Entry::Vacant(entry) => {
                entry.insert(item);
            }
            im::ordmap::Entry::Occupied(entry) => {
                let path = file.path(db);
                let location = Location::new(path.clone(), item.range());
                let original_definition = Location::new(path, entry.get().range());
                let diag = Diagnostic::duplicate_name(name.into(), location, original_definition);

                Diagnostics::push(db, diag);
            }
        }
    }

    let names = OrdMap::new();
    let interfaces = Vector::new();
    let worlds = Vector::new();

    Items::new(db, names, interfaces, worlds)
}

fn named_item(item: ast::TopLevelItem<'_>) -> Option<NamedItem<'_>> {
    let node = item.syntax();

    if node.has_error() {
        return None;
    }

    if let Some(interface) = item.interface_item() {
        Some(NamedItem::Interface(interface))
    } else if let Some(world) = item.world_item() {
        Some(NamedItem::World(world))
    } else if let Some(import) = item.top_level_use_item() {
        Some(NamedItem::Use(import))
    } else {
        unreachable!("Unknown top level item node: {}", node.kind());
    }
}

#[salsa::tracked]
pub struct Items {
    pub item_names: OrdMap<Text, hir::ItemId>,
    pub interfaces: Vector<hir::Interface>,
    pub worlds: Vector<hir::World>,
}

#[derive(Debug, Copy, Clone)]
enum NamedItem<'a> {
    World(ast::WorldItem<'a>),
    Interface(ast::InterfaceItem<'a>),
    Use(ast::TopLevelUseItem<'a>),
}

impl<'a> NamedItem<'a> {
    fn node(self) -> tree_sitter::Node<'a> {
        match self {
            NamedItem::World(world) => world.syntax(),
            NamedItem::Interface(interface) => interface.syntax(),
            NamedItem::Use(node) => node.syntax(),
        }
    }

    fn range(self) -> Range {
        self.node().range()
    }

    fn ident(self) -> ast::Identifier<'a> {
        match self {
            NamedItem::World(w) => w.name().expect("syntax error"),
            NamedItem::Interface(i) => i.name().expect("syntax error"),
            NamedItem::Use(u) => {
                if let Some(ident) = u.alias_opt() {
                    return ident;
                }

                let use_path = u.use_path().expect("syntax error");
                if let Some(p) = use_path.fully_qualified_use_path() {
                    p.package()
                        .and_then(|pkg| pkg.identifier())
                        .expect("syntax error")
                } else if let Some(p) = use_path.local_use_path() {
                    p.identifier().expect("syntax error")
                } else {
                    unreachable!("syntax error")
                }
            }
        }
    }
}
