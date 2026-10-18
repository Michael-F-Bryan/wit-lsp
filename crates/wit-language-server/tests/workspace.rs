mod support;

use std::{fs, path::PathBuf};

use futures::{SinkExt, StreamExt};
use serde_json::{Value, json};
use support::*;
use tower_lsp::{jsonrpc::Response, lsp_types::Url};
use wit_language_server::LanguageServerService;

struct Files(PathBuf);

impl Files {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("wit workspace {}", uuid::Uuid::new_v4()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn write(&self, name: &str, text: &str) -> String {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, text).unwrap();
        self.uri(name)
    }

    fn uri(&self, name: &str) -> String {
        Url::from_file_path(self.0.join(name)).unwrap().to_string()
    }

    fn folder(&self, name: &str) -> Value {
        json!({"uri": self.uri(name), "name": name})
    }
}

impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

async fn start(server: &mut impl LanguageServerService, folders: Value) {
    let caps = request(
        server,
        "initialize",
        json!({
            "workspaceFolders": folders, "capabilities": {}
        }),
    )
    .await["capabilities"]
        .clone();
    assert_eq!(caps["workspace"]["workspaceFolders"]["supported"], true);
    assert_eq!(
        caps["workspace"]["workspaceFolders"]["changeNotifications"],
        true
    );
    assert_eq!(caps["textDocumentSync"]["openClose"], true);
    assert_eq!(caps["textDocumentSync"]["save"]["includeText"], true);
    notify(server, "initialized", json!({})).await;
}

async fn hover(
    server: &mut impl LanguageServerService,
    uri: &str,
    source: &str,
    name: &str,
) -> Value {
    request(
        server,
        "textDocument/hover",
        json!({
            "textDocument": {"uri": uri}, "position": position(source, source.rfind(name).unwrap())
        }),
    )
    .await
}

async fn change(server: &mut impl LanguageServerService, uri: &str, text: &str, version: i32) {
    notify(
        server,
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": uri, "version": version}, "contentChanges": [{"text": text}]
        }),
    )
    .await;
}

async fn watched(server: &mut impl LanguageServerService, uri: &str, kind: i32) {
    notify(
        server,
        "workspace/didChangeWatchedFiles",
        json!({
            "changes": [{"uri": uri, "type": kind}]
        }),
    )
    .await;
}

async fn close(server: &mut impl LanguageServerService, uri: &str) {
    notify(
        server,
        "textDocument/didClose",
        json!({"textDocument": {"uri": uri}}),
    )
    .await;
}

const TYPES: &str = "package example:types; interface types { type value = u32; }";
const CONSUMER: &str =
    "interface app { use example:types/types.{value}; type result-type = value; }";

#[tokio::test]
async fn rename_includes_unopened_files_without_writing_them() {
    let files = Files::new();
    let dependency = files.write("deps/types/types.wit", TYPES);
    let consumer = files.write("app.wit", CONSUMER);
    let (mut server, mut socket) = wit_language_server::service();
    request(
        &mut server,
        "initialize",
        json!({
            "workspaceFolders": [files.folder("")],
            "capabilities": {"workspace": {"workspaceEdit": {"documentChanges": true}}}
        }),
    )
    .await;
    notify(&mut server, "initialized", json!({})).await;
    open(&mut server, &consumer, CONSUMER).await;
    let params = json!({"textDocument": {"uri": consumer},
        "position": position(CONSUMER, CONSUMER.rfind("value").unwrap()), "newName": "count"});
    let edit = request(&mut server, "textDocument/rename", params.clone()).await;
    let documents = edit["documentChanges"].as_array().unwrap();
    assert_eq!(documents.len(), 2);
    let dep = documents
        .iter()
        .find(|doc| doc["textDocument"]["uri"] == dependency)
        .unwrap();
    assert_eq!(dep["textDocument"]["version"], Value::Null);
    assert_eq!(
        dep["edits"],
        json!([{"range": range(TYPES, "value"), "newText": "count"}])
    );
    let app = documents
        .iter()
        .find(|doc| doc["textDocument"]["uri"] == consumer)
        .unwrap();
    assert_eq!(app["textDocument"]["version"], 1);
    assert_eq!(app["edits"].as_array().unwrap().len(), 2);
    assert_eq!(
        fs::read_to_string(files.0.join("deps/types/types.wit")).unwrap(),
        TYPES
    );
    assert_eq!(
        fs::read_to_string(files.0.join("app.wit")).unwrap(),
        CONSUMER
    );

    files.write(
        "deps/types/types.wit",
        "package example:types; interface types {}",
    );
    watched(&mut server, &dependency, 2).await;
    rejected_rename(&mut server, &mut socket, params).await;
}

