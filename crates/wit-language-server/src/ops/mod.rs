mod completion;
mod diagnostics;
mod fold;

pub use self::{completion::complete, diagnostics::file_diagnostics, fold::folding_range};
