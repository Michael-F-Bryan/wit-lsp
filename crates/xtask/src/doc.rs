use std::path::PathBuf;

use clap::Parser;
use once_cell::sync::Lazy;
use xshell::{cmd, Shell};

use crate::utils;

static DEFAULT_OUTPUT_DIR: Lazy<PathBuf> = Lazy::new(|| utils::project_root().join("public"));

#[derive(Debug, Clone, Parser)]
pub struct Doc {
    /// Where to save the generated docs to.
    #[clap(short, long, default_value = DEFAULT_OUTPUT_DIR.as_os_str())]
    out: PathBuf,
}

impl Doc {
    #[tracing::instrument(skip_all)]
    pub fn run(self) -> color_eyre::Result<()> {
        let sh = Shell::new()?;
        let project_root = utils::project_root();
        sh.change_dir(project_root);

        let _ = std::fs::remove_dir_all(&self.out);
        sh.create_dir(&self.out)?;
        sh.change_dir(&self.out);

        tracing::info!("API docs");
        api_docs(&sh)?;
        let docs_dir = project_root.join("target").join("doc");
        utils::copy_dir(&sh, docs_dir, "crate-docs")?;

        tracing::info!("Code coverage");
        code_coverage(&sh)?;
        let coverage_dir = project_root.join("target").join("llvm-cov").join("html");
        utils::copy_dir(&sh, coverage_dir, "coverage")?;

        tracing::info!("Creating redirects");
        sh.write_file(
            "index.html",
            r#"<meta http-equiv="refresh" content="0; url=crate-docs/index.html" />"#,
        )?;
        sh.write_file(
            "crate-docs/index.html",
            r#"<meta http-equiv="refresh" content="0; url=wit_compiler/index.html" />"#,
        )?;

        Ok(())
    }
}

#[tracing::instrument(skip_all)]
fn api_docs(sh: &Shell) -> color_eyre::Result<()> {
    cmd!(sh, "cargo doc --workspace --verbose --locked").run()?;
    Ok(())
}

#[tracing::instrument(skip_all)]
fn code_coverage(sh: &Shell) -> color_eyre::Result<()> {
    cmd!(
        sh,
        "cargo llvm-cov nextest --html --workspace --no-fail-fast --show-instantiations"
    )
    .run()?;

    Ok(())
}