#[tokio::test]
async fn symbol_search_tracks_unopened_files_and_workspace_removal() {
    let files = Files::new();
    let uri = files.write("deps/types/types.wit", TYPES);
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, json!([files.folder("")])).await;
    let params = json!({"query": "value"});
    let result = request(&mut server, "workspace/symbol", params.clone()).await;
    assert_eq!(result.as_array().unwrap().len(), 1);
    assert_eq!(
        result[0]["location"],
        json!({"uri": uri, "range": range(TYPES, "value")})
    );

    files.write("deps/types/types.wit", &TYPES.replace("value", "renamed"));
    watched(&mut server, &uri, 2).await;
    assert_eq!(
        request(&mut server, "workspace/symbol", params.clone()).await,
        json!([])
    );
    assert_eq!(
        request(&mut server, "workspace/symbol", json!({"query": "renamed"})).await[0]["location"]
            ["uri"],
        uri
    );

    fs::remove_file(files.0.join("deps/types/types.wit")).unwrap();
    watched(&mut server, &uri, 3).await;
    assert_eq!(
        request(&mut server, "workspace/symbol", json!({"query": ""})).await,
        json!([])
    );

    files.write("deps/types/types.wit", TYPES);
    watched(&mut server, &uri, 1).await;
    assert_eq!(
        request(&mut server, "workspace/symbol", params.clone()).await,
        result
    );
    notify(
        &mut server,
        "workspace/didChangeWorkspaceFolders",
        json!({"event": {
            "added": [], "removed": [files.folder("")]
        }}),
    )
    .await;
    assert_eq!(
        request(&mut server, "workspace/symbol", params).await,
        json!([])
    );
}

#[tokio::test]
async fn references_include_unopened_files_and_follow_disk_changes() {
    let files = Files::new();
    let dependency = files.write("deps/types/types.wit", TYPES);
    let consumer = files.write("app.wit", CONSUMER);
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, json!([files.folder("")])).await;
    let params = json!({
        "textDocument": {"uri": dependency},
        "position": position(TYPES, TYPES.find("value").unwrap()),
        "context": {"includeDeclaration": false}
    });
    let expected: Vec<_> = CONSUMER
        .match_indices("value")
        .map(|(offset, name)| {
            json!({"uri": consumer, "range": {
                "start": position(CONSUMER, offset),
                "end": position(CONSUMER, offset + name.len())
            }})
        })
        .collect();
    assert_eq!(
        request(&mut server, "textDocument/references", params.clone()).await,
        json!(expected)
    );

    files.write("app.wit", &CONSUMER.replace("= value", "= u32"));
    watched(&mut server, &consumer, 2).await;
    assert_eq!(
        request(&mut server, "textDocument/references", params.clone()).await,
        json!([expected[0]])
    );

    fs::remove_file(files.0.join("app.wit")).unwrap();
    watched(&mut server, &consumer, 3).await;
    assert_eq!(
        request(&mut server, "textDocument/references", params.clone()).await,
        json!([])
    );

    files.write("app.wit", CONSUMER);
    watched(&mut server, &consumer, 1).await;
    assert_eq!(
        request(&mut server, "textDocument/references", params).await,
        json!(expected)
    );
}

