use std::path::{Path, PathBuf};

use codespan_reporting::term::{
    termcolor::{ColorChoice, StandardStream, WriteColor},
    Config,
};
use im::OrdMap;
use libtest_mimic::{Failed, Trial};
use wit_compiler::{
    diagnostics::Diagnostic,
    queries::{FilePath, SourceFile, Workspace},
    Compiler, Db, Text,
};

#[tracing::instrument(skip_all, fields(base_dir = %base_dir.as_ref().display()))]
pub fn discover(base_dir: impl AsRef<Path>) -> color_eyre::Result<Vec<Trial>> {
    let base_dir = base_dir.as_ref();

    let mut trials = Vec::new();

    let compile_pass = base_dir.join("compile-pass");
    if compile_pass.exists() {
        trials.extend(discover_compile_pass(compile_pass)?);
    }

    Ok(trials)
}

#[tracing::instrument(skip_all, fields(dir = %dir.as_ref().display()))]
fn discover_compile_pass(dir: impl AsRef<Path>) -> color_eyre::Result<Vec<Trial>> {
    let dir = dir.as_ref();

    let mut trials = Vec::new();

    for entry in dir.read_dir()?.flatten() {
        let path = entry.path();
        let meta = entry.metadata()?;

        let filename = path.file_stem().unwrap().to_str().unwrap();
        let (name, ignored) = match filename.strip_prefix('_') {
            Some(n) => (n, true),
            None => (filename, false),
        };
        let name = format!("compile_pass::{name}");

        let paths = if meta.is_dir() {
            path.read_dir()?
                .flatten()
                .map(|entry| entry.path())
                .filter(|path| path.extension() == Some("wit".as_ref()))
                .collect()
        } else if meta.is_file() && path.extension() == Some("wit".as_ref()) {
            vec![path]
        } else {
            continue;
        };

        let t = Trial::test(name, move || do_compile_test(paths).map_err(Failed::from))
            .with_ignored_flag(ignored);
        trials.push(t);
    }

    Ok(trials)
}

#[tracing::instrument(skip_all)]
fn do_compile_test(paths: Vec<PathBuf>) -> color_eyre::Result<()> {
    let db = Compiler::default();

    let ws = read_all(&db, &paths)?;

    let diags = wit_compiler::diagnostics::check_all(&db, ws);

    if diags.is_empty() {
        return Ok(());
    }

    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    print_diagnostics(&mut stderr, &db, ws, &diags)?;
    color_eyre::eyre::bail!("One or more diagnostics were emitted");
}

#[tracing::instrument(skip(db))]
fn read_all(db: &dyn Db, paths: &[PathBuf]) -> color_eyre::Result<Workspace> {
    let mut files = OrdMap::new();

    for path in paths {
        let src = std::fs::read_to_string(path)?;
        let path = FilePath::new(db, Text::from(path.display().to_string()));
        files.insert(path, SourceFile::new(db, path, src.into()));
    }

    let ws = Workspace::new(db, files);
    Ok(ws)
}

fn print_diagnostics(
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
