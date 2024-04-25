//! Errors and user-facing messages that may be generated as the result of
//! analysis.

mod all;

pub use self::all::all_diagnostics;

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

    pub fn into_diagnostic(&self) -> codespan_reporting::diagnostic::Diagnostic<FilePath> {
        match self {
            Diagnostic::DuplicateName(diag) => diag.as_diagnostic(),
            Diagnostic::MultipleConstructors(diag) => diag.as_diagnostic(),
            Diagnostic::SyntaxError(diag) => diag.as_diagnostic(),
            Diagnostic::UnknownName(diag) => diag.as_diagnostic(),
            Diagnostic::Bug(diag) => diag.as_diagnostic(),
            Diagnostic::MismatchedPackageDeclaration(diag) => diag.as_diagnostic(),
        }
    }
}

impl Bug {
    #[track_caller]
    pub fn new(message: impl Into<Text>, location: Location) -> Self {
        let message = message.into();

        if cfg!(debug_assertions) {
            panic!("BUG: {message} at {location:?}");
        }

        let caller = std::panic::Location::caller();

        Bug {
            message,
            location,
            caller,
        }
    }
}

pub trait IntoDiagnostic: Into<Diagnostic> {
    const ERROR_CODE: &'static str;
    const VERBOSE_DESCRIPTION: &'static str;

    fn as_diagnostic(&self) -> codespan_reporting::diagnostic::Diagnostic<FilePath>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub rule: String,
    pub location: Location,
}

impl IntoDiagnostic for SyntaxError {
    const ERROR_CODE: &'static str = "E001";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E001-syntax-error.md");

    fn as_diagnostic(&self) -> codespan_reporting::diagnostic::Diagnostic<FilePath> {
        todo!()
    }
}

impl From<SyntaxError> for Diagnostic {
    fn from(value: SyntaxError) -> Self {
        Diagnostic::SyntaxError(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateName {
    pub name: Text,
    pub location: Location,
    pub original_definition: Location,
}

impl IntoDiagnostic for DuplicateName {
    const ERROR_CODE: &'static str = "E002";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E002-duplicate-name.md");

    fn as_diagnostic(&self) -> codespan_reporting::diagnostic::Diagnostic<FilePath> {
        todo!()
    }
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

impl IntoDiagnostic for MultipleConstructors {
    const ERROR_CODE: &'static str = "E003";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E003-multiple-constructors.md");

    fn as_diagnostic(&self) -> codespan_reporting::diagnostic::Diagnostic<FilePath> {
        todo!()
    }
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

impl IntoDiagnostic for UnknownName {
    const ERROR_CODE: &'static str = "E004";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E004-unknown-name.md");

    fn as_diagnostic(&self) -> codespan_reporting::diagnostic::Diagnostic<FilePath> {
        todo!()
    }
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

impl IntoDiagnostic for Bug {
    const ERROR_CODE: &'static str = "E500";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E500-bug.md");

    fn as_diagnostic(&self) -> codespan_reporting::diagnostic::Diagnostic<FilePath> {
        todo!()
    }
}

impl From<Bug> for Diagnostic {
    fn from(value: Bug) -> Self {
        Diagnostic::Bug(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MismatchedPackageDeclaration {
    pub second_id: crate::queries::PackageId,
    pub second_location: Location,
    pub original_id: crate::queries::PackageId,
    pub original_definition: Location,
}

impl IntoDiagnostic for MismatchedPackageDeclaration {
    const ERROR_CODE: &'static str = "E005";
    const VERBOSE_DESCRIPTION: &'static str =
        include_str!("E005-mismatched-package-declaration.md");

    fn as_diagnostic(&self) -> codespan_reporting::diagnostic::Diagnostic<FilePath> {
        todo!()
    }
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

#[derive(Clone, Debug, serde::Serialize)]
pub struct DiagnosticInfo {
    pub type_name: &'static str,
    pub error_code: &'static str,
    pub description: &'static str,
}

impl DiagnosticInfo {
    pub fn for_type<T: IntoDiagnostic + 'static>() -> Self {
        DiagnosticInfo {
            type_name: std::any::type_name::<T>(),
            error_code: T::ERROR_CODE,
            description: T::VERBOSE_DESCRIPTION,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn diagnostic_codes_are_well_formed() {
        for diag in all_diagnostics() {
            let DiagnosticInfo {
                type_name: _,
                error_code,
                description,
            } = diag;

            assert!(
                error_code.starts_with('E'),
                "{error_code} must start with an 'E'"
            );
            let _: u32 = error_code[1..].parse().unwrap();

            let opening_line = description.lines().next().unwrap();
            let expected = format!("# {error_code}: ");
            assert!(
                opening_line.contains(&expected),
                "Expected {expected:?} in {opening_line:?}"
            );
        }
    }

    #[test]
    fn diagnostics_json_is_up_to_date() {
        let diags = all_diagnostics();
        let json = serde_json::to_string_pretty(&diags).unwrap();
        let path = project_root()
            .join("crates")
            .join("wit-compiler")
            .join("src")
            .join("diagnostics")
            .join("diagnostics.json");

        ensure_file_contents(path, json);
    }

    fn ensure_file_contents(path: impl AsRef<Path>, contents: impl AsRef<str>) {
        let path = path.as_ref();
        let contents = normalize_newlines(contents.as_ref());

        if let Ok(old_contents) = std::fs::read_to_string(path) {
            if contents == normalize_newlines(&old_contents) {
                tracing::debug!("File is already up to date");
                return;
            }
        }

        let display_path = path.strip_prefix(project_root()).unwrap_or(path);

        tracing::warn!(
            path=%display_path.display(),
            "File was not up-to-date, updating...",
        );

        if std::env::var("CI").is_ok() {
            tracing::warn!("Note: run codegen locally and commit the updated files");
        }

        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        tracing::info!(
            path=%path.display(),
            bytes_written=contents.len(),
            "Updating file",
        );
        std::fs::write(path, contents).unwrap();

        panic!(
            "\"{}\" was not up to date and has been updated. Please re-run code generation.",
            display_path.display()
        );
    }

    fn normalize_newlines(s: &str) -> String {
        s.replace("\r\n", "\n")
    }

    /// Get the root directory for this repository.
    pub fn project_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .find(|p| p.join(".git").exists())
            .unwrap()
    }
}
