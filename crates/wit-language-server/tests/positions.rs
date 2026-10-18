mod support;

use serde_json::{Value, json};
use support::*;

#[tokio::test]
async fn unicode_navigation_completion_diagnostics_and_selections() {
    for newline in ["\n", "\r\n"] {
        let (mut server, _socket) = wit_language_server::service();
        initialize(&mut server).await;
        let source_uri = "file:///types.wit";
        let consumer_uri = "file:///consumer.wit";
        let source = format!(
            "interface types {{{newline}/* é😀 */ record position {{ line: u32 }}{newline}}}"
        );
        let consumer = format!(
            "interface consumer {{{newline}use types.{{position as location}};{newline}/* é😀 */ type alias = location;{newline}}}"
        );
        open(&mut server, source_uri, &source).await;
        open(&mut server, consumer_uri, &consumer).await;
        assert_eq!(diagnostics(&mut server, consumer_uri).await, json!([]));

        let offset = consumer.rfind("location").unwrap();
        let params = json!({"textDocument": {"uri": consumer_uri}, "position": position(&consumer, offset + 3)});
        let definition = request(&mut server, "textDocument/definition", params.clone()).await;
        assert_eq!(
            definition,
            json!({"uri": source_uri, "range": range(&source, "record position { line: u32 }")})
        );
        let completion = request(&mut server, "textDocument/completion", params).await;
        assert!(
            completion
                .as_array()
                .unwrap()
                .iter()
                .any(|item| item["label"] == "location")
        );

        let selection = request(&mut server, "textDocument/selectionRange", json!({
            "textDocument": {"uri": consumer_uri}, "positions": [position(&consumer, offset + 3)]
        })).await;
        assert_eq!(
            selection[0]["range"],
            json!({"start": position(&consumer, offset), "end": position(&consumer, offset + "location".len())}),
            "selection={selection}"
        );

        let broken = consumer.replacen("type alias = location", "type alias = missing", 1);
        notify(
            &mut server,
            "textDocument/didChange",
            json!({"textDocument": {"uri": consumer_uri, "version": 2},
            "contentChanges": [{"text": broken}]}),
        )
        .await;
        let errors = diagnostics(&mut server, consumer_uri).await;
        assert_eq!(errors.as_array().unwrap().len(), 1, "{errors}");
        assert_eq!(errors[0]["code"], "E004");
        assert_eq!(errors[0]["range"], range(&broken, "missing"));
        notify(
            &mut server,
            "textDocument/didChange",
            json!({"textDocument": {"uri": consumer_uri, "version": 3},
            "contentChanges": [{"text": consumer}]}),
        )
        .await;
        assert_eq!(diagnostics(&mut server, consumer_uri).await, json!([]));
    }
}

#[tokio::test]
async fn unicode_code_action_edits_use_utf16_ranges() {
    let (mut server, _socket) = wit_language_server::service();
    initialize(&mut server).await;
    let uri = "file:///edits.wit";
    let src = "interface example {\r\n/* é😀 */ f: func(x: list<list<u32>>);\r\n}";
    open(&mut server, uri, src).await;
    let offset = src.find("list<list").unwrap();
    let actions = request(
        &mut server,
        "textDocument/codeAction",
        json!({
            "textDocument": {"uri": uri},
            "range": {"start": position(src, offset + 1), "end": position(src, offset + 1)},
            "context": {"diagnostics": []}
        }),
    )
    .await;
    let action = actions
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["title"] == "Extract Type Alias")
        .unwrap();
    let edits = action["edit"]["changes"][uri].as_array().unwrap();
    assert_eq!(edits.len(), 2);
    assert!(
        edits
            .iter()
            .any(|edit| edit["range"] == range(src, "list<list<u32>>")
                && edit["newText"] == "list-list-u32"),
        "{edits:?}"
    );
    let insertion = position(src, src.find("f: func").unwrap());
    assert!(
        edits
            .iter()
            .any(|edit| edit["range"] == json!({"start": insertion, "end": insertion})),
        "{edits:?}"
    );
}

#[tokio::test]
async fn invalid_positions_return_errors_without_breaking_the_server() {
    let (mut server, _socket) = wit_language_server::service();
    initialize(&mut server).await;
    let uri = "file:///invalid.wit";
    let src = "/* 😀 */ interface example { type value = u32; }";
    open(&mut server, uri, src).await;
    for position in [
        json!({"line": 0, "character": 4}),
        json!({"line": 99, "character": 0}),
    ] {
        for method in [
            "textDocument/completion",
            "textDocument/definition",
            "textDocument/hover",
            "textDocument/references",
        ] {
            let response = call(
                &mut server,
                method,
                json!({"textDocument": {"uri": uri}, "position": position,
                    "context": {"includeDeclaration": false}}),
            )
            .await;
            assert_eq!(response.error().unwrap().code.code(), -32602);
        }
    }
    assert_eq!(diagnostics(&mut server, uri).await, json!([]));
}

