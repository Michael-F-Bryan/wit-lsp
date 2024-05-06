//! Automatically generated code. DO NOT EDIT!
use super::CodeAction;
/// A list of all registered [`CodeAction`]s.
pub const CODE_ACTIONS: &[CodeAction] = &[
    CodeAction {
        name: "Extract Type Alias",
        description: "Extract a generic type (e.g. `list<tuple<string, string>>`) into a type\nalias.",
        execute: super::extract_type_alias::extract_type_alias_action,
    },
];
