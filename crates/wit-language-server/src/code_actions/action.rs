use tower_lsp::lsp_types::CodeActionOrCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    title: String,
    kind: ActionKind,
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

impl From<Action> for CodeActionOrCommand {
    fn from(_value: Action) -> Self {
        todo!();
    }
}
