use clap::Parser;
use clap_stdin::{FileOrStdin, Source};
use im::OrdMap;
use wit_compiler::{
    diagnostics::Diagnostics,
    queries::{SourceFile, Workspace},
    Compiler, FilePath, Text,
};

use crate::Format;

#[derive(Debug, Clone, Parser)]
pub(crate) struct Parse {
    #[clap(short, long, default_value_t = Format::default())]
    format: Format,
    #[clap(default_value = "-")]
    input: FileOrStdin,
}

impl Parse {
    #[tracing::instrument(skip_all)]
    pub fn run(self) -> color_eyre::Result<()> {
        let db = Compiler::default();

        let path = match &self.input.source {
            Source::Arg(path) => FilePath::from(path),
            Source::Stdin => FilePath::from("unnamed.wit"),
        };

        let input = self.input.contents()?;
        let input = Text::from(input);

        let file = SourceFile::new(&db, path, input.clone());

        let ast = wit_compiler::queries::parse(&db, file);
        let diags = wit_compiler::queries::parse::accumulated::<Diagnostics>(&db, file);

        let mut stderr = std::io::stderr();
        let ws = Workspace::new(&db, OrdMap::unit(file.path(&db).clone(), file));
        crate::print_diags(&mut stderr, &db, ws, &diags)?;

        let root = ast.root_node(&db);

        match self.format {
            Format::Text => println!("{}", root.to_sexp()),
            Format::Json => print_json_ast(root)?,
        }

        Ok(())
    }
}

fn print_json_ast(_node: tree_sitter::Node<'_>) -> color_eyre::Result<()> {
    todo!();
}
