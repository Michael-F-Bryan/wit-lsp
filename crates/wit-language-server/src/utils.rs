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
    location: wit_compiler::diagnostics::Location,
) -> tower_lsp::lsp_types::Location {
    let wit_compiler::diagnostics::Location { filename, range } = location;

    tower_lsp::lsp_types::Location {
        uri: filename.parse().expect("All filenames should be URIs"),
        range: ts_to_range(range),
    }
}

pub fn position_to_ts(position: tower_lsp::lsp_types::Position) -> tree_sitter::Point {
    tree_sitter::Point {
        row: position.line as usize,
        column: position.character as usize,
    }
}
