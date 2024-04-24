//! Various operations used by the compiler and language server.

mod hover;
mod items;
pub(crate) mod lowering;
mod namespaces;
mod parsing;
mod selection;

pub(crate) use self::parsing::Workspace_packages;
pub use self::{
    hover::{hover_info, HoverInfo, HoverTarget},
    items::{
        file_items, InterfaceMetadata, ItemDefinitionMetadata, Items, ResourceMetadata,
        WorldMetadata,
    },
    lowering::lower,
    namespaces::{resolve_name, resolve_namespace, Namespace},
    parsing::{parse, Ast, FilePath, Package, PackageId, SourceFile, Workspace},
    selection::selection_ranges,
};
