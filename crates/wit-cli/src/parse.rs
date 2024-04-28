use clap::Parser;
use clap_stdin::{FileOrStdin, Source};
use codespan_reporting::term::termcolor::ColorChoice;
use im::OrdMap;
use wit_compiler::{
    diagnostics::Diagnostics,
    queries::{FilePath, SourceFile, Workspace},
    traverse::Cursor,
    Compiler, Text,
};

use crate::Format;

#[derive(Debug, Clone, Parser)]
pub(crate) struct Parse {
    #[clap(short, long, default_value_t = Format::default())]
    format: Format,
    #[clap(long, default_value = "auto")]
    color: ColorChoice,
    #[clap(default_value = "-")]
    input: FileOrStdin,
}

impl Parse {
    #[tracing::instrument(skip_all)]
    pub fn run(self) -> color_eyre::Result<()> {
        let Parse {
            format,
            color,
            input,
        } = self;

        let db = Compiler::default();

        let path = match &input.source {
            Source::Arg(path) => FilePath::new(&db, path.into()),
            Source::Stdin => FilePath::new(&db, "<stdin>".into()),
        };

        let input = input.contents()?;
        let input = Text::from(input);

        let file = SourceFile::new(&db, path, input.clone());

        let ast = wit_compiler::queries::parse(&db, file);
        let diags = wit_compiler::queries::parse::accumulated::<Diagnostics>(&db, file);

        let ws = Workspace::new(&db, OrdMap::unit(file.path(&db), file));
        let mut stderr = codespan_reporting::term::termcolor::StandardStream::stderr(color);
        crate::print_diagnostics(&mut stderr, &db, ws, &diags)?;

        let tree = ast.tree(&db);

        match format {
            Format::Text => println!("{:#}", tree.root_node()),
            Format::Json => print_json_ast(tree, &input)?,
        }

        Ok(())
    }
}

fn print_json_ast(tree: &tree_sitter::Tree, src: &str) -> color_eyre::Result<()> {
    let lowered = lower_tree(tree.root_node(), src);
    let serialized = serde_json::to_string_pretty(&lowered)?;
    println!("{serialized}");
    Ok(())
}

fn lower_tree<'a>(node: tree_sitter::Node<'a>, src: &'a str) -> Node<'a> {
    let mut cursor = node.walk();
    build_node(&mut cursor, src)
}

fn build_node<'a>(cursor: &mut tree_sitter::TreeCursor<'a>, src: &'a str) -> Node<'a> {
    let node = cursor.node();
    let kind = node.kind();
    let range = Range::from(node.range());
    let name = cursor.field_name();

    if node.child_count() == 0 {
        // It's a token node (leaf node)
        Node {
            kind,
            location: range,
            name,
            inner: NodeKind::Token {
                value: node.utf8_text(src.as_bytes()).unwrap_or(""),
            },
        }
    } else {
        // It's a non-terminal node
        let mut children = Vec::new();

        if cursor.goto_first_child() {
            loop {
                children.push(build_node(cursor, src));
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }

        Node {
            kind,
            location: range,
            name,
            inner: NodeKind::NonTerminal { children },
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct Node<'a> {
    #[serde(flatten)]
    inner: NodeKind<'a>,
    kind: &'a str,
    location: Range,
    name: Option<&'a str>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type")]
enum NodeKind<'a> {
    Token { value: &'a str },
    NonTerminal { children: Vec<Node<'a>> },
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
struct Range {
    start_byte: usize,
    end_byte: usize,
    start_point: Point,
    end_point: Point,
}

impl From<tree_sitter::Range> for Range {
    fn from(value: tree_sitter::Range) -> Range {
        let tree_sitter::Range {
            start_byte,
            end_byte,
            start_point,
            end_point,
        } = value;
        Range {
            start_byte,
            end_byte,
            start_point: start_point.into(),
            end_point: end_point.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
struct Point {
    row: usize,
    column: usize,
}

impl From<tree_sitter::Point> for Point {
    fn from(value: tree_sitter::Point) -> Self {
        let tree_sitter::Point { column, row } = value;
        Point { column, row }
    }
}
