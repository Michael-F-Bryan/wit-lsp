//! Various operations used by the compiler and language server.

mod hover;
mod items;
pub(crate) mod lowering;
mod namespaces;
mod parsing;
mod selection;

pub use self::{
    hover::{hover_info, HoverInfo, HoverTarget},
    items::{
        file_items, InterfaceMetadata, ItemDefinitionMetadata, Items, ResourceMetadata,
        WorldMetadata,
    },
    lowering::lower,
    namespaces::{resolve_name, resolve_namespace, Namespace},
    parsing::{parse, Ast, FilePath, SourceFile, Workspace},
    selection::selection_ranges,
};
