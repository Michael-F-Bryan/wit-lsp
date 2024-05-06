use im::OrdMap;
use tower_lsp::lsp_types::{CodeAction, CodeActionKind, WorkspaceEdit};
use wit_compiler::{
    queries::{FilePath, LineNumbers, Workspace},
    Db,
};

use crate::code_actions::{Indel, TextEdit};

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    title: String,
    modifications: OrdMap<FilePath, TextEdit>,
    kind: ActionKind,
}

impl Action {
    pub fn refactor(title: impl Into<String>) -> Action {
        Action {
            title: title.into(),
            modifications: OrdMap::new(),
            kind: ActionKind::Refactor,
        }
    }

    pub fn modify(mut self, path: FilePath, edit: TextEdit) -> Self {
        self.modifications.insert(path, edit);
        self
    }

    pub fn apply(&self, db: &mut dyn wit_compiler::Db, ws: Workspace) {
        let files = ws.files(db);

        for (path, edit) in &self.modifications {
            let file = files[path];
            let mut contents = file.contents(db).to_string();
            edit.apply(&mut contents);
            file.set_contents(db).to(contents.into());
        }
    }

    pub fn to_lsp(self, db: &dyn wit_compiler::Db, ws: Workspace) -> CodeAction {
        let Action {
            title,
            modifications,
            kind,
        } = self;

        CodeAction {
            title,
            kind: Some(kind.into()),
            edit: Some(workplace_edit(db, ws, modifications)),
            ..Default::default()
        }
    }
}

fn workplace_edit(
    db: &dyn Db,
    ws: Workspace,
    modifications: OrdMap<FilePath, TextEdit>,
) -> tower_lsp::lsp_types::WorkspaceEdit {
    let files = ws.files(db);

    let changes = modifications
        .into_iter()
        .map(|(path, edit)| {
            let line_numbers = wit_compiler::queries::calculate_line_numbers(db, files[&path]);
            (
                path.raw_path(db).to_string().parse().unwrap(),
                text_edits(edit, &line_numbers),
            )
        })
        .collect();

    WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    }
}

fn text_edits(edit: TextEdit, line_numbers: &LineNumbers) -> Vec<tower_lsp::lsp_types::TextEdit> {
    edit.into_iter()
        .map(|Indel { insert, delete }| tower_lsp::lsp_types::TextEdit {
            range: tower_lsp::lsp_types::Range {
                start: crate::utils::ts_to_position(line_numbers.point(delete.start).unwrap()),
                end: crate::utils::ts_to_position(line_numbers.point(delete.end).unwrap()),
            },
            new_text: insert,
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ActionKind {
    Quickfix,
    /// A generic refactor.
    ///
    /// Prefer [`ActionKind::Extract`], [`ActionKind::Inline`], or
    /// [`ActionKind::Rewrite`] if possible.
    Refactor,
    /// Take the selected code and extract it to somewhere else.
    ///
    /// For example:
    ///
    /// - Extract method
    /// - Extract function
    /// - Extract variable
    /// - Extract interface from class
    Extract,
    /// Inline an abstraction.
    ///
    /// For example:
    ///
    /// - Inline function
    /// - Inline variable
    /// - Inline constant
    Inline,
    /// Rewrite something.
    ///
    /// For example:
    ///
    /// - Convert JavaScript function to class
    /// - Add or remove parameter
    /// - Encapsulate field
    /// - Make method static
    /// - Move method to base class
    Rewrite,
    OrganiseImports,
}

impl From<ActionKind> for CodeActionKind {
    fn from(value: ActionKind) -> Self {
        match value {
            ActionKind::Quickfix => CodeActionKind::QUICKFIX,
            ActionKind::Refactor => CodeActionKind::REFACTOR,
            ActionKind::Extract => CodeActionKind::REFACTOR_EXTRACT,
            ActionKind::Inline => CodeActionKind::REFACTOR_INLINE,
            ActionKind::Rewrite => CodeActionKind::REFACTOR_REWRITE,
            ActionKind::OrganiseImports => CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
        }
    }
}
