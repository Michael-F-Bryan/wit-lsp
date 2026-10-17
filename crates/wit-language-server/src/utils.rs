use tower_lsp::{
    jsonrpc::Error,
    lsp_types::{Position, Range},
};
use wit_compiler::{
    Db,
    diagnostics::{Diagnostic, Location},
    queries::{LineNumbers, SourceFile, Workspace, calculate_line_numbers},
};

/// Translate the server's UTF-16 positions and the parser's UTF-8 byte offsets.
/// Each mapper uses the contents of the document whose positions it converts.
pub(crate) struct Positions<'a> {
    src: &'a str,
    lines: LineNumbers,
}

impl<'a> Positions<'a> {
    pub fn for_text(src: &'a str) -> Self {
        Self {
            src,
            lines: LineNumbers::for_text(src.into()),
        }
    }

    pub fn new(db: &'a dyn Db, file: SourceFile) -> Self {
        Self {
            src: file.contents(db),
            lines: calculate_line_numbers(db, file),
        }
    }

    pub fn to_lsp(&self, range: tree_sitter::Range) -> Range {
        Range::new(
            self.offset_to_lsp(range.start_byte),
            self.offset_to_lsp(range.end_byte),
        )
    }

    pub fn offset_to_lsp(&self, offset: usize) -> Position {
        let row = self
            .lines
            .line_index(offset)
            .expect("source offset within document");
        let start = self.lines.line_start(row).unwrap();
        let prefix = &self.src[start..offset];
        let prefix = prefix.strip_suffix('\r').unwrap_or(prefix);
        Position::new(
            row.try_into().unwrap(),
            prefix.encode_utf16().count().try_into().unwrap(),
        )
    }

    pub fn to_point(&self, position: Position) -> Result<tree_sitter::Point, Error> {
        let row = position.line as usize;
        if row > self.lines.line_index(self.src.len()).unwrap() {
            return Err(Error::invalid_params(
                "Position line is outside the document",
            ));
        }
        let range = self.lines.line_range(row).unwrap();
        let line = &self.src[range];
        let line = line.strip_suffix('\n').unwrap_or(line);
        let line = line.strip_suffix('\r').unwrap_or(line);
        let target = position.character as usize;
        let mut units = 0;
        for (column, ch) in line.char_indices() {
            if units == target {
                return Ok(tree_sitter::Point { row, column });
            }
            units += ch.len_utf16();
            if units > target {
                return Err(Error::invalid_params(
                    "Position splits a UTF-16 surrogate pair",
                ));
            }
        }
        // LSP positions beyond the line's length refer to the end of the line.
        Ok(tree_sitter::Point {
            row,
            column: line.len(),
        })
    }

    pub fn to_range(&self, range: Range) -> Result<tree_sitter::Range, Error> {
        let start_point = self.to_point(range.start)?;
        let end_point = self.to_point(range.end)?;
        let start_byte = self.lines.offset_for_point(start_point).unwrap();
        let end_byte = self.lines.offset_for_point(end_point).unwrap();
        if start_byte > end_byte {
            return Err(Error::invalid_params("Range ends before it starts"));
        }
        Ok(tree_sitter::Range {
            start_byte,
            end_byte,
            start_point,
            end_point,
        })
    }
}

pub fn location_to_lsp(
    db: &dyn Db,
    ws: Workspace,
    location: Location,
) -> Option<tower_lsp::lsp_types::Location> {
    let Location { filename, range } = location;
    let file = ws.lookup_by_path(db, filename.raw_path(db))?;
    Some(tower_lsp::lsp_types::Location {
        uri: filename
            .raw_path(db)
            .parse()
            .expect("All filenames should be URIs"),
        range: Positions::new(db, file).to_lsp(range),
    })
}

/// Generate a hash code for a particular [`Diagnostic`] that is guaranteed to
/// be stable across multiple runs within the same process.
pub fn hash_diagnostic(diag: &Diagnostic) -> u64 {
    ahash::RandomState::with_seed(0xdead_beef_cafe_babe).hash_one(diag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_positions_round_trip() {
        for newline in ["\n", "\r\n"] {
            let src = format!("aé😀z{newline}β{newline}");
            let positions = Positions::for_text(&src);
            for (offset, line, character) in [
                (0, 0, 0),
                (1, 0, 1),
                (3, 0, 2),
                (7, 0, 4),
                (8, 0, 5),
                (8 + newline.len(), 1, 0),
                (10 + newline.len(), 1, 1),
                (src.len(), 2, 0),
            ] {
                let expected = Position::new(line, character);
                assert_eq!(positions.offset_to_lsp(offset), expected);
                let point = positions.to_point(expected).unwrap();
                assert_eq!(positions.lines.offset_for_point(point).unwrap(), offset);
            }
            assert_eq!(positions.to_point(Position::new(0, 999)).unwrap().column, 8);
            assert!(positions.to_point(Position::new(0, 3)).is_err());
            assert!(positions.to_point(Position::new(3, 0)).is_err());
            assert!(
                positions
                    .to_range(Range::new(Position::new(1, 0), Position::new(0, 0)))
                    .is_err()
            );
        }
    }

    #[test]
    fn empty_document_has_one_empty_line() {
        let positions = Positions::for_text("");
        assert_eq!(positions.offset_to_lsp(0), Position::new(0, 0));
        assert_eq!(
            positions.to_point(Position::new(0, 99)).unwrap(),
            tree_sitter::Point::new(0, 0)
        );
        assert!(positions.to_point(Position::new(1, 0)).is_err());
    }
}
