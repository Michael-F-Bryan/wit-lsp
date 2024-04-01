use std::{future::Future, io::IsTerminal, net::SocketAddr, pin::Pin};

use clap::Parser;
use color_eyre::config::Theme;
use tower_lsp::Server;
use tower_service::Service;
use tracing::Instrument;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};
use uuid::Uuid;
use wit_language_server::LanguageServer;

const RUST_LOG: &[&str] = &["info", "wit_language_server=trace"];

#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    initialize_logging()?;

    let args = Args::parse();

    tracing::info!(
        lsp.name = env!("CARGO_PKG_NAME"),
        lsp.version = env!("CARGO_PKG_VERSION"),
        "Starting",
    );

    serve(args.mode()).await
}

async fn serve(mode: Mode) -> Result<(), color_eyre::Report> {
    let (service, socket) = LanguageServer::service();
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
struct Args {
    #[clap(flatten)]
    mode: ModeArgs,
}

impl Args {
    fn mode(&self) -> Mode {
        if let Some(addr) = self.mode.connect {
            Mode::Connect(addr)
        } else if let Some(addr) = self.mode.listen {
            Mode::Listen(addr)
        } else if self.mode.stdio {
            Mode::Stdio
        } else {
            unreachable!()
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
#[group(required = true)]
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
                    tracing::warn!(
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
