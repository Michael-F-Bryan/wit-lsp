use std::collections::HashSet;

use tower_lsp::lsp_types::CodeActionParams;
use tree_sitter::Range;
use wit_compiler::{
    diagnostics::{Diagnostic, Diagnostics},
    queries::{Package, SourceFile, Workspace},
    Tree,
};

use crate::{utils, Db};

#[derive(Debug, Clone)]
pub struct CodeActionContext<'db> {
    db: &'db dyn Db,
    ws: Workspace,
    tree: Tree,
    diagnostics: Vec<Diagnostic>,
    current_package: Package,
    current_file: SourceFile,
    selection: Range,
}

impl<'db> CodeActionContext<'db> {
    pub(super) fn from_lsp(
        db: &'db dyn Db,
        ws: Workspace,
        params: CodeActionParams,
    ) -> Option<Self> {
        let uri = params.text_document.uri.as_str();
        let packages = wit_compiler::queries::workspace_packages(db.as_wit(), ws);

        let current_file = ws.lookup_by_path(db.as_wit(), uri)?;
        let path = current_file.path(db.as_wit());
        let current_package = packages
            .iter()
            .copied()
            .find(|pkg| pkg.contains(db.as_wit(), path))?;

        let tree = wit_compiler::queries::parse(db.as_wit(), current_file)
            .tree(db.as_wit())
            .clone();

        let hash_codes: HashSet<u64> = params
            .context
            .diagnostics
            .iter()
            .filter_map(|diag| diag.data.as_ref()?.as_u64())
            .collect();
        let mut diagnostics = wit_compiler::queries::lower_package::accumulated::<Diagnostics>(
            db.as_wit(),
            ws,
            current_package,
        );
        diagnostics.retain(|diag| {
            let hash_code = utils::hash_diagnostic(diag);
            hash_codes.contains(&hash_code)
        });

        let start = utils::position_to_ts(params.range.start);
        let end = utils::position_to_ts(params.range.end);
        let line_numbers = wit_compiler::queries::calculate_line_numbers(db.as_wit(), current_file);
        let start_byte = line_numbers.offset_for_point(start).ok()?;
        let end_byte = line_numbers.offset_for_point(end).ok()?;

        let selection = Range {
            start_byte,
            end_byte,
            start_point: start,
            end_point: end,
        };

        Some(CodeActionContext {
            db,
            ws,
            tree,
            diagnostics,
            current_package,
            current_file,
            selection,
        })
    }

    pub fn db(&self) -> &'db dyn Db {
        self.db
    }

    pub fn wit_db(&self) -> &'db dyn wit_compiler::Db {
        self.db().as_wit()
    }

    pub fn workspace(&self) -> Workspace {
        self.ws
    }

    pub fn current_package(&self) -> Package {
        self.current_package
    }

    pub fn current_file(&self) -> SourceFile {
        self.current_file
    }

    pub fn src(&self) -> &str {
        self.current_file().contents(self.wit_db())
    }

    pub fn selection(&self) -> Range {
        self.selection
    }

    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// The list of diagnostics that apply to this area.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}
