use either::Either;
use tree_sitter::Range;

use crate::{
    ast::{AstNode, HasSource},
    access::{
        EnumIndex, FlagsIndex, FuncItemIndex, GetAstNode, GetByIndex, Index, InterfaceIndex,
        RecordIndex, ResourceIndex, TypeAliasIndex, VariantIndex, WorldIndex,
    },
    queries::{FilePath, ItemDefinitionMetadata, Items, SourceFile},
    Db, Text, Tree,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HoverTarget {
    Interface(InterfaceIndex),
    World(WorldIndex),
    Enum(Either<InterfaceIndex, WorldIndex>, EnumIndex),
    Flags(Either<InterfaceIndex, WorldIndex>, FlagsIndex),
    Resource(Either<InterfaceIndex, WorldIndex>, ResourceIndex),
    Variant(Either<InterfaceIndex, WorldIndex>, VariantIndex),
    FuncItem(Either<InterfaceIndex, WorldIndex>, FuncItemIndex),
    TypeAlias(Either<InterfaceIndex, WorldIndex>, TypeAliasIndex),
    Record(Either<InterfaceIndex, WorldIndex>, RecordIndex),
}

#[salsa::tracked]
pub fn hover_info(db: &dyn Db, file: SourceFile, target: HoverTarget) -> HoverInfo {
    let items = crate::queries::file_items(db, file);
    let ast = crate::queries::parse(db, file);
    let tree = ast.tree(db);

    let state = State {
        db,
        file,
        items,
        tree,
    };

    match target {
        HoverTarget::Interface(index) => hover_interface(index, state),
        HoverTarget::World(index) => hover_world(index, state),
        HoverTarget::Enum(scope, index) => state.hover_type(scope, index),
        HoverTarget::Flags(scope, index) => state.hover_type(scope, index),
        HoverTarget::Resource(scope, index) => state.hover_type(scope, index),
        HoverTarget::Variant(scope, index) => state.hover_type(scope, index),
        HoverTarget::FuncItem(scope, index) => state.hover_type(scope, index),
        HoverTarget::TypeAlias(scope, index) => state.hover_type(scope, index),
        HoverTarget::Record(scope, index) => state.hover_type(scope, index),
    }
}

struct State<'db> {
    db: &'db dyn Db,
    file: SourceFile,
    items: Items,
    tree: &'db Tree,
}

impl<'db> State<'db> {
    fn source(&self, node: impl HasSource) -> &str {
        let src = self.file.contents(self.db);
        node.utf8_text(src)
    }

    fn hover_type<I>(&self, scope: Either<InterfaceIndex, WorldIndex>, index: I) -> HoverInfo
    where
        I: Index,
        ItemDefinitionMetadata: GetByIndex<I>,
        <ItemDefinitionMetadata as GetByIndex<I>>::Metadata: GetAstNode,
    {
        let ptr = match scope {
            Either::Left(interface) => {
                let world = self.items.get_by_index(self.db, interface);
                world.get_by_index(self.db, index)
            }
            Either::Right(world) => {
                let world = self.items.get_by_index(self.db, world);
                world.get_by_index(self.db, index)
            }
        };

        let node = ptr.ast_node(self.tree);

        HoverInfo {
            filename: self.file.path(self.db).clone(),
            definition: node.range(),
            content: self.source(node).into(),
        }
    }
}

fn hover_interface(index: InterfaceIndex, s: State<'_>) -> HoverInfo {
    let meta = s.items.get_interface(s.db, index);
    let node = meta.location(s.db).ast_node(s.tree);

    HoverInfo {
        filename: s.file.path(s.db).clone(),
        definition: node.range(),
        content: s.source(node).into(),
    }
}

fn hover_world(index: WorldIndex, s: State<'_>) -> HoverInfo {
    let meta = s.items.get_world(s.db, index);
    let node = meta.location(s.db).ast_node(s.tree);

    HoverInfo {
        filename: s.file.path(s.db).clone(),
        definition: node.range(),
        content: s.source(node).into(),
    }
}

/// Information shown when hovering over an item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoverInfo {
    /// The name of the file this thing came from.
    pub filename: FilePath,
    /// Where the item was defined. Useful when you want to jump to definition.
    pub definition: Range,
    /// The contents of the tooltip, formatted as Markdown text.
    pub content: Text,
}
