use tower_lsp::lsp_types::{
    DiagnosticRelatedInformation, DiagnosticSeverity, DocumentDiagnosticReport,
    DocumentDiagnosticReportResult,
};
use wit_compiler::{
    diagnostics::{Diagnostics, DuplicateName, Location, SyntaxError, UnknownName},
    queries::{SourceFile, Workspace},
};

use crate::utils;

pub fn file_diagnostics(
    db: &dyn wit_compiler::Db,
    ws: Workspace,
    file: SourceFile,
) -> DocumentDiagnosticReportResult {
    let path = file.path(db);
    let pkg = wit_compiler::queries::workspace_packages(db, ws)
        .into_iter()
        .find(|pkg| pkg.contains(db, path))
        .expect("unreachable");

    let diags = wit_compiler::queries::lower_package::accumulated::<Diagnostics>(db, ws, pkg);
    let items = diags
        .into_iter()
        .filter_map(|diag| lsp_diagnostic(db, diag, file.path(db).raw_path(db)))
        .collect();

    DocumentDiagnosticReportResult::Report(DocumentDiagnosticReport::Full(
        tower_lsp::lsp_types::RelatedFullDocumentDiagnosticReport {
            full_document_diagnostic_report: tower_lsp::lsp_types::FullDocumentDiagnosticReport {
                items,
                ..Default::default()
            },
            ..Default::default()
        },
    ))
}

/// Convert a [`wit_compiler::diagnostics::Diagnostic`] to a
/// [`tower_lsp::lsp_types::Diagnostic`].
fn lsp_diagnostic(
    db: &dyn wit_compiler::Db,
    diag: wit_compiler::diagnostics::Diagnostic,
    uri: &str,
) -> Option<tower_lsp::lsp_types::Diagnostic> {
    if diag.location().filename.raw_path(db) != uri {
        return None;
    }

    match diag {
        wit_compiler::diagnostics::Diagnostic::DuplicateName(DuplicateName {
            name,
            location: Location { range, .. },
            original_definition,
        }) => {
            let diagnostic = tower_lsp::lsp_types::Diagnostic {
                range: utils::ts_to_range(range),
                message: format!("\"{name}\" is already defined"),
                related_information: Some(vec![DiagnosticRelatedInformation {
                    location: utils::location_to_lsp(db, original_definition),
                    message: "Original definition".into(),
                }]),
                severity: Some(DiagnosticSeverity::ERROR),
                ..Default::default()
            };
            Some(diagnostic)
        }
        wit_compiler::diagnostics::Diagnostic::SyntaxError(SyntaxError {
            location: Location { range, .. },
            ..
        }) => {
            let diagnostic = tower_lsp::lsp_types::Diagnostic {
                range: utils::ts_to_range(range),
                message: "Syntax error".into(),
                severity: Some(DiagnosticSeverity::ERROR),
                ..Default::default()
            };
            Some(diagnostic)
        }
        wit_compiler::diagnostics::Diagnostic::UnknownName(UnknownName {
            name,
            location: Location { range, .. },
            ..
        }) => {
            let diagnostic = tower_lsp::lsp_types::Diagnostic {
                range: utils::ts_to_range(range),
                message: format!("Attempted to reference unknown item, \"{name}\""),
                severity: Some(DiagnosticSeverity::ERROR),
                ..Default::default()
            };
            Some(diagnostic)
        }
        _ => None,
    }
}
