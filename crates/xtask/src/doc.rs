use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use minijinja::Environment;
use once_cell::sync::Lazy;
use xshell::{cmd, Shell};

use crate::utils;

static DEFAULT_OUTPUT_DIR: Lazy<PathBuf> = Lazy::new(|| utils::project_root().join("public"));

#[derive(Debug, Clone, Parser)]
pub struct Doc {
    /// Where to save the generated docs to.
    #[clap(short, long, default_value = DEFAULT_OUTPUT_DIR.as_os_str())]
    out: PathBuf,
    /// The item to generate docs for.
    targets: Vec<Target>,
}

impl Doc {
    #[tracing::instrument(skip_all)]
    pub fn run(self) -> color_eyre::Result<()> {
        let sh = Shell::new()?;
        let project_root = utils::project_root();
        sh.change_dir(project_root);

        let targets = if !self.targets.is_empty() {
            &self.targets[..]
        } else {
            Target::value_variants()
        };

        let _ = std::fs::remove_dir_all(&self.out);
        sh.create_dir(&self.out)?;
        sh.change_dir(&self.out);

        for target in targets {
            target.execute(&sh, project_root)?;
        }

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

fn diagnostics_index() -> color_eyre::Result<String> {
    let template = include_str!("diagnostics.html.j2");
    let diagnostics_json = include_str!("../../wit-compiler/src/diagnostics/diagnostics.json");
    let mut diags: Vec<DiagnosticInfo> = serde_json::from_str(diagnostics_json)?;

    for diag in &mut diags {
        // Note: all well-formed diagnostic descriptions start with a heading
        // like "# E005: Some diagnostic", but we want to render it seperately
        let (_heading, rest) = diag.description.split_once('\n').unwrap();

        let parser = pulldown_cmark::Parser::new_ext(rest, pulldown_cmark::Options::all());
        let mut rendered = String::new();
        pulldown_cmark::html::push_html(&mut rendered, parser);
        diag.description = rendered;
    }

    diags.sort_by_key(|d| d.error_code.clone());

    let ctx = serde_json::json!({
        "diagnostics": diags,
    });

    let rendered = Environment::new().render_str(template, ctx)?;

    Ok(rendered)
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct DiagnosticInfo {
    type_name: String,
    message: String,
    severity: String,
    error_code: String,
    description: String,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum Target {
    Api,
    Coverage,
    Diagnostics,
    Redirects,
}

impl Target {
    fn execute(self, sh: &Shell, project_root: &Path) -> color_eyre::Result<()> {
        match self {
            Target::Api => {
                tracing::info!("API docs");
                api_docs(sh)?;
                let docs_dir = project_root.join("target").join("doc");
                utils::copy_dir(sh, docs_dir, "crate-docs")?;
            }
            Target::Coverage => {
                tracing::info!("Code coverage");
                code_coverage(sh)?;
                let coverage_dir = project_root.join("target").join("llvm-cov").join("html");
                utils::copy_dir(sh, coverage_dir, "coverage")?;
            }
            Target::Diagnostics => {
                tracing::info!("Diagnostics Index");
                let diagnostics = diagnostics_index()?;
                sh.write_file("diagnostics.html", diagnostics)?;
            }
            Target::Redirects => {
                tracing::info!("Creating redirects");
                sh.write_file(
                    "index.html",
                    r#"<meta http-equiv="refresh" content="0; url=crate-docs/index.html" />"#,
                )?;
                sh.write_file(
                    "crate-docs/index.html",
                    r#"<meta http-equiv="refresh" content="0; url=wit_compiler/index.html" />"#,
                )?;
            }
        }

        Ok(())
    }
}
