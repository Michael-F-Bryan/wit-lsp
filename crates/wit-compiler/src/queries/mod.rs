//! Various operations used by the compiler and language server.

mod hover;
mod items;
mod line_numbers;
pub(crate) mod lowering;
pub(crate) mod lowering2;
pub mod metadata;
mod namespaces;
mod parsing;
mod selection;
mod workspace;

pub use self::{
    hover::{hover_info, HoverInfo, HoverTarget},
    items::{
        file_items, InterfaceMetadata, ItemDefinitionMetadata, Items, ResourceMetadata,
        WorldMetadata,
    },
    line_numbers::{calculate_line_numbers, LineNumbers},
    lowering::lower,
    lowering2::lower_package,
    metadata::{file_items as file_items2, package_items},
    namespaces::{resolve_name, resolve_namespace, Namespace},
    parsing::{parse, Ast},
    selection::selection_ranges,
    workspace::{
        workspace_packages, FilePath, Package, PackageId, SourceFile, Workspace, WorkspaceFiles,
    },
};
