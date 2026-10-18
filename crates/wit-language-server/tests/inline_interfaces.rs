mod support;

use serde_json::{Value, json};
use support::*;

#[tokio::test]
async fn inline_navigation_completion_and_diagnostics_track_buffer_edits() {
    for newline in ["\n", "\r\n"] {
        let (mut server, _socket) = wit_language_server::service();
        initialize(&mut server).await;
        let uri = "file:///inline.wit";
        let source = "interface types { type remote = u32; }\nworld app {\n type outer = u32;\n import api: interface {\n  use types.{remote as imported};\n  /* é😀 */ record local { field: imported }\n  f: func(value: local);\n }\n export api: interface {\n  type sibling = bool;\n  type local = string;\n  f: func(value: local);\n }\n}".replace('\n', newline);
        open(&mut server, uri, &source).await;
        assert_eq!(diagnostics(&mut server, uri).await, json!([]));
        for (offset, declaration) in [
            (
                source.find("value: local").unwrap() + 7,
                "record local { field: imported }",
            ),
            (
                source.rfind("value: local").unwrap() + 7,
                "type local = string;",
            ),
            (
                source.find("field: imported").unwrap() + 7,
                "type remote = u32;",
            ),
        ] {
            let params =
                json!({"textDocument": {"uri": uri}, "position": position(&source, offset)});
            assert_eq!(
                request(&mut server, "textDocument/definition", params.clone()).await,
                json!({"uri": uri, "range": range(&source, declaration)})
            );
            let hover = request(&mut server, "textDocument/hover", params).await;
            assert!(
                hover["contents"]["value"]
                    .as_str()
                    .unwrap()
                    .contains(declaration),
                "{hover}"
            );
        }
        let completion = request(&mut server, "textDocument/completion", json!({
            "textDocument": {"uri": uri}, "position": position(&source, source.find("/* é😀 */").unwrap())
        })).await;
        let labels: Vec<_> = completion
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| item["label"].as_str())
            .collect();
        for name in ["local", "imported"] {
            assert!(labels.contains(&name), "{labels:?}");
        }
        for name in ["outer", "sibling", "remote"] {
            assert!(!labels.contains(&name), "{labels:?}");
        }

        let broken = source.replace(
            "record local { field: imported }",
            "record renamed { field: imported }",
        );
        notify(
            &mut server,
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": 2}, "contentChanges": [{"text": broken}]
            }),
        )
        .await;
        let errors = diagnostics(&mut server, uri).await;
        assert_eq!(errors.as_array().unwrap().len(), 1, "{errors}");
        assert_eq!(errors[0]["code"], "E004");
        let offset = broken.find("value: local").unwrap() + 7;
        let params = json!({"textDocument": {"uri": uri}, "position": position(&broken, offset)});
        assert_eq!(
            request(&mut server, "textDocument/definition", params).await,
            Value::Null
        );
        assert_eq!(
            errors[0]["range"],
            json!({"start": position(&broken, offset), "end": position(&broken, offset + 5)})
        );
        notify(
            &mut server,
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": 3}, "contentChanges": [{"text": source}]
            }),
        )
        .await;
        assert_eq!(diagnostics(&mut server, uri).await, json!([]));
    }
}

#[tokio::test]
async fn inline_bodies_report_unknown_names_and_duplicates() {
    for source in [
        "world app { type outer = u32; import api: interface { type x = outer; } }",
        "world app { import a: interface { type sibling = u32; } export b: interface { type x = sibling; } }",
        "interface types {} world app { import api: interface { use types.{missing}; } }",
        "world app { export api: interface { type repeated = u32; type repeated = bool; } }",
    ] {
        let (mut server, _socket) = wit_language_server::service();
        initialize(&mut server).await;
        let uri = "file:///inline.wit";
        open(&mut server, uri, source).await;
        let errors = diagnostics(&mut server, uri).await;
        assert_eq!(errors.as_array().unwrap().len(), 1, "{source}: {errors}");
    }
}
