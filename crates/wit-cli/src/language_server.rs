use std::net::SocketAddr;

use clap::Parser;
use color_eyre::eyre::Report;
use tower_lsp::Server;

#[derive(Debug, Clone, Parser)]
pub(crate) struct LanguageServer {
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

impl LanguageServer {
    #[tokio::main]
    pub async fn run(self) -> Result<(), Report> {
        let mode = self.mode().unwrap_or(Mode::Stdio);

        tracing::info!(
            lsp.name = env!("CARGO_PKG_NAME"),
            lsp.version = env!("CARGO_PKG_VERSION"),
            "Starting server",
        );

        serve(mode).await?;

        Ok(())
    }

    fn mode(&self) -> Option<Mode> {
        if let Some(addr) = self.connect {
            Some(Mode::Connect(addr))
        } else if let Some(addr) = self.listen {
            Some(Mode::Listen(addr))
        } else if self.stdio {
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
