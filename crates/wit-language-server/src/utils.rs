use wit_compiler::diagnostics::{Diagnostic, Location};

pub fn ts_to_range(range: tree_sitter::Range) -> tower_lsp::lsp_types::Range {
    let tree_sitter::Range {
        start_point,
        end_point,
        ..
    } = range;

    tower_lsp::lsp_types::Range {
        start: ts_to_position(start_point),
        end: ts_to_position(end_point),
    }
}

pub fn ts_to_position(point: tree_sitter::Point) -> tower_lsp::lsp_types::Position {
    let tree_sitter::Point { row, column } = point;
    tower_lsp::lsp_types::Position {
        line: row.try_into().unwrap(),
        character: column.try_into().unwrap(),
    }
}

pub fn location_to_lsp(
    db: &dyn wit_compiler::Db,
    location: Location,
) -> tower_lsp::lsp_types::Location {
    let Location { filename, range } = location;

    tower_lsp::lsp_types::Location {
        uri: filename
            .raw_path(db)
            .parse()
            .expect("All filenames should be URIs"),
        range: ts_to_range(range),
    }
}

pub fn position_to_ts(position: tower_lsp::lsp_types::Position) -> tree_sitter::Point {
    tree_sitter::Point {
        row: position.line as usize,
        column: position.character as usize,
    }
}

/// Generate a hash code for a particular [`Diagnostic`] that is guaranteed to
/// be stable across multiple runs within the same process.
pub fn hash_diagnostic(diag: &Diagnostic) -> u64 {
    ahash::RandomState::with_seed(0xdead_beef_cafe_babe).hash_one(diag)
}
