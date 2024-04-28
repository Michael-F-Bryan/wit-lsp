use std::{io::IsTerminal, path::PathBuf};

use clap::Parser;
use libtest_mimic::Arguments;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

const RUST_LOG: &[&str] = &["info", "wit_compiler=info", "salsa_2022=warn"];

#[derive(Debug, Parser)]
struct Args {
    #[clap(long, env, default_value = env!("CARGO_MANIFEST_DIR"))]
    base_dir: PathBuf,
    #[clap(flatten)]
    inner: Arguments,
}

fn main() -> color_eyre::Result<()> {
    initialize_logging()?;

    let Args { base_dir, inner } = Args::parse();

    let cases = integration_tests::discover(base_dir)?;
    libtest_mimic::run(&inner, cases).exit_if_failed();

    Ok(())
}

fn initialize_logging() -> color_eyre::Result<()> {
    let isatty = std::io::stderr().is_terminal();

    color_eyre::config::HookBuilder::default()
        .capture_span_trace_by_default(true)
        .add_default_filters()
        .install()?;

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", RUST_LOG.join(","));
    }

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
