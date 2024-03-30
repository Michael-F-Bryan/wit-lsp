use im::OrdMap;

use crate::{
    ast::AstNode,
    diagnostics::{Diagnostic, Diagnostics, Location},
    traverse::Order,
    Db, Text, Tree,
};

#[salsa::tracked]
pub fn parse(db: &dyn Db, file: SourceFile) -> Ast {
    let src = file.contents(db);
    let tree = Tree::parse(&src);

    let root = tree.root_node();

    if root.has_error() {
        for error_node in crate::traverse::tree(&tree, Order::Pre).filter(|node| node.is_error()) {
            if let Some(parent) = error_node.parent() {
                let location = Location::new(file.path(db), error_node.range());
                Diagnostics::push(db, Diagnostic::parse_error(parent.kind(), location));
            }
        }
    }

    Ast::new(db, tree, src.clone())
}

/// A workspace keeps track of all files known to the compiler.
#[salsa::input]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Workspace {
    pub files: OrdMap<Text, SourceFile>,
}

impl Workspace {
    /// Update a file's contents.
    pub fn update(&self, db: &mut dyn Db, path: impl Into<Text>, text: impl Into<Text>) {
        let mut files = self.files(db);
        let path = path.into();
        let file = SourceFile::new(db, path.clone(), text.into());
        files.insert(path, file);
        self.set_files(db).to(files);
    }

    pub fn lookup(&self, db: &dyn Db, path: &str) -> Option<SourceFile> {
        let files = self.files(db);
        files.get(path).cloned()
    }
}

#[salsa::input]
pub struct SourceFile {
    pub path: Text,
    pub contents: Text,
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
