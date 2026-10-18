use tower_lsp::lsp_types::{
    DocumentSymbol, DocumentSymbolResponse, Location, SymbolInformation, SymbolKind, Url,
};
use tree_sitter::Node;
use wit_compiler::{
    Db,
    queries::{self, SourceFile, Workspace},
};

use crate::utils::Positions;

/// Search declarations in all known files using a case-insensitive name fragment.
pub fn workspace_symbols(
    db: &dyn Db,
    ws: Workspace,
    query: &str,
    supported_kinds: Option<&[SymbolKind]>,
) -> Vec<SymbolInformation> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for (path, file) in ws.files(db) {
        let Ok(uri) = path.raw_path(db).parse::<Url>() else {
            continue;
        };
        if let DocumentSymbolResponse::Flat(symbols) =
            document_symbols(db, file, uri, false, supported_kinds.is_some())
        {
            results.extend(symbols.into_iter().filter_map(|mut symbol| {
                if !symbol.name.to_lowercase().contains(&query) {
                    return None;
                }
                if let Some(kinds) = supported_kinds
                    && !kinds.contains(&symbol.kind)
                {
                    let fallback = legacy_kind(symbol.kind);
                    symbol.kind = if kinds.contains(&fallback) {
                        fallback
                    } else {
                        kinds.first().copied().unwrap_or(fallback)
                    };
                }
                Some(symbol)
            }));
        }
    }
    results.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then(left.location.uri.cmp(&right.location.uri))
            .then(left.location.range.start.cmp(&right.location.range.start))
    });
    results
}

/// Describe declarations in source order, preserving their syntactic owners.
pub fn document_symbols(
    db: &dyn Db,
    file: SourceFile,
    uri: Url,
    hierarchical: bool,
    extended_kinds: bool,
) -> DocumentSymbolResponse {
    let ast = queries::parse(db, file);
    let symbols = collect(
        ast.root_node(db),
        file.contents(db),
        &Positions::new(db, file),
        extended_kinds,
    );
    if hierarchical {
        DocumentSymbolResponse::Nested(symbols)
    } else {
        let mut flat = Vec::new();
        flatten(symbols, &uri, None, &mut flat);
        DocumentSymbolResponse::Flat(flat)
    }
}

fn collect(
    node: Node<'_>,
    src: &str,
    positions: &Positions<'_>,
    extended_kinds: bool,
) -> Vec<DocumentSymbol> {
    let mut cursor = node.walk();
    let children: Vec<_> = node
        .named_children(&mut cursor)
        .flat_map(|child| collect(child, src, positions, extended_kinds))
        .collect();
    if let Some((name, kind)) = symbol_name(node) {
        if name.is_missing() || name.start_byte() == name.end_byte() {
            return children;
        }
        let name_text = &src[name.byte_range()];
        let detail = node
            .child_by_field_name("type")
            .or_else(|| {
                let mut cursor = node.walk();
                node.named_children(&mut cursor)
                    .find(|child| child.kind() == "func_type")
            })
            .map(|ty| {
                src[ty.byte_range()]
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            });
        #[allow(deprecated)] // lsp-types requires the legacy field in struct literals.
        let symbol = DocumentSymbol {
            name: name_text.strip_prefix('%').unwrap_or(name_text).to_owned(),
            detail,
            kind: if extended_kinds {
                kind
            } else {
                legacy_kind(kind)
            },
            tags: None,
            deprecated: None,
            range: positions.to_lsp(node.range()),
            selection_range: positions.to_lsp(name.range()),
            children: (!children.is_empty()).then_some(children),
        };
        vec![symbol]
    } else {
        children
    }
}

pub(super) fn symbol_name(node: Node<'_>) -> Option<(Node<'_>, SymbolKind)> {
    let (field, kind) = match node.kind() {
        "interface_item" => ("name", SymbolKind::INTERFACE),
        "world_item" => ("name", SymbolKind::NAMESPACE),
        "record_item" => ("name", SymbolKind::STRUCT),
        "resource_item" => ("name", SymbolKind::CLASS),
        "enum_items" | "flags_items" | "variant_items" => ("name", SymbolKind::ENUM),
        "enum_case" | "flags_case" | "variant_case" => ("name", SymbolKind::ENUM_MEMBER),
        "record_field" => ("name", SymbolKind::FIELD),
        "type_item" => ("alias", SymbolKind::TYPE_PARAMETER),
        "func_item" => (
            "name",
            if node
                .parent()
                .is_some_and(|parent| parent.kind() == "resource_method")
            {
                SymbolKind::METHOD
            } else {
                SymbolKind::FUNCTION
            },
        ),
        "static_resource_method" => ("name", SymbolKind::METHOD),
        "resource_constructor" => {
            let mut cursor = node.walk();
            return node
                .children(&mut cursor)
                .find(|child| child.kind() == "constructor")
                .map(|name| (name, SymbolKind::CONSTRUCTOR));
        }
        "package_decl" => {
            return child(node, "package_name").map(|name| (name, SymbolKind::PACKAGE));
        }
        "use_names_item" => {
            return node
                .child_by_field_name("alias")
                .or_else(|| node.child_by_field_name("name"))
                .map(|name| (name, SymbolKind::TYPE_PARAMETER));
        }
        "toplevel_use_item" => {
            let mut cursor = node.walk();
            let alias = node
                .children_by_field_name("alias", &mut cursor)
                .find(|child| child.kind() == "id");
            return alias
                .or_else(|| node.child_by_field_name("path"))
                .map(|name| (name, SymbolKind::NAMESPACE));
        }
        "import_item" | "export_item" => {
            let external = child(node, "extern_type");
            let kind = if external.and_then(|ty| child(ty, "func_type")).is_some() {
                SymbolKind::FUNCTION
            } else {
                SymbolKind::INTERFACE
            };
            return node
                .child_by_field_name("name")
                .or_else(|| child(node, "use_path"))
                .map(|name| (name, kind));
        }
        _ => return None,
    };
    node.child_by_field_name(field).map(|name| (name, kind))
}

// Without a valueSet capability, clients only support the original 18 kinds.
fn legacy_kind(kind: SymbolKind) -> SymbolKind {
    match kind {
        SymbolKind::STRUCT => SymbolKind::CLASS,
        SymbolKind::ENUM_MEMBER => SymbolKind::CONSTANT,
        SymbolKind::TYPE_PARAMETER => SymbolKind::VARIABLE,
        _ => kind,
    }
}

fn child<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find(|child| child.kind() == kind)
}

fn flatten(
    symbols: Vec<DocumentSymbol>,
    uri: &Url,
    parent: Option<&str>,
    output: &mut Vec<SymbolInformation>,
) {
    for symbol in symbols {
        #[allow(deprecated)] // lsp-types requires the legacy field in struct literals.
        output.push(SymbolInformation {
            name: symbol.name.clone(),
            kind: symbol.kind,
            tags: symbol.tags,
            deprecated: None,
            location: Location {
                uri: uri.clone(),
                range: symbol.selection_range,
            },
            container_name: parent.map(str::to_owned),
        });
        if let Some(children) = symbol.children {
            flatten(children, uri, Some(&symbol.name), output);
        }
    }
}
