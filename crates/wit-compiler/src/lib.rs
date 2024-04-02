#![recursion_limit = "256"]

pub mod ast;
mod compiler;
pub mod diagnostics;
pub mod hir;
pub mod queries;
mod text;
pub mod traverse;
mod tree;
mod workspace;
pub mod pointer;

pub use crate::{compiler::Compiler, text::Text, tree::Tree};

/// A [`salsa::jar::Jar`] containing all queries and tracked types used by this
/// crate.
#[salsa::jar(db = Db)]
pub struct Jar(
    crate::diagnostics::Diagnostics,
    crate::queries::Ast,
    crate::queries::parse,
    crate::queries::file_items,
    crate::queries::Items,
    crate::queries::WorldMetadata,
    crate::queries::InterfaceMetadata,
    crate::queries::selection_ranges,
    crate::queries::SourceFile,
    crate::queries::lower,
    crate::queries::Workspace,
);

/// The trait that all [`salsa::Database`] implementations must implement to
/// be used with this crate's [`queries`].
pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
