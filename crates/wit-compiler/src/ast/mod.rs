//! Strongly-typed AST nodes.

#[rustfmt::skip]
mod generated;
mod traits;

pub use self::{
    generated::*,
    traits::{AstNode, HasAttr, HasIdent, NodeExt},
};
