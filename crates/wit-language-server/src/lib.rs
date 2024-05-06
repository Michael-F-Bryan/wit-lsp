#![recursion_limit = "256"]

pub mod code_actions;
mod database;
mod middleware;
mod ops;
mod server;
mod utils;

pub(crate) use crate::database::{Database, Db, Jar};

pub use crate::middleware::{CatchPanicError, LanguageServerService, PanicMessage};

/// The changelog for this crate.
pub(crate) const CHANGELOG: &str = include_str!("../CHANGELOG.md");

/// Create a new [`LanguageServerService`] that you can run with
/// [`tower_lsp::Server`].
pub fn service() -> (impl LanguageServerService, tower_lsp::ClientSocket) {
    let (service, socket) = crate::server::LanguageServer::service();
    let service = crate::middleware::wrap(service);

    (service, socket)
}
