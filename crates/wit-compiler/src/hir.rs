//! The high-level intermediate representation.

use std::num::NonZeroU16;

use im::{OrdMap, Vector};

use crate::Text;

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
pub struct Index(NonZeroU16);

impl Index {
    const MAX: u16 = u16::MAX - 1;
    pub const ZERO: Index = Index::new(0);

    const fn new(raw: usize) -> Self {
        assert!(raw <= Index::MAX as usize);
        match NonZeroU16::new(raw as u16 + 1) {
            Some(raw) => Index(raw),
            None => panic!(),
        }
    }

    pub const fn next(self) -> Index {
        Index::new(self.raw() + 1)
    }

    pub const fn raw(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Package {
    pub decl: Option<PackageDeclaration>,
    pub worlds: Vector<World>,
    pub interfaces: Vector<Interface>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDeclaration {
    pub docs: Option<Text>,
    pub package: Text,
    pub path: Vector<Text>,
    pub version: Option<Text>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    pub name: Text,
    pub docs: Option<Text>,
    pub items: Vector<WorldItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorldItem {
    Import(ExposableItem),
    Export(ExposableItem),
    Include(Include),
}

/// Include one [`World`]'s items in another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Include {
    pub path: Path,
    pub aliases: OrdMap<Text, Text>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub namespace: Option<Text>,
    pub path: Text,
}

/// Something that can be exposed via either [`WorldItem::Import`] or
/// [`WorldItem::Export`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExposableItem {
    Named(Path),
    /// An item that is defined in the [`World`] itself.
    Inline {
        name: Text,
        value: ExternType,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternType {
    Function(FuncItem),
    Interface(Interface),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interface {
    pub name: Text,
    pub docs: Option<Text>,
    pub items: Vector<InterfaceItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceItem {
    Func(FuncItem),
    Enum(Enum),
    Flags(Flags),
    Resource(Resource),
    TypeAlias(TypeAlias),
    Variant(Variant),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncItem {
    pub name: Text,
    pub docs: Option<Text>,
    pub params: Vector<Parameter>,
    pub return_value: Option<ReturnValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: Text,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnValue {
    Single(Type),
    Named(OrdMap<Text, Type>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Builtin(Builtin),
    Handle {
        borrowed: bool,
    },
    List(Box<Type>),
    Option(Box<Type>),
    Result {
        ok: Option<Box<Type>>,
        err: Option<Box<Type>>,
    },
    Tuple(Vector<Type>),
    Error,
}

impl From<Builtin> for Type {
    fn from(value: Builtin) -> Self {
        Type::Builtin(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Builtin {
    Boolean,
    I16,
    I32,
    I64,
    I8,
    U16,
    U32,
    U64,
    U8,
    Float32,
    Float64,
    Char,
    String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub name: Text,
    pub docs: Option<Text>,
    pub cases: Vector<EnumCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCase {
    pub name: Text,
    pub docs: Option<Text>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flags {
    pub name: Text,
    pub docs: Option<Text>,
    pub cases: Vector<FlagsCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsCase {
    pub name: Text,
    pub docs: Option<Text>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: Text,
    pub docs: Option<Text>,
    pub cases: Vector<VariantCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
    pub name: Text,
    pub docs: Option<Text>,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAlias {
    pub name: Text,
    pub docs: Option<Text>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub name: Text,
    pub docs: Option<Text>,
    pub constructor: Option<Constructor>,
    pub methods: Vector<FuncItem>,
    pub static_methods: Vector<FuncItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub docs: Option<Text>,
    pub params: Vector<Parameter>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        assert_eq!(Index::ZERO.raw(), 0);
        assert_eq!(Index::new(42).raw(), 42);
        assert_eq!(Index::new(42).next().raw(), 43);
    }
}
