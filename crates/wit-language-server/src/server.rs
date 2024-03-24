use tower_lsp::{
    jsonrpc::Error,
    lsp_types::{InitializeParams, InitializeResult, ServerCapabilities, ServerInfo},
    Client, ClientSocket, LspService,
};

/// The language server implementation.
#[derive(Debug)]
pub struct LanguageServer {
    _client: Client,
}

impl LanguageServer {
    pub(crate) fn new(client: Client) -> Self {
        LanguageServer { _client: client }
    }

    pub fn service() -> (LspService<LanguageServer>, ClientSocket) {
        LspService::build(LanguageServer::new)
            .custom_method("wit-language-server/changelog", LanguageServer::changelog)
            .finish()
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn changelog(&self) -> Result<&'static str, Error> {
        Ok(crate::CHANGELOG)
    }
}

#[tower_lsp::async_trait]
impl tower_lsp::LanguageServer for LanguageServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult, Error> {
        let client = params.client_info.as_ref();
        tracing::info!(
            client.name = client.map(|c| c.name.as_str()),
            client.version = client.and_then(|c| c.version.as_deref()),
            "Initializing"
        );

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn shutdown(&self) -> Result<(), Error> {
        tracing::info!("Shutting down");
        Ok(())
    }
}
