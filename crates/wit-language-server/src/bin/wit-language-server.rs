#![recursion_limit = "256"]

use std::{io::IsTerminal, net::SocketAddr};

use build_info::VersionControl;
use clap::{CommandFactory, Parser};
use color_eyre::config::Theme;
use tower_lsp::Server;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

const RUST_LOG: &[&str] = &["warn", "wit_language_server=info", "wit_compiler=info"];

#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    initialize_logging()?;

    let args = Args::parse();

    if args.version {
        print_version(args.verbose);
        return Ok(());
    }

    let Some(mode) = args.mode() else {
        Args::command().print_long_help()?;
        return Ok(());
    };

    tracing::info!(
        lsp.name = env!("CARGO_PKG_NAME"),
        lsp.version = env!("CARGO_PKG_VERSION"),
        "Starting",
    );

    serve(mode).await
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

async fn serve(mode: Mode) -> Result<(), color_eyre::Report> {
    let (service, socket) = wit_language_server::service();

    match mode {
        Mode::Connect(addr) => {
            // The client has opened a socket and wants us to connect to them. This
            // is pretty typical for clients based on vscode-langaugeclient.
            tracing::debug!(%addr, "Connecting to client");
            let mut stream = tokio::net::TcpStream::connect(addr).await?;
            let (reader, writer) = stream.split();
            Server::new(reader, writer, socket).serve(service).await;
        }
        Mode::Listen(addr) => {
            // Looks like we need to start a server and the client will connect to
            // us
            let listener = tokio::net::TcpListener::bind(addr).await?;
            let local_addr = listener.local_addr()?;

            tracing::info!(addr=%local_addr, "Waiting for connections");
            let (mut stream, client_addr) = listener.accept().await?;
            tracing::debug!(client=%client_addr, "Client connected");

            let (reader, writer) = stream.split();
            Server::new(reader, writer, socket).serve(service).await;
        }
        Mode::Stdio => {
            tracing::debug!("Communicating via stdin/stdout");
            let stdin = tokio::io::stdin();
            let stdout = tokio::io::stdout();
            Server::new(stdin, stdout, socket).serve(service).await;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Parser)]
#[clap(about)]
struct Args {
    #[clap(flatten)]
    mode: ModeArgs,
    /// Print out the version info.
    #[clap(short = 'V', long)]
    version: bool,
    /// Print out extra
    #[clap(short, long, requires = "version", conflicts_with = "mode")]
    verbose: bool,
}

impl Args {
    fn mode(&self) -> Option<Mode> {
        if let Some(addr) = self.mode.connect {
            Some(Mode::Connect(addr))
        } else if let Some(addr) = self.mode.listen {
            Some(Mode::Listen(addr))
        } else if self.mode.stdio {
            Some(Mode::Stdio)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Mode {
    Connect(SocketAddr),
    Listen(SocketAddr),
    Stdio,
}

#[derive(Debug, Clone, Parser)]
struct ModeArgs {
    /// Connect to a port that the client is serving on.
    #[clap(short, long, env, group = "mode")]
    connect: Option<SocketAddr>,
    /// Start listening on an address and wait for the client to connect.
    #[clap(short, long, env, group = "mode")]
    listen: Option<SocketAddr>,
    /// Serve on stdin and stdout.
    #[clap(long, env, group = "mode")]
    stdio: bool,
}

fn initialize_logging() -> Result<(), color_eyre::Report> {
    let isatty = std::io::stderr().is_terminal();

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
