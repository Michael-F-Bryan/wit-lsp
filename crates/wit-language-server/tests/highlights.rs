mod support;

use serde_json::{Value, json};
use support::*;

async fn verify(marked: &str) {
    let (mut server, _socket) = wit_language_server::service();
    let result = request(&mut server, "initialize", json!({"capabilities": {}})).await;
    assert_eq!(result["capabilities"]["documentHighlightProvider"], true);
    notify(&mut server, "initialized", json!({})).await;
    open(
        &mut server,
        "file:///deps/types.wit",
        "package example:types; interface types { record value { x: u32 } }",
    )
    .await;
    let uri = "file:///app.wit";
    let mut text = String::new();
    let mut offsets = Vec::new();
    let mut rest = marked;
    while let Some(start) = rest.find('«') {
        text.push_str(&rest[..start]);
        rest = &rest[start + '«'.len_utf8()..];
        let end = rest.find('»').unwrap();
        offsets.push((text.len(), text.len() + end));
        text.push_str(&rest[..end]);
        rest = &rest[end + '»'.len_utf8()..];
    }
    text.push_str(rest);
    open(&mut server, uri, &text).await;
    let expected: Vec<_> = offsets
        .iter()
        .map(|&(start, end)| {
            json!({
                "range": {"start": position(&text, start), "end": position(&text, end)}, "kind": 1
            })
        })
        .collect();
    assert!(!offsets.is_empty());
    for (start, _) in offsets {
        assert_eq!(
            request(
                &mut server,
                "textDocument/documentHighlight",
                json!({
                    "textDocument": {"uri": uri}, "position": position(&text, start)
                })
            )
            .await,
            json!(expected),
            "{marked} at {start}"
        );
    }
}

#[tokio::test]
async fn highlights_respect_scope_and_exclude_textual_matches() {
    for source in [
        "interface i { record «value» { value: u32 } // value\n type x = list<«value»>; f: func(x: «value»); } interface j { type value = bool; type x = value; }",
        "world w { type value = string; import a: interface { record «value» { x: u32 } f: func(x: «value»); } export b: interface { type value = bool; f: func(x: value); } }",
        "interface i { record r { «field»: u32 } record s { field: u32 } }",
        "world «base» {} world app { include «base»; }",
    ] {
        verify(source).await;
    }
}

#[tokio::test]
async fn highlights_keep_import_alias_bindings_separate() {
    for source in [
        "interface i { use example:types/types.{value as «local»}; f: func(x: «local»); } interface j { use example:types/types.{value as local}; f: func(x: local); }",
        "interface types { type «value» = u32; } interface i { use types.{«value» as local}; type x = local; } interface j { use types.{«value»}; type x = «value»; }",
        "use example:types/types as «api»; interface i { use «api».{value}; } world w { import «api»; export «api»; }",
        "use example:types/«types» as api; world w { import api; export example:types/«types»; }",
        "world w { import a: interface { use example:types/types.{value as «local»}; f: func(x: «local»); } export b: interface { type local = bool; f: func(x: local); } }",
        "interface bridge { use example:types/types.{value as «local»}; } interface i { use bridge.{«local»}; f: func(x: «local»); }",
    ] {
        verify(source).await;
    }
}

#[tokio::test]
async fn highlights_use_utf16_ranges_for_escaped_names() {
    for newline in ["\n", "\r\n"] {
        verify(&format!("interface i {{{newline}/* é😀 */ resource «%type»;{newline}/* 😀 */ f: func(x: borrow<«%type»>) -> own<«%type»>;{newline}}}")).await;
    }
}

#[tokio::test]
async fn highlights_update_after_edits_and_ignore_unresolved_targets() {
    let (mut server, _socket) = wit_language_server::service();
    initialize(&mut server).await;
    let uri = "file:///app.wit";
    let src = "interface i { type value = u32; type x = value; }";
    open(&mut server, uri, src).await;
    for (version, text, expected_count) in [
        (2, src.to_owned(), 2),
        (3, src.replace("type x = value", "type x = u32"), 1),
        (4, src.to_owned(), 2),
    ] {
        notify(
            &mut server,
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": version}, "contentChanges": [{"text": text}]
            }),
        )
        .await;
        let result = request(&mut server, "textDocument/documentHighlight", json!({
            "textDocument": {"uri": uri}, "position": position(&text, text.find("value").unwrap())
        })).await;
        assert_eq!(result.as_array().unwrap().len(), expected_count);
    }
    let broken = "use missing:missing/types as alias; interface i { use absent.{value as local}; type x = missing; type y = list<; }";
    notify(
        &mut server,
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": uri, "version": 5}, "contentChanges": [{"text": broken}]
        }),
    )
    .await;
    for offset in [
        0,
        broken.find("missing").unwrap(),
        broken.find("alias").unwrap(),
        broken.find("local").unwrap(),
        broken.rfind("missing").unwrap(),
        broken.len(),
    ] {
        assert_eq!(
            request(
                &mut server,
                "textDocument/documentHighlight",
                json!({
                    "textDocument": {"uri": uri}, "position": position(broken, offset)
                })
            )
            .await,
            json!([])
        );
    }
    assert_eq!(request(&mut server, "textDocument/documentHighlight", json!({
        "textDocument": {"uri": "file:///unknown.wit"}, "position": {"line": 0, "character": 0}
    })).await, Value::Null);
}
