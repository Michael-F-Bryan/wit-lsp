mod completion;
mod definition;
mod diagnostics;
mod fold;

pub use self::{
    completion::complete, definition::goto_definition, diagnostics::file_diagnostics,
    fold::folding_range,
};
