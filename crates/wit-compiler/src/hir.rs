//! The high-level intermediate representation.

use im::{OrdMap, Vector};

use crate::{
    access::{
        AnyFuncItemIndex, EnumIndex, FlagsIndex, FuncItemIndex, InterfaceIndex, RecordIndex,
        ResourceIndex, ScopeIndex, TypeAliasIndex, VariantIndex, WorldIndex,
    },
    queries::FilePath,
    Text,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Package {
    pub decl: Option<PackageDeclaration>,
    pub worlds: OrdMap<WorldIndex, World>,
    pub interfaces: OrdMap<InterfaceIndex, Interface>,
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
    Record(Record),
}

impl From<Enum> for InterfaceItem {
    fn from(value: Enum) -> Self {
        InterfaceItem::Enum(value)
    }
}

impl From<FuncItem> for InterfaceItem {
    fn from(value: FuncItem) -> Self {
        InterfaceItem::Func(value)
    }
}

impl From<Flags> for InterfaceItem {
    fn from(value: Flags) -> Self {
        InterfaceItem::Flags(value)
    }
}

impl From<Resource> for InterfaceItem {
    fn from(value: Resource) -> Self {
        InterfaceItem::Resource(value)
    }
}

impl From<TypeAlias> for InterfaceItem {
    fn from(value: TypeAlias) -> Self {
        InterfaceItem::TypeAlias(value)
    }
}

impl From<Variant> for InterfaceItem {
    fn from(value: Variant) -> Self {
        InterfaceItem::Variant(value)
    }
}

impl From<Record> for InterfaceItem {
    fn from(value: Record) -> Self {
        InterfaceItem::Record(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncItem {
    pub name: Text,
    pub index: AnyFuncItemIndex,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemReference {
    pub file: FilePath,
    pub scope: ScopeIndex,
    pub item: ItemReferenceKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ItemReferenceKind {
    Enum(EnumIndex),
    Variant(VariantIndex),
    Flags(FlagsIndex),
    Record(RecordIndex),
    Resource(ResourceIndex),
    Func(FuncItemIndex),
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

impl From<FuncItemIndex> for ItemReferenceKind {
    fn from(v: FuncItemIndex) -> Self {
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
    pub name: Text,
    pub index: RecordIndex,
    pub docs: Option<Text>,
    pub fields: Vector<RecordField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordField {
    pub name: Text,
    pub docs: Option<Text>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub name: Text,
    pub index: EnumIndex,
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
    pub index: FlagsIndex,
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
    pub index: VariantIndex,
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
    pub index: TypeAliasIndex,
    pub docs: Option<Text>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub name: Text,
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
