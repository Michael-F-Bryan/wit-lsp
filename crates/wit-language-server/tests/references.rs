mod support;

use serde_json::{Value, json};
use support::*;
use wit_language_server::LanguageServerService;

struct Source {
    text: String,
    occurrences: Vec<(usize, usize, bool)>,
}

impl Source {
    // «name» is a reference, ‹name› is a declaration or an explicit alias binding.
    fn marked(source: &str) -> Self {
        let mut text = String::new();
        let mut occurrences = Vec::new();
        let mut rest = source;
        while let Some(offset) = rest.find(['«', '‹']) {
            text.push_str(&rest[..offset]);
            let declaration = rest[offset..].starts_with('‹');
            let (opening, closing) = if declaration {
                ('‹', '›')
            } else {
                ('«', '»')
            };
            rest = &rest[offset + opening.len_utf8()..];
            let end = rest.find(closing).unwrap();
            occurrences.push((text.len(), end, declaration));
            text.push_str(&rest[..end]);
            rest = &rest[end + closing.len_utf8()..];
        }
        text.push_str(rest);
        Self { text, occurrences }
    }

    fn locations(&self, uri: &str, declarations: bool) -> Vec<Value> {
        self.occurrences
            .iter()
            .filter(|(_, _, decl)| declarations || !decl)
            .map(|&(offset, len, _)| {
                json!({"uri": uri, "range": {
                    "start": position(&self.text, offset), "end": position(&self.text, offset + len)
                }})
            })
            .collect()
    }
}

async fn references(
    server: &mut impl LanguageServerService,
    uri: &str,
    text: &str,
    offset: usize,
    declarations: bool,
) -> Value {
    request(
        server,
        "textDocument/references",
        json!({
            "textDocument": {"uri": uri}, "position": position(text, offset),
            "context": {"includeDeclaration": declarations}
        }),
    )
    .await
}

async fn verify(fixtures: &[(&str, &str)]) {
    let (mut server, _socket) = wit_language_server::service();
    let caps = request(&mut server, "initialize", json!({"capabilities": {}})).await;
    assert_eq!(caps["capabilities"]["referencesProvider"], true);
    notify(&mut server, "initialized", json!({})).await;
    let mut sources: Vec<_> = fixtures
        .iter()
        .map(|(uri, text)| (*uri, Source::marked(text)))
        .collect();
    // Deliberately open in a different order from the expected URI-sorted results.
    for (uri, source) in sources.iter().rev() {
        open(&mut server, uri, &source.text).await;
    }
    sources.sort_by_key(|(uri, _)| *uri);
    for declarations in [false, true] {
        let expected: Vec<_> = sources
            .iter()
            .flat_map(|(uri, source)| source.locations(uri, declarations))
            .collect();
        for (uri, source) in &sources {
            for &(offset, _, _) in &source.occurrences {
                assert_eq!(
                    references(&mut server, uri, &source.text, offset, declarations).await,
                    json!(expected),
                    "{uri} at {offset}, declarations={declarations}"
                );
            }
        }
    }
}

#[tokio::test]
async fn local_references_obey_scope_and_ignore_comments_and_field_names() {
    verify(&[(
        "file:///types.wit",
        r#"
interface first {
    record ‹point› { point: u32 }
    // point is mentioned here but is not a reference.
    type alias = list<«point»>;
    f: func(p: «point») -> «point»;
    type alias-list = list<alias>;
}
interface second { record point { x: u32 } type x = point; }
"#,
    )])
    .await;
    verify(&[("file:///types.wit", "interface first { type ‹alias› = u32; type x = «alias»; } interface second { type alias = bool; type x = alias; }")]).await;
}

#[tokio::test]
async fn imported_aliases_and_transitive_imports_resolve_to_the_source() {
    verify(&[
        ("file:///deps/types.wit", "package example:types@1.2.3; interface types { record ‹point› { x: u32 } }"),
        ("file:///bridge/types.wit", "package example:bridge; use example:types/types@1.2.3 as remote; interface bridge { use remote.{«point» as ‹other›}; type x = «other»; }"),
        ("file:///app/main.wit", "use example:bridge/bridge as b; interface app { use b.{«other» as ‹local›}; type x = «local»; } interface unrelated { type local = bool; type x = local; }")
    ]).await;
}

