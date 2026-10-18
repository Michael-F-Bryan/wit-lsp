use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};
use wit_compiler::{
    Db,
    ast::{self, AstNode, HasSource},
    queries::{self, SourceFile, Workspace},
};

use super::definition::{identifier_at, symbol_definition};

/// Show the declaration and documentation of the name under the cursor.
pub fn hover(
    db: &dyn Db,
    ws: Workspace,
    file: SourceFile,
    point: tree_sitter::Point,
    markdown: bool,
) -> Option<Hover> {
    let id = identifier_at(db, file, point)?;
    let location = symbol_definition(db, ws, file, id)?;
    let target = ws.lookup_by_path(db, location.filename.raw_path(db))?;
    let ast = queries::parse(db, target);
    let node = ast
        .root_node(db)
        .named_descendant_for_byte_range(location.range.start_byte, location.range.end_byte)?;
    let src = target.contents(db);
    let mut start = node.start_byte();
    let mut docs = Vec::new();
    let mut cursor = node.walk();
    for attribute in node.children_by_field_name("attributes", &mut cursor) {
        if let Some(doc) = ast::Attribute::cast(attribute).and_then(|attr| attr.doc_comment()) {
            let text = doc.utf8_text(src).strip_prefix("///")?;
            docs.push(
                text.strip_prefix(' ')
                    .unwrap_or(text)
                    .trim_end_matches(['\r', '\n']),
            );
        }
        start = attribute.end_byte();
    }
    let declaration = declaration_text(node, src, start);
    let docs = docs.join("\n");
    let mut value = if markdown {
        // A declaration may contain comments with Markdown fences.
        let fence = "`".repeat(
            declaration
                .split(|c| c != '`')
                .map(str::len)
                .max()
                .unwrap_or(0)
                .max(2)
                + 1,
        );
        format!("{fence}wit\n{declaration}\n{fence}")
    } else {
        declaration.to_owned()
    };
    if !docs.is_empty() {
        value.push_str("\n\n");
        value.push_str(&docs);
    }
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: if markdown {
                MarkupKind::Markdown
            } else {
                MarkupKind::PlainText
            },
            value,
        }),
        range: Some(crate::utils::Positions::new(db, file).to_lsp(id.range())),
    })
}

