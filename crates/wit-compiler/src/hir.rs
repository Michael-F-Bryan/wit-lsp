//! The high-level intermediate representation.

use std::fmt::Debug;

use im::{OrdMap, Vector};

use crate::{
    access::{
        AnyFuncItemIndex, EnumIndex, FlagsIndex, FunctionIndex, InterfaceIndex, RecordIndex,
        ResourceIndex, ScopeIndex, TypeAliasIndex, VariantIndex, WorldIndex,
    },
    queries::{metadata::Ident, FilePath, PackageId},
    Text,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Package {
    pub docs: Option<Text>,
    pub id: Option<PackageId>,
    pub worlds: OrdMap<WorldIndex, World>,
    pub interfaces: OrdMap<InterfaceIndex, Interface>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    pub name: Ident,
    pub docs: Option<Text>,
    pub type_definitions: Vector<TypeDefinition>,
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
    pub path: ModulePath,
    pub aliases: OrdMap<Text, Text>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePath {
    pub namespace: Option<Text>,
    pub path: Text,
}

/// Something that can be exposed via either [`WorldItem::Import`] or
/// [`WorldItem::Export`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExposableItem {
    Named(ModulePath),
    /// An item that is defined in the [`World`] itself.
    Inline {
        name: Ident,
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
    pub name: Ident,
    pub docs: Option<Text>,
    pub items: Vector<InterfaceItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceItem {
    Func(FuncItem),
    Type(TypeDefinition),
}

impl From<FuncItem> for InterfaceItem {
    fn from(value: FuncItem) -> Self {
        InterfaceItem::Func(value)
    }
}

impl From<TypeDefinition> for InterfaceItem {
    fn from(value: TypeDefinition) -> Self {
        InterfaceItem::Type(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDefinition {
    Enum(Enum),
    Flags(Flags),
    Resource(Resource),
    TypeAlias(TypeAlias),
    Variant(Variant),
    Record(Record),
}

impl From<Enum> for TypeDefinition {
    fn from(value: Enum) -> Self {
        TypeDefinition::Enum(value)
    }
}

impl From<Flags> for TypeDefinition {
    fn from(value: Flags) -> Self {
        TypeDefinition::Flags(value)
    }
}

impl From<Resource> for TypeDefinition {
    fn from(value: Resource) -> Self {
        TypeDefinition::Resource(value)
    }
}

impl From<TypeAlias> for TypeDefinition {
    fn from(value: TypeAlias) -> Self {
        TypeDefinition::TypeAlias(value)
    }
}

impl From<Variant> for TypeDefinition {
    fn from(value: Variant) -> Self {
        TypeDefinition::Variant(value)
    }
}

impl From<Record> for TypeDefinition {
    fn from(value: Record) -> Self {
        TypeDefinition::Record(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncItem {
    pub name: Ident,
    pub index: AnyFuncItemIndex,
    pub docs: Option<Text>,
    pub params: Vector<Parameter>,
    pub return_value: Option<ReturnValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: Ident,
    pub docs: Option<Text>,
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
        ty: Box<Type>,
    },
    List(Box<Type>),
    Option(Box<Type>),
    Result {
        ok: Option<Box<Type>>,
        err: Option<Box<Type>>,
    },
    Tuple(Vector<Type>),
    UserDefinedType(ItemReference),
    Error,
}

/// A reference to an item defined elsewhere.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ItemReference {
    pub file: FilePath,
    pub scope: ScopeIndex,
    pub kind: ItemReferenceKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ItemReferenceKind {
    Enum(EnumIndex),
    Variant(VariantIndex),
    Flags(FlagsIndex),
    Record(RecordIndex),
    Resource(ResourceIndex),
    Func(FunctionIndex),
    TypeAlias(TypeAliasIndex),
}

impl From<EnumIndex> for ItemReferenceKind {
    fn from(v: EnumIndex) -> Self {
        Self::Enum(v)
    }
}

impl From<TypeAliasIndex> for ItemReferenceKind {
    fn from(v: TypeAliasIndex) -> Self {
        Self::TypeAlias(v)
    }
}

impl From<FunctionIndex> for ItemReferenceKind {
    fn from(v: FunctionIndex) -> Self {
        Self::Func(v)
    }
}

impl From<ResourceIndex> for ItemReferenceKind {
    fn from(v: ResourceIndex) -> Self {
        Self::Resource(v)
    }
}

impl From<RecordIndex> for ItemReferenceKind {
    fn from(v: RecordIndex) -> Self {
        Self::Record(v)
    }
}

impl From<FlagsIndex> for ItemReferenceKind {
    fn from(v: FlagsIndex) -> Self {
        Self::Flags(v)
    }
}

impl From<VariantIndex> for ItemReferenceKind {
    fn from(v: VariantIndex) -> Self {
        Self::Variant(v)
    }
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
pub struct Record {
    pub name: Ident,
    pub index: RecordIndex,
    pub docs: Option<Text>,
    pub fields: Vector<RecordField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordField {
    pub name: Ident,
    pub docs: Option<Text>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub name: Ident,
    pub index: EnumIndex,
    pub docs: Option<Text>,
    pub cases: Vector<EnumCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCase {
    pub name: Ident,
    pub docs: Option<Text>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flags {
    pub name: Ident,
    pub index: FlagsIndex,
    pub docs: Option<Text>,
    pub cases: Vector<FlagsCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsCase {
    pub name: Ident,
    pub docs: Option<Text>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: Ident,
    pub index: VariantIndex,
    pub docs: Option<Text>,
    pub cases: Vector<VariantCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
    pub name: Ident,
    pub docs: Option<Text>,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAlias {
    pub name: Ident,
    pub index: TypeAliasIndex,
    pub docs: Option<Text>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub name: Ident,
    pub index: ResourceIndex,
    pub docs: Option<Text>,
    pub constructor: Option<Constructor>,
    pub methods: Vector<ResourceMethod>,
    pub static_methods: Vector<StaticResourceMethod>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceMethod(pub FuncItem);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticResourceMethod(pub FuncItem);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub docs: Option<Text>,
    pub params: Vector<Parameter>,
}
