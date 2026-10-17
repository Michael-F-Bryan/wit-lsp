mod support;

use serde_json::{Value, json};
use support::*;
use wit_language_server::LanguageServerService;

async fn start(server: &mut impl LanguageServerService, hierarchical: bool) {
    let result = request(
        server,
        "initialize",
        json!({"capabilities": {
            "textDocument": {"documentSymbol": {"hierarchicalDocumentSymbolSupport": hierarchical, "symbolKind": {"valueSet": (1..=26).collect::<Vec<_>>() }}}
        }}),
    )
    .await;
    assert_eq!(result["capabilities"]["documentSymbolProvider"], true);
    notify(server, "initialized", json!({})).await;
}

async fn symbols(server: &mut impl LanguageServerService, uri: &str) -> Value {
    request(
        server,
        "textDocument/documentSymbol",
        json!({"textDocument": {"uri": uri}}),
    )
    .await
}

fn outline(symbols: &Value, parent: &str, output: &mut Vec<(String, u64)>) {
    for symbol in symbols.as_array().unwrap() {
        let name = symbol["name"].as_str().unwrap();
        let path = if parent.is_empty() {
            name.to_owned()
        } else {
            format!("{parent}/{name}")
        };
        output.push((path.clone(), symbol["kind"].as_u64().unwrap()));
        if let Some(children) = symbol.get("children") {
            outline(children, &path, output);
        }
    }
}

fn assert_ranges(symbols: &Value, parent: Option<&Value>) {
    fn point(value: &Value) -> (u64, u64) {
        (
            value["line"].as_u64().unwrap(),
            value["character"].as_u64().unwrap(),
        )
    }
    for symbol in symbols.as_array().unwrap() {
        let range = &symbol["range"];
        let selection = &symbol["selectionRange"];
        assert!(
            point(&range["start"]) <= point(&selection["start"]),
            "{symbol}"
        );
        assert!(
            point(&selection["start"]) < point(&selection["end"]),
            "{symbol}"
        );
        assert!(point(&selection["end"]) <= point(&range["end"]), "{symbol}");
        if let Some(parent) = parent {
            assert!(
                point(&parent["start"]) <= point(&range["start"]),
                "{symbol}"
            );
            assert!(point(&range["end"]) <= point(&parent["end"]), "{symbol}");
        }
        if let Some(children) = symbol.get("children") {
            assert_ranges(children, Some(range));
        }
    }
}

#[tokio::test]
async fn outline_preserves_declaration_order_and_member_ownership() {
    let source = r#"package example:outline;
use example:other/types as imported;
interface api {
    use imported.{external as local};
    record z { y: list<u32>, x: string }
    type %type = z;
    enum e { second, first }
    flags bits { write, read }
    variant v { some(z), none }
    resource handle {
        constructor();
        close: func();
        create: static func() -> handle;
    }
    run: func(argument: z) -> (result-value: z);
}
world app {
    import api;
    export run: func();
    import inline: interface { record nested { field: u32 } }
    include other-world;
}
"#;
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, true).await;
    let uri = "file:///outline.wit";
    open(&mut server, uri, source).await;
    let result = symbols(&mut server, uri).await;
    let mut actual = Vec::new();
    outline(&result, "", &mut actual);
    let expected = [
        ("example:outline", 4),
        ("imported", 3),
        ("api", 11),
        ("api/local", 26),
        ("api/z", 23),
        ("api/z/y", 8),
        ("api/z/x", 8),
        ("api/type", 26),
        ("api/e", 10),
        ("api/e/second", 22),
        ("api/e/first", 22),
        ("api/bits", 10),
        ("api/bits/write", 22),
        ("api/bits/read", 22),
        ("api/v", 10),
        ("api/v/some", 22),
        ("api/v/none", 22),
        ("api/handle", 5),
        ("api/handle/constructor", 9),
        ("api/handle/close", 6),
        ("api/handle/create", 6),
        ("api/run", 12),
        ("app", 3),
        ("app/api", 11),
        ("app/run", 12),
        ("app/inline", 11),
        ("app/inline/nested", 23),
        ("app/inline/nested/field", 8),
    ]
    .map(|(name, kind)| (name.to_owned(), kind));
    assert_eq!(actual, expected);
    assert_ranges(&result, None);
    let record = &result[2]["children"][1];
    assert_eq!(
        record["range"],
        range(source, "record z { y: list<u32>, x: string }")
    );
    assert_eq!(record["children"][0]["detail"], "list<u32>");
    assert_eq!(
        result[2]["children"][2]["selectionRange"],
        range(source, "%type")
    );
}

