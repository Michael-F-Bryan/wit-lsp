use wit_compiler::{
    Db,
    ast::{self, AstNode, HasSource},
    diagnostics::Location,
    queries::{self, SourceFile, Workspace, metadata::Ident},
};

/// Find the declaration of the type reference under the cursor.
pub fn goto_definition(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
) -> Option<Location> {
    let ast = queries::parse(db, file);
    if point >= ast.root_node(db).end_position() {
        return None;
    }
    let node = ast.root_node(db).descendant_for_point_range(point, point)?;
    let ty = std::iter::successors(Some(node), |node| node.parent()).find_map(ast::Ty::cast)?;
    let name = ty.id().or_else(|| {
        let handle = ty.handle()?;
        handle
            .borrowed_handle()
            .and_then(|h| h.id())
            .or_else(|| handle.owned_handle().and_then(|h| h.id()))
    })?;
    let range = name.range();
    if point < range.start_point || point >= range.end_point {
        return None;
    }
    let name = Ident::new(db, name.utf8_text(file.contents(db)).into());
    let reference = queries::resolve_type(db, ws, file, point, name)?;
    queries::type_definition(db, ws, reference)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wit_compiler::{Compiler, queries::FilePath};

    fn marked_source(source: &str) -> (String, tree_sitter::Point) {
        let offset = source.find('|').unwrap();
        let prefix = &source[..offset];
        let point = tree_sitter::Point::new(
            prefix.bytes().filter(|b| *b == b'\n').count(),
            prefix.rsplit('\n').next().unwrap().len(),
        );
        (source.replacen('|', "", 1), point)
    }

    #[test]
    fn resolves_type_uses_to_their_declarations() {
        for (source, expected) in [
            (
                "interface i { record point { x: u32 } type points = list<|point>; }",
                "record point { x: u32 }",
            ),
            (
                "interface i { type points = list<|point>; record point { x: u32 } }",
                "record point { x: u32 }",
            ),
            (
                "world w { type number = u32; type numbers = list<|number>; }",
                "type number = u32;",
            ),
            (
                "interface i { resource file; read: func(f: borrow<|file>); }",
                "resource file;",
            ),
            (
                "interface i { resource file; read: func(f: own<|file>); }",
                "resource file;",
            ),
            (
                "interface i { type count = u32; record r { field: |count } }",
                "type count = u32;",
            ),
        ] {
            let db = Compiler::default();
            let (source, point) = marked_source(source);
            let path = FilePath::new(&db, "file:///types.wit".into());
            let file = SourceFile::new(&db, path, source.clone().into());
            let ws = Workspace::new(&db, [(path, file)].into_iter().collect());
            let location = goto_definition(&db, ws, file, point).unwrap();
            assert_eq!(location.filename, path);
            assert_eq!(
                &source[location.range.start_byte..location.range.end_byte],
                expected
            );
        }
    }

    #[test]
    fn follows_renamed_import_to_original_file() {
        let db = Compiler::default();
        let (source, point) = marked_source(
            "interface consumer { use types.{point as location}; type x = option<|location>; }",
        );
        let source_path = FilePath::new(&db, "file:///types.wit".into());
        let target_path = FilePath::new(&db, "file:///consumer.wit".into());
        let definitions = "interface types { record point { x: u32 } }";
        let source_file = SourceFile::new(&db, source_path, definitions.into());
        let target_file = SourceFile::new(&db, target_path, source.into());
        let ws = Workspace::new(
            &db,
            [(source_path, source_file), (target_path, target_file)]
                .into_iter()
                .collect(),
        );
        let location = goto_definition(&db, ws, target_file, point).unwrap();
        assert_eq!(location.filename, source_path);
        assert_eq!(
            &definitions[location.range.start_byte..location.range.end_byte],
            "record point { x: u32 }"
        );
    }

    #[test]
    fn does_not_navigate_from_unrelated_identifiers_or_missing_types() {
        for source in [
            "interface i { type point = u32; record r { |point: u32 } }",
            "interface i { type point = u32; |point: func(); }",
            "interface i { type point = u32; type x = |list<point>; }",
            "interface i { type x = |missing; }",
            "interface i { type x = |u32; }",
            "interface i { /* |comment */ }",
            "interface i {}|",
        ] {
            let db = Compiler::default();
            let (source, point) = marked_source(source);
            let path = FilePath::new(&db, "file:///types.wit".into());
            let file = SourceFile::new(&db, path, source.into());
            let ws = Workspace::new(&db, [(path, file)].into_iter().collect());
            assert!(goto_definition(&db, ws, file, point).is_none());
        }
    }
}
