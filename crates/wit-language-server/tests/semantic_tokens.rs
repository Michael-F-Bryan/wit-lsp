mod support;

use futures::{SinkExt, StreamExt};
use serde_json::{Value, json};
use support::*;
use tower_lsp::jsonrpc::Response;
use wit_language_server::LanguageServerService;

async fn start(server: &mut impl LanguageServerService, capabilities: Value) -> Value {
    let result = request(server, "initialize", json!({"capabilities": capabilities})).await;
    let provider = &result["capabilities"]["semanticTokensProvider"];
    assert_eq!(provider["full"], true);
    assert!(provider.get("range").is_none());
    notify(server, "initialized", json!({})).await;
    provider["legend"].clone()
}

async fn tokens(
    server: &mut impl LanguageServerService,
    uri: &str,
    text: &str,
    legend: &Value,
) -> Vec<(String, String, bool)> {
    let result = request(
        server,
        "textDocument/semanticTokens/full",
        json!({"textDocument": {"uri": uri}}),
    )
    .await;
    let data = result["data"].as_array().unwrap();
    assert_eq!(data.len() % 5, 0);
    let lines: Vec<Vec<u16>> = text
        .lines()
        .map(|line| line.encode_utf16().collect())
        .collect();
    let (mut line, mut column, mut previous_end) = (0, 0, 0);
    data.as_chunks::<5>()
        .0
        .iter()
        .map(|token| {
            let delta_line = token[0].as_u64().unwrap() as usize;
            line += delta_line;
            if delta_line != 0 {
                column = 0;
                previous_end = 0;
            }
            column += token[1].as_u64().unwrap() as usize;
            let end = column + token[2].as_u64().unwrap() as usize;
            assert!(
                column >= previous_end && end > column,
                "overlapping or empty token"
            );
            previous_end = end;
            let name = String::from_utf16(&lines[line][column..end]).unwrap();
            let kind = legend["tokenTypes"][token[3].as_u64().unwrap() as usize]
                .as_str()
                .unwrap()
                .to_owned();
            let modifiers = token[4].as_u64().unwrap();
            assert!(modifiers < (1 << legend["tokenModifiers"].as_array().unwrap().len()));
            (name, kind, modifiers != 0)
        })
        .collect()
}

#[tokio::test]
async fn classifies_declarations_and_references_with_unicode_positions() {
    for newline in ["\n", "\r\n"] {
        let (mut server, _socket) = wit_language_server::service();
        let legend = start(&mut server, json!({})).await;
        let src = "interface api {\n /* é😀 */ record point { x: u32 }\n type points = list<point>;\n enum color { red }\n flags mode { read }\n variant result-type { ok(point), err }\n resource %type { constructor(); read: func(arg: point); make: static func() -> own<%type>; }\n f: func(p: point) -> points;\n}\nworld app { import run: func(); import nested: interface { type local = bool; f: func(v: local); } }".replace('\n', newline);
        let uri = "file:///kinds.wit";
        open(&mut server, uri, &src).await;
        let got = tokens(&mut server, uri, &src, &legend).await;
        let expected = [
            ("api", "interface", true),
            ("point", "struct", true),
            ("x", "property", true),
            ("points", "type", true),
            ("point", "struct", false),
            ("color", "enum", true),
            ("red", "enumMember", true),
            ("mode", "enum", true),
            ("read", "enumMember", true),
            ("result-type", "enum", true),
            ("ok", "enumMember", true),
            ("point", "struct", false),
            ("err", "enumMember", true),
            ("%type", "class", true),
            ("read", "method", true),
            ("arg", "parameter", true),
            ("point", "struct", false),
            ("make", "method", true),
            ("%type", "class", false),
            ("f", "function", true),
            ("p", "parameter", true),
            ("point", "struct", false),
            ("points", "type", false),
            ("app", "namespace", true),
            ("run", "function", true),
            ("nested", "interface", true),
            ("local", "type", true),
            ("f", "function", true),
            ("v", "parameter", true),
            ("local", "type", false),
        ]
        .map(|(name, kind, declaration)| (name.to_owned(), kind.to_owned(), declaration));
        assert_eq!(got, expected);
    }
}

