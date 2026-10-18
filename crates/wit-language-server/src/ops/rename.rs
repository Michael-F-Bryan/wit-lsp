use std::collections::{BTreeMap, HashMap, HashSet};

use tower_lsp::{
    jsonrpc::Error,
    lsp_types::{
        DocumentChanges, OneOf, OptionalVersionedTextDocumentIdentifier, PrepareRenameResponse,
        TextDocumentEdit, TextEdit, Url, WorkspaceEdit,
    },
};
use wit_compiler::{
    Compiler, Db, Tree,
    ast::{self, AstNode, HasSource},
    diagnostics::{self, Diagnostic, Location},
    queries::{FilePath, SourceFile, Workspace},
};

use super::{
    definition::{identifier_at, symbol_binding, symbol_definition},
    references::identifiers,
};
use crate::utils::Positions;

pub fn prepare_rename(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
) -> Option<PrepareRenameResponse> {
    let (id, _) = target(db, ws, file, point)?;
    Some(PrepareRenameResponse::RangeWithPlaceholder {
        range: Positions::new(db, file).to_lsp(id.range()),
        placeholder: id
            .utf8_text(file.contents(db))
            .trim_start_matches('%')
            .to_owned(),
    })
}

fn target(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
) -> Option<(ast::Id<'_>, Location)> {
    let id = identifier_at(db, file, point)?;
    if !identifiers(db, file).contains(&id) {
        return None;
    }
    // Unresolved imports and invalid path kinds must not become rename targets.
    symbol_definition(db, ws, file, id)?;
    Some((id, symbol_binding(db, ws, file, id)?))
}

