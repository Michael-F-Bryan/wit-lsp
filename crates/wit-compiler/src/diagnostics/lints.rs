use crate::{
    diagnostics::{Diagnostics, IntoDiagnostic, Location},
    queries::{Ast, Package, SourceFile, Workspace},
    Db,
};

#[derive(Debug, Copy, Clone)]
pub struct LintPass {
    pub name: &'static str,
    pub default_severity: codespan_reporting::diagnostic::Severity,
    pub run: fn(LintContext<'_>),
}

#[derive(Debug, Clone)]
pub struct LintContext<'db> {
    db: &'db dyn Db,
    ws: Workspace,
    pkg: Package,
    current_file: SourceFile,
    ast: Ast,
    src: &'db str,
}

impl<'db> LintContext<'db> {
    pub(crate) fn new(
        db: &'db dyn Db,
        ws: Workspace,
        pkg: Package,
        current_file: SourceFile,
    ) -> Self {
        let src = current_file.contents(db);
        let ast = crate::queries::parse(db, current_file);

        LintContext {
            db,
            ws,
            pkg,
            current_file,
            ast,
            src,
        }
    }

    pub fn db(&self) -> &'db dyn Db {
        self.db
    }

    pub fn ws(&self) -> Workspace {
        self.ws
    }

    pub fn pkg(&self) -> Package {
        self.pkg
    }

    pub fn current_file(&self) -> SourceFile {
        self.current_file
    }

    pub fn ast(&self) -> Ast {
        self.ast
    }

    pub fn root_node(&self) -> tree_sitter::Node<'db> {
        self.ast().root_node(self.db())
    }

    pub fn src(&self) -> &str {
        self.src
    }

    pub fn location<'tree>(&self, node: impl Into<tree_sitter::Node<'tree>>) -> Location {
        let node = node.into();
        let range = node.range();
        let filename = self.current_file().path(self.db());

        Location { filename, range }
    }

    pub fn emit<D: IntoDiagnostic>(&self, lint: D) {
        Diagnostics::push(self.db(), lint.into());
    }
}
