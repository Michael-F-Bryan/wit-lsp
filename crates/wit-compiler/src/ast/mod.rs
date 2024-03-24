//! Strongly-typed AST nodes.

mod generated;
mod traits;

pub use self::{
    generated::*,
    traits::{AstNode, HasAttr, HasIdent, NodeExt},
};