#[tokio::test]
async fn discovers_unopened_dependencies_and_tracks_creation_changes_and_deletion() {
    let files = Files::new();
    let dependency = files.write("deps/types/types.wit", TYPES);
    let uri = files.write("app.wit", CONSUMER);
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, json!([files.folder("")])).await;
    open(&mut server, &uri, CONSUMER).await;
    assert_eq!(diagnostics(&mut server, &uri).await, json!([]));
    let definition = request(&mut server, "textDocument/definition", json!({
        "textDocument": {"uri": uri}, "position": position(CONSUMER, CONSUMER.rfind("value").unwrap())
    })).await;
    assert_eq!(definition["uri"], dependency);
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = u32;"
    );
    let completion = request(&mut server, "textDocument/completion", json!({
        "textDocument": {"uri": uri}, "position": position(CONSUMER, CONSUMER.rfind("value").unwrap() + 3)
    })).await;
    assert!(
        completion
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["label"] == "value")
    );

    files.write("deps/types/types.wit", &TYPES.replace("u32", "string"));
    watched(&mut server, &dependency, 2).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = string;"
    );
    fs::remove_file(files.0.join("deps/types/types.wit")).unwrap();
    watched(&mut server, &dependency, 3).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await,
        Value::Null
    );
    assert!(
        !diagnostics(&mut server, &uri)
            .await
            .as_array()
            .unwrap()
            .is_empty()
    );
    files.write("deps/types/types.wit", TYPES);
    watched(&mut server, &dependency, 1).await;
    assert_eq!(diagnostics(&mut server, &uri).await, json!([]));
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = u32;"
    );
}

#[tokio::test]
async fn open_buffers_win_over_disk_and_close_restores_saved_contents() {
    let files = Files::new();
    let dependency = files.write("deps/types.wit", TYPES);
    let uri = files.write("app.wit", CONSUMER);
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, json!([files.folder("")])).await;
    open(&mut server, &uri, CONSUMER).await;
    open(&mut server, &dependency, TYPES).await;
    change(&mut server, &dependency, &TYPES.replace("u32", "bool"), 2).await;
    files.write("deps/types.wit", &TYPES.replace("u32", "string"));
    watched(&mut server, &dependency, 2).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = bool;"
    );
    // Neither a rescan nor an older document update may replace the overlay.
    notify(
        &mut server,
        "workspace/didChangeWorkspaceFolders",
        json!({"event": {
            "added": [files.folder("")], "removed": []
        }}),
    )
    .await;
    change(&mut server, &dependency, TYPES, 1).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = bool;"
    );
    close(&mut server, &dependency).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = string;"
    );
    // Late changes to a closed buffer must not resurrect its old text.
    change(&mut server, &dependency, TYPES, 3).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = string;"
    );

    open(&mut server, &dependency, TYPES).await;
    fs::remove_file(files.0.join("deps/types.wit")).unwrap();
    watched(&mut server, &dependency, 3).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = u32;"
    );
    close(&mut server, &dependency).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await,
        Value::Null
    );
}

#[tokio::test]
async fn folders_can_be_added_removed_and_overlap_without_losing_open_buffers() {
    let files = Files::new();
    let dependency = files.write("deps/types.wit", TYPES);
    let uri = files.write("app/app.wit", CONSUMER);
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, json!([files.folder("app")])).await;
    open(&mut server, &uri, CONSUMER).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await,
        Value::Null
    );
    notify(
        &mut server,
        "workspace/didChangeWorkspaceFolders",
        json!({"event": {
            "added": [files.folder("deps"), files.folder("")], "removed": []
        }}),
    )
    .await;
    assert_eq!(diagnostics(&mut server, &uri).await, json!([]));
    notify(
        &mut server,
        "workspace/didChangeWorkspaceFolders",
        json!({"event": {
            "added": [], "removed": [files.folder("deps")]
        }}),
    )
    .await;
    assert_eq!(diagnostics(&mut server, &uri).await, json!([]));
    open(&mut server, &dependency, TYPES).await;
    notify(
        &mut server,
        "workspace/didChangeWorkspaceFolders",
        json!({"event": {
            "added": [], "removed": [files.folder("")]
        }}),
    )
    .await;
    assert_eq!(diagnostics(&mut server, &uri).await, json!([]));
    close(&mut server, &dependency).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await,
        Value::Null
    );
    watched(&mut server, &dependency, 1).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await,
        Value::Null
    );
}