pub fn rename(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
    new_name: &str,
    versions: Option<&HashMap<String, i32>>,
) -> Result<WorkspaceEdit, Error> {
    let (_, binding) = target(db, ws, file, point)
        .ok_or_else(|| Error::invalid_params("This position does not name a renameable symbol"))?;
    let replacement = replacement_name(new_name)
        .ok_or_else(|| Error::invalid_params("Invalid WIT name. Use hyphen-separated ASCII words such as my-type, with each word entirely lowercase or uppercase. Digits are allowed after the first letter."))?;
    let mut edits = BTreeMap::new();
    for (path, file) in ws.files(db) {
        let mut ranges: Vec<_> = identifiers(db, file)
            .into_iter()
            .filter(|id| symbol_binding(db, ws, file, *id) == Some(binding))
            .map(|id| id.range())
            .collect();
        if ranges.is_empty() {
            continue;
        }
        ranges.sort_by_key(|range| range.start_byte);
        edits.insert(path.raw_path(db).to_string(), ranges);
    }
    validate_rename(db, ws, &edits, &replacement)?;
    let documents = edits
        .into_iter()
        .map(|(path, ranges)| {
            let uri = path
                .parse::<Url>()
                .map_err(|_| Error::invalid_params("The symbol's document has no valid URI"))?;
            let file = ws
                .lookup_by_path(db, &path)
                .ok_or_else(Error::internal_error)?;
            let positions = Positions::new(db, file);
            Ok(TextDocumentEdit {
                text_document: OptionalVersionedTextDocumentIdentifier {
                    uri,
                    version: versions.and_then(|versions| versions.get(&path).copied()),
                },
                edits: ranges
                    .into_iter()
                    .map(|range| {
                        OneOf::Left(TextEdit {
                            range: positions.to_lsp(range),
                            new_text: replacement.clone(),
                        })
                    })
                    .collect(),
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    if versions.is_some() {
        Ok(WorkspaceEdit {
            document_changes: Some(DocumentChanges::Edits(documents)),
            ..Default::default()
        })
    } else {
        Ok(WorkspaceEdit {
            changes: Some(
                documents
                    .into_iter()
                    .map(|document| {
                        let edits = document
                            .edits
                            .into_iter()
                            .filter_map(|edit| match edit {
                                OneOf::Left(edit) => Some(edit),
                                OneOf::Right(_) => None,
                            })
                            .collect();
                        (document.text_document.uri, edits)
                    })
                    .collect(),
            ),
            ..Default::default()
        })
    }
}

// Parse both declaration and reference positions, so keywords and primitive types
// receive %-escaping while whitespace, punctuation and injected syntax are rejected.
fn replacement_name(name: &str) -> Option<String> {
    if name.is_empty() || name.trim() != name {
        return None;
    }
    for candidate in [name.to_owned(), format!("%{name}")] {
        if ast::KEYWORDS.contains(&candidate.as_str()) {
            continue;
        }
        let source = format!(
            "interface rename-test {{ type {candidate} = u32; type result-type = {candidate}; }}"
        );
        let tree = Tree::parse(&source);
        if tree.root_node().has_error() {
            continue;
        }
        let declaration_start = "interface rename-test { type ".len();
        let reference_start = source.rfind(" = ")? + " = ".len();
        if [declaration_start, reference_start]
            .into_iter()
            .all(|start| {
                tree.iter()
                    .filter_map(ast::Id::cast)
                    .any(|id| id.range().start_byte == start && id.utf8_text(&source) == candidate)
            })
        {
            return Some(candidate);
        }
    }
    None
}

type Edits = BTreeMap<String, Vec<tree_sitter::Range>>;
type BindingKey = (String, usize, usize);

fn shifted(offset: usize, ranges: &[tree_sitter::Range], replacement_len: usize) -> usize {
    ranges
        .iter()
        .filter(|range| range.end_byte <= offset)
        .fold(offset as isize, |value, range| {
            value + replacement_len as isize - (range.end_byte - range.start_byte) as isize
        }) as usize
}

fn binding_key(
    db: &dyn Db,
    location: Location,
    edits: &Edits,
    replacement_len: usize,
) -> BindingKey {
    let path = location.filename.raw_path(db).to_string();
    let ranges = edits.get(&path).map(Vec::as_slice).unwrap_or_default();
    (
        path,
        shifted(location.range.start_byte, ranges, replacement_len),
        shifted(location.range.end_byte, ranges, replacement_len),
    )
}

/// Check a proposed edit in isolation. Every existing name must still resolve to
/// the same binding, and the compiler must not report a new duplicate declaration.
fn validate_rename(
    db: &dyn Db,
    ws: Workspace,
    edits: &Edits,
    replacement: &str,
) -> Result<(), Error> {
    let preview = Compiler::default();
    let files = ws
        .files(db)
        .iter()
        .map(|(path, file)| {
            let uri = path.raw_path(db);
            let mut text = file.contents(db).to_string();
            if let Some(ranges) = edits.get(uri.as_str()) {
                for range in ranges.iter().rev() {
                    text.replace_range(range.start_byte..range.end_byte, replacement);
                }
            }
            let path = FilePath::new(&preview, uri.clone());
            (path, SourceFile::new(&preview, path, text.into()))
        })
        .collect();
    let workspace = Workspace::new(&preview, files);
    let empty = Edits::new();
    for (path, file) in ws.files(db) {
        let uri = path.raw_path(db);
        let after = workspace
            .lookup_by_path(&preview, uri)
            .ok_or_else(Error::internal_error)?;
        let ranges = edits
            .get(uri.as_str())
            .map(Vec::as_slice)
            .unwrap_or_default();
        let new_ids: HashMap<_, _> = identifiers(&preview, after)
            .into_iter()
            .map(|id| (id.range().start_byte, id))
            .collect();
        for id in identifiers(db, file) {
            let offset = shifted(id.range().start_byte, ranges, replacement.len());
            let before = symbol_binding(db, ws, file, id)
                .map(|loc| binding_key(db, loc, edits, replacement.len()));
            let after_binding = new_ids
                .get(&offset)
                .and_then(|id| symbol_binding(&preview, workspace, after, *id))
                .map(|loc| binding_key(&preview, loc, &empty, 0));
            if before != after_binding {
                return Err(Error::invalid_params(
                    "Renaming would change which declaration another name refers to",
                ));
            }
        }
    }
    let duplicates = |db: &dyn Db, ws, edits: &Edits, len| -> HashSet<_> {
        diagnostics::check_all(db, ws)
            .into_iter()
            .filter_map(|diag| match diag {
                Diagnostic::DuplicateName(duplicate) => Some((
                    binding_key(db, duplicate.location, edits, len),
                    binding_key(db, duplicate.original_definition, edits, len),
                )),
                _ => None,
            })
            .collect()
    };
    let existing = duplicates(db, ws, edits, replacement.len());
    if !duplicates(&preview, workspace, &empty, 0).is_subset(&existing) {
        return Err(Error::invalid_params(
            "The new name conflicts with an existing declaration",
        ));
    }
    Ok(())
}
