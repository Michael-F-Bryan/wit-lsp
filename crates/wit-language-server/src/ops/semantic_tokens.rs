use tower_lsp::lsp_types::{
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
    SemanticTokensClientCapabilities, SemanticTokensLegend, SymbolKind,
};
use wit_compiler::{
    Db,
    ast::AstNode,
    queries::{self, SourceFile, Workspace},
};

use super::{
    definition::{declaration_for_identifier, symbol_definition},
    references::{identifiers, is_alias},
    symbols::symbol_name,
};
use crate::utils::Positions;

pub fn semantic_tokens_legend(
    client: Option<&SemanticTokensClientCapabilities>,
) -> SemanticTokensLegend {
    let token_types = vec![
        SemanticTokenType::NAMESPACE,
        SemanticTokenType::INTERFACE,
        SemanticTokenType::STRUCT,
        SemanticTokenType::CLASS,
        SemanticTokenType::ENUM,
        SemanticTokenType::ENUM_MEMBER,
        SemanticTokenType::TYPE,
        SemanticTokenType::FUNCTION,
        SemanticTokenType::METHOD,
        SemanticTokenType::PROPERTY,
        SemanticTokenType::PARAMETER,
    ];
    SemanticTokensLegend {
        token_types: token_types
            .into_iter()
            .filter(|kind| client.is_none_or(|client| client.token_types.contains(kind)))
            .collect(),
        token_modifiers: [SemanticTokenModifier::DECLARATION]
            .into_iter()
            .filter(|modifier| {
                client.is_none_or(|client| client.token_modifiers.contains(modifier))
            })
            .collect(),
    }
}

/// Classify identifiers through the same declarations used by navigation and hover.
pub fn semantic_tokens(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    legend: &SemanticTokensLegend,
) -> SemanticTokens {
    let mut ids = identifiers(db, file);
    ids.sort_by_key(|id| id.range().start_byte);
    let positions = Positions::new(db, file);
    let mut data = Vec::new();
    let mut previous = (0, 0);
    for id in ids {
        let declaration = declaration_for_identifier(db, file, id);
        let kind = if declaration.is_some() {
            id.syntax().parent().and_then(token_kind)
        } else {
            symbol_definition(db, ws, file, id).and_then(|location| {
                let target = ws.lookup_by_path(db, location.filename.raw_path(db))?;
                queries::parse(db, target)
                    .root_node(db)
                    .named_descendant_for_byte_range(
                        location.range.start_byte,
                        location.range.end_byte,
                    )
                    .and_then(token_kind)
            })
        };
        let Some(kind) = kind else { continue };
        let Some(token_type) = legend
            .token_types
            .iter()
            .position(|supported| *supported == kind)
            .or_else(|| {
                [
                    SemanticTokenType::STRUCT,
                    SemanticTokenType::CLASS,
                    SemanticTokenType::ENUM,
                ]
                .contains(&kind)
                .then(|| {
                    legend
                        .token_types
                        .iter()
                        .position(|kind| *kind == SemanticTokenType::TYPE)
                })
                .flatten()
            })
        else {
            continue;
        };
        let range = positions.to_lsp(id.range());
        if range.start.line != range.end.line || range.start == range.end {
            continue;
        }
        let modifier = (declaration.is_some() || is_alias(id))
            && legend
                .token_modifiers
                .contains(&SemanticTokenModifier::DECLARATION);
        data.push(SemanticToken {
            delta_line: range.start.line - previous.0,
            delta_start: if range.start.line == previous.0 {
                range.start.character - previous.1
            } else {
                range.start.character
            },
            length: range.end.character - range.start.character,
            token_type: token_type as u32,
            token_modifiers_bitset: u32::from(modifier),
        });
        previous = (range.start.line, range.start.character);
    }
    SemanticTokens {
        result_id: None,
        data,
    }
}

fn token_kind(node: tree_sitter::Node<'_>) -> Option<SemanticTokenType> {
    if node.kind() == "named_type" {
        return Some(SemanticTokenType::PARAMETER);
    }
    let (_, kind) = symbol_name(node)?;
    Some(match kind {
        SymbolKind::NAMESPACE | SymbolKind::PACKAGE => SemanticTokenType::NAMESPACE,
        SymbolKind::INTERFACE => SemanticTokenType::INTERFACE,
        SymbolKind::STRUCT => SemanticTokenType::STRUCT,
        SymbolKind::CLASS => SemanticTokenType::CLASS,
        SymbolKind::ENUM => SemanticTokenType::ENUM,
        SymbolKind::ENUM_MEMBER => SemanticTokenType::ENUM_MEMBER,
        SymbolKind::TYPE_PARAMETER => SemanticTokenType::TYPE,
        SymbolKind::FUNCTION => SemanticTokenType::FUNCTION,
        SymbolKind::METHOD => SemanticTokenType::METHOD,
        SymbolKind::FIELD => SemanticTokenType::PROPERTY,
        _ => return None,
    })
}