#[tokio::test]
async fn excluded_directories_and_invalid_files_do_not_enter_the_workspace() {
    let files = Files::new();
    let source = "interface i { type value = u32; }";
    let uri = files.write("valid.wit", source);
    let mut excluded = Vec::new();
    for dir in [
        "target",
        "node_modules",
        ".git",
        "build",
        "dist",
        ".cache",
        ".venv",
    ] {
        excluded.push(files.write(&format!("{dir}/ignored.wit"), source));
    }
    let non_wit = files.write("ignored.txt", source);
    let invalid = files.write("invalid.wit", "");
    fs::write(files.0.join("invalid.wit"), [0xff, 0xfe]).unwrap();
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, json!([files.folder("")])).await;
    assert_ne!(hover(&mut server, &uri, source, "value").await, Value::Null);
    for uri in excluded.iter().chain([&non_wit, &invalid]) {
        watched(&mut server, uri, 1).await;
        assert_eq!(hover(&mut server, uri, source, "value").await, Value::Null);
    }
    // Explicitly opening an excluded file is still allowed, until it is closed.
    open(&mut server, &excluded[0], source).await;
    assert_ne!(
        hover(&mut server, &excluded[0], source, "value").await,
        Value::Null
    );
    close(&mut server, &excluded[0]).await;
    assert_eq!(
        hover(&mut server, &excluded[0], source, "value").await,
        Value::Null
    );
}

#[tokio::test]
async fn root_uri_fallback_and_non_file_documents_work() {
    let files = Files::new();
    let source = "interface i { type value = u32; }";
    let uri = files.write("types.wit", source);
    for key in ["rootUri", "rootPath"] {
        let root = if key == "rootUri" {
            files.uri("")
        } else {
            files.0.to_str().unwrap().to_owned()
        };
        let (mut server, _socket) = wit_language_server::service();
        request(
            &mut server,
            "initialize",
            json!({"capabilities": {}, key: root}),
        )
        .await;
        notify(&mut server, "initialized", json!({})).await;
        assert_ne!(hover(&mut server, &uri, source, "value").await, Value::Null);
        let untitled = "untitled:types.wit";
        open(&mut server, untitled, source).await;
        assert_ne!(
            hover(&mut server, untitled, source, "value").await,
            Value::Null
        );
        close(&mut server, untitled).await;
        assert_eq!(
            hover(&mut server, untitled, source, "value").await,
            Value::Null
        );
    }
}

#[cfg(unix)]
#[tokio::test]
async fn discovery_and_watcher_events_do_not_follow_symlinks() {
    use std::os::unix::fs::symlink;
    let files = Files::new();
    let external = Files::new();
    let source = "interface i { type value = u32; }";
    external.write("types.wit", source);
    symlink(&external.0, files.0.join("linked")).unwrap();
    symlink(external.0.join("types.wit"), files.0.join("linked.wit")).unwrap();
    symlink(&files.0, files.0.join("cycle")).unwrap();
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, json!([files.folder("")])).await;
    for name in ["linked/types.wit", "linked.wit", "cycle/linked.wit"] {
        let uri = files.uri(name);
        watched(&mut server, &uri, 1).await;
        assert_eq!(hover(&mut server, &uri, source, "value").await, Value::Null);
    }
}

