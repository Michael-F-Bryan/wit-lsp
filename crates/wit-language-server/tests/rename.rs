mod support;

use serde_json::{Value, json};
use support::*;

struct Source {
    text: String,
    ranges: Vec<std::ops::Range<usize>>,
}

impl Source {
    fn marked(source: &str) -> Self {
        let mut text = String::new();
        let mut ranges = Vec::new();
        let mut rest = source;
        while let Some(start) = rest.find('«') {
            text.push_str(&rest[..start]);
            rest = &rest[start + '«'.len_utf8()..];
            let end = rest.find('»').unwrap();
            ranges.push(text.len()..text.len() + end);
            text.push_str(&rest[..end]);
            rest = &rest[end + '»'.len_utf8()..];
        }
        text.push_str(rest);
        Self { text, ranges }
    }

    fn renamed(&self, replacement: &str) -> String {
        let mut text = self.text.clone();
        for range in self.ranges.iter().rev() {
            text.replace_range(range.clone(), replacement);
        }
        text
    }
}

async fn verify(fixtures: &[(&str, &str)], new_name: &str, replacement: &str) {
    let (mut server, _socket) = wit_language_server::service();
    let caps = request(
        &mut server,
        "initialize",
        json!({"capabilities": {"workspace": {"workspaceEdit": {"documentChanges": true}}}}),
    )
    .await;
    assert_eq!(
        caps["capabilities"]["renameProvider"]["prepareProvider"],
        true
    );
    notify(&mut server, "initialized", json!({})).await;
    let mut sources: Vec<_> = fixtures
        .iter()
        .map(|(uri, text)| (*uri, Source::marked(text)))
        .collect();
    sources.sort_by_key(|(uri, _)| *uri);
    for (uri, source) in sources.iter().rev() {
        open(&mut server, uri, &source.text).await;
    }
    let expected: Vec<_> = sources.iter().filter(|(_, s)| !s.ranges.is_empty()).map(|(uri, source)| {
        let edits: Vec<_> = source.ranges.iter().map(|r| json!({
            "range": {"start": position(&source.text, r.start), "end": position(&source.text, r.end)},
            "newText": replacement,
        })).collect();
        json!({"textDocument": {"uri": uri, "version": 1}, "edits": edits})
    }).collect();
    for (uri, source) in &sources {
        for range in &source.ranges {
            let params = json!({"textDocument": {"uri": uri}, "position": position(&source.text, range.start)});
            assert_eq!(
                request(&mut server, "textDocument/prepareRename", params.clone()).await,
                json!({
                    "range": {"start": position(&source.text, range.start), "end": position(&source.text, range.end)},
                    "placeholder": source.text[range.clone()].trim_start_matches('%'),
                })
            );
            let mut params = params;
            params["newName"] = json!(new_name);
            assert_eq!(
                request(&mut server, "textDocument/rename", params).await,
                json!({"documentChanges": expected}),
                "{uri} {range:?}"
            );
        }
    }
    for (uri, source) in &sources {
        notify(&mut server, "textDocument/didChange", json!({
            "textDocument": {"uri": uri, "version": 2}, "contentChanges": [{"text": source.renamed(replacement)}]
        })).await;
    }
    for (uri, _) in &sources {
        assert_eq!(diagnostics(&mut server, uri).await, json!([]), "{uri}");
    }
}

#[tokio::test]
async fn renames_scoped_types_and_preserves_unrelated_names() {
    verify(&[("file:///types.wit", "interface first { record «point» { point: u32 } // point\n type points = list<«point»>; f: func(point: «point») -> «point»; } interface second { type point = u32; type x = point; }")], "position", "position").await;
}

#[tokio::test]
async fn original_type_rename_stops_at_explicit_aliases() {
    verify(&[
        ("file:///dep/types.wit", "package example:types; interface types { record «point» { x: u32 } }"),
        ("file:///bridge/types.wit", "package example:bridge; interface bridge { use example:types/types.{«point» as local}; type x = local; }"),
        ("file:///app/main.wit", "interface app { use example:bridge/bridge.{local as mine}; type x = mine; }")
    ], "position", "position").await;
}

