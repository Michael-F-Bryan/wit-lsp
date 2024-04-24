#![recursion_limit = "256"]

mod database;
pub mod middleware;
mod ops;
mod server;
mod utils;

use crate::middleware::LanguageServerService;
pub use crate::{
    database::{Database, Db, Jar},
    server::LanguageServer,
};

/// The changelog for this crate.
pub const CHANGELOG: &str = include_str!("../CHANGELOG.md");

/// Create a new [`LanguageServerService`] that you can run with
/// [`tower_lsp::Server`].
pub fn service() -> (impl LanguageServerService, tower_lsp::ClientSocket) {
    let (service, socket) = LanguageServer::service();
    let service = crate::middleware::wrap(service);

    (service, socket)
}
