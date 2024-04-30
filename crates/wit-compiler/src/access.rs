//! Indirection used when navigating the [`crate::hir`] and
//! [`crate::queries::Items`].

use std::{marker::PhantomData, num::NonZeroU16};

use crate::{
    ast::AstNode,
    diagnostics::Location,
    queries::{metadata::HasDefinition, FilePath},
    Tree,
};

pub trait NodeKind {
    type Ast<'tree>: AstNode<'tree>;
    type Metadata: HasDefinition;
}

macro_rules! node_kinds {
    ($( $name:ident => ($ast_node:ident, $meta:path)),+ $(,)?) => {
        paste::paste! {
            $(
                #[doc = "A type tag for a [`" $ast_node "`] node."]
                #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
                pub enum $name {}

                impl NodeKind for $name {
                    type Ast<'tree> = crate::ast::$ast_node<'tree>;
                    type Metadata = $meta;
                }

                pub type [< $name Ptr >] = Pointer<$name>;
                pub type [< $name Index >] = Index<$name>;
            )*

        }
    };
}

node_kinds! {
    World => (WorldItem, crate::queries::metadata::WorldMetadata),
    Interface => (InterfaceItem, crate::queries::metadata::InterfaceMetadata),
    Record => (RecordItem, crate::queries::metadata::RecordMetadata),
    TypeAlias => (TypeItem, crate::queries::metadata::TypeAliasMetadata),
    Enum => (EnumItem, crate::queries::metadata::EnumMetadata),
    Flags => (FlagsItem, crate::queries::metadata::FlagsMetadata),
    Resource => (ResourceItem, crate::queries::metadata::ResourceMetadata),
    Variant => (VariantItem, crate::queries::metadata::VariantMetadata),
    Function => (FuncItem, crate::queries::metadata::FuncItemMetadata),
    Constructor => (ResourceConstructor, crate::queries::metadata::ConstructorMetadata),
    Method => (FuncItem, crate::queries::metadata::MethodMetadata),
    StaticMethod => (StaticMethod, crate::queries::metadata::StaticMethodMetadata),
    RecordField => (RecordField, crate::queries::metadata::FieldMetadata),
    VariantCase => (VariantCase, crate::queries::metadata::VariantCaseMetadata),
    EnumCase => (EnumCase, crate::queries::metadata::EnumCaseMetadata),
    FlagsCase => (FlagsCase, crate::queries::metadata::FlagsCaseMetadata),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index<K> {
    file: FilePath,
    index: RawIndex,
    _ty: PhantomData<K>,
}

impl<K: NodeKind> Index<K> {
    pub(crate) fn new(file: FilePath, index: RawIndex) -> Self {
        Index {
            file,
            index,
            _ty: PhantomData,
        }
    }

    pub fn file(self) -> FilePath {
        self.file
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnyFuncItemIndex {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScopeIndex {
    World(WorldIndex),
    Interface(InterfaceIndex),
}

impl From<InterfaceIndex> for ScopeIndex {
    fn from(v: InterfaceIndex) -> Self {
        Self::Interface(v)
    }
}

impl From<WorldIndex> for ScopeIndex {
    fn from(v: WorldIndex) -> Self {
        Self::World(v)
    }
}

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
#[repr(transparent)]
pub(crate) struct RawIndex(NonZeroU16);

impl RawIndex {
    const MAX: u16 = u16::MAX - 1;
    pub const ZERO: RawIndex = RawIndex::new(0);

    pub(crate) const fn new(raw: usize) -> Self {
        assert!(raw <= RawIndex::MAX as usize);

        let raw = match (raw as u16).checked_add(1) {
            Some(raw) => raw,
            None => panic!(),
        };

        match NonZeroU16::new(raw) {
            Some(raw) => RawIndex(raw),
            None => panic!(),
        }
    }

    pub(crate) fn next(self) -> Self {
        RawIndex::new(self.as_usize().checked_add(1).unwrap())
    }

    pub const fn as_usize(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Pointer<K> {
    location: Location,
    _ty: PhantomData<K>,
}

impl<K: NodeKind> Pointer<K> {
    pub fn for_node(file: FilePath, node: K::Ast<'_>) -> Self {
        let range = node.range();
        let location = Location::new(file, range);

        Pointer {
            location,
            _ty: PhantomData,
        }
    }

    pub fn file(self) -> FilePath {
        self.location.filename
    }

    pub fn range(self) -> tree_sitter::Range {
        self.location.range
    }

    pub fn location(self) -> crate::diagnostics::Location {
        self.location
    }

    /// Get the [`crate::ast`] node from the AST [`Tree`].
    pub fn ast_node(self, tree: &Tree) -> K::Ast<'_> {
        tree.find(self.location.range)
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
