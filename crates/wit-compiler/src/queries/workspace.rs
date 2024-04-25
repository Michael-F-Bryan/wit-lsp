use std::ops::Range;

use codespan_reporting::files::Error as CodespanError;
use im::{OrdMap, Vector};

use crate::{
    ast::{self, AstNode, HasSource},
    diagnostics::{Diagnostics, Location, MismatchedPackageDeclaration},
    Db, Text,
};

/// The path to a [`SourceFile`] in the [`Workspace`].
///
/// Ideally, this should only ever be passed around as an opaque identifier and
/// shown to the user. You shouldn't make any assumptions about its contents.
#[salsa::interned]
pub struct FilePath {
    #[return_ref]
    pub raw_path: Text,
}

/// A workspace keeps track of all files known to the compiler.
#[salsa::input]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Workspace {
    pub files: OrdMap<FilePath, SourceFile>,
}

impl Workspace {
    /// Update a file's contents.
    pub fn update(self, db: &mut dyn Db, path: &str, text: impl Into<Text>) {
        let text = text.into();

        match self.lookup_by_path(db, path) {
            Some(file) => {
                file.set_contents(db).to(text);
            }
            None => {
                let mut files = self.files(db);
                let path = FilePath::new(db, path.into());
                let file = SourceFile::new(db, path, text);
                files.insert(path, file);
                self.set_files(db).to(files);
            }
        }
    }

    pub fn lookup_by_path(self, db: &dyn Db, path: &str) -> Option<SourceFile> {
        let files = self.files(db);

        files
            .iter()
            .find_map(|(p, f)| (p.raw_path(db) == path).then_some(*f))
    }

    /// Get a view of this [`Workspace`] as a
    /// [`codespan_reporting::files::Files`] database that can be used when
    /// rendering [`Diagnostics`].
    pub fn as_codespan_files(self, db: &dyn Db) -> WorkspaceFiles<'_> {
        WorkspaceFiles { db, ws: self }
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
                            original_definition: *original_definition,
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

#[salsa::tracked]
pub fn workspace_packages(db: &dyn Db, ws: Workspace) -> Vector<Package> {
    let files = ws.files(db);
    let mut folders: OrdMap<&str, Vector<SourceFile>> = OrdMap::new();

    for (path, f) in &files {
        // Note: We're kinda breaking FilePath's encapsulation by assuming
        // the path has some sort of internal structure (i.e. it contains
        // slash-separated segments), but this should be okay because in
        // practice we'll only ever be receiving URIs or fs paths, and both
        // satisfy that assumption.
        let path = path.raw_path(db);
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
            let dir = FilePath::new(db, dir.into());
            let id = resolve_id(db, &files);
            Package::new(db, dir, id, files)
        })
        .collect()
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
    pub path: FilePath,
    #[return_ref]
    pub contents: Text,
}

/// An implementation of [`codespan_reporting::files::Files`] which will query
/// the [`Db`] as required.
#[derive(Copy, Clone)]
pub struct WorkspaceFiles<'db> {
    ws: Workspace,
    db: &'db dyn Db,
}

impl<'db> WorkspaceFiles<'db> {
    fn file(&self, path: FilePath) -> Result<SourceFile, CodespanError> {
        let files = self.ws.files(self.db);
        files.get(&path).copied().ok_or(CodespanError::FileMissing)
    }
}

impl<'db> codespan_reporting::files::Files<'db> for WorkspaceFiles<'db> {
    type FileId = FilePath;
    type Name = Text;
    type Source = &'db str;

    fn name(&'db self, id: FilePath) -> Result<Text, CodespanError> {
        let file = self.file(id)?;
        Ok(file.path(self.db).raw_path(self.db).clone())
    }

    fn source(&'db self, id: FilePath) -> Result<&'db str, CodespanError> {
        let file = self.file(id)?;
        Ok(file.contents(self.db))
    }

    fn line_index(&'db self, path: FilePath, byte_index: usize) -> Result<usize, CodespanError> {
        let file = self.file(path)?;
        crate::queries::calculate_line_numbers(self.db, file).line_index(byte_index)
    }

    fn line_range(
        &'db self,
        path: FilePath,
        line_index: usize,
    ) -> Result<Range<usize>, CodespanError> {
        let file = self.file(path)?;
        crate::queries::calculate_line_numbers(self.db, file).line_range(line_index)
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
                let path = FilePath::new(&$db, $path.into());
                let f = SourceFile::new(&$db, path, $contents.into());
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

        let packages = workspace_packages(&db, ws);

        let diags = workspace_packages::accumulated::<Diagnostics>(&db, ws);
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

        let diags = workspace_packages::accumulated::<Diagnostics>(&db, ws);
        assert_eq!(diags.len(), 1);
        insta::assert_debug_snapshot!(diags);
    }
}
