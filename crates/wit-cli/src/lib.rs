mod diags;
mod language_server;
mod parse;

pub(crate) use self::diags::print_diagnostics;

use std::fmt::Display;

use build_info::VersionControl;
use clap::{Parser, ValueEnum};
use color_eyre::eyre::Report;

use crate::{language_server::LanguageServer, parse::Parse};

#[derive(Debug, Clone, Parser)]
#[clap(about)]
pub struct Args {
    /// Print out the version info.
    #[clap(short = 'V', long, global = true)]
    version: bool,
    /// Print out extra information.
    #[clap(short, long, requires = "version", global = true)]
    verbose: bool,
    #[clap(subcommand)]
    cmd: Cmd,
}

impl Args {
    pub fn run(self) -> Result<(), Report> {
        if self.version {
            print_version(self.verbose);
            return Ok(());
        }

        match self.cmd {
            Cmd::LanguageServer(c) => c.run(),
            Cmd::Parse(p) => p.run(),
        }
    }
}

#[derive(Debug, Clone, Parser)]
enum Cmd {
    /// Start the language server.
    #[clap(alias = "serve")]
    LanguageServer(LanguageServer),
    /// Parse a file and dump its syntax tree.
    Parse(Parse),
}

fn print_version(verbose: bool) {
    build_info::build_info!(fn version);

    let version = version();

    print!(
        "{} {} (",
        version.crate_info.name, version.crate_info.version
    );
    if let Some(VersionControl::Git(git)) = &version.version_control {
        print!("{}", git.commit_short_id);
        if git.dirty {
            print!("-dirty");
        }
        print!(" ");
    }
    println!("{})", version.timestamp.date_naive());

    if verbose {
        println!("rustc: {}", version.compiler);
        if let Some(VersionControl::Git(git)) = &version.version_control {
            println!("commit-hash: {}", git.commit_id);
            println!("commit-date: {}", git.commit_timestamp.date_naive());
        }
        println!("host: {}", version.target.triple);
        println!("crate: {}", version.crate_info.name);
        println!("release: {}", version.crate_info.version);
        println!("authors: {}", version.crate_info.authors.join(", "));
    }
}

#[derive(Debug, Default, Copy, Clone, ValueEnum, PartialEq, Eq)]
pub enum Format {
    #[default]
    Text,
    Json,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Text => f.write_str("text"),
            Format::Json => f.write_str("json"),
        }
    }
}
