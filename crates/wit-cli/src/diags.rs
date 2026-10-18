use codespan_reporting::term::{Config, termcolor::WriteColor};
use wit_compiler::{Db, diagnostics::Diagnostic, queries::Workspace};

#[tracing::instrument(skip_all)]
pub(crate) fn print_diagnostics(
    writer: &mut dyn WriteColor,
    db: &dyn Db,
    ws: Workspace,
    diags: &[Diagnostic],
) -> color_eyre::Result<()> {
    let config = Config::default();
    let files = ws.as_codespan_files(db);

    for diag in diags {
        let diag = diag.as_diagnostic();
        codespan_reporting::term::emit_to_write_style(writer, &config, &files, &diag)?;
    }

    Ok(())
}
