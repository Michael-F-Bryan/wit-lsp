mod codegen;
mod doc;
mod utils;

use std::io::IsTerminal;

use clap::Parser;
use color_eyre::{config::Theme, Report};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
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

fn initialize_logging() -> Result<(), color_eyre::Report> {
    let isatty = std::io::stderr().is_terminal();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", RUST_LOG.join(","));
    }

    let theme = if isatty {
        Theme::dark()
    } else {
        Theme::default()
    };

    color_eyre::config::HookBuilder::default()
        .capture_span_trace_by_default(true)
        .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))
        .add_default_filters()
        .add_issue_metadata("arch", std::env::consts::ARCH)
        .add_issue_metadata("os", std::env::consts::OS)
        .add_issue_metadata("package", env!("CARGO_PKG_NAME"))
        .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
        .theme(theme)
        .install()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(isatty)
        .with_span_events(FmtSpan::CLOSE)
        .finish()
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
