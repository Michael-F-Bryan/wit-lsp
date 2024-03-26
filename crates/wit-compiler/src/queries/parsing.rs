use im::OrdMap;

use crate::{
    ast::AstNode,
    diagnostics::{Diagnostic, Diagnostics},
    traverse::Order,
    Db, Text, Tree,
};

#[salsa::tracked]
pub fn parse(db: &dyn Db, ws: Workspace, path: Text) -> Option<Ast> {
    let files = ws.files(db);
    let src = files.get(&path)?;
    let tree = Tree::parse(src);

    let root = tree.root_node();

    if root.has_error() {
        for error_node in crate::traverse::tree(&tree, Order::Pre).filter(|node| node.is_error()) {
            Diagnostics::push(db, Diagnostic::parse_error(error_node));
        }
    }

    Some(Ast::new(db, tree, src.clone()))
}

/// A workspace keeps track of all files known to the compiler.
#[salsa::input]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Workspace {
    pub files: OrdMap<Text, Text>,
}

impl Workspace {
    /// Update a file's contents.
    pub fn update(&self, db: &mut dyn Db, path: impl Into<Text>, text: impl Into<Text>) {
        let mut files = self.files(db);
        files.insert(path.into(), text.into());
        self.set_files(db).to(files);
    }
}

#[salsa::tracked]
#[derive(Debug, Clone)]
pub struct Ast {
    #[return_ref]
    pub tree: Tree,
    #[return_ref]
    pub src: Text,
}

impl Ast {
    pub fn root_node(self, db: &dyn Db) -> tree_sitter::Node<'_> {
        self.tree(db).root_node()
    }

    pub fn source_file(self, db: &dyn Db) -> crate::ast::SourceFile<'_> {
        let root = self.root_node(db);
        crate::ast::SourceFile::cast(root).unwrap()
    }
}
