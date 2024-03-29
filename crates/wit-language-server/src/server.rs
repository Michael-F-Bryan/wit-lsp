use im::{OrdMap, Vector};
use salsa::ParallelDatabase;
use tokio::sync::{Mutex, MutexGuard};
use tower_lsp::{
    jsonrpc::Error,
    lsp_types::{
        DiagnosticOptions, DiagnosticRelatedInformation, DiagnosticServerCapabilities,
        DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
        DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentDiagnosticParams,
        DocumentDiagnosticReportResult, FoldingRange, FoldingRangeParams, InitializeParams,
        InitializeResult, Location, SelectionRange, SelectionRangeParams, ServerCapabilities,
        ServerInfo, TextDocumentContentChangeEvent, TextDocumentItem, TextDocumentSyncCapability,
        TextDocumentSyncKind, Url,
    },
    Client, ClientSocket, LspService,
};
use wit_compiler::{
    diagnostics::{Diagnostics, DuplicateName, SyntaxError, Unimplemented, UnknownName},
    queries::Workspace,
    Text,
};

use crate::Database;

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

    pub fn service() -> (LspService<LanguageServer>, ClientSocket) {
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

        let Some(file) = snap.ws.lookup(db, &path) else {
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
                        // TODO: Enable this when we can generate diagnostics
                        // for the entire workspace.
                        inter_file_dependencies: false,
                        ..Default::default()
                    },
                )),
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

        let Some(file) = snap.ws.lookup(db, path) else {
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
        let Some(file) = snap.ws.lookup(db, path) else {
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

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult, Error> {
        let snap = self.snapshot().await;
        let db = snap.wit_db();
        let path = &params.text_document.uri;
        let Some(file) = snap.ws.lookup(db, path.as_str()) else {
            return Err(Error::invalid_params(format!(
                "\"{path}\" isn't tracked by the workspace"
            )));
        };

        let diags = wit_compiler::queries::parse::accumulated::<Diagnostics>(db, file);
        let items = diags
            .into_iter()
            .filter_map(|diag| lsp_diagnostic(diag, path))
            .collect();

        Ok(DocumentDiagnosticReportResult::Report(
            tower_lsp::lsp_types::DocumentDiagnosticReport::Full(
                tower_lsp::lsp_types::RelatedFullDocumentDiagnosticReport {
                    full_document_diagnostic_report:
                        tower_lsp::lsp_types::FullDocumentDiagnosticReport {
                            items,
                            ..Default::default()
                        },
                    ..Default::default()
                },
            ),
        ))
    }
}

/// Convert a [`wit_compiler::diagnostics::Diagnostic`] to a
/// [`tower_lsp::lsp_types::Diagnostic`].
fn lsp_diagnostic(
    diag: wit_compiler::diagnostics::Diagnostic,
    uri: &Url,
) -> Option<tower_lsp::lsp_types::Diagnostic> {
    match diag {
        wit_compiler::diagnostics::Diagnostic::DuplicateName(DuplicateName {
            name,
            duplicate_definition,
            original_definition,
        }) => {
            let diagnostic = tower_lsp::lsp_types::Diagnostic {
                range: ts_to_range(duplicate_definition),
                message: format!("\"{name}\" is already defined"),
                related_information: Some(vec![DiagnosticRelatedInformation {
                    location: Location {
                        uri: uri.clone(),
                        range: ts_to_range(original_definition),
                    },
                    message: "Original definition".into(),
                }]),
                severity: Some(DiagnosticSeverity::ERROR),
                ..Default::default()
            };
            Some(diagnostic)
        }
        wit_compiler::diagnostics::Diagnostic::Parse(SyntaxError { range, rule }) => {
            let msg = format!("Syntax error while parsing \"{rule}\"");
            let diagnostic = tower_lsp::lsp_types::Diagnostic {
                range: ts_to_range(range),
                message: msg,
                severity: Some(DiagnosticSeverity::ERROR),
                ..Default::default()
            };
            Some(diagnostic)
        }
        wit_compiler::diagnostics::Diagnostic::Unimplemented(Unimplemented {
            message,
            range,
            ..
        }) => {
            let diagnostic = tower_lsp::lsp_types::Diagnostic {
                range: ts_to_range(range),
                message: message.to_string(),
                severity: Some(DiagnosticSeverity::ERROR),
                ..Default::default()
            };
            Some(diagnostic)
        }
        wit_compiler::diagnostics::Diagnostic::UnknownName(UnknownName { name, range, .. }) => {
            let diagnostic = tower_lsp::lsp_types::Diagnostic {
                range: ts_to_range(range),
                message: format!("Attempted to reference unknown item, \"{name}\""),
                severity: Some(DiagnosticSeverity::ERROR),
                ..Default::default()
            };
            Some(diagnostic)
        }
        _ => None,
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
