//! Indirection used when navigating the [`crate::hir`] and
//! [`crate::queries::Items`].

use std::num::NonZeroU16;

use crate::{
    ast::AstNode,
    hir,
    queries::{
        InterfaceMetadata, ItemDefinitionMetadata, Items, SourceFile, Workspace, WorldMetadata,
    },
    Db, Text, Tree,
};

/// An index optimised for use in item IDs.
///
/// You typically won't use this directly, and instead rely on strongly-typed
/// wrappers.
///
/// # Implementation
///
/// Under the hood, the index is represented as a [`NonZeroU16`].  We make the
/// assumption that no file will contain more than `2^16-2` sequential elements
/// of the same type, so we can get away with only using 2 bytes for our indices
/// rather than the 8 we would need if we stored a `usize`.
///
/// Strongly typed wrappers will sometimes include enums, so by using
/// [`NonZeroU16`] over [`u16`], we are more likely to benefit from niche
/// optimisations.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawIndex(NonZeroU16);

impl RawIndex {
    const MAX: u16 = u16::MAX - 1;
    pub const ZERO: RawIndex = RawIndex::new(0);

    pub(crate) const fn new(raw: usize) -> Self {
        assert!(raw <= RawIndex::MAX as usize);
        match NonZeroU16::new(raw as u16 + 1) {
            Some(raw) => RawIndex(raw),
            None => panic!(),
        }
    }

    pub const fn as_usize(self) -> usize {
        self.0.get() as usize - 1
    }
}

/// An index into a
pub trait Index: Copy {
    type Hir;

    fn from_raw(raw: RawIndex) -> Self;
    fn raw(self) -> RawIndex;
}

macro_rules! indices {
    ($( $name:ident),* ) => {
        $(
            paste::paste! {
                #[doc = concat!("The index of a [`crate::hir::", stringify!($name), "`].")]
                #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
                pub struct [< $name "Index" >](RawIndex);

                impl Index for [< $name "Index" >] {
                    type Hir = hir::$name;

                    fn from_raw(raw: RawIndex) -> Self { Self(raw) }

                    fn raw(self) -> RawIndex { self.0 }
                }
            }
        )*
    };
}

indices!(Enum, Flags, Resource, Variant, FuncItem, TypeAlias, Record, World, Interface);

/// A reference to an AST node.
pub trait Pointer {
    type Node<'tree>: crate::ast::AstNode<'tree>;

    fn for_node(node: Self::Node<'_>) -> Self;
    fn range(self) -> tree_sitter::Range;
    fn lookup(self, tree: &Tree) -> Self::Node<'_>;
}

macro_rules! item_pointers {
    ($( $pointer:ident => $ast_node:ident),+ $(,)?) => {
        $(
            #[doc = concat!("A strongly-typed reference to a [`crate::ast::", stringify!($ast_node), "`].")]
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub struct $pointer(tree_sitter::Range);

            impl Pointer for $pointer {
                type Node<'tree> = crate::ast::$ast_node<'tree>;

                fn for_node(node: crate::ast::$ast_node<'_>) -> Self {
                    $pointer(node.syntax().range())
                }

                fn range(self) -> tree_sitter::Range {
                    self.0
                }

                fn lookup(self, tree: &Tree) -> Self::Node<'_> {
                    tree.find(self.0)
                }
            }
        )*
    };
}

item_pointers! {
    WorldPtr => WorldItem,
    InterfacePtr => InterfaceItem,
    RecordPtr => RecordItem,
    TypeAliasPtr => TypeItem,
    EnumPtr => EnumItem,
    FlagsPtr => FlagsItem,
    ResourcePtr => ResourceItem,
    VariantPtr => VariantItem,
    FunctionPtr => FuncItem,
}

/// Look up an item's metadata using its [`Index`].
pub trait GetByIndex<Index> {
    type Metadata;

    fn get_by_index(&self, db: &dyn Db, index: Index) -> Self::Metadata;
}

macro_rules! get_metadata {
    ($( $index:ty => $field:ident => $meta:ty  ),+ $(,)?) => {
        $(
            impl GetByIndex<$index> for ItemDefinitionMetadata {
                type Metadata = $meta;

                #[allow(unused_variables)]
                fn get_by_index(&self, db: &dyn Db, index: $index) -> Self::Metadata {
                    let index = index.raw().as_usize();
                    self.$field[index]
                }
            }
        )*
    };
}

