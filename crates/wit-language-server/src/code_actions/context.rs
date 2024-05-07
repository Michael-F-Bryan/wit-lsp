use std::collections::HashSet;

use tower_lsp::lsp_types::CodeActionParams;
use tree_sitter::{Range, TreeCursor};
use wit_compiler::{
    ast::AstNode,
    diagnostics::{Diagnostic, Diagnostics},
    queries::{FilePath, Package, SourceFile, Workspace},
    Tree,
};

use crate::{utils, Db};

#[derive(Debug, Clone)]
pub struct CodeActionContext<'db> {
    db: &'db dyn Db,
    ws: Workspace,
    tree: &'db Tree,
    diagnostics: Vec<Diagnostic>,
    path: FilePath,
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

        let current_file = ws.lookup_by_path(db.as_wit(), uri).unwrap();
        let path = current_file.path(db.as_wit());
        let current_package = packages
            .iter()
            .copied()
            .find(|pkg| pkg.contains(db.as_wit(), path))
            .unwrap();

        let tree = wit_compiler::queries::parse(db.as_wit(), current_file).tree(db.as_wit());

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
        let start_byte = line_numbers.offset_for_point(start).unwrap();
        let end_byte = line_numbers.offset_for_point(end).unwrap();

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
            path,
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

    pub fn path(&self) -> FilePath {
        self.path
    }

    pub fn src(&self) -> &str {
        self.current_file().contents(self.wit_db())
    }

    /// The part of the file that was selected.
    pub fn selection(&self) -> Range {
        self.selection
    }

    /// Get a [`TreeCursor`] that points to the node that was selected.
    ///
    /// Note that this may not exactly match [`Self::selection()`] because it
    /// finds the smallest node that contains the current selection, meaning the
    /// range may have needed to be expanded.
    pub fn cursor(&self) -> TreeCursor<'_> {
        let mut cursor = self.tree().walk();
        let selection = self.selection();

        fn contains(node: tree_sitter::Node<'_>, range: Range) -> bool {
            let std::ops::Range { start, end } = node.byte_range();
            start <= range.start_byte && range.end_byte < end
        }

        loop {
            if contains(cursor.node(), selection) {
                if cursor.goto_first_child() {
                    continue;
                } else {
                    break;
                }
            } else if cursor.goto_next_sibling() {
                continue;
            } else {
                break;
            }
        }

        cursor
    }

    /// Walk up the tree from [`Self::cursor()`] looking for a particular node,
    /// `T`.
    pub fn next_ancestor<'this, T: AstNode<'this>>(&'this self) -> Option<T> {
        let mut cursor = self.cursor();

        loop {
            if let Some(node) = T::cast(cursor.node()) {
                return Some(node);
            }

            if !cursor.goto_parent() {
                return None;
            }
        }
    }

    pub fn selection_bytes(&self) -> std::ops::Range<usize> {
        let Range {
            start_byte,
            end_byte,
            ..
        } = self.selection();
        start_byte..end_byte
    }

    pub fn tree(&self) -> &'db Tree {
        self.tree
    }

    /// The list of diagnostics that apply to this area.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}
