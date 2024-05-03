use std::ops::Range;

use codespan_reporting::files::Error as CodespanError;
use im::Vector;

use crate::{queries::SourceFile, Db, Text};

#[salsa::tracked]
pub fn calculate_line_numbers(db: &dyn Db, file: SourceFile) -> LineNumbers {
    let src = file.contents(db);
    LineNumbers::for_text(src.clone())
}

/// A lookup table for efficiently looking up line numbers in a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineNumbers {
    line_starts: Vector<usize>,
    src: Text,
}

impl LineNumbers {
    pub fn for_text(src: Text) -> Self {
        let line_starts = codespan_reporting::files::line_starts(&src).collect();
        LineNumbers { line_starts, src }
    }

    /// The index of the line at the given byte offset.
    ///
    /// If the byte index is past the end of the file, an error is returned.
    pub fn line_index(&self, byte_offset: usize) -> Result<usize, CodespanError> {
        if byte_offset > self.src.len() {
            return Err(CodespanError::IndexTooLarge {
                given: byte_offset,
                max: self.src.len(),
            });
        }

        match self.line_starts.binary_search(&byte_offset) {
            Ok(line) => Ok(line),
            Err(next_line) => Ok(next_line - 1),
        }
    }

    /// The byte range of a line in the source of the file.
    pub fn line_range(&self, line_index: usize) -> Result<Range<usize>, CodespanError> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }

    /// Return the starting byte index of the line with the specified line index.
    pub fn line_start(&self, line_index: usize) -> Result<usize, CodespanError> {
        use std::cmp::Ordering;

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(self
                .line_starts
                .get(line_index)
                .cloned()
                .expect("failed despite previous check")),
            Ordering::Equal => Ok(self.src.len()),
            Ordering::Greater => Err(CodespanError::LineTooLarge {
                given: line_index,
                max: self.line_starts.len() - 1,
            }),
        }
    }

    pub fn point(&self, byte_index: usize) -> Result<tree_sitter::Point, CodespanError> {
        let row = self.line_index(byte_index)?;
        let range = self.line_range(row)?;
        let column = byte_index - range.start;

        Ok(tree_sitter::Point { row, column })
    }
}
