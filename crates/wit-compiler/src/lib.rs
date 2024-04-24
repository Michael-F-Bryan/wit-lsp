#![recursion_limit = "256"]

pub mod access;
pub mod ast;
mod compiler;
pub mod diagnostics;
mod file_path;
pub mod hir;
pub mod queries;
mod text;
pub mod traverse;
mod tree;
mod workspace;

pub use crate::{compiler::Compiler, file_path::FilePath, text::Text, tree::Tree};

/// A [`salsa::jar::Jar`] containing all queries and tracked types used by this
/// crate.
#[salsa::jar(db = Db)]
pub struct Jar(
    crate::diagnostics::Diagnostics,
    crate::queries::Ast,
    crate::queries::file_items,
    crate::queries::hover_info,
    crate::queries::InterfaceMetadata,
    crate::queries::Items,
    crate::queries::lower,
    crate::queries::lowering::lower_enum,
    crate::queries::lowering::lower_flags,
    crate::queries::lowering::lower_func_item,
    crate::queries::lowering::lower_interface,
    crate::queries::lowering::lower_record,
    crate::queries::lowering::lower_resource,
    crate::queries::lowering::lower_type_alias,
    crate::queries::lowering::lower_variant,
    crate::queries::lowering::lower_world,
    crate::queries::PackageId,
    crate::queries::Package,
    crate::queries::parse,
    crate::queries::resolve_name,
    crate::queries::resolve_namespace,
    crate::queries::selection_ranges,
    crate::queries::SourceFile,
    crate::queries::Workspace,
    crate::queries::workspace_packages,
    crate::queries::WorldMetadata,
);

/// The trait that all [`salsa::Database`] implementations must implement to
/// be used with this crate's [`queries`].
pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
