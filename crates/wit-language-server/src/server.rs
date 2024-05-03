use im::{OrdMap, Vector};
use salsa::ParallelDatabase;
use tokio::sync::{Mutex, MutexGuard};
use tower_lsp::{
    jsonrpc::Error,
    lsp_types::{
        CompletionOptions, CompletionParams, CompletionResponse, DiagnosticOptions,
        DiagnosticServerCapabilities, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
        DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentDiagnosticParams,
        DocumentDiagnosticReportResult, FoldingRange, FoldingRangeParams, GotoDefinitionParams,
        GotoDefinitionResponse, InitializeParams, InitializeResult, OneOf, SelectionRange,
        SelectionRangeParams, ServerCapabilities, ServerInfo, TextDocumentContentChangeEvent,
        TextDocumentItem, TextDocumentSyncCapability, TextDocumentSyncKind, Url,
    },
    Client, ClientSocket, LspService,
};
use wit_compiler::{queries::Workspace, Text};

use crate::{utils, Database};

/// The language server implementation.
#[derive(Debug)]
pub struct LanguageServer {
    _client: Client,
    state: Mutex<State>,
}

impl LanguageServer {
    pub(crate) fn new(client: Client) -> Self {
        LanguageServer {
            _client: client,
            state: Mutex::default(),
        }
    }

    pub(crate) fn service() -> (LspService<LanguageServer>, ClientSocket) {
        LspService::build(LanguageServer::new)
            .custom_method("wit-language-server/changelog", LanguageServer::changelog)
            .custom_method("wit-language-server/dump-ast", LanguageServer::dump_ast)
            .finish()
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn dump_ast(&self, params: DumpAstParams) -> Result<DumpAstResponse, Error> {
        let path = Text::from(params.uri.as_str());

        let snap = self.snapshot().await;

        let ws = snap.ws;
        let db = snap.wit_db();

        let Some(file) = snap.ws.lookup_by_path(db, &path) else {
            let msg = format!("\"{path}\" isn't in the workspace");
            tracing::warn!(%path, "File not found in database");
            tracing::debug!(
                tracked_files = ?ws.files(db).keys().collect::<Vec<_>>(),
            );
            return Err(Error::invalid_params(msg));
        };

        let ast = wit_compiler::queries::parse(db, file);

        Ok(DumpAstResponse {
            ast: ast.tree(db).to_string(),
        })
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn changelog(&self) -> Result<&'static str, Error> {
        Ok(crate::CHANGELOG)
    }

    async fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().await
    }

