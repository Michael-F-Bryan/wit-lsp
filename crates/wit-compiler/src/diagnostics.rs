//! Errors and user-facing messages that may be generated as the result of
//! analysis.

use tree_sitter::Range;

use crate::{queries::FilePath, Text};

/// An accumulator for all [`Diagnostic`]s that have been emitted.
#[salsa::accumulator]
pub struct Diagnostics(Diagnostic);

/// Diagnostic messages that are emitted using [`Diagnostics`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Diagnostic {
    DuplicateName(DuplicateName),
    MultipleConstructors(MultipleConstructors),
    SyntaxError(SyntaxError),
    UnknownName(UnknownName),
    Bug(Bug),
    MismatchedPackageDeclaration(MismatchedPackageDeclaration),
}

impl Diagnostic {
    pub fn location(&self) -> &Location {
        match self {
            Diagnostic::DuplicateName(DuplicateName { location, .. })
            | Diagnostic::SyntaxError(SyntaxError { location, .. })
            | Diagnostic::UnknownName(UnknownName { location, .. })
            | Diagnostic::MultipleConstructors(MultipleConstructors { location, .. })
            | Diagnostic::MismatchedPackageDeclaration(MismatchedPackageDeclaration {
                second_location: location,
                ..
            })
            | Diagnostic::Bug(Bug { location, .. }) => location,
        }
    }

    pub fn parse_error(rule: impl Into<String>, location: Location) -> Self {
        Diagnostic::SyntaxError(SyntaxError {
            rule: rule.into(),
            location,
        })
    }

    pub fn duplicate_name(name: Text, location: Location, original_definition: Location) -> Self {
        Diagnostic::DuplicateName(DuplicateName {
            name,
            location,
            original_definition,
        })
    }

    pub fn multiple_constructors(location: Location, original_definition: Location) -> Self {
        Diagnostic::MultipleConstructors(MultipleConstructors {
            location,
            original_definition,
        })
    }

    pub fn unknown_name(filename: FilePath, name: Text, range: Range) -> Self {
        Diagnostic::UnknownName(UnknownName {
            name,
            location: Location::new(filename, range),
        })
    }

    #[track_caller]
    pub fn bug(message: impl Into<Text>, location: Location) -> Self {
        let message = message.into();

        if cfg!(debug_assertions) {
            panic!("BUG: {message} at {location:?}");
        }

        let backtrace = std::panic::Location::caller();

        Diagnostic::Bug(Bug {
            message,
            location,
            caller: backtrace,
        })
    }
}

trait IntoDiagnostic: Into<Diagnostic> {
    const ERROR_CODE: &'static str;
    const VERBOSE_DESCRIPTION: &'static str;

    fn into_diagnostic(self) -> codespan_reporting::diagnostic::Diagnostic<FilePath>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub rule: String,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateName {
    pub name: Text,
    pub location: Location,
    pub original_definition: Location,
}

impl From<DuplicateName> for Diagnostic {
    fn from(value: DuplicateName) -> Self {
        Diagnostic::DuplicateName(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultipleConstructors {
    pub location: Location,
    pub original_definition: Location,
}

impl From<MultipleConstructors> for Diagnostic {
    fn from(value: MultipleConstructors) -> Self {
        Diagnostic::MultipleConstructors(value)
    }
}

/// The user referenced an unknown identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownName {
    /// The name being referenced.
    pub name: Text,
    pub location: Location,
}

impl From<UnknownName> for Diagnostic {
    fn from(value: UnknownName) -> Self {
        Diagnostic::UnknownName(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bug {
    /// The message.
    pub message: Text,
    pub location: Location,
    pub caller: &'static std::panic::Location<'static>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MismatchedPackageDeclaration {
    pub second_id: crate::queries::PackageId,
    pub second_location: Location,
    pub original_id: crate::queries::PackageId,
    pub original_definition: Location,
}

impl From<MismatchedPackageDeclaration> for Diagnostic {
    fn from(value: MismatchedPackageDeclaration) -> Self {
        Diagnostic::MismatchedPackageDeclaration(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    /// The file this error came from.
    pub filename: FilePath,
    /// Where in the file we are referencing this name.
    pub range: Range,
}

impl Location {
    pub fn new(filename: FilePath, range: Range) -> Self {
        Location { filename, range }
    }
}
