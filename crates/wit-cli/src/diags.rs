use std::io::Write;

use wit_compiler::{diagnostics::Diagnostic, queries::Workspace, Db};

#[tracing::instrument(skip_all)]
pub(crate) fn print_diags(
    writer: &mut dyn Write,
    db: &dyn Db,
    ws: Workspace,
    diags: &[Diagnostic],
) -> color_eyre::Result<()> {
    for diag in diags {
        print_diag(writer, db, ws, diag)?;
    }

    Ok(())
}

fn print_diag(
    _writer: &mut dyn Write,
    _db: &dyn Db,
    _ws: Workspace,
    diag: &Diagnostic,
) -> color_eyre::Result<()> {
    match diag {
        Diagnostic::DuplicateName(_) => todo!(),
        Diagnostic::MultipleConstructors(_) => todo!(),
        Diagnostic::SyntaxError(_) => todo!(),
        Diagnostic::UnknownName(_) => todo!(),
        Diagnostic::Bug(_) => todo!(),
        Diagnostic::MismatchedPackageDeclaration(_) => todo!(),
        _other => todo!(),
    }
}