    async fn snapshot(&self) -> Snapshot {
        self.state.lock().await.snapshot()
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

        if let Some(_folders) = params.workspace_folders {
            // FIXME: Should we try to preload all *.wit files?
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                folding_range_provider: Some(true.into()),
                selection_range_provider: Some(true.into()),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        inter_file_dependencies: true,
                        ..Default::default()
                    },
                )),
                definition_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions::default()),
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
        let mut state = self.lock().await;

        tracing::debug!(size = text.len(), path, "File opened");
        let ws = state.ws;
        ws.update(state.wit_db(), path, text);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        tracing::debug!(document.uri=%params.text_document.uri);
        let path = params.text_document.uri.as_str();

        let mut state = self.lock().await;

        debug_assert!(
            params.content_changes.len() < 2,
            "TODO: figure out how to handle bulk changes"
        );

        for change in params.content_changes {
            let TextDocumentContentChangeEvent { text, range, .. } = change;

            if let Some(range) = range {
                tracing::warn!(?range, "Incremental document updating isn't supported");
                return;
            }

            tracing::debug!(size = text.len(), path, "File changed");
            let ws = state.ws;
            ws.update(state.wit_db(), path, text);
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        tracing::debug!(document.uri=%params.text_document.uri);

        if let Some(text) = params.text {
            let path = params.text_document.uri.as_str();

            let mut state = self.lock().await;

            tracing::debug!(size = text.len(), path, "File saved");
            let ws = state.ws;
            ws.update(state.wit_db(), path, text);
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
        let snap = self.snapshot().await;
        let db = snap.wit_db();

        let Some(file) = snap.ws.lookup_by_path(db, path) else {
            return Ok(None);
        };

        let ast = wit_compiler::queries::parse(db, file);

        let ranges = crate::ops::folding_range(&*snap.db, ast)
            .into_iter()
            .collect();

        Ok(Some(ranges))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn selection_range(
        &self,
        params: SelectionRangeParams,
    ) -> Result<Option<Vec<SelectionRange>>, Error> {
        tracing::debug!(document.uri=%params.text_document.uri);

        let path = params.text_document.uri.as_str();
        let snap = self.snapshot().await;

        let db = snap.wit_db();
        let Some(file) = snap.ws.lookup_by_path(db, path) else {
            return Ok(None);
        };

        let ast = wit_compiler::queries::parse(db, file);

        let mut ranges = Vec::new();

        for position in params.positions {
            let point = tree_sitter::Point {
                row: position.line.try_into().unwrap(),
                column: position.character.try_into().unwrap(),
            };

            match wit_compiler::queries::selection_ranges(db, ast, point) {
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

        Ok(Some(ranges))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult, Error> {
        tracing::debug!(document.uri=%params.text_document.uri);

        let snap = self.snapshot().await;
        let db = snap.wit_db();
        let path = &params.text_document.uri;
        let Some(file) = snap.ws.lookup_by_path(db, path.as_str()) else {
            return Err(Error::invalid_params(format!(
                "\"{path}\" isn't tracked by the workspace"
            )));
        };

        Ok(crate::ops::file_diagnostics(db, snap.ws, file))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>, Error> {
        let path = &params.text_document_position.text_document.uri;
        tracing::debug!(document.uri=%path);
        let snap = self.snapshot().await;

        let Some(file) = snap.ws.lookup_by_path(snap.wit_db(), path.as_str()) else {
            return Ok(None);
        };

        let point = crate::utils::position_to_ts(params.text_document_position.position);
        let completions = crate::ops::complete(snap.wit_db(), snap.ws, file, point)
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(Some(CompletionResponse::Array(completions)))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>, Error> {
        let path = &params.text_document_position_params.text_document.uri;
        tracing::debug!(document.uri=%path);
        let snap = self.snapshot().await;

        let Some(_file) = snap.ws.lookup_by_path(snap.wit_db(), path.as_str()) else {
            return Ok(None);
        };

        // TODO: Finish wiring this up
        Err(Error::method_not_found())
    }
}

fn selection_range(first: tree_sitter::Range, rest: Vector<tree_sitter::Range>) -> SelectionRange {
    let mut parent = None;

    for range in rest.iter().rev() {
        let sel = SelectionRange {
            range: utils::ts_to_range(*range),
            parent: parent.take(),
        };
        parent = Some(Box::new(sel));
    }

    SelectionRange {
        range: utils::ts_to_range(first),
        parent,
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

/// The language server's internal state.
#[derive(Debug)]
struct State {
    ws: Workspace,
    db: Database,
}

impl State {
    fn db(&mut self) -> &mut dyn crate::Db {
        &mut self.db
    }

    fn wit_db(&mut self) -> &mut dyn wit_compiler::Db {
        self.db().as_wit_mut()
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            db: self.db.snapshot(),
            ws: self.ws,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let db = Database::default();
        let workspace = Workspace::new(&db, OrdMap::new());
        Self { ws: workspace, db }
    }
}

/// A readonly snapshot of the [`State`].
///
/// This is often used when you want to do something that doesn't require update
/// inputs. By using a [`Snapshot`] instead of holding onto the [`State`]
/// directly you allow other readonly queries to run in parallel.
#[derive(Debug)]
struct Snapshot {
    ws: Workspace,
    db: salsa::Snapshot<Database>,
}

impl Snapshot {
    fn db(&self) -> &dyn crate::Db {
        &*self.db
    }

    fn wit_db(&self) -> &dyn wit_compiler::Db {
        self.db().as_wit()
    }
}
