use codespan_reporting::diagnostic::{Label, LabelStyle, Severity};
use tower_lsp::lsp_types::{
    DiagnosticRelatedInformation, DiagnosticSeverity, DocumentDiagnosticReport,
    DocumentDiagnosticReportResult,
};
use wit_compiler::{
    diagnostics::{Diagnostics, Location},
    queries::{FilePath, SourceFile, Workspace},
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
        .filter_map(|diag| lsp_diagnostic(db, ws, diag, file.path(db).raw_path(db)))
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
///
/// # Implementation Detail
///
/// This stashes a hash of the [`wit_compiler::diagnostics::Diagnostic`] inside
/// the resulting [`tower_lsp::lsp_types::Diagnostic`] so it can be retrieved
/// at a later date (e.g. when running a *Code Action* that can fix the issue).
/// The hash code is generated using [`utils::hash_diagnostic()`].
fn lsp_diagnostic(
    db: &dyn wit_compiler::Db,
    ws: Workspace,
    diag: wit_compiler::diagnostics::Diagnostic,
    uri: &str,
) -> Option<tower_lsp::lsp_types::Diagnostic> {
    let location = diag.location();

    if location.filename.raw_path(db) != uri {
        return None;
    }

    let hash = utils::hash_diagnostic(&diag);
    let diag = diag.as_diagnostic();
    let code = diag.code.as_deref()?;
    let msg = &diag.message;

    let secondaries: Vec<_> = diag
        .labels
        .iter()
        .filter(|label| label.style != LabelStyle::Primary)
        .collect();

    let related_information = secondaries
        .iter()
        .filter_map(|label| {
            let location = label_location(db, ws, label)?;
            Some(DiagnosticRelatedInformation {
                location: utils::location_to_lsp(db, location),
                message: label.message.clone(),
            })
        })
        .collect();

    Some(tower_lsp::lsp_types::Diagnostic {
        range: utils::ts_to_range(location.range),
        message: msg.to_string(),
        related_information: Some(related_information),
        code: Some(tower_lsp::lsp_types::NumberOrString::String(
            code.to_string(),
        )),
        severity: Some(lsp_severity(diag.severity)),
        data: Some(hash.into()),
        ..Default::default()
    })
}

fn lsp_severity(severity: Severity) -> DiagnosticSeverity {
    match severity {
        Severity::Bug | Severity::Error => DiagnosticSeverity::ERROR,
        Severity::Warning => DiagnosticSeverity::WARNING,
        Severity::Note => DiagnosticSeverity::INFORMATION,
        Severity::Help => DiagnosticSeverity::HINT,
    }
}

fn label_location(
    db: &dyn wit_compiler::Db,
    ws: Workspace,
    label: &Label<FilePath>,
) -> Option<Location> {
    let files = ws.files(db);
    let path = label.file_id;
    let line_numbers = wit_compiler::queries::calculate_line_numbers(db, files[&path]);

    let std::ops::Range { start, end } = label.range;
    let start_point = line_numbers.point(start).ok()?;
    let end_point = line_numbers.point(end).ok()?;
    let range = tree_sitter::Range {
        start_byte: start,
        end_byte: end,
        start_point,
        end_point,
    };

    Some(Location::new(path, range))
}
