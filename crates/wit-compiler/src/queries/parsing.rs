use std::borrow::Borrow;

use im::{OrdMap, Vector};

use crate::{
    ast::{self, AstNode, HasSource},
    diagnostics::{Diagnostic, Diagnostics, Location, MismatchedPackageDeclaration},
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

#[salsa::tracked]
impl Workspace {
    /// Update a file's contents.
    pub fn update(self, db: &mut dyn Db, path: impl Into<FilePath>, text: impl Into<Text>) {
        let mut files = self.files(db);
        let path = path.into();
        let file = SourceFile::new(db, path.clone(), text.into());
        files.insert(path, file);
        self.set_files(db).to(files);
    }

    pub fn lookup(self, db: &dyn Db, path: &str) -> Option<SourceFile> {
        let files = self.files(db);
        files.get(path).cloned()
    }

    #[salsa::tracked]
    pub fn packages(self, db: &dyn Db) -> Vector<Package> {
        let files = self.files(db);
        let mut folders: OrdMap<&str, Vector<SourceFile>> = OrdMap::new();

        for (path, f) in &files {
            // Note: We're kinda breaking FilePath's encapsulation by assuming
            // the path has some sort of internal structure (i.e. it contains
            // slash-separated segments), but this should be okay because in
            // practice we'll only ever be receiving URIs or fs paths, and both
            // satisfy that assumption.
            let dir = match path.rsplit_once(|c| c == '/' || c == '\\') {
                Some((dir, _)) => dir,
                None => ".",
            };

            let entries = folders.entry(dir).or_default();
            entries.push_back(*f);
            entries.sort();
        }

        folders
            .into_iter()
            .map(|(dir, files)| {
                let dir = FilePath::from(dir);
                let id = resolve_id(db, &files);
                Package::new(db, dir, id, files)
            })
            .collect()
    }
}

fn resolve_id(db: &dyn Db, files: &Vector<SourceFile>) -> Option<PackageId> {
    let mut id: Option<(PackageId, Location)> = None;

    for f in files.iter().copied() {
        let ast = crate::queries::parse(db, f);

        if let Some(node) = ast.source_file(db).package_opt() {
            if let Some(new_id) = package_id(db, node, f) {
                let location = Location::new(f.path(db), node.range());

                match &id {
                    Some((original_id, _)) if new_id == *original_id => {
                        // Same ID, so nothing to complain about
                    }
                    Some((original_id, original_definition)) => {
                        let diag = MismatchedPackageDeclaration {
                            original_definition: original_definition.clone(),
                            original_id: *original_id,
                            second_id: new_id,
                            second_location: location,
                        };
                        Diagnostics::push(db, diag.into());
                    }
                    None => {
                        id = Some((new_id, location));
                    }
                }
            }
        }
    }

    id.map(|(id, _)| id)
}

fn package_id(db: &dyn Db, node: ast::PackageDecl<'_>, file: SourceFile) -> Option<PackageId> {
    let node = node.fully_qualified_package_name()?;
    let src = file.contents(db);

    let pkg = node.package()?.utf8_text(src);
    let pkg = Vector::unit(Text::from(pkg));

    let path = node
        .path()?
        .iter_identifiers()
        .map(|ident| Text::from(ident.utf8_text(src)))
        .collect();

    let version = node.version_opt().map(|v| v.utf8_text(src)).map(Text::from);

    Some(PackageId::new(db, pkg, path, version))
}

/// A group of files in a [`Workspace`].
#[salsa::tracked]
pub struct Package {
    pub dir: FilePath,
    pub id: Option<PackageId>,
    pub files: Vector<SourceFile>,
}

#[salsa::interned]
pub struct PackageId {
    pub package: Vector<Text>,
    pub path: Vector<Text>,
    pub version: Option<Text>,
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

#[cfg(test)]
mod tests {
    use salsa::DebugWithDb;

    use crate::Compiler;

    use super::*;

    macro_rules! workspace {
        ($db:ident, { $( $path:expr => $contents:expr),* $(,)? }) => {{
            let mut files = OrdMap::new();
            $(
                let path = FilePath::from($path);
                let f = SourceFile::new(&$db, path.clone(), $contents.into());
                files.insert(path, f);
            )*
            Workspace::new(&$db, files)
        }};
    }

    #[test]
    fn determine_packages() {
        let db = Compiler::default();

        let ws = workspace!(db, {
            "root.wit" => "",
            "simple/unnamed.wit" => "",
            "named/package.wit" => "package named:package;",
            "named/with/version.wit" => "package named:with/version@1.2.3;",
            "merge/empty.wit" => "",
            "merge/package-decl.wit" => "package merge:some-package",
        });

        let packages = ws.packages(&db);

        let diags = Workspace_packages::accumulated::<Diagnostics>(&db, ws);
        assert!(diags.is_empty(), "{diags:?}");

        let debug: Vec<_> = packages.iter().map(|pkg| pkg.debug_all(&db)).collect();
        insta::assert_debug_snapshot!(debug);
    }

    #[test]
    fn conflicting_package_declarations() {
        let db = Compiler::default();

        let ws = workspace!(db, {
            "first.wit" => "package conflict:first",
            "second.wit" => "package conflict:second",
        });

        let diags = Workspace_packages::accumulated::<Diagnostics>(&db, ws);
        assert_eq!(diags.len(), 1);
        insta::assert_debug_snapshot!(diags);
    }
}
