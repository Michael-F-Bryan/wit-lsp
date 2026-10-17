//! Various operations used by the compiler and language server.

mod line_numbers;
pub(crate) mod lowering;
pub mod metadata;
mod namespace;
mod parsing;
mod selection;
mod workspace;

pub use self::{
    line_numbers::{LineNumbers, calculate_line_numbers},
    lowering::lower_package,
    metadata::{file_items, package_items},
    namespace::{
        enclosing_scope, imported_types, resolve_type, top_level_names, type_definition,
        visible_types,
    },
    parsing::{Ast, parse},
    selection::selection_ranges,
    workspace::{
        FilePath, Package, PackageId, SourceFile, Workspace, WorkspaceFiles, workspace_packages,
    },
};
