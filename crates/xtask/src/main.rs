mod codegen;
mod utils;

use clap::Parser;
use color_eyre::Report;

use crate::codegen::Codegen;

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let cmd = Cmd::parse();

    match cmd {
        Cmd::Codegen(c) => c.run(),
    }
}

#[derive(Parser, Debug)]
#[command(author, version)]
enum Cmd {
    /// Run code generation.
    Codegen(Codegen),
}