fn declaration_text(node: tree_sitter::Node<'_>, src: &str, start: usize) -> String {
    let text = &src[start..node.end_byte()];
    let start = start + text.len() - text.trim_start().len();
    let body = match node.kind() {
        "interface_item" | "world_item" => node.child_by_field_name("body"),
        "resource_item" => ast::ResourceItem::cast(node)
            .and_then(|resource| resource.resource_body_opt())
            .map(|body| body.syntax()),
        _ => None,
    };
    // Container hovers describe the container, not every declaration inside it.
    if let Some(body) = body {
        let header = src[start..body.start_byte()].trim_end();
        let contents = if body.named_child_count() == 0 {
            "{}"
        } else {
            "{ … }"
        };
        return format!("{header} {contents}");
    }

    let line_start = src[..start].rfind('\n').map_or(0, |offset| offset + 1);
    let prefix = &src[line_start..start];
    let indent = if prefix.bytes().all(|b| matches!(b, b' ' | b'\t')) {
        prefix
    } else {
        ""
    };
    src[start..node.end_byte()]
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if index == 0 {
                line
            } else {
                line.strip_prefix(indent).unwrap_or(line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use wit_compiler::{Compiler, queries::FilePath};

    fn contents(source: &str, markdown: bool) -> Option<String> {
        let offset = source.find('|').unwrap();
        let prefix = &source[..offset];
        let point = tree_sitter::Point::new(
            prefix.bytes().filter(|b| *b == b'\n').count(),
            prefix.rsplit('\n').next().unwrap().len(),
        );
        let db = Compiler::default();
        let path = FilePath::new(&db, "file:///types.wit".into());
        let file = SourceFile::new(&db, path, source.replacen('|', "", 1).into());
        let ws = Workspace::new(&db, [(path, file)].into_iter().collect());
        let HoverContents::Markup(content) = hover(&db, ws, file, point, markdown)?.contents else {
            panic!("expected markup content");
        };
        Some(content.value)
    }

    #[test]
    fn shows_declarations_and_attached_documentation() {
        for (source, expected) in [
            (
                "/// An interface.\ninterface |i {}",
                "interface i {}\n\nAn interface.",
            ),
            ("/// A world.\nworld |w {}", "world w {}\n\nA world."),
            (
                "interface i { /// Count.\ntype |count = u32; }",
                "type count = u32;\n\nCount.",
            ),
            (
                "interface i { /// A record.\nrecord |r { x: u32 } }",
                "record r { x: u32 }\n\nA record.",
            ),
            (
                "interface i { record r { /// X.\n|x: u32 } }",
                "x: u32\n\nX.",
            ),
            (
                "interface i { /// Run.\n|run: func(x: u32); }",
                "run: func(x: u32);\n\nRun.",
            ),
            ("interface i { f: func(|x: u32); }", "x: u32"),
            ("interface i { enum e { /// Value.\n|v } }", "v\n\nValue."),
            ("interface i { flags f { |v } }", "v"),
            ("interface i { variant v { |some(u32) } }", "some(u32)"),
            ("interface i { resource |r; }", "resource r;"),
            ("interface i { resource r { |f: func(); } }", "f: func();"),
            (
                "interface i { resource r { |f: static func(); } }",
                "f: static func();",
            ),
            ("world w { import |f: func(); }", "import f: func();"),
            ("world w { export |f: func(); }", "export f: func();"),
        ] {
            assert_eq!(
                contents(source, false).as_deref(),
                Some(expected),
                "{source}"
            );
        }
    }

    #[test]
    fn preserves_markdown_and_doc_indentation_without_ordinary_comments() {
        let source = "interface i { // not documentation\n/// A **count**.\r\n///\r\n///     example\r\ntype count = u32; type x = |count; }";
        assert_eq!(
            contents(source, true).unwrap(),
            "```wit\ntype count = u32;\n```\n\nA **count**.\n\n    example"
        );
        assert_eq!(
            contents("interface i { record |r { /* ``` */ x: u32 } }", true).unwrap(),
            "````wit\nrecord r { /* ``` */ x: u32 }\n````"
        );
    }

    #[test]
    fn removes_enclosing_indentation_but_preserves_member_indentation() {
        for (indent, member) in [("    ", "        "), ("\t", "\t\t")] {
            for newline in ["\n", "\r\n"] {
                let source = format!(
                    "interface i {{{newline}{indent}/// A record.{newline}{indent}record |r {{{newline}{member}x: u32,{newline}{indent}}}{newline}}}"
                );
                let relative = member.strip_prefix(indent).unwrap();
                assert_eq!(
                    contents(&source, true).unwrap(),
                    format!("```wit\nrecord r {{\n{relative}x: u32,\n}}\n```\n\nA record.")
                );
            }
        }
    }

    #[test]
    fn container_hovers_omit_members_and_their_documentation() {
        for (source, expected) in [
            (
                "/// API docs.\ninterface |i { /// Member docs.\nrecord r { x: u32 } }",
                "interface i { … }\n\nAPI docs.",
            ),
            (
                "/// World docs.\nworld |w { import f: func(); }",
                "world w { … }\n\nWorld docs.",
            ),
            (
                "interface i { /// Resource docs.\nresource |r { /// Method docs.\nf: func(); } }",
                "resource r { … }\n\nResource docs.",
            ),
        ] {
            assert_eq!(contents(source, false).as_deref(), Some(expected));
        }
    }

    #[test]
    fn unresolved_names_and_non_identifiers_have_no_hover() {
        for source in [
            "interface i { type x = |missing; }",
            "interface i { use missing.{|value}; }",
            "interface i { use missing.{value as |alias}; }",
            "world w { include missing with { |value as alias }; }",
            "interface i { type x = |u32; }",
            "interface | i {}",
            "interface i {}|",
            "/// |Documentation\ninterface i {}",
            "interface i { type x| = u32; }",
        ] {
            assert_eq!(contents(source, true), None, "{source}");
        }
    }
}