#[tokio::test]
async fn related_diagnostics_and_folding_use_utf16_ranges() {
    let (mut server, _socket) = wit_language_server::service();
    initialize(&mut server).await;
    let uri = "file:///related.wit";
    let src = "/* é😀 */ interface example { type same = u32; /* β😀 */ type same = string; }";
    open(&mut server, uri, src).await;
    let errors = diagnostics(&mut server, uri).await;
    assert_eq!(errors.as_array().unwrap().len(), 1, "{errors}");
    assert_eq!(errors[0]["range"], range(src, "type same = string;"));
    assert_eq!(
        errors[0]["relatedInformation"][0]["location"],
        json!({
            "uri": uri, "range": range(src, "type same = u32;")
        })
    );
    let folds = request(
        &mut server,
        "textDocument/foldingRange",
        json!({"textDocument": {"uri": uri}}),
    )
    .await;
    let body = range(src, &src[src.find('{').unwrap()..]);
    assert!(
        folds.as_array().unwrap().iter().any(|fold| {
            fold["startLine"] == body["start"]["line"]
                && fold["startCharacter"] == body["start"]["character"]
                && fold["endLine"] == body["end"]["line"]
                && fold["endCharacter"] == body["end"]["character"]
        }),
        "{folds}"
    );
}

#[tokio::test]
async fn import_navigation_uses_shared_resolution_and_utf16_positions() {
    let (mut server, _socket) = wit_language_server::service();
    initialize(&mut server).await;
    let source_uri = "file:///deps/types.wit";
    let consumer_uri = "file:///app/main.wit";
    let source = "package example:remote@1.2.3;\r\n/* é😀 */ interface types { record position { line: u32 } }\r\nworld base { import log: func(); }";
    let consumer = "/* é😀 */ use example:remote/types@1.2.3 as remote-types;\r\ninterface consumer { use remote-types.{position as location}; type alias = location; }\r\nworld w { import remote-types; export remote-types; include example:remote/base@1.2.3 with {log as logger}; }";
    open(&mut server, source_uri, source).await;
    open(&mut server, consumer_uri, consumer).await;
    assert_eq!(diagnostics(&mut server, consumer_uri).await, json!([]));
    let interface = "interface types { record position { line: u32 } }";
    for (marked, expected) in [
        ("use |example:remote/types", interface),
        ("types@1.2.3 as |remote-types", interface),
        ("use |remote-types.{", interface),
        ("remote-types.{|position", "record position { line: u32 }"),
        ("position as |location", "record position { line: u32 }"),
        ("type alias = |location", "record position { line: u32 }"),
        ("import |remote-types", interface),
        ("export |remote-types", interface),
        ("example:remote/|base", "world base { import log: func(); }"),
        ("with {|log", "import log: func();"),
        ("log as |logger", "import log: func();"),
    ] {
        let offset = consumer.find(&marked.replace('|', "")).unwrap() + marked.find('|').unwrap();
        let definition = request(
            &mut server,
            "textDocument/definition",
            json!({
                "textDocument": {"uri": consumer_uri}, "position": position(consumer, offset)
            }),
        )
        .await;
        assert_eq!(
            definition,
            json!({"uri": source_uri, "range": range(source, expected)}),
            "{marked}"
        );
    }

    let renamed = source.replace("record position", "record renamed");
    notify(
        &mut server,
        "textDocument/didChange",
        json!({"textDocument": {"uri": source_uri, "version": 2},
        "contentChanges": [{"text": renamed}]}),
    )
    .await;
    let missing = request(&mut server, "textDocument/definition", json!({
        "textDocument": {"uri": consumer_uri}, "position": position(consumer, consumer.find("position as").unwrap())
    })).await;
    assert!(missing.is_null());
    assert!(
        !diagnostics(&mut server, consumer_uri)
            .await
            .as_array()
            .unwrap()
            .is_empty()
    );
    notify(
        &mut server,
        "textDocument/didChange",
        json!({"textDocument": {"uri": source_uri, "version": 3},
        "contentChanges": [{"text": source}]}),
    )
    .await;
    assert_eq!(diagnostics(&mut server, consumer_uri).await, json!([]));
}

