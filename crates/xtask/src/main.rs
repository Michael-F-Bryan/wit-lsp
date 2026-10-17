mod codegen;
mod doc;
mod utils;

use std::io::IsTerminal;

use clap::Parser;
use color_eyre::Report;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    EnvFilter, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::{codegen::Codegen, doc::Doc};

const RUST_LOG: &[&str] = &["warn"];

fn main() -> Result<(), Report> {
    initialize_logging()?;

    let cmd = Cmd::parse();

    match cmd {
        Cmd::Codegen(c) => c.run(),
        Cmd::Doc(d) => d.run(),
    }
}

#[derive(Parser, Debug)]
#[command(author, version)]
enum Cmd {
    /// Run code generation.
    Codegen(Codegen),
    /// Generate project documentation.
    Doc(Doc),
}

fn initialize_logging() -> color_eyre::Result<()> {
    let isatty = std::io::stderr().is_terminal();

    let filter = if std::env::var_os("RUST_LOG").is_some() {
        EnvFilter::from_default_env()
    } else {
        EnvFilter::new(RUST_LOG.join(","))
    };

    color_eyre::config::HookBuilder::default()
        .capture_span_trace_by_default(true)
        .add_default_filters()
        .install()?;

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_ansi(isatty)
        .with_span_events(FmtSpan::CLOSE)
        .finish()
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
