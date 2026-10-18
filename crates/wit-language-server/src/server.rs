use std::collections::HashMap;

use im::{OrdMap, Vector};
use salsa::ParallelDatabase;
use tokio::sync::{Mutex, MutexGuard};
use tower_lsp::{
    Client, ClientSocket, LspService,
    jsonrpc::Error,
    lsp_types::{
        CodeActionParams, CodeActionResponse, CompletionOptions, CompletionParams,
        CompletionResponse, DeclarationCapability, DiagnosticOptions, DiagnosticServerCapabilities,
        DidChangeTextDocumentParams, DidChangeWatchedFilesParams,
        DidChangeWatchedFilesRegistrationOptions, DidChangeWorkspaceFoldersParams,
        DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
        DocumentDiagnosticParams, DocumentDiagnosticReportResult, DocumentHighlight,
        DocumentHighlightParams, DocumentSymbolParams, DocumentSymbolResponse, FileSystemWatcher,
        FoldingRange, FoldingRangeParams, GotoDefinitionParams, GotoDefinitionResponse, Hover,
        HoverParams, InitializeParams, InitializeResult, InitializedParams, MarkupKind,
        MessageType, OneOf, PositionEncodingKind, PrepareRenameResponse, ReferenceParams,
        Registration, RenameOptions, RenameParams, SaveOptions, SelectionRange,
        SelectionRangeParams, ServerCapabilities, ServerInfo, SymbolInformation, SymbolKind,
        TextDocumentContentChangeEvent, TextDocumentItem, TextDocumentPositionParams,
        TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
        TextDocumentSyncSaveOptions, Url, WorkspaceEdit, WorkspaceFoldersServerCapabilities,
        WorkspaceServerCapabilities, WorkspaceSymbolParams,
        request::{GotoDeclarationParams, GotoDeclarationResponse},
    },
};
use wit_compiler::{Text, queries::Workspace};

use crate::{Database, utils, workspace_files::WorkspaceFiles};

/// The language server implementation.
#[derive(Debug)]
pub struct LanguageServer {
    client: Client,
    state: Mutex<State>,
}