#[tokio::test]
async fn imported_aliases_follow_dependency_kinds_and_buffer_changes() {
    let (mut server, _socket) = wit_language_server::service();
    let legend = start(&mut server, json!({})).await;
    let dep_uri = "file:///deps/types.wit";
    let app_uri = "file:///app.wit";
    let source = "package example:types@1.2.3; interface types { record value { x: u32 } }";
    let app = "use example:types/types@1.2.3 as api;\ninterface bridge { use api.{value as local}; }\nworld app { import nested: interface { use bridge.{local as inner}; f: func(v: inner); } }\ninterface other { type inner = bool; f: func(v: inner); }";
    open(&mut server, dep_uri, source).await;
    open(&mut server, app_uri, app).await;
    for (version, updated, expected_kind) in [
        (2, source.to_owned(), "struct"),
        (
            3,
            source.replace("record value { x: u32 }", "enum value { x }"),
            "enum",
        ),
        (4, source.to_owned(), "struct"),
    ] {
        notify(&mut server, "textDocument/didChange", json!({"textDocument": {"uri": dep_uri, "version": version}, "contentChanges": [{"text": updated}]})).await;
        let got = tokens(&mut server, app_uri, app, &legend).await;
        assert!(!got.iter().any(|(name, _, _)| name == "example"));
        for (_, kind, _) in got
            .iter()
            .filter(|(name, _, _)| ["value", "local"].contains(&name.as_str()))
        {
            assert_eq!(kind, expected_kind);
        }
        let inner: Vec<_> = got
            .iter()
            .filter(|(name, _, _)| name == "inner")
            .map(|(_, kind, decl)| (kind.as_str(), *decl))
            .collect();
        assert_eq!(
            inner,
            [
                (expected_kind, true),
                (expected_kind, false),
                ("type", true),
                ("type", false)
            ]
        );
        assert!(got.contains(&("api".into(), "interface".into(), true)));
        assert!(got.contains(&("api".into(), "interface".into(), false)));
    }
}

#[tokio::test]
async fn respects_client_legend_and_falls_back_to_generic_types() {
    let (mut server, _socket) = wit_language_server::service();
    let legend = start(&mut server, json!({"textDocument": {"semanticTokens": {
        "requests": {"full": true}, "tokenTypes": ["function", "type"], "tokenModifiers": [], "formats": ["relative"]
    }}})).await;
    assert_eq!(
        legend,
        json!({"tokenTypes": ["type", "function"], "tokenModifiers": []})
    );
    let uri = "file:///limited.wit";
    let src = "interface i { record value { x: u32 } f: func(v: value); }";
    open(&mut server, uri, src).await;
    assert_eq!(
        tokens(&mut server, uri, src, &legend).await,
        [
            ("value".into(), "type".into(), false),
            ("f".into(), "function".into(), false),
            ("value".into(), "type".into(), false)
        ]
    );
}

#[tokio::test]
async fn missing_and_incomplete_documents_are_safe() {
    let (mut server, _socket) = wit_language_server::service();
    let legend = start(&mut server, json!({})).await;
    let uri = "file:///broken.wit";
    assert_eq!(
        request(
            &mut server,
            "textDocument/semanticTokens/full",
            json!({"textDocument": {"uri": uri}})
        )
        .await,
        Value::Null
    );
    for src in [
        "",
        "// type value = u32;",
        "interface i { use absent.{missing as local}; type value = missing; type other = list<; record",
    ] {
        open(&mut server, uri, src).await;
        let got = tokens(&mut server, uri, src, &legend).await;
        assert!(
            !got.iter()
                .any(|(name, _, _)| ["absent", "missing", "local"].contains(&name.as_str()))
        );
    }
}

#[tokio::test]
async fn refreshes_semantic_tokens_when_supported() {
    let (mut server, mut socket) = wit_language_server::service();
    start(
        &mut server,
        json!({"workspace": {"semanticTokens": {"refreshSupport": true}}}),
    )
    .await;
    let respond = async {
        let message = socket.next().await.unwrap();
        assert_eq!(message.method(), "workspace/semanticTokens/refresh");
        socket
            .send(Response::from_ok(
                message.id().unwrap().clone(),
                Value::Null,
            ))
            .await
            .unwrap();
    };
    futures::join!(
        open(
            &mut server,
            "file:///types.wit",
            "interface i { type value = u32; }"
        ),
        respond
    );
}
