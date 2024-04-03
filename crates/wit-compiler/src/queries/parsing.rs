use std::borrow::Borrow;

use im::OrdMap;

use crate::{
    ast::AstNode,
    diagnostics::{Diagnostic, Diagnostics, Location},
    Db, Text, Tree,
};

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(file = %file.path(db)))]
pub fn parse(db: &dyn Db, file: SourceFile) -> Ast {
    let src = file.contents(db);
    let tree = Tree::parse(src);

    let root = tree.root_node();

    if root.has_error() {
        for error_node in tree.iter().filter(|node| node.is_error()) {
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
    pub files: OrdMap<FilePath, SourceFile>,
}

impl Workspace {
    /// Update a file's contents.
    pub fn update(&self, db: &mut dyn Db, path: impl Into<FilePath>, text: impl Into<Text>) {
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

/// A file attached to a [`Workspace`].
#[salsa::input]
pub struct SourceFile {
    #[return_ref]
    pub path: FilePath,
    #[return_ref]
    pub contents: Text,
}

/// The path to a [`SourceFile`] in the [`Workspace`].
///
/// Ideally, this should only ever be passed around as an opaque identifier and
/// shown to the user. You shouldn't make any assumptions about its contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FilePath(pub Text);

impl std::ops::Deref for FilePath {
    type Target = Text;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for FilePath
where
    T: Into<Text>,
{
    fn from(value: T) -> Self {
        FilePath(value.into())
    }
}

impl<T> Borrow<T> for FilePath
where
    Text: Borrow<T>,
    T: ?Sized,
{
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&FilePath> for FilePath {
    fn from(value: &FilePath) -> Self {
        value.clone()
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
