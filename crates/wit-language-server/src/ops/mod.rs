mod completion;
mod definition;
mod diagnostics;
mod fold;
mod hover;

pub use self::{
    completion::complete, definition::goto_definition, diagnostics::file_diagnostics,
    fold::folding_range, hover::hover,
};