#[tokio::test]
async fn hover_resolves_imports_with_utf16_ranges_and_refreshes_after_edits() {
    for newline in ["\n", "\r\n"] {
        let (mut server, _socket) = wit_language_server::service();
        let result = request(
            &mut server,
            "initialize",
            json!({"capabilities": {
                "textDocument": {"hover": {"contentFormat": ["markdown", "plaintext"]}}
            }}),
        )
        .await;
        assert_eq!(result["capabilities"]["hoverProvider"], true);
        notify(&mut server, "initialized", json!({})).await;
        let source_uri = "file:///deps/types.wit";
        let consumer_uri = "file:///app/main.wit";
        let source = format!(
            "package example:remote@1.2.3;{newline}/// Types.{newline}interface types {{{newline}/// A **position**. 😀{newline}record position {{ line: u32 }}{newline}}}"
        );
        let consumer = format!(
            "/* é😀 */ use example:remote/types@1.2.3 as remote-types;{newline}interface consumer {{ use remote-types.{{position as location}};{newline}/* é😀 */ type alias = location; }}"
        );
        open(&mut server, source_uri, &source).await;
        open(&mut server, consumer_uri, &consumer).await;
        for (marked, token, expected) in [
            (
                "as |remote-types",
                "remote-types",
                "```wit\ninterface types { … }\n```\n\nTypes.",
            ),
            (
                "remote-types.{|position",
                "position",
                "```wit\nrecord position { line: u32 }\n```\n\nA **position**. 😀",
            ),
            (
                "position as |location",
                "location",
                "```wit\nrecord position { line: u32 }\n```\n\nA **position**. 😀",
            ),
            (
                "alias = |location",
                "location",
                "```wit\nrecord position { line: u32 }\n```\n\nA **position**. 😀",
            ),
        ] {
            let offset =
                consumer.find(&marked.replace('|', "")).unwrap() + marked.find('|').unwrap();
            let hover = request(&mut server, "textDocument/hover", json!({
                "textDocument": {"uri": consumer_uri}, "position": position(&consumer, offset + 1)
            })).await;
            assert_eq!(
                hover,
                json!({
                    "contents": {"kind": "markdown", "value": expected},
                    "range": {"start": position(&consumer, offset), "end": position(&consumer, offset + token.len())}
                }),
                "{marked}"
            );
        }
        let params = json!({"textDocument": {"uri": consumer_uri}, "position": position(&consumer, consumer.rfind("location").unwrap())});
        let revised = source
            .replace("A **position**.", "Updated docs.")
            .replace("line: u32", "line: u64");
        notify(&mut server, "textDocument/didChange", json!({
            "textDocument": {"uri": source_uri, "version": 2}, "contentChanges": [{"text": revised}]
        })).await;
        let hover = request(&mut server, "textDocument/hover", params.clone()).await;
        assert_eq!(
            hover["contents"]["value"],
            "```wit\nrecord position { line: u64 }\n```\n\nUpdated docs. 😀"
        );
        notify(&mut server, "textDocument/didChange", json!({
            "textDocument": {"uri": source_uri, "version": 3}, "contentChanges": [{"text": source.replace("record position", "record renamed")}]
        })).await;
        assert_eq!(
            request(&mut server, "textDocument/hover", params.clone()).await,
            Value::Null
        );
        notify(&mut server, "textDocument/didChange", json!({
            "textDocument": {"uri": source_uri, "version": 4}, "contentChanges": [{"text": source}]
        })).await;
        assert_eq!(
            request(&mut server, "textDocument/hover", params).await["contents"]["value"],
            "```wit\nrecord position { line: u32 }\n```\n\nA **position**. 😀"
        );
    }
}

#[tokio::test]
async fn hover_defaults_to_plain_text_and_handles_missing_targets() {
    for formats in [json!(null), json!(["plaintext", "markdown"])] {
        let (mut server, _socket) = wit_language_server::service();
        request(
            &mut server,
            "initialize",
            json!({"capabilities": {
                "textDocument": {"hover": {"contentFormat": formats}}
            }}),
        )
        .await;
        notify(&mut server, "initialized", json!({})).await;
        let uri = "file:///types.wit";
        let src = "interface types {\n/// A count.\ntype count = u32;\n/* é😀 */ type x = count; type y = missing; }";
        open(&mut server, uri, src).await;
        for offset in [src.find("count =").unwrap(), src.rfind("count;").unwrap()] {
            let hover = request(
                &mut server,
                "textDocument/hover",
                json!({
                    "textDocument": {"uri": uri}, "position": position(src, offset)
                }),
            )
            .await;
            assert_eq!(
                hover,
                json!({
                    "contents": {"kind": "plaintext", "value": "type count = u32;\n\nA count."},
                    "range": {"start": position(src, offset), "end": position(src, offset + 5)}
                })
            );
        }
        for (uri, offset) in [
            (uri, src.find("missing").unwrap()),
            (uri, src.find("count;").unwrap() + 5),
            ("file:///unopened.wit", 0),
        ] {
            assert_eq!(
                request(
                    &mut server,
                    "textDocument/hover",
                    json!({
                        "textDocument": {"uri": uri}, "position": position(src, offset)
                    })
                )
                .await,
                Value::Null
            );
        }
    }
}
