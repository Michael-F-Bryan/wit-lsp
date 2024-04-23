#![recursion_limit = "256"]

use std::{
    any::Any, future::Future, io::IsTerminal, net::SocketAddr, panic::AssertUnwindSafe, pin::Pin,
};

use build_info::VersionControl;
use clap::{CommandFactory, Parser};
use color_eyre::config::Theme;
use futures::future::FutureExt;
use tower_lsp::Server;
use tower_service::Service;
use tracing::Instrument;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};
use uuid::Uuid;
use wit_language_server::LanguageServer;

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
    let (service, socket) = LanguageServer::service();
    let service = CatchPanic(service);
    let service = LoggingService(service);

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

#[derive(Debug, Clone)]
struct LoggingService<S>(S);

impl<S> Service<tower_lsp::jsonrpc::Request> for LoggingService<S>
where
    S: Service<tower_lsp::jsonrpc::Request, Response = Option<tower_lsp::jsonrpc::Response>>,
    S::Error: std::error::Error + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: tower_lsp::jsonrpc::Request) -> Self::Future {
        let id = match req.id() {
            Some(tower_lsp::jsonrpc::Id::Number(n)) => n.to_string(),
            Some(tower_lsp::jsonrpc::Id::String(s)) => s.clone(),
            None | Some(tower_lsp::jsonrpc::Id::Null) => Uuid::new_v4().to_string(),
        };
        let method = req.method().to_string();

        let fut = self.0.call(req);
        let fut = async move {
            let ret = fut.await;

            match ret.as_ref() {
                Ok(r) => {
                    if let Some(err) = r.as_ref().and_then(|r| r.error()) {
                        tracing::debug!(error = err as &dyn std::error::Error, "Returned an error",);
                    }
                }
                Err(err) => {
                    tracing::error!(
                        error = err as &dyn std::error::Error,
                        "An error occurred while handling the request",
                    )
                }
            }

            ret
        };

        Box::pin(fut.instrument(tracing::debug_span!("request", %id, %method)))
    }
}

#[derive(Debug, Clone)]
struct CatchPanic<S>(S);

impl<S> Service<tower_lsp::jsonrpc::Request> for CatchPanic<S>
where
    S: Service<
        tower_lsp::jsonrpc::Request,
        Response = Option<tower_lsp::jsonrpc::Response>,
        Error = tower_lsp::ExitedError,
    >,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = CatchPanicError;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx).map_err(CatchPanicError::Exited)
    }

    fn call(&mut self, req: tower_lsp::jsonrpc::Request) -> Self::Future {
        let fut = AssertUnwindSafe(self.0.call(req)).catch_unwind();

        Box::pin(async move {
            match fut.await {
                Ok(result) => result.map_err(CatchPanicError::Exited),
                Err(payload) => Err(CatchPanicError::Panic(PanicMessage::new(payload))),
            }
        })
    }
}

#[derive(Debug, Clone)]
enum CatchPanicError {
    Panic(PanicMessage),
    Exited(tower_lsp::ExitedError),
}

impl std::error::Error for CatchPanicError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CatchPanicError::Panic(p) => Some(p),
            CatchPanicError::Exited(e) => Some(e),
        }
    }
}

impl std::fmt::Display for CatchPanicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatchPanicError::Panic(p) => write!(f, "{p}"),
            CatchPanicError::Exited(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug, Clone)]
struct PanicMessage {
    msg: Option<String>,
}

impl PanicMessage {
    fn new(payload: Box<dyn Any + Send>) -> Self {
        let msg = if let Some(msg) = payload.downcast_ref::<String>() {
            Some(msg.as_str())
        } else if let Some(&msg) = payload.downcast_ref::<&str>() {
            Some(msg)
        } else {
            None
        };

        PanicMessage {
            msg: msg.map(String::from),
        }
    }
}

impl std::error::Error for PanicMessage {}

impl std::fmt::Display for PanicMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = self.msg.as_deref().unwrap_or("<unknown>");
        write!(f, "A panic occurred while handling the request: {msg}")
    }
}
