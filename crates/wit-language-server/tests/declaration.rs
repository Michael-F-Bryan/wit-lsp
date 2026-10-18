mod support;

use serde_json::{Value, json};
use support::*;

#[tokio::test]
async fn declaration_preserves_aliases_while_definition_follows_them() {
    for newline in ["\n", "\r\n"] {
        let (mut server, _socket) = wit_language_server::service();
        let initialized = request(&mut server, "initialize", json!({"capabilities": {}})).await;
        assert_eq!(initialized["capabilities"]["declarationProvider"], true);
        notify(&mut server, "initialized", json!({})).await;
        let types_uri = "file:///types.wit";
        let app_uri = "file:///app.wit";
        let types = "interface types { record value { x: u32 } }\ninterface bridge { use types.{value as forwarded}; }".replace('\n', newline);
        let app = "use types as api;\ninterface app {\n /* é😀 */ use api.{value as local};\n use bridge.{forwarded};\n f: func(x: local, y: forwarded);\n}\nworld w {\n import incoming: interface {\n  use types.{value as inner};\n  f: func(x: inner);\n }\n export outgoing: interface {\n  type inner = bool;\n  f: func(x: inner);\n }\n}".replace('\n', newline);
        open(&mut server, types_uri, &types).await;
        open(&mut server, app_uri, &app).await;
        assert_eq!(diagnostics(&mut server, app_uri).await, json!([]));
        for (offset, target_uri, target, declaration, definition) in [
            (
                app.find("x: local").unwrap() + 3,
                app_uri,
                &app,
                "local",
                "record value { x: u32 }",
            ),
            (
                app.find("as local").unwrap() + 3,
                app_uri,
                &app,
                "local",
                "record value { x: u32 }",
            ),
            (
                app.find("api.{").unwrap(),
                app_uri,
                &app,
                "api",
                "interface types { record value { x: u32 } }",
            ),
            (
                app.find("value as").unwrap(),
                types_uri,
                &types,
                "record value { x: u32 }",
                "record value { x: u32 }",
            ),
            (
                app.find("y: forwarded").unwrap() + 3,
                types_uri,
                &types,
                "forwarded",
                "record value { x: u32 }",
            ),
            (
                app.find("x: inner").unwrap() + 3,
                app_uri,
                &app,
                "inner",
                "record value { x: u32 }",
            ),
        ] {
            let params =
                json!({"textDocument": {"uri": app_uri}, "position": position(&app, offset)});
            assert_eq!(
                request(&mut server, "textDocument/declaration", params.clone()).await,
                json!({"uri": target_uri, "range": range(target, declaration)}),
                "offset {offset}"
            );
            assert_eq!(
                request(&mut server, "textDocument/definition", params).await,
                json!({"uri": types_uri, "range": range(&types, definition)}),
                "offset {offset}"
            );
        }
        let params = json!({"textDocument": {"uri": app_uri}, "position": position(&app, app.rfind("x: inner").unwrap() + 3)});
        assert_eq!(
            request(&mut server, "textDocument/declaration", params).await,
            json!({"uri": app_uri, "range": range(&app, "type inner = bool;")})
        );
    }
}

#[tokio::test]
async fn declaration_handles_local_symbols_paths_and_missing_targets() {
    let (mut server, _socket) = wit_language_server::service();
    initialize(&mut server).await;
    let uri = "file:///symbols.wit";
    let src = "interface types { record value { field: u32 } type alias = value; f: func(x: alias); }\nworld base { import log: func(); }\nworld app { include base with { log as logger }; }\ninterface broken { type value = missing; }";
    open(&mut server, uri, src).await;
    for (needle, declaration) in [
        ("value;", "record value { field: u32 }"),
        ("value {", "record value { field: u32 }"),
        ("alias);", "type alias = value;"),
        ("field:", "field: u32"),
        ("f: func", "f: func(x: alias);"),
        ("base with", "world base { import log: func(); }"),
        ("log as", "import log: func();"),
        ("logger", "logger"),
    ] {
        let params = json!({"textDocument": {"uri": uri}, "position": position(src, src.find(needle).unwrap())});
        assert_eq!(
            request(&mut server, "textDocument/declaration", params).await,
            json!({"uri": uri, "range": range(src, declaration)}),
            "{needle}"
        );
    }
    for offset in [
        src.find("missing").unwrap(),
        src.find("u32").unwrap(),
        src.find('{').unwrap(),
        src.len(),
    ] {
        let params = json!({"textDocument": {"uri": uri}, "position": position(src, offset)});
        assert_eq!(
            request(&mut server, "textDocument/declaration", params).await,
            Value::Null
        );
    }
    assert_eq!(request(&mut server, "textDocument/declaration", json!({
        "textDocument": {"uri": "file:///unknown.wit"}, "position": {"line": 0, "character": 0}
    })).await, Value::Null);
}

#[tokio::test]
async fn declaration_tracks_unsaved_alias_changes() {
    let (mut server, _socket) = wit_language_server::service();
    initialize(&mut server).await;
    let uri = "file:///changing.wit";
    let src = "interface types { type value = u32; }\ninterface app { use types.{value as local}; type result-type = local; }";
    open(&mut server, uri, src).await;
    for (version, text, expected) in [
        (2, src.replace("as local", "as renamed"), Value::Null),
        (
            3,
            src.to_owned(),
            json!({"uri": uri, "range": range(src, "local")}),
        ),
    ] {
        notify(
            &mut server,
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": version}, "contentChanges": [{"text": text}]
            }),
        )
        .await;
        assert_eq!(request(&mut server, "textDocument/declaration", json!({
            "textDocument": {"uri": uri}, "position": position(&text, text.rfind("local").unwrap())
        })).await, expected);
    }
}
