//! Errors and user-facing messages that may be generated as the result of
//! analysis.

use tree_sitter::Range;

use crate::Text;

/// An accumulator for all [`Diagnostic`]s that have been emitted.
#[salsa::accumulator]
pub struct Diagnostics(Diagnostic);

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Diagnostic {
    DuplicateName(DuplicateName),
    Parse(SyntaxError),
    Unimplemented(Unimplemented),
    UnknownName(UnknownName),
}

impl Diagnostic {
    pub fn parse_error(rule: impl Into<String>, range: Range) -> Self {
        Diagnostic::Parse(SyntaxError {
            rule: rule.into(),
            range,
        })
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

    pub fn unimplemented(message: Text, filename: Text, range: Range) -> Self {
        Diagnostic::Unimplemented(Unimplemented {
            message,
            filename,
            range,
        })
    }

    pub fn unknown_name(name: Text, filename: Text, range: Range) -> Self {
        Diagnostic::UnknownName(UnknownName {
            name,
            filename,
            range,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub rule: String,
    pub range: Range,
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
    pub range: Range,
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
    pub range: Range,
}

impl From<UnknownName> for Diagnostic {
    fn from(value: UnknownName) -> Self {
        Diagnostic::UnknownName(value)
    }
}
