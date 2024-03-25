use std::sync::Arc;

use im::{OrdMap, Vector};
use tokio::sync::Mutex;
use tower_lsp::{
    jsonrpc::Error,
    lsp_types::{
        DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
        DidSaveTextDocumentParams, FoldingRange, FoldingRangeParams, InitializeParams,
        InitializeResult, SelectionRange, SelectionRangeParams, ServerCapabilities, ServerInfo,
        TextDocumentContentChangeEvent, TextDocumentItem, TextDocumentSyncCapability,
        TextDocumentSyncKind, Url,
    },
    Client, ClientSocket, LspService,
};
use wit_compiler::{queries::Workspace, Text};

use crate::{Database, Db};

/// The language server implementation.
#[derive(Debug)]
pub struct LanguageServer {
    _client: Client,
    db: Arc<Mutex<Database>>,
    workspaces: Mutex<OrdMap<Text, Workspace>>,
}

impl LanguageServer {
    pub(crate) fn new(client: Client) -> Self {
        LanguageServer {
            _client: client,
            db: Arc::default(),
            workspaces: Mutex::default(),
        }
    }

    pub fn service() -> (LspService<LanguageServer>, ClientSocket) {
        LspService::build(LanguageServer::new)
            .custom_method("wit-language-server/changelog", LanguageServer::changelog)
            .custom_method("wit-language-server/dump-ast", LanguageServer::dump_ast)
            .finish()
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn dump_ast(&self, params: DumpAstParams) -> Result<DumpAstResponse, Error> {
        let path = Text::from(params.uri.as_str());

        let ws = self.workspace_for_path(&path).await.ok_or_else(|| {
            Error::invalid_params(format!("\"{path}\" doesn't belong to a known workspace"))
        })?;

        let db = self.db.lock().await;

        match wit_compiler::queries::parse(&*db, ws, path.clone()) {
            Some(ast) => Ok(DumpAstResponse {
                ast: ast.tree(&*db).to_string(),
            }),
            None => {
                let ws_root = ws.root(&*db);
                let msg = format!("\"{path}\" doesn't belong to the \"{ws_root}\" workspace");
                tracing::warn!(
                    workspace = ?ws.debug(&*db),
                    %path,
                    "File not foind in database",
                );
                Err(Error::invalid_params(msg))
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn changelog(&self) -> Result<&'static str, Error> {
        Ok(crate::CHANGELOG)
    }

    async fn workspace_for_path(&self, path: &str) -> Option<Workspace> {
        let workspaces = self.workspaces.lock().await;

        for (name, ws) in &*workspaces {
            if path.starts_with(name.as_str()) {
                return Some(*ws);
            }
        }

        None
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
        tracing::trace!(?params, "Initialization parameters");

        if let Some(folders) = params.workspace_folders {
            let db = self.db.lock().await;

            let mut workspaces = self.workspaces.lock().await;
            workspaces.clear();

            for folder in folders {
                let workspace = Workspace::new(&*db, folder.uri.as_str().into(), OrdMap::new());
                tracing::debug!(
                    workspace = ?workspace.debug(&*db),
                    "Loaded workspace",
                );

                // TODO: Pre-emptively load all files in the workspace

                workspaces.insert(folder.uri.as_str().into(), workspace);
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                folding_range_provider: Some(true.into()),
                selection_range_provider: Some(true.into()),
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

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        tracing::debug!(document.uri=%params.text_document.uri);

        let TextDocumentItem { uri, text, .. } = params.text_document;
        let path = uri.as_str();
        let mut db = self.db.lock().await;

        if let Some(ws) = self.workspace_for_path(path).await {
            tracing::debug!(size = text.len(), path, "File opened");
            ws.update(&mut *db, path, text);
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        tracing::debug!(document.uri=%params.text_document.uri);
        let path = params.text_document.uri.as_str();

        let Some(ws) = self.workspace_for_path(path).await else {
            return;
        };

        debug_assert!(
            params.content_changes.len() < 2,
            "TODO: figure out how to handle bulk changes"
        );
        let mut db = self.db.lock().await;

        for change in params.content_changes {
            let TextDocumentContentChangeEvent { text, range, .. } = change;

            if let Some(range) = range {
                tracing::warn!(?range, "Incremental document updating isn't supported");
                return;
            }

            tracing::debug!(size = text.len(), path, "File changed");
            ws.update(&mut *db, path, text);
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        tracing::debug!(document.uri=%params.text_document.uri);

        if let Some(text) = params.text {
            let path = params.text_document.uri.as_str();

            if let Some(ws) = self.workspace_for_path(path).await {
                let mut db = self.db.lock().await;

                tracing::debug!(size = text.len(), path, "File saved");
                ws.update(&mut *db, path, text);
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        tracing::debug!(document.uri=%params.text_document.uri);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn folding_range(
        &self,
        params: FoldingRangeParams,
    ) -> Result<Option<Vec<FoldingRange>>, Error> {
        tracing::debug!(document.uri=%params.text_document.uri);

        let path = params.text_document.uri.as_str();
        let db = self.db.lock().await;

        match self.workspace_for_path(path).await {
            Some(ws) => Ok(wit_compiler::queries::parse(db.as_wit(), ws, path.into())
                .map(|ast| crate::ops::folding_range(&*db, ast).into_iter().collect())),
            _ => {
                tracing::warn!(%path, "Workspace not found for file");
                Ok(None)
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn selection_range(
        &self,
        params: SelectionRangeParams,
    ) -> Result<Option<Vec<SelectionRange>>, Error> {
        tracing::debug!(document.uri=%params.text_document.uri);

        let path = params.text_document.uri.as_str();
        let db = self.db.lock().await;

        let Some(ws) = self.workspace_for_path(path).await else {
            return Ok(None);
        };
        let Some(ast) = wit_compiler::queries::parse(&*db, ws, path.into()) else {
            return Ok(None);
        };

        let mut ranges = Vec::new();

        for position in params.positions {
            let point = tree_sitter::Point {
                row: position.line.try_into().unwrap(),
                column: position.character.try_into().unwrap(),
            };

            match wit_compiler::queries::selection_ranges(db.as_wit(), ast, point) {
                Some(mut r) => {
                    let first = r
                        .pop_front()
                        .expect("Should always return at least one range");
                    ranges.push(selection_range(first, r));
                }
                None => {
                    return Err(Error::invalid_params(format!(
                        "The position, {point}, doesn't exist in \"{path}\""
                    )));
                }
            }
        }

        tracing::warn!(?ranges);

        Ok(Some(ranges))
    }
}

fn selection_range(first: tree_sitter::Range, rest: Vector<tree_sitter::Range>) -> SelectionRange {
    let mut parent = None;

    for range in rest.iter().rev() {
        let sel = SelectionRange {
            range: ts_to_range(*range),
            parent: parent.take(),
        };
        parent = Some(Box::new(sel));
    }

    SelectionRange {
        range: ts_to_range(first),
        parent,
    }
}

fn ts_to_range(range: tree_sitter::Range) -> tower_lsp::lsp_types::Range {
    let tree_sitter::Range {
        start_point,
        end_point,
        ..
    } = range;

    tower_lsp::lsp_types::Range {
        start: ts_to_position(start_point),
        end: ts_to_position(end_point),
    }
}

fn ts_to_position(point: tree_sitter::Point) -> tower_lsp::lsp_types::Position {
    let tree_sitter::Point { row, column } = point;
    tower_lsp::lsp_types::Position {
        line: row.try_into().unwrap(),
        character: column.try_into().unwrap(),
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct DumpAstParams {
    uri: Url,
}

#[derive(Debug, Clone, serde::Serialize)]
struct DumpAstResponse {
    ast: String,
}
