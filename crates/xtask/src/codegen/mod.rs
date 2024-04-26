mod ast;
mod diagnostics;

use clap::Parser;

use crate::codegen::{ast::Ast, diagnostics::Diagnostics};

#[derive(Debug, Clone, Parser)]
#[clap(subcommand_value_name = "TARGET", subcommand_help_heading = "Targets")]
pub struct Codegen {
    #[clap(subcommand)]
    target: Option<Target>,
}

impl Codegen {
    #[tracing::instrument(skip_all)]
    pub fn run(self) -> color_eyre::Result<()> {
        let Codegen { target } = self;

        match target {
            Some(target) => target.generate(),
            None => run_all_generators(),
        }
    }
}

/// Run all code generators using the default settings.
#[tracing::instrument(skip_all)]
fn run_all_generators() -> color_eyre::Result<()> {
    Ast::default().generate()?;
    Diagnostics::default().generate()?;

    Ok(())
}

#[derive(Debug, Clone, Parser)]
enum Target {
    /// Generate strongly-typed AST nodes.
    Ast(Ast),
    Diagnostics(Diagnostics),
}

impl Target {
    fn generate(self) -> color_eyre::Result<()> {
        match self {
            Target::Ast(a) => a.generate(),
            Target::Diagnostics(d) => d.generate(),
        }
    }
}
