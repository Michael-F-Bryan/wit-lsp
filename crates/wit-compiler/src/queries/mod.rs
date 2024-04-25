//! Various operations used by the compiler and language server.

mod hover;
mod items;
pub(crate) mod lowering;
mod namespaces;
mod parsing;
mod selection;
mod workspace;
mod line_numbers;

pub use self::{
    hover::{hover_info, HoverInfo, HoverTarget},
    items::{
        file_items, InterfaceMetadata, ItemDefinitionMetadata, Items, ResourceMetadata,
        WorldMetadata,
    },
    lowering::lower,
    namespaces::{resolve_name, resolve_namespace, Namespace},
    parsing::{parse, Ast},
    selection::selection_ranges,
    workspace::{workspace_packages, Package, PackageId, SourceFile, Workspace},
    line_numbers::{LineNumbers, calculate_line_numbers},
};
