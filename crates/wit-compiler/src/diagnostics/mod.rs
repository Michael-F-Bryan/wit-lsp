//! Errors and user-facing messages that may be generated as the result of
//! analysis.

mod all;

pub use self::all::all_diagnostics;

use codespan_reporting::diagnostic::Label;
use tree_sitter::Range;

use crate::{queries::FilePath, Text};

type Diag = codespan_reporting::diagnostic::Diagnostic<FilePath>;

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
    MultiplePackageDocs(MultiplePackageDocs),
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
            | Diagnostic::MultiplePackageDocs(MultiplePackageDocs {
                second_location: location,
                ..
            })
            | Diagnostic::Bug(Bug { location, .. }) => location,
        }
    }

    pub fn as_diagnostic(&self) -> Diag {
        match self {
            Diagnostic::DuplicateName(diag) => diag.as_diagnostic(),
            Diagnostic::MultipleConstructors(diag) => diag.as_diagnostic(),
            Diagnostic::SyntaxError(diag) => diag.as_diagnostic(),
            Diagnostic::UnknownName(diag) => diag.as_diagnostic(),
            Diagnostic::Bug(diag) => diag.as_diagnostic(),
            Diagnostic::MismatchedPackageDeclaration(diag) => diag.as_diagnostic(),
            Diagnostic::MultiplePackageDocs(diag) => diag.as_diagnostic(),
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
    /// A unique code which can be used when referring to this error.
    const ERROR_CODE: &'static str;
    /// A simple message that is displayed with the error.
    const MESSAGE: &'static str;
    /// A verbose explanation of the error.
    const VERBOSE_DESCRIPTION: &'static str;

    fn as_diagnostic(&self) -> Diag;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub rule: Option<String>,
    pub location: Location,
}

impl IntoDiagnostic for SyntaxError {
    const ERROR_CODE: &'static str = "E001";
    const MESSAGE: &'static str = "Syntax error";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E001-syntax-error.md");

    fn as_diagnostic(&self) -> Diag {
        let mut label = self.location.label();

        if let Some(rule) = &self.rule {
            label = label.with_message(format!("Expected a \"{rule}\""))
        }

        Diag::error()
            .with_message(Self::MESSAGE)
            .with_code(Self::ERROR_CODE)
            .with_labels(vec![label])
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
    const MESSAGE: &'static str = "Name defined multiple times";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E002-duplicate-name.md");

    fn as_diagnostic(&self) -> Diag {
        Diag::error()
            .with_message(Self::MESSAGE)
            .with_code(Self::ERROR_CODE)
            .with_labels(vec![
                self.location
                    .label()
                    .with_message("The duplicate was defined here"),
                self.original_definition
                    .secondary_label()
                    .with_message("Original definition is here"),
            ])
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
    const MESSAGE: &'static str = "Resource has multiple constructors";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E003-multiple-constructors.md");

    fn as_diagnostic(&self) -> Diag {
        Diag::error()
            .with_message(Self::MESSAGE)
            .with_code(Self::ERROR_CODE)
            .with_labels(vec![
                self.location
                    .label()
                    .with_message("The duplicate was defined here"),
                self.original_definition
                    .secondary_label()
                    .with_message("Original constructor was defined is here"),
            ])
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
    const MESSAGE: &'static str = "Reference to unknown name";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E004-unknown-name.md");

    fn as_diagnostic(&self) -> Diag {
        Diag::error()
            .with_message(Self::MESSAGE)
            .with_code(Self::ERROR_CODE)
            .with_labels(vec![self.location.label()])
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
    const MESSAGE: &'static str = "You ran into a bug 🐛";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E500-bug.md");

    fn as_diagnostic(&self) -> Diag {
        Diag::error()
            .with_message(Self::MESSAGE)
            .with_code(Self::ERROR_CODE)
            .with_labels(vec![self.location.label()])
            .with_notes(vec![format!("Triggered from {}", self.caller)])
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
    const MESSAGE: &'static str = "Mismatched package declarations";
    const VERBOSE_DESCRIPTION: &'static str =
        include_str!("E005-mismatched-package-declaration.md");

    fn as_diagnostic(&self) -> Diag {
        Diag::error()
            .with_message(Self::MESSAGE)
            .with_code(Self::ERROR_CODE)
            .with_labels(vec![
                self.second_location.label().with_message("Defined here"),
                self.original_definition
                    .secondary_label()
                    .with_message("Originally defined here"),
            ])
    }
}

impl From<MismatchedPackageDeclaration> for Diagnostic {
    fn from(value: MismatchedPackageDeclaration) -> Self {
        Diagnostic::MismatchedPackageDeclaration(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiplePackageDocs {
    pub second_location: Location,
    pub original_definition: Location,
}

impl IntoDiagnostic for MultiplePackageDocs {
    const ERROR_CODE: &'static str = "E006";
    const MESSAGE: &'static str = "Package docs can only be defined in a single file";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("E006-multiple-package-docs.md");

    fn as_diagnostic(&self) -> Diag {
        Diag::error()
            .with_message(Self::MESSAGE)
            .with_code(Self::ERROR_CODE)
            .with_labels(vec![
                self.second_location.label().with_message("Defined here"),
                self.original_definition
                    .secondary_label()
                    .with_message("Originally defined here"),
            ])
    }
}

impl From<MultiplePackageDocs> for Diagnostic {
    fn from(value: MultiplePackageDocs) -> Self {
        Diagnostic::MultiplePackageDocs(value)
    }
}

/// The location of an element within the workspace.
///
/// Typically used for debugging purposes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

    fn label(&self) -> Label<FilePath> {
        let Range {
            start_byte,
            end_byte,
            ..
        } = self.range;
        Label::primary(self.filename, start_byte..end_byte)
    }

    fn secondary_label(&self) -> Label<FilePath> {
        let Range {
            start_byte,
            end_byte,
            ..
        } = self.range;
        Label::secondary(self.filename, start_byte..end_byte)
    }

    pub fn contains(&self, point: tree_sitter::Point) -> bool {
        let Range {
            start_point,
            end_point,
            ..
        } = self.range;
        start_point <= point && end_point < point
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
    use std::{collections::HashMap, path::Path};

    use super::*;

    #[test]
    fn diagnostic_codes_are_well_formed() {
        let mut codes = HashMap::new();

        for diag in all_diagnostics() {
            let DiagnosticInfo {
                type_name,
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

            match codes.entry(error_code) {
                std::collections::hash_map::Entry::Occupied(entry) => {
                    panic!(
                        "Duplicate entries for {error_code}: {} and {}",
                        entry.get(),
                        type_name,
                    );
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(type_name);
                }
            }
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
