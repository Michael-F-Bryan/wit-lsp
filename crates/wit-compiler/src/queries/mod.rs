//! Various operations used by the compiler and language server.

mod line_numbers;
pub(crate) mod lowering;
pub mod metadata;
mod parsing;
mod selection;
mod workspace;

pub use self::{
    line_numbers::{calculate_line_numbers, LineNumbers},
    lowering::lower_package,
    metadata::{file_items, package_items},
    parsing::{parse, Ast},
    selection::selection_ranges,
    workspace::{
        workspace_packages, FilePath, Package, PackageId, SourceFile, Workspace, WorkspaceFiles,
    },
};
