use crate::{ast::AstNode, hir, Tree};

pub trait Index: Copy {
    type Hir;

    fn from_raw(raw: hir::Index) -> Self;
    fn raw(self) -> hir::Index;
}

macro_rules! indices {
    ($( $name:ident),* ) => {
        $(
            paste::paste! {
                #[doc = concat!("The index of a [`crate::hir::", stringify!($name), "`].")]
                #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
                pub struct [< $name "Index" >](hir::Index);

                impl Index for [< $name "Index" >] {
                    type Hir = hir::$name;

                    fn from_raw(raw: hir::Index) -> Self { Self(raw) }

                    fn raw(self) -> hir::Index { self.0 }
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
