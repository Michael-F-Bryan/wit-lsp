use codespan_reporting::term::{termcolor::WriteColor, Config};
use wit_compiler::{diagnostics::Diagnostic, queries::Workspace, Db};

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
        codespan_reporting::term::emit(writer, &config, &files, &diag)?;
    }

    Ok(())
}
