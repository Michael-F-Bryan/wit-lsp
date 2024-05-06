use crate::code_actions::{Action, CodeActionContext};

/// # Extract Type Alias
///
/// Extract a generic type (e.g. `list<tuple<string, string>>`) into a type
/// alias.
pub(crate) fn extract_type_alias_action(_ctx: &CodeActionContext<'_>) -> Option<Action> {
    // TODO: implement this
    None
}
