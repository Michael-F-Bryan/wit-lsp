mod support;

use serde_json::{Value, json};
use support::*;
use wit_language_server::LanguageServerService;

async fn search(server: &mut impl LanguageServerService, query: &str) -> Value {
    request(server, "workspace/symbol", json!({"query": query})).await
}

#[tokio::test]
async fn searches_all_files_with_stable_locations_and_owners() {
    let (mut server, _socket) = wit_language_server::service();
    let caps = request(
        &mut server,
        "initialize",
        json!({"capabilities": {
            "workspace": {"symbol": {"symbolKind": {"valueSet": (1..=26).collect::<Vec<_>>()}}}
        }}),
    )
    .await;
    assert_eq!(caps["capabilities"]["workspaceSymbolProvider"], true);
    notify(&mut server, "initialized", json!({})).await;
    assert_eq!(search(&mut server, "").await, json!([]));
    let a = "interface first { record point { column: u32 } run: func(p: point); }";
    let b = "interface second { record point { line: u32 } type points = list<point>; }";
    open(&mut server, "file:///b.wit", b).await;
    open(&mut server, "file:///a.wit", a).await;
    let expected = json!([
        {"name": "point", "kind": 23, "containerName": "first", "location": {
            "uri": "file:///a.wit", "range": range(a, "point")
        }},
        {"name": "point", "kind": 23, "containerName": "second", "location": {
            "uri": "file:///b.wit", "range": range(b, "point")
        }},
        {"name": "points", "kind": 26, "containerName": "second", "location": {
            "uri": "file:///b.wit", "range": range(b, "points")
        }}
    ]);
    assert_eq!(search(&mut server, "PoInT").await, expected);
    assert_eq!(search(&mut server, "oint").await, expected);
    assert_eq!(search(&mut server, "missing").await, json!([]));
    let all = search(&mut server, "").await;
    assert_eq!(all.as_array().unwrap().len(), 8);
    assert_eq!(
        search(&mut server, "column").await[0]["containerName"],
        "point"
    );
    assert_eq!(search(&mut server, "run").await[0]["kind"], 12);
}

#[tokio::test]
async fn workspace_symbol_kinds_use_workspace_capabilities() {
    for (workspace, document, expected) in [
        (
            json!({}),
            json!({"symbolKind": {"valueSet": (1..=26).collect::<Vec<_>>()}}),
            5,
        ),
        (
            json!({"symbol": {"symbolKind": {"valueSet": (1..=26).collect::<Vec<_>>()}}}),
            json!({}),
            23,
        ),
        (
            json!({"symbol": {"symbolKind": {"valueSet": [13]}}}),
            json!({}),
            13,
        ),
    ] {
        let (mut server, _socket) = wit_language_server::service();
        request(
            &mut server,
            "initialize",
            json!({"capabilities": {
                "workspace": workspace, "textDocument": {"documentSymbol": document}
            }}),
        )
        .await;
        notify(&mut server, "initialized", json!({})).await;
        open(
            &mut server,
            "file:///types.wit",
            "interface types { record point { x: u32 } }",
        )
        .await;
        assert_eq!(search(&mut server, "point").await[0]["kind"], expected);
    }
}

#[tokio::test]
async fn unicode_ranges_and_escaped_names_follow_open_buffer_edits() {
    for newline in ["\n", "\r\n"] {
        let (mut server, _socket) = wit_language_server::service();
        initialize(&mut server).await;
        let uri = "file:///unicode.wit";
        let source = format!("interface types {{{newline}/* é😀 */ type %type = u32;{newline}}}");
        open(&mut server, uri, &source).await;
        open(
            &mut server,
            "file:///incomplete.wit",
            "interface broken { record",
        )
        .await;
        let result = search(&mut server, "type").await;
        let symbol = result
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == "type")
            .unwrap();
        assert_eq!(symbol["location"]["range"], range(&source, "%type"));
        let updated = source.replace("%type", "renamed");
        notify(
            &mut server,
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": 2}, "contentChanges": [{"text": updated}]
            }),
        )
        .await;
        assert!(
            !search(&mut server, "type")
                .await
                .as_array()
                .unwrap()
                .iter()
                .any(|s| s["name"] == "type")
        );
        assert_eq!(
            search(&mut server, "renamed").await[0]["location"]["range"],
            range(&updated, "renamed")
        );
        notify(
            &mut server,
            "textDocument/didClose",
            json!({"textDocument": {"uri": uri}}),
        )
        .await;
        assert_eq!(search(&mut server, "renamed").await, json!([]));
    }
}
