//! The high-level intermediate representation.

use im::{OrdMap, Vector};

use crate::Text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    World(World),
    Interface(Interface),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemId {
    pub filename: Text,
    pub name: Text,
}

#[salsa::tracked]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
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

#[salsa::tracked]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interface {
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
    pub docs: Option<Text>,
    pub name: Text,
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
    Error,
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
    pub docs: Option<Text>,
    pub cases: Vector<EnumCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCase {
    pub docs: Option<Text>,
    pub name: Text,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flags {
    pub docs: Option<Text>,
    pub cases: Vector<FlagsCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsCase {
    pub docs: Option<Text>,
    pub name: Text,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub docs: Option<Text>,
    pub cases: Vector<VariantCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
    pub docs: Option<Text>,
    pub name: Text,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAlias {
    pub docs: Option<Text>,
    pub name: Text,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub docs: Option<Text>,
    pub name: Text,
    pub constructor: Option<Constructor>,
    pub methods: Vector<FuncItem>,
    pub static_methods: Vector<FuncItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub docs: Option<Text>,
    pub params: Vector<Parameter>,
}