#[tokio::test]
async fn registers_watcher_and_refreshes_diagnostics_when_supported() {
    let files = Files::new();
    let dependency = files.write("types.wit", TYPES);
    let (mut server, mut socket) = wit_language_server::service();
    request(
        &mut server,
        "initialize",
        json!({
            "workspaceFolders": [files.folder("")], "capabilities": {"workspace": {
                "didChangeWatchedFiles": {"dynamicRegistration": true},
                "diagnostic": {"refreshSupport": true}
            }}
        }),
    )
    .await;
    let respond = async {
        let message = socket.next().await.unwrap();
        assert_eq!(message.method(), "client/registerCapability");
        assert_eq!(
            message.params().unwrap()["registrations"][0]["method"],
            "workspace/didChangeWatchedFiles"
        );
        assert_eq!(
            message.params().unwrap()["registrations"][0]["registerOptions"]["watchers"][0]["globPattern"],
            "**/*.wit"
        );
        // This edit occurs after initialize but before the watcher is active.
        files.write("types.wit", &TYPES.replace("u32", "string"));
        socket
            .send(Response::from_ok(
                message.id().unwrap().clone(),
                Value::Null,
            ))
            .await
            .unwrap();
    };
    futures::join!(notify(&mut server, "initialized", json!({})), respond);
    assert_eq!(
        hover(&mut server, &dependency, TYPES, "value").await["contents"]["value"],
        "type value = string;"
    );
    files.write("types.wit", &TYPES.replace("u32", "bool"));
    let respond = async {
        let message = socket.next().await.unwrap();
        assert_eq!(message.method(), "workspace/diagnostic/refresh");
        socket
            .send(Response::from_ok(
                message.id().unwrap().clone(),
                Value::Null,
            ))
            .await
            .unwrap();
    };
    futures::join!(watched(&mut server, &dependency, 2), respond);
}

#[tokio::test]
async fn save_and_directory_notifications_preserve_buffer_ownership() {
    let files = Files::new();
    let dependency = files.write("deps/types.wit", TYPES);
    let uri = files.write("app.wit", CONSUMER);
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, json!([files.folder("")])).await;
    open(&mut server, &dependency, TYPES).await;
    let saved = TYPES.replace("u32", "bool");
    files.write("deps/types.wit", &saved);
    notify(
        &mut server,
        "textDocument/didSave",
        json!({
            "textDocument": {"uri": dependency}, "text": saved
        }),
    )
    .await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = bool;"
    );
    change(&mut server, &dependency, &TYPES.replace("u32", "string"), 2).await;
    notify(
        &mut server,
        "textDocument/didSave",
        json!({"textDocument": {"uri": dependency}}),
    )
    .await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = string;"
    );
    close(&mut server, &dependency).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = bool;"
    );
    fs::remove_dir_all(files.0.join("deps")).unwrap();
    watched(&mut server, &files.uri("deps"), 3).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await,
        Value::Null
    );
    files.write("deps/types.wit", TYPES);
    watched(&mut server, &files.uri("deps"), 1).await;
    assert_eq!(
        hover(&mut server, &uri, CONSUMER, "value").await["contents"]["value"],
        "type value = u32;"
    );
}

#[tokio::test]
async fn multiple_roots_and_an_explicit_build_directory_are_discovered() {
    let files = Files::new();
    let source = "interface i { type value = u32; }";
    let first = files.write("first/types.wit", source);
    let second = files.write("target/types.wit", source);
    let (mut server, _socket) = wit_language_server::service();
    start(
        &mut server,
        json!([files.folder("first"), files.folder("target"),
        {"uri": "https://example.com/wit", "name": "remote"}]),
    )
    .await;
    for uri in [first, second] {
        assert_ne!(hover(&mut server, &uri, source, "value").await, Value::Null);
    }
}

#[tokio::test]
async fn rejected_watcher_registration_keeps_initial_discovery_usable() {
    let files = Files::new();
    let source = "interface i { type value = u32; }";
    let uri = files.write("types.wit", source);
    let (mut server, mut socket) = wit_language_server::service();
    request(
        &mut server,
        "initialize",
        json!({
            "workspaceFolders": [files.folder("")], "capabilities": {"workspace": {
                "didChangeWatchedFiles": {"dynamicRegistration": true}
            }}
        }),
    )
    .await;
    let respond = async {
        let message = socket.next().await.unwrap();
        socket
            .send(Response::from_error(
                message.id().unwrap().clone(),
                tower_lsp::jsonrpc::Error::method_not_found(),
            ))
            .await
            .unwrap();
    };
    futures::join!(notify(&mut server, "initialized", json!({})), respond);
    assert_ne!(hover(&mut server, &uri, source, "value").await, Value::Null);
}
