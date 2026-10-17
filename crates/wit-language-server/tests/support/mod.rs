#![allow(dead_code)]

use futures::future::poll_fn;
use serde_json::{Value, json};
use tower_lsp::jsonrpc::{Request, Response};
use wit_language_server::LanguageServerService;

pub async fn call(
    server: &mut impl LanguageServerService,
    method: &'static str,
    params: Value,
) -> Response {
    poll_fn(|cx| server.poll_ready(cx)).await.unwrap();
    server
        .call(Request::build(method).id(1).params(params).finish())
        .await
        .unwrap()
        .unwrap()
}

pub async fn request(
    server: &mut impl LanguageServerService,
    method: &'static str,
    params: Value,
) -> Value {
    let response = call(server, method, params).await;
    assert!(response.error().is_none(), "{response:?}");
    response.result().unwrap().clone()
}

pub async fn notify(server: &mut impl LanguageServerService, method: &'static str, params: Value) {
    poll_fn(|cx| server.poll_ready(cx)).await.unwrap();
    assert!(
        server
            .call(Request::build(method).params(params).finish())
            .await
            .unwrap()
            .is_none()
    );
}

pub async fn initialize(server: &mut impl LanguageServerService) {
    let result = request(
        server,
        "initialize",
        json!({"capabilities": {
            "general": {"positionEncodings": ["utf-8", "utf-16"]}
        }}),
    )
    .await;
    assert_eq!(result["capabilities"]["positionEncoding"], "utf-16");
    notify(server, "initialized", json!({})).await;
}

pub async fn open(server: &mut impl LanguageServerService, uri: &str, text: &str) {
    notify(
        server,
        "textDocument/didOpen",
        json!({"textDocument": {
            "uri": uri, "languageId": "wit", "version": 1, "text": text
        }}),
    )
    .await;
}

pub async fn diagnostics(server: &mut impl LanguageServerService, uri: &str) -> Value {
    request(
        server,
        "textDocument/diagnostic",
        json!({"textDocument": {"uri": uri}}),
    )
    .await["items"]
        .clone()
}

pub fn position(src: &str, offset: usize) -> Value {
    let prefix = &src[..offset];
    json!({"line": prefix.bytes().filter(|&b| b == b'\n').count(),
        "character": prefix.rsplit('\n').next().unwrap().encode_utf16().count()})
}

pub fn range(src: &str, text: &str) -> Value {
    let start = src.find(text).unwrap();
    json!({"start": position(src, start), "end": position(src, start + text.len())})
}