#[tokio::test]
async fn symbol_ranges_use_utf16_and_follow_edits() {
    for newline in ["\n", "\r\n"] {
        let (mut server, _socket) = wit_language_server::service();
        start(&mut server, true).await;
        let uri = "file:///unicode.wit";
        let source = format!(
            "/// Documentation.{newline}interface api {{{newline}/* é😀 */ record point {{{newline}/* β😀 */ column: u32{newline}}}{newline}}}"
        );
        open(&mut server, uri, &source).await;
        let result = symbols(&mut server, uri).await;
        assert_ranges(&result, None);
        assert_eq!(result[0]["range"], range(&source, &source));
        assert_eq!(result[0]["selectionRange"], range(&source, "api"));
        let record = &result[0]["children"][0];
        assert_eq!(record["selectionRange"], range(&source, "point"));
        assert_eq!(
            record["children"][0]["selectionRange"],
            range(&source, "column")
        );
        let updated = source
            .replace("point", "position")
            .replace("column", "offset");
        notify(
            &mut server,
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": 2}, "contentChanges": [{"text": updated}]
            }),
        )
        .await;
        let result = symbols(&mut server, uri).await;
        assert_eq!(result[0]["children"][0]["name"], "position");
        assert_eq!(
            result[0]["children"][0]["selectionRange"],
            range(&updated, "position")
        );
        assert_eq!(result[0]["children"][0]["children"][0]["name"], "offset");
        assert_ranges(&result, None);
    }
}

#[tokio::test]
async fn clients_without_hierarchical_support_receive_flat_symbols() {
    for capabilities in [
        json!({}),
        json!({"textDocument": {"documentSymbol": {"hierarchicalDocumentSymbolSupport": false}}}),
    ] {
        let (mut server, _socket) = wit_language_server::service();
        request(
            &mut server,
            "initialize",
            json!({"capabilities": capabilities}),
        )
        .await;
        notify(&mut server, "initialized", json!({})).await;
        let uri = "file:///flat.wit";
        let source = "interface api { record point { column: u32 } }";
        open(&mut server, uri, source).await;
        let result = symbols(&mut server, uri).await;
        assert_eq!(result.as_array().unwrap().len(), 3);
        assert_eq!(result[1]["kind"], 5); // Class is the legacy fallback for Struct.
        for (index, name, parent) in [
            (0, "api", None),
            (1, "point", Some("api")),
            (2, "column", Some("point")),
        ] {
            let symbol = &result[index];
            assert_eq!(symbol["name"], name);
            assert_eq!(symbol["containerName"], json!(parent));
            assert_eq!(
                symbol["location"],
                json!({"uri": uri, "range": range(source, name)})
            );
            assert!(symbol.get("children").is_none());
        }
    }
}

#[tokio::test]
async fn empty_missing_and_incomplete_documents_are_handled() {
    let (mut server, _socket) = wit_language_server::service();
    start(&mut server, true).await;
    let uri = "file:///editing.wit";
    assert_eq!(symbols(&mut server, uri).await, Value::Null);
    open(&mut server, uri, "").await;
    assert_eq!(symbols(&mut server, uri).await, json!([]));
    let source = "interface api { type first = u32; record partial {";
    notify(
        &mut server,
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": uri, "version": 2}, "contentChanges": [{"text": source}]
        }),
    )
    .await;
    let result = symbols(&mut server, uri).await;
    assert_ranges(&result, None);
    let mut names = Vec::new();
    outline(&result, "", &mut names);
    assert!(
        names.iter().any(|(name, _)| name.ends_with("first")),
        "{result}"
    );
    notify(&mut server, "textDocument/didChange", json!({
        "textDocument": {"uri": uri, "version": 3}, "contentChanges": [{"text": "interface restored {}"}]
    })).await;
    let result = symbols(&mut server, uri).await;
    assert_eq!(result.as_array().unwrap().len(), 1);
    assert_eq!(result[0]["name"], "restored");
}
