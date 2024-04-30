#![recursion_limit = "256"]

pub mod access;
pub mod ast;
mod compiler;
pub mod diagnostics;
pub mod hir;
pub mod queries;
mod text;
pub mod traverse;
mod tree;

pub use crate::{compiler::Compiler, text::Text, tree::Tree};

/// A [`salsa::jar::Jar`] containing all queries and tracked types used by this
/// crate.
#[salsa::jar(db = Db)]
pub struct Jar(
    crate::diagnostics::Diagnostics,
    crate::queries::Ast,
    crate::queries::calculate_line_numbers,
    crate::queries::FilePath,
    crate::queries::lowering2::lower_package,
    crate::queries::lowering2::lower_package_docs,
    crate::queries::lowering2::lower_world,
    crate::queries::lowering2::lower_interface,
    crate::queries::metadata::Metadata,
    crate::queries::metadata::ConstructorMetadata,
    crate::queries::metadata::EnumCaseMetadata,
    crate::queries::metadata::EnumMetadata,
    crate::queries::metadata::FieldMetadata,
    crate::queries::metadata::file_items,
    crate::queries::metadata::FlagsCaseMetadata,
    crate::queries::metadata::FlagsMetadata,
    crate::queries::metadata::FuncItemMetadata,
    crate::queries::metadata::Ident,
    crate::queries::metadata::InterfaceMetadata,
    crate::queries::metadata::MethodMetadata,
    crate::queries::metadata::package_items,
    crate::queries::metadata::PackageMetadata,
    crate::queries::metadata::RecordMetadata,
    crate::queries::metadata::ResourceMetadata,
    crate::queries::metadata::StaticMethodMetadata,
    crate::queries::metadata::TypeAliasMetadata,
    crate::queries::metadata::VariantCaseMetadata,
    crate::queries::metadata::VariantMetadata,
    crate::queries::metadata::WorldMetadata,
    crate::queries::Package,
    crate::queries::PackageId,
    crate::queries::parse,
    crate::queries::selection_ranges,
    crate::queries::SourceFile,
    crate::queries::workspace_packages,
    crate::queries::Workspace,
);

/// The trait that all [`salsa::Database`] implementations must implement to
/// be used with this crate's [`queries`].
pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