get_metadata! {
    EnumIndex => enums => EnumPtr,
    FlagsIndex => flags => FlagsPtr,
    FuncItemIndex => functions => FunctionPtr,
    RecordIndex => records => RecordPtr,
    ResourceIndex => resources => ResourcePtr,
    TypeAliasIndex => typedefs => TypeAliasPtr,
    VariantIndex => variants => VariantPtr,
}

impl GetByIndex<WorldIndex> for Items {
    type Metadata = WorldMetadata;

    fn get_by_index(&self, db: &dyn Db, index: WorldIndex) -> Self::Metadata {
        let index = index.raw().as_usize();
        let worlds = self.worlds(db);
        worlds[index]
    }
}

impl GetByIndex<InterfaceIndex> for Items {
    type Metadata = InterfaceMetadata;

    fn get_by_index(&self, db: &dyn Db, index: InterfaceIndex) -> Self::Metadata {
        let index = index.raw().as_usize();
        let interfaces = self.interfaces(db);
        interfaces[index]
    }
}

impl<ScopeIndex> GetByIndex<(Text, ScopeIndex)> for Workspace
where
    SourceFile: GetByIndex<ScopeIndex>,
{
    type Metadata = Option<<SourceFile as GetByIndex<ScopeIndex>>::Metadata>;

    fn get_by_index(&self, db: &dyn Db, (filename, ix): (Text, ScopeIndex)) -> Self::Metadata {
        let files = self.files(db);
        let f = files.get(&filename)?;
        Some(f.get_by_index(db, ix))
    }
}

impl<ScopeIndex, ItemIndex> GetByIndex<(Text, ScopeIndex, ItemIndex)> for Workspace
where
    SourceFile: GetByIndex<(ScopeIndex, ItemIndex)>,
{
    type Metadata = Option<<SourceFile as GetByIndex<(ScopeIndex, ItemIndex)>>::Metadata>;

    fn get_by_index(
        &self,
        db: &dyn Db,
        (filename, scope, index): (Text, ScopeIndex, ItemIndex),
    ) -> Self::Metadata {
        let files = self.files(db);
        files
            .get(&filename)
            .map(|f| f.get_by_index(db, (scope, index)))
    }
}

impl<Ix> GetByIndex<Ix> for SourceFile
where
    Items: GetByIndex<Ix>,
{
    type Metadata = <Items as GetByIndex<Ix>>::Metadata;

    fn get_by_index(&self, db: &dyn Db, index: Ix) -> Self::Metadata {
        crate::queries::file_items(db, *self).get_by_index(db, index)
    }
}

impl<Ix> GetByIndex<(WorldIndex, Ix)> for Items
where
    WorldMetadata: GetByIndex<Ix>,
{
    type Metadata = <WorldMetadata as GetByIndex<Ix>>::Metadata;

    fn get_by_index(&self, db: &dyn Db, index: (WorldIndex, Ix)) -> Self::Metadata {
        self.get_by_index(db, index.0).get_by_index(db, index.1)
    }
}

impl<Ix> GetByIndex<(InterfaceIndex, Ix)> for Items
where
    InterfaceMetadata: GetByIndex<Ix>,
{
    type Metadata = <InterfaceMetadata as GetByIndex<Ix>>::Metadata;

    fn get_by_index(&self, db: &dyn Db, index: (InterfaceIndex, Ix)) -> Self::Metadata {
        self.get_by_index(db, index.0).get_by_index(db, index.1)
    }
}

impl<Ix> GetByIndex<Ix> for WorldMetadata
where
    ItemDefinitionMetadata: GetByIndex<Ix>,
{
    type Metadata = <ItemDefinitionMetadata as GetByIndex<Ix>>::Metadata;

    fn get_by_index(&self, db: &dyn Db, index: Ix) -> Self::Metadata {
        let items = self.items(db);
        items.get_by_index(db, index)
    }
}

impl<Ix> GetByIndex<Ix> for InterfaceMetadata
where
    ItemDefinitionMetadata: GetByIndex<Ix>,
{
    type Metadata = <ItemDefinitionMetadata as GetByIndex<Ix>>::Metadata;

    fn get_by_index(&self, db: &dyn Db, index: Ix) -> Self::Metadata {
        let items = self.items(db);
        items.get_by_index(db, index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        assert_eq!(RawIndex::ZERO.as_usize(), 0);
        assert_eq!(RawIndex::new(42).as_usize(), 42);
    }
}
