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
    crate::queries::file_items,
    crate::queries::FilePath,
    crate::queries::hover_info,
    crate::queries::InterfaceMetadata,
    crate::queries::Items,
    crate::queries::metadata::file_items,
    crate::queries::metadata::Ident,
    crate::queries::metadata::InterfaceMetadata,
    crate::queries::metadata::lower_package,
    crate::queries::metadata::package_items,
    crate::queries::metadata::PackageMetadata,
    crate::queries::metadata::WorldMetadata,
    crate::queries::metadata::FuncItemMetadata,
    crate::queries::metadata::ConstructorMetadata,
    crate::queries::metadata::EnumMetadata,
    crate::queries::metadata::FieldMetadata,
    crate::queries::metadata::FlagsMetadata,
    crate::queries::metadata::RecordMetadata,
    crate::queries::metadata::ResourceMetadata,
    crate::queries::metadata::VariantMetadata,
    crate::queries::metadata::TypeAliasMetadata,
    crate::queries::metadata::MethodMetadata,
    crate::queries::metadata::StaticMethodMetadata,
    crate::queries::metadata::VariantCaseMetadata,
    crate::queries::metadata::EnumCaseMetadata,
    crate::queries::metadata::FlagsCaseMetadata,
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
    crate::queries::Package,
    crate::queries::PackageId,
    crate::queries::parse,
    crate::queries::resolve_name,
    crate::queries::resolve_namespace,
    crate::queries::selection_ranges,
    crate::queries::SourceFile,
    crate::queries::workspace_packages,
    crate::queries::Workspace,
    crate::queries::WorldMetadata,
);

/// The trait that all [`salsa::Database`] implementations must implement to
/// be used with this crate's [`queries`].
pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
