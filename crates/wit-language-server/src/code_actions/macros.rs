macro_rules! code_action_test {
    (
        $(#[$meta:meta])*
        name: $name:ident,
        code_action: $code_action:expr,
        before: { $($before_path:literal : $before_value:literal ),* $(,)? },
        after: { $($after_path:literal : $after_value:literal ),* $(,)? } $(,)?
    ) => {
        $( #[$meta] )*
        #[test]
        fn $name() {
            use wit_compiler::ast::AstNode as _;

            let mut db = crate::Database::default();
            let mut files = im::OrdMap::new();
            $(
                let path = format!("file:///{}", $before_path);
                let path = wit_compiler::queries::FilePath::new(&db, path.into());
                let f = wit_compiler::queries::SourceFile::new(&db, path, $before_value.into());
                files.insert(path, f);
            )*
            let ws = wit_compiler::queries::Workspace::new(&db, files);

            let (filename, range) = ws.files(&db)
                .values()
                .find_map(|&f| {
                    let ast = wit_compiler::queries::parse(&db, f);
                    let comment = ast
                        .tree(&db)
                        .iter()
                        .find_map(wit_compiler::ast::Comment::cast)?;
                    let path = f.path(&db).raw_path(&db).clone();
                    Some((path, comment.range()))
                })
                .unwrap();

            let ctx = crate::code_actions::CodeActionContext::from_lsp(
                &db,
                ws,
                tower_lsp::lsp_types::CodeActionParams {
                    text_document: tower_lsp::lsp_types::TextDocumentIdentifier {
                        uri: filename.parse().unwrap(),
                    },
                    range: crate::utils::ts_to_range(range),
                    context: Default::default(),
                    partial_result_params: Default::default(),
                    work_done_progress_params: Default::default(),
                },
            )
            .unwrap();

            let action = $code_action(&ctx).expect("No action generated");

            action.apply(&mut db, ws);

            $(
                let path = format!("file:///{}", $after_path);
                let file = ws.lookup_by_path(&db, &path)
                    .expect(concat!("Lookup ", stringify!($after_path), " failed"));
                let contents = file.contents(&db).as_str();
                pretty_assertions::assert_str_eq!(contents, $after_value, $after_path);
            )*
        }
    };
}
