use either::Either;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse,
};
use wit_compiler::{
    ast::{self, AstNode},
    queries::{SourceFile, Workspace},
};

pub fn complete(
    db: &dyn wit_compiler::Db,
    _ws: Workspace,
    file: SourceFile,
    params: CompletionParams,
) -> Option<CompletionResponse> {
    let point = crate::utils::position_to_ts(params.text_document_position.position);

    let mut completions = Vec::new();

    // First, add all known keywords
    let keywords = wit_compiler::ast::KEYWORDS.iter().map(|kw| CompletionItem {
        label: kw.to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    });
    completions.extend(keywords);

    // Now, we do the hard job of figuring out which identifiers are in scope.
    // FIXME: This doesn't take imported items into account
    let items = wit_compiler::queries::file_items(db, file);
    if let Some(index) = items.enclosing_item(db, point) {
        let types = match index {
            Either::Left(index) => items.get_world(db, index).items(db),
            Either::Right(index) => items.get_interface(db, index).items(db),
        };

        completions.extend(types.names().map(|label| CompletionItem {
            label: label.to_string(),
            ..Default::default()
        }));
    }

    let ast = wit_compiler::queries::parse(db, file);
    if let Some(ident) = ast
        .tree(db)
        .ancestors(point)
        .find_map(ast::Identifier::cast)
    {
        let src = file.contents(db);
        let ident = ident.value(&src);
        // The user has started writing an identifier, so limit the completions
        // to whatever might match what they've written.
        completions.retain(|c| c.label.starts_with(ident));
    }

    Some(CompletionResponse::Array(completions))
}