#[tokio::test]
async fn interface_paths_match_only_the_final_qualified_segment() {
    verify(&[
        ("file:///deps/types.wit", "package types:types@1.2.3; interface ‹types› { type value = u32; }"),
        ("file:///app/main.wit", "use types:types/«types»@1.2.3 as ‹t›; interface app { use «t».{value}; } world w { import «t»; export types:types/«types»@1.2.3; }")
    ]).await;
}

#[tokio::test]
async fn worlds_and_included_member_renames_have_references() {
    verify(&[
        (
            "file:///deps/world.wit",
            "package example:worlds; world ‹base› { import run: func(); }",
        ),
        (
            "file:///app/main.wit",
            "use example:worlds/«base» as ‹base-world›; world app { include «base-world»; }",
        ),
    ])
    .await;
    verify(&[
        ("file:///deps/world.wit", "package example:worlds; world base { import ‹run›: func(); export other: func(); }"),
        ("file:///app/main.wit", "world app { include example:worlds/base with { «run» as ‹renamed› }; } world unrelated { import run: func(); }")
    ]).await;
}

#[tokio::test]
async fn handles_escaped_names_and_unicode_positions() {
    for newline in ["\n", "\r\n"] {
        let source = format!(
            "interface i {{{newline}/* é😀 */ resource ‹%type›;{newline}/* β😀 */ read: func(x: borrow<«%type»>) -> own<«%type»>;{newline}}}"
        );
        verify(&[("file:///unicode.wit", &source)]).await;
    }
}

#[tokio::test]
async fn cyclic_imports_and_incomplete_documents_do_not_break_references() {
    verify(&[
        ("file:///valid.wit", "interface types { type ‹value› = u32; type x = «value»; }"),
        ("file:///cycles.wit", "interface a { use b.{value}; type x = value; } interface b { use a.{value}; type y = value; }"),
        ("file:///incomplete.wit", "interface broken { use missing.{value}; type x = list<; record"),
    ]).await;
}

#[tokio::test]
async fn non_referencable_members_only_return_their_own_declaration() {
    for source in [
        "interface i { record r { ‹field›: u32 } record s { field: u32 } }",
        "interface i { ‹f›: func(); } interface j { f: func(); }",
        "interface i { enum e { ‹case› } enum other { case } }",
        "interface i { resource r { ‹f›: func(); } resource s { f: func(); } }",
        "interface i { f: func(‹arg›: u32); g: func(arg: u32); }",
    ] {
        verify(&[("file:///members.wit", source)]).await;
    }
}

#[tokio::test]
async fn edits_invalidate_references_and_unresolved_names_do_not_match() {
    let (mut server, _socket) = wit_language_server::service();
    initialize(&mut server).await;
    let uri = "file:///edited.wit";
    let source =
        Source::marked("interface i { type ‹value› = u32; type x = «value»; type y = missing; }");
    open(&mut server, uri, &source.text).await;
    assert_eq!(
        references(
            &mut server,
            uri,
            &source.text,
            source.occurrences[0].0,
            true
        )
        .await,
        json!(source.locations(uri, true))
    );
    let updated = source.text.replace("type x = value", "type x = u32");
    notify(
        &mut server,
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": uri, "version": 2}, "contentChanges": [{"text": updated}]
        }),
    )
    .await;
    assert_eq!(
        references(
            &mut server,
            uri,
            &updated,
            updated.find("value").unwrap(),
            false
        )
        .await,
        json!([])
    );
    for offset in [
        updated.find("missing").unwrap(),
        updated.find("u32").unwrap(),
        updated.len(),
    ] {
        assert_eq!(
            references(&mut server, uri, &updated, offset, true).await,
            json!([])
        );
    }
    assert_eq!(
        references(&mut server, "file:///missing.wit", "", 0, true).await,
        Value::Null
    );
}

#[tokio::test]
async fn inline_references_respect_scopes_and_import_origins() {
    verify(&[("file:///inline.wit", "world app { type value = u32; import api: interface { record ‹value› { x: u32 } f: func(v: «value»); } export api: interface { type value = bool; f: func(v: value); } }")]).await;
    verify(&[
        ("file:///deps/types.wit", "package example:types; interface types { type ‹value› = u32; }"),
        ("file:///app/main.wit", "world app { import api: interface { use example:types/types.{«value» as ‹local›}; f: func(v: «local»); } export api: interface { type local = bool; f: func(v: local); } }")
    ]).await;
}
