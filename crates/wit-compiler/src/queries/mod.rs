//! Various operations used by the compiler and language server.

mod parsing;
mod selection;

pub use self::{
    parsing::{parse, Ast, Workspace},
    selection::selection_ranges,
};
