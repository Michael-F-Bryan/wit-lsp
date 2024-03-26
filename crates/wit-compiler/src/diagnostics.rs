//! Errors and user-facing messages that may be generated as the result of
//! analysis.

use tree_sitter::{Node, Range};

use crate::Text;

/// An accumulator for all [`Diagnostic`]s that have been emitted.
#[salsa::accumulator]
pub struct Diagnostics(Diagnostic);

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Diagnostic {
    DuplicateName(DuplicateName),
    Parse { range: Range },
    Unimplemented(Unimplemented),
    UnknownName(UnknownName),
}

impl Diagnostic {
    pub fn parse_error(node: Node<'_>) -> Self {
        debug_assert!(node.is_error());

        Diagnostic::Parse {
            range: node.range(),
        }
    }

    pub fn duplicate_name(
        name: Text,
        original_definition: Range,
        duplicate_definition: Range,
    ) -> Self {
        Diagnostic::DuplicateName(DuplicateName {
            name,
            original_definition,
            duplicate_definition,
        })
    }

    pub fn unimplemented(message: Text, filename: Text, span: Range) -> Self {
        Diagnostic::Unimplemented(Unimplemented {
            message,
            filename,
            span,
        })
    }

    pub fn unknown_name(name: Text, filename: Text, span: Range) -> Self {
        Diagnostic::UnknownName(UnknownName {
            name,
            filename,
            span,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateName {
    pub name: Text,
    pub original_definition: Range,
    pub duplicate_definition: Range,
}

impl From<DuplicateName> for Diagnostic {
    fn from(value: DuplicateName) -> Self {
        Diagnostic::DuplicateName(value)
    }
}

/// An unimplemented feature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unimplemented {
    pub message: Text,
    pub filename: Text,
    pub span: Range,
}

impl From<Unimplemented> for Diagnostic {
    fn from(value: Unimplemented) -> Self {
        Diagnostic::Unimplemented(value)
    }
}

/// The user referenced an unknown identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownName {
    /// The name being referenced.
    pub name: Text,
    /// The file this error came from.
    pub filename: Text,
    /// Where in the file we are referencing this name.
    pub span: Range,
}

impl From<UnknownName> for Diagnostic {
    fn from(value: UnknownName) -> Self {
        Diagnostic::UnknownName(value)
    }
}