#[tokio::test]
async fn alias_rename_follows_reexports_without_renaming_the_source() {
    verify(&[
        ("file:///dep/types.wit", "package example:types; interface types { record point { x: u32 } }"),
        ("file:///bridge/types.wit", "package example:bridge; interface bridge { use example:types/types.{point as «local»}; type x = «local»; }"),
        ("file:///app/main.wit", "interface app { use example:bridge/bridge.{«local» as mine}; type x = mine; }")
    ], "position", "position").await;
}

#[tokio::test]
async fn implicit_imports_follow_the_original_binding() {
    verify(&[
        ("file:///dep/types.wit", "package example:types; interface types { type «value» = u32; }"),
        ("file:///bridge/types.wit", "package example:bridge; interface bridge { use example:types/types.{«value»}; type x = «value»; }"),
        ("file:///app/main.wit", "interface app { use example:bridge/bridge.{«value»}; type x = «value»; }")
    ], "count", "count").await;
}

#[tokio::test]
async fn interface_and_world_names_preserve_file_aliases_and_package_segments() {
    verify(&[
        ("file:///dep/types.wit", "package types:types@1.2.3; interface «types» { type value = u32; }"),
        ("file:///app/main.wit", "use types:types/«types»@1.2.3 as t; interface app { use t.{value}; } world w { import t; export types:types/«types»@1.2.3; }")
    ], "api", "api").await;
    verify(&[
        ("file:///dep/types.wit", "package example:types; interface types { type value = u32; }"),
        ("file:///app/main.wit", "use example:types/types as «t»; use «t» as other; interface app { use «t».{value}; } world w { import «t»; export other; }")
    ], "api", "api").await;
    verify(
        &[(
            "file:///worlds.wit",
            "world «base» { import run: func(); } use «base» as b; world app { include b; }",
        )],
        "foundation",
        "foundation",
    )
    .await;
}

#[tokio::test]
async fn member_renames_do_not_touch_other_scopes_or_include_aliases() {
    for source in [
        "interface i { record r { «field»: u32 } record s { field: u32 } }",
        "interface i { enum e { «case» } enum f { case } }",
        "interface i { flags f { «flag» } flags g { flag } }",
        "interface i { variant v { «case»(u32) } }",
        "interface i { resource r { «method»: func(); } }",
        "interface i { «function»: func(); }",
        "interface i { function: func(«argument»: u32); }",
        "world base { import «run»: func(); } world app { include base with { «run» as renamed }; }",
        "world base { import run: func(); } world app { include base with { run as «renamed» }; }",
    ] {
        verify(&[("file:///members.wit", source)], "updated", "updated").await;
    }
}

#[tokio::test]
async fn keyword_names_are_escaped_and_ranges_use_utf16() {
    for newline in ["\n", "\r\n"] {
        let source = format!(
            "interface i {{{newline}/* é😀 */ resource «file»;{newline}read: func(f: borrow<«file»>) -> own<«file»>;{newline}}}"
        );
        verify(&[("file:///unicode.wit", &source)], "type", "%type").await;
    }
    verify(
        &[(
            "file:///types.wit",
            "interface i { type «%type» = u32; type x = «%type»; }",
        )],
        "%interface",
        "%interface",
    )
    .await;
    verify(
        &[(
            "file:///types.wit",
            "interface i { type «value» = u32; type x = «value»; }",
        )],
        "u32",
        "%u32",
    )
    .await;
}

