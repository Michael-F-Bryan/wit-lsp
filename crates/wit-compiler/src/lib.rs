pub mod ast;
mod compiler;
pub mod queries;
mod text;
pub mod traverse;
mod workspace;

pub use crate::{compiler::Compiler, text::Text};

/// A [`salsa::jar::Jar`] containing all queries and tracked types used by this
/// crate.
#[salsa::jar(db = Db)]
pub struct Jar(
    crate::queries::Workspace,
    crate::queries::Ast,
    crate::queries::parse,
);

/// The trait that all [`salsa::Database`] implementations must implement to
/// be used with this crate's [`queries`].
pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
