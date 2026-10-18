mod completion;
mod definition;
mod diagnostics;
mod fold;
mod highlights;
mod hover;
mod references;
mod rename;
mod semantic_tokens;
mod symbols;

pub use self::{
    completion::complete,
    definition::{goto_declaration, goto_definition},
    diagnostics::file_diagnostics,
    fold::folding_range,
    highlights::document_highlights,
    hover::hover,
    references::find_references,
    rename::{prepare_rename, rename},
    semantic_tokens::{semantic_tokens, semantic_tokens_legend},
    symbols::{document_symbols, workspace_symbols},
};