#[tokio::test]
async fn collisions_and_invalid_names_are_rejected_without_editing() {
    for (source, new_name) in [
        (
            "world app { import api: interface { type |value = u32; type occupied = bool; } }",
            "occupied",
        ),
        (
            "interface types { type |value = u32; } world app { import api: interface { use types.{value}; type occupied = bool; f: func(v: value); } }",
            "occupied",
        ),
        (
            "world app { export api: interface { record r { |value: u32, occupied: bool } } }",
            "occupied",
        ),
        (
            "interface i { type |value = u32; type occupied = u32; }",
            "occupied",
        ),
        (
            "interface i { record r { |value: u32, occupied: u32 } }",
            "occupied",
        ),
        (
            "interface types { type |value = u32; } interface consumer { use types.{value}; type occupied = bool; type x = value; }",
            "occupied",
        ),
        (
            "interface |types {} interface other {} world app { import types; export other; }",
            "other",
        ),
        ("interface i { type |value = u32; }", ""),
        ("interface i { type |value = u32; }", "two words"),
        ("interface i { type |value = u32; }", "a.b"),
        ("interface i { type |value = u32; }", "camelCase"),
        ("interface i { type |value = u32; }", "snake_case"),
        ("interface i { type |value = u32; }", "value--name"),
        (
            "interface i { type |value = u32; }",
            "x; type injected = u32",
        ),
        ("interface i { type |value = u32; }", " name "),
    ] {
        let (mut server, mut socket) = wit_language_server::service();
        initialize(&mut server).await;
        let offset = source.find('|').unwrap();
        let source = source.replace('|', "");
        let uri = "file:///collision.wit";
        open(&mut server, uri, &source).await;
        let before = diagnostics(&mut server, uri).await;
        rejected_rename(&mut server, &mut socket, json!({
            "textDocument": {"uri": uri}, "position": position(&source, offset), "newName": new_name
        })).await;
        assert_eq!(diagnostics(&mut server, uri).await, before);
    }
}

#[tokio::test]
async fn preparation_rejects_non_symbols_and_unresolved_names() {
    let (mut server, mut socket) = wit_language_server::service();
    initialize(&mut server).await;
    let source = "package example:test; interface i { type value = u32; type x = missing; }";
    let uri = "file:///types.wit";
    open(&mut server, uri, source).await;
    for token in ["example", "u32", "missing", "interface"] {
        let params = json!({"textDocument": {"uri": uri}, "position": position(source, source.find(token).unwrap())});
        assert_eq!(
            request(&mut server, "textDocument/prepareRename", params.clone()).await,
            Value::Null
        );
        let mut params = params;
        params["newName"] = json!("updated");
        rejected_rename(&mut server, &mut socket, params).await;
    }
}

#[tokio::test]
async fn aliases_with_the_same_spelling_remain_distinct_bindings() {
    verify(&[("file:///types.wit", "interface types { type value = u32; } interface consumer { use types.{value as «value»}; type x = «value»; }")], "renamed", "renamed").await;
    verify(&[("file:///types.wit", "interface types { type «value» = u32; } interface consumer { use types.{«value» as value}; type x = value; }")], "renamed", "renamed").await;
    verify(&[("file:///types.wit", "interface types { type value = u32; } interface consumer { use types.{value as imported}; type «value» = bool; type x = «value»; type y = imported; }")], "renamed", "renamed").await;
}

#[tokio::test]
async fn no_op_and_identifier_names_used_by_validation_are_supported() {
    for name in ["value", "rename-test", "result-type", "HTTP-response"] {
        verify(
            &[(
                "file:///types.wit",
                "interface i { type «value» = u32; type x = «value»; }",
            )],
            name,
            name,
        )
        .await;
    }
}

