//! Automated refactoring and code fixes.

#[cfg(test)]
#[macro_use]
mod macros;

mod action;
#[rustfmt::skip]
mod all;
mod context;
mod extract_type_alias;
mod text_edit;

use tower_lsp::lsp_types::{CodeActionParams, CodeActionResponse};
use wit_compiler::queries::Workspace;

use crate::Db;

pub use self::{
    action::{Action, ActionKind},
    all::CODE_ACTIONS,
    context::CodeActionContext,
    text_edit::{Indel, TextEdit, TextEditBuilder},
};

/// Find all the *Code Actions* that apply in the current context.
#[tracing::instrument(skip_all)]
pub fn resolve(db: &dyn Db, ws: Workspace, params: CodeActionParams) -> Option<CodeActionResponse> {
    let ctx = CodeActionContext::from_lsp(db, ws, params)?;

    let actions = CODE_ACTIONS
        .iter()
        .filter_map(|action| (action.execute)(&ctx))
        .map(|action| action.to_lsp(db.as_wit(), ws).into())
        .collect();

    Some(actions)
}

#[derive(Debug, Copy, Clone)]
pub struct CodeAction {
    pub name: &'static str,
    pub description: &'static str,
    pub execute: fn(&CodeActionContext<'_>) -> Option<Action>,
}