impl LanguageServer {
    pub(crate) fn new(client: Client) -> Self {
        LanguageServer {
            client,
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

    async fn refresh_diagnostics(&self) {
        let supported = self.lock().await.diagnostic_refresh;
        if supported && let Err(error) = self.client.workspace_diagnostic_refresh().await {
            tracing::warn!(%error, "Client rejected diagnostic refresh");
        }
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

        {
            let mut state = self.lock().await;
            state.disk.initialize(&params);
            state.hierarchical_symbols = params
                .capabilities
                .text_document
                .as_ref()
                .and_then(|caps| caps.document_symbol.as_ref())
                .and_then(|caps| caps.hierarchical_document_symbol_support)
                .unwrap_or(false);
            state.extended_symbol_kinds = params
                .capabilities
                .text_document
                .as_ref()
                .and_then(|caps| caps.document_symbol.as_ref())
                .and_then(|caps| caps.symbol_kind.as_ref())
                .and_then(|caps| caps.value_set.as_ref())
                .is_some();
            if let Some(workspace) = &params.capabilities.workspace {
                state.versioned_edits = workspace
                    .workspace_edit
                    .as_ref()
                    .and_then(|caps| caps.document_changes)
                    .unwrap_or(false);
                state.workspace_symbol_kinds = workspace
                    .symbol
                    .as_ref()
                    .and_then(|caps| caps.symbol_kind.as_ref())
                    .and_then(|caps| caps.value_set.clone());
                state.watch_registration = workspace
                    .did_change_watched_files
                    .as_ref()
                    .and_then(|caps| caps.dynamic_registration)
                    .unwrap_or(false);
                state.diagnostic_refresh = workspace
                    .diagnostic
                    .as_ref()
                    .and_then(|caps| caps.refresh_support)
                    .unwrap_or(false);
            }
            state.reload_workspace().await;
        }

        self.lock().await.hover_markdown = params
            .capabilities
            .text_document
            .and_then(|caps| caps.hover)
            .and_then(|caps| caps.content_format)
            .and_then(|formats| formats.into_iter().next())
            == Some(MarkupKind::Markdown);

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                position_encoding: Some(PositionEncodingKind::UTF16),
                folding_range_provider: Some(true.into()),
                selection_range_provider: Some(true.into()),
                code_action_provider: Some(true.into()),
                hover_provider: Some(true.into()),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_highlight_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                definition_provider: Some(OneOf::Left(true)),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    ..Default::default()
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        inter_file_dependencies: true,
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

    async fn initialized(&self, _: InitializedParams) {
        let supported = self.lock().await.watch_registration;
        if !supported {
            tracing::info!("Client does not support dynamic WIT file watching");
            return;
        }
        let options = DidChangeWatchedFilesRegistrationOptions {
            watchers: vec![FileSystemWatcher {
                glob_pattern: "**/*.wit".to_owned().into(),
                kind: None,
            }],
        };
        let registration = Registration {
            id: "wit-workspace-files".into(),
            method: "workspace/didChangeWatchedFiles".into(),
            register_options: Some(serde_json::to_value(options).expect("watch options serialize")),
        };
        match self.client.register_capability(vec![registration]).await {
            Ok(()) => {
                // Catch changes made between initial discovery and watcher registration.
                self.lock().await.reload_workspace().await;
            }
            Err(error) => tracing::warn!(%error, "Client rejected WIT file watching"),
        }
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        let mut state = self.lock().await;
        let uris: Vec<_> = params
            .changes
            .into_iter()
            .map(|event| event.uri)
            .filter(|uri| state.disk.contains(uri))
            .collect();
        if uris.is_empty() {
            return;
        }
        if uris.iter().any(|uri| !uri.path().ends_with(".wit")) {
            state.reload_workspace().await;
        } else {
            state.reload_files(uris).await;
        }
        drop(state);
        self.refresh_diagnostics().await;
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        let mut state = self.lock().await;
        for folder in params.event.removed {
            state.disk.remove(&folder.uri);
        }
        for folder in params.event.added {
            state.disk.add(&folder.uri);
        }
        state.reload_workspace().await;
        drop(state);
        self.refresh_diagnostics().await;
    }

    async fn shutdown(&self) -> Result<(), Error> {
        tracing::info!("Shutting down");
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        tracing::debug!(document.uri=%params.text_document.uri);

        let TextDocumentItem {
            uri, text, version, ..
        } = params.text_document;
        let path = uri.as_str();
        let mut state = self.lock().await;

        tracing::debug!(size = text.len(), path, "File opened");
        let ws = state.ws;
        ws.update(state.wit_db(), path, text);
        state.open.insert(uri.to_string(), version);
        drop(state);
        self.refresh_diagnostics().await;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        tracing::debug!(document.uri=%params.text_document.uri);
        let path = params.text_document.uri.as_str();

        let mut state = self.lock().await;

        let Some(version) = state.open.get(path) else {
            return;
        };
        if params.text_document.version <= *version {
            return;
        }
        if params
            .content_changes
            .iter()
            .any(|change| change.range.is_some())
        {
            tracing::warn!("Incremental document updating isn't supported");
            return;
        }
        state
            .open
            .insert(path.to_owned(), params.text_document.version);

        for change in params.content_changes {
            let TextDocumentContentChangeEvent { text, .. } = change;

            tracing::debug!(size = text.len(), path, "File changed");
            let ws = state.ws;
            ws.update(state.wit_db(), path, text);
        }
        drop(state);
        self.refresh_diagnostics().await;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        tracing::debug!(document.uri=%params.text_document.uri);

        let mut state = self.lock().await;
        let uri = params.text_document.uri;
        if state.open.contains_key(uri.as_str()) {
            if let Some(text) = params.text {
                let ws = state.ws;
                ws.update(state.wit_db(), uri.as_str(), text);
            }
        } else {
            state.reload_files(vec![uri]).await;
        }
        drop(state);
        self.refresh_diagnostics().await;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut state = self.lock().await;
        state.open.remove(uri.as_str());
        state.reload_files(vec![uri]).await;
        drop(state);
        self.refresh_diagnostics().await;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>, Error> {
        let snap = self.snapshot().await;
        Ok(Some(crate::ops::workspace_symbols(
            snap.wit_db(),
            snap.ws,
            &params.query,
            snap.workspace_symbol_kinds.as_deref(),
        )))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>, Error> {
        let uri = params.text_document.uri;
        let snap = self.snapshot().await;
        let db = snap.wit_db();
        let Some(file) = snap.ws.lookup_by_path(db, uri.as_str()) else {
            return Ok(None);
        };
        Ok(Some(crate::ops::document_symbols(
            db,
            file,
            uri,
            snap.hierarchical_symbols,
            snap.extended_symbol_kinds,
        )))
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

        let positions = utils::Positions::new(db, file);
        let mut ranges = Vec::new();

        for position in params.positions {
            let point = positions.to_point(position)?;

            match wit_compiler::queries::selection_ranges(db, ast, point) {
                Some(mut r) => {
                    let first = r
                        .pop_front()
                        .expect("Should always return at least one range");
                    ranges.push(selection_range(&positions, first, r));
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

        let point = utils::Positions::new(snap.wit_db(), file)
            .to_point(params.text_document_position.position)?;
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

        let Some(file) = snap.ws.lookup_by_path(snap.wit_db(), path.as_str()) else {
            return Ok(None);
        };

        let point = utils::Positions::new(snap.wit_db(), file)
            .to_point(params.text_document_position_params.position)?;
        let location = crate::ops::goto_definition(snap.wit_db(), snap.ws, file, point);
        Ok(location
            .and_then(|location| utils::location_to_lsp(snap.wit_db(), snap.ws, location))
            .map(GotoDefinitionResponse::Scalar))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn goto_declaration(
        &self,
        params: GotoDeclarationParams,
    ) -> Result<Option<GotoDeclarationResponse>, Error> {
        let path = &params.text_document_position_params.text_document.uri;
        tracing::debug!(document.uri=%path);
        let snap = self.snapshot().await;
        let db = snap.wit_db();
        let Some(file) = snap.ws.lookup_by_path(db, path.as_str()) else {
            return Ok(None);
        };
        let point = utils::Positions::new(db, file)
            .to_point(params.text_document_position_params.position)?;
        Ok(crate::ops::goto_declaration(db, snap.ws, file, point)
            .and_then(|location| utils::location_to_lsp(db, snap.ws, location))
            .map(GotoDeclarationResponse::Scalar))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>, Error> {
        let snap = self.snapshot().await;
        let db = snap.wit_db();
        let Some(file) = snap
            .ws
            .lookup_by_path(db, params.text_document.uri.as_str())
        else {
            return Ok(None);
        };
        let point = utils::Positions::new(db, file).to_point(params.position)?;
        Ok(crate::ops::prepare_rename(db, snap.ws, file, point))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>, Error> {
        let result = {
            let snap = self.snapshot().await;
            let db = snap.wit_db();
            let position = params.text_document_position;
            let file = snap
                .ws
                .lookup_by_path(db, position.text_document.uri.as_str())
                .ok_or_else(|| Error::invalid_params("The document is not in the workspace"))?;
            let point = utils::Positions::new(db, file).to_point(position.position)?;
            crate::ops::rename(
                db,
                snap.ws,
                file,
                point,
                &params.new_name,
                snap.versioned_edits.then_some(&snap.versions),
            )
            .map(Some)
        };
        if let Err(error) = &result {
            let message = format!("Rename failed: {}", error.message);
            tracing::warn!(%message);
            self.client
                .log_message(MessageType::WARNING, &message)
                .await;
            if error.code == tower_lsp::jsonrpc::ErrorCode::InvalidParams {
                self.client
                    .show_message(MessageType::WARNING, &message)
                    .await;
                // The warning explains the rejection. An RPC error would make
                // the editor display a second, generic failure notification.
                return Ok(None);
            }
        }
        result
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn references(
        &self,
        params: ReferenceParams,
    ) -> Result<Option<Vec<tower_lsp::lsp_types::Location>>, Error> {
        let snap = self.snapshot().await;
        let db = snap.wit_db();
        let uri = params.text_document_position.text_document.uri;
        let Some(file) = snap.ws.lookup_by_path(db, uri.as_str()) else {
            return Ok(None);
        };
        let point =
            utils::Positions::new(db, file).to_point(params.text_document_position.position)?;
        Ok(Some(crate::ops::find_references(
            db,
            snap.ws,
            file,
            point,
            params.context.include_declaration,
        )))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>, Error> {
        let params = params.text_document_position_params;
        let snap = self.snapshot().await;
        let db = snap.wit_db();
        let Some(file) = snap
            .ws
            .lookup_by_path(db, params.text_document.uri.as_str())
        else {
            return Ok(None);
        };
        let point = utils::Positions::new(db, file).to_point(params.position)?;
        Ok(Some(crate::ops::document_highlights(
            db, snap.ws, file, point,
        )))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>, Error> {
        let params = params.text_document_position_params;
        let snap = self.snapshot().await;
        let db = snap.wit_db();
        let Some(file) = snap
            .ws
            .lookup_by_path(db, params.text_document.uri.as_str())
        else {
            return Ok(None);
        };
        let point = utils::Positions::new(db, file).to_point(params.position)?;
        Ok(crate::ops::hover(
            db,
            snap.ws,
            file,
            point,
            snap.hover_markdown,
        ))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn code_action(
        &self,
        params: CodeActionParams,
    ) -> Result<Option<CodeActionResponse>, Error> {
        let snap = self.snapshot().await;

        let actions = crate::code_actions::resolve(snap.db(), snap.ws, params);

        Ok(actions)
    }
}

fn selection_range(
    positions: &utils::Positions<'_>,
    first: tree_sitter::Range,
    rest: Vector<tree_sitter::Range>,
) -> SelectionRange {
    let mut parent = None;

    for range in rest.iter().rev() {
        let sel = SelectionRange {
            range: positions.to_lsp(*range),
            parent: parent.take(),
        };
        parent = Some(Box::new(sel));
    }

    SelectionRange {
        range: positions.to_lsp(first),
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
    hover_markdown: bool,
    hierarchical_symbols: bool,
    extended_symbol_kinds: bool,
    workspace_symbol_kinds: Option<Vec<SymbolKind>>,
    versioned_edits: bool,
    disk: WorkspaceFiles,
    open: HashMap<String, i32>,
    watch_registration: bool,
    diagnostic_refresh: bool,
}

impl State {
    async fn reload_workspace(&mut self) {
        let files = match self.disk.scan().await {
            Ok(files) => files,
            Err(error) => {
                tracing::error!(%error, "WIT discovery task failed");
                return;
            }
        };
        let ws = self.ws;
        let obsolete: Vec<_> = ws
            .files(&self.db)
            .keys()
            .map(|path| path.raw_path(&self.db).to_string())
            .filter(|path| !self.open.contains_key(path) && !files.contains_key(path))
            .collect();
        for path in obsolete {
            ws.remove(self.wit_db(), &path);
        }
        for (path, text) in files {
            self.apply_disk(&path, Some(text));
        }
    }

    async fn reload_files(&mut self, uris: Vec<Url>) {
        let uris = uris
            .into_iter()
            .filter(|uri| !self.open.contains_key(uri.as_str()))
            .collect();
        match self.disk.read(uris).await {
            Ok(files) => {
                for (path, text) in files {
                    self.apply_disk(&path, text);
                }
            }
            Err(error) => tracing::error!(%error, "WIT file read task failed"),
        }
    }

    fn apply_disk(&mut self, path: &str, text: Option<String>) {
        if self.open.contains_key(path) {
            return;
        }
        let ws = self.ws;
        match text {
            Some(text) => {
                if !ws
                    .lookup_by_path(&self.db, path)
                    .is_some_and(|file| file.contents(&self.db).as_str() == text)
                {
                    ws.update(self.wit_db(), path, text);
                }
            }
            None => ws.remove(self.wit_db(), path),
        }
    }

    fn db(&mut self) -> &mut dyn crate::Db {
        &mut self.db
    }

    fn wit_db(&mut self) -> &mut dyn wit_compiler::Db {
        self.db().as_wit_mut()
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            db: self.db.snapshot(),
            versions: self.open.clone(),
            versioned_edits: self.versioned_edits,
            ws: self.ws,
            hover_markdown: self.hover_markdown,
            hierarchical_symbols: self.hierarchical_symbols,
            extended_symbol_kinds: self.extended_symbol_kinds,
            workspace_symbol_kinds: self.workspace_symbol_kinds.clone(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let db = Database::default();
        let workspace = Workspace::new(&db, OrdMap::new());
        Self {
            ws: workspace,
            db,
            hover_markdown: false,
            hierarchical_symbols: false,
            extended_symbol_kinds: false,
            workspace_symbol_kinds: None,
            versioned_edits: false,
            disk: WorkspaceFiles::default(),
            open: HashMap::new(),
            watch_registration: false,
            diagnostic_refresh: false,
        }
    }
}

/// A readonly snapshot of the [`State`].
///
/// This is often used when you want to do something that doesn't require update
/// inputs. By using a [`Snapshot`] instead of holding onto the [`State`]
/// directly you allow other readonly queries to run in parallel.
#[derive(Debug)]
struct Snapshot {
    versions: HashMap<String, i32>,
    ws: Workspace,
    db: salsa::Snapshot<Database>,
    hover_markdown: bool,
    hierarchical_symbols: bool,
    extended_symbol_kinds: bool,
    workspace_symbol_kinds: Option<Vec<SymbolKind>>,
    versioned_edits: bool,
}

impl Snapshot {
    fn db(&self) -> &dyn crate::Db {
        &*self.db
    }

    fn wit_db(&self) -> &dyn wit_compiler::Db {
        self.db().as_wit()
    }
}