#[tokio::test]
async fn edits_use_current_versions_and_support_older_clients() {
    for versioned in [false, true] {
        let (mut server, _socket) = wit_language_server::service();
        request(
            &mut server,
            "initialize",
            json!({"capabilities": {
                "workspace": {"workspaceEdit": {"documentChanges": versioned}}
            }}),
        )
        .await;
        notify(&mut server, "initialized", json!({})).await;
        let uri = "file:///types.wit";
        open(&mut server, uri, "interface i {}").await;
        let source = "interface i { type value = u32; }";
        notify(
            &mut server,
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": 7}, "contentChanges": [{"text": source}]
            }),
        )
        .await;
        let edit = request(&mut server, "textDocument/rename", json!({
            "textDocument": {"uri": uri}, "position": position(source, source.find("value").unwrap()), "newName": "count"
        })).await;
        let edits = json!([{"range": range(source, "value"), "newText": "count"}]);
        if versioned {
            assert_eq!(
                edit,
                json!({"documentChanges": [{"textDocument": {"uri": uri, "version": 7}, "edits": edits}]})
            );
        } else {
            assert_eq!(edit, json!({"changes": {uri: edits}}));
        }
    }
}

#[tokio::test]
async fn inline_interface_renames_preserve_sibling_and_world_bindings() {
    verify(&[("file:///inline.wit", "world app { type value = u32; import api: interface { record «value» { x: u32 } f: func(v: «value»); } export api: interface { type value = bool; f: func(v: value); } }")], "updated", "updated").await;
    verify(&[("file:///inline.wit", "interface types { type «value» = u32; } world app { import api: interface { type value = bool; f: func(v: value); } }")], "updated", "updated").await;
}

#[tokio::test]
async fn inline_imports_follow_shared_alias_binding_rules() {
    verify(&[
        ("file:///deps/types.wit", "package example:types; interface types { type «value» = u32; }"),
        ("file:///app/main.wit", "world app { import api: interface { use example:types/types.{«value»}; f: func(v: «value»); } export api: interface { use example:types/types.{«value» as local}; f: func(v: local); } }")
    ], "updated", "updated").await;
    verify(&[("file:///inline.wit", "interface types { type value = u32; } world app { import api: interface { use types.{value as «local»}; f: func(v: «local»); } export api: interface { use types.{value as local}; f: func(v: local); } }")], "updated", "updated").await;
}

#[tokio::test]
async fn unrelated_existing_errors_do_not_block_rename() {
    let (mut server, _socket) = wit_language_server::service();
    initialize(&mut server).await;
    open(
        &mut server,
        "file:///broken.wit",
        "interface broken { type duplicate = u32; type duplicate = u32; type unknown = missing; }",
    )
    .await;
    let uri = "file:///valid.wit";
    let source = "interface i { type value = u32; }";
    open(&mut server, uri, source).await;
    let edit = request(&mut server, "textDocument/rename", json!({
        "textDocument": {"uri": uri}, "position": position(source, source.find("value").unwrap()), "newName": "updated"
    })).await;
    assert_eq!(
        edit["changes"][uri],
        json!([{"range": range(source, "value"), "newText": "updated"}])
    );
}

#[tokio::test]
async fn invalid_name_explains_the_rule_and_success_sends_no_warning() {
    use futures::{FutureExt, StreamExt};
    let (mut server, mut socket) = wit_language_server::service();
    initialize(&mut server).await;
    let uri = "file:///rename.wit";
    let source = "interface api { record source-reference { line: u32 } type reference = option<source-reference>; }";
    open(&mut server, uri, source).await;
    let params = json!({"textDocument": {"uri": uri},
        "position": position(source, source.rfind("source-reference").unwrap()),
        "newName": "sourceReference"});
    let reason = rejected_rename(&mut server, &mut socket, params.clone()).await;
    assert!(reason.contains("hyphen-separated"), "{reason}");
    assert!(reason.contains("lowercase or uppercase"), "{reason}");
    assert_eq!(diagnostics(&mut server, uri).await, json!([]));

    let mut params = params;
    params["newName"] = json!("source-ref");
    let edits = request(&mut server, "textDocument/rename", params).await;
    assert_eq!(edits["changes"][uri].as_array().unwrap().len(), 2);
    assert!(
        socket.next().now_or_never().is_none(),
        "Successful rename sent a notification"
    );
}
