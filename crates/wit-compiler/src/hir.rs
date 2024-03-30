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
    pub docs: Text,
}

#[salsa::tracked]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interface {
    pub docs: Text,
    pub items: Vector<InterfaceItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceItem {
    Func(FuncItem),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncItem {
    pub docs: Text,
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Builtin {
    Float32,
    Float64,
    I16,
    I32,
    I64,
    I8,
    String,
    U16,
    U32,
    U64,
    U8,
    Boolean,
}
