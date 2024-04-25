use crate::{
    ast::AstNode,
    diagnostics::{Diagnostics, Location, SyntaxError},
    queries::SourceFile,
    Db, Text, Tree,
};

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(file = %file.path(db).raw_path(db)))]
pub fn parse(db: &dyn Db, file: SourceFile) -> Ast {
    let src = file.contents(db);
    let tree = Tree::parse(src);

    let root = tree.root_node();

    if root.has_error() {
        for error_node in tree.iter().filter(|node| node.is_error()) {
            if let Some(parent) = error_node.parent() {
                let location = Location::new(file.path(db), error_node.range());
                let diag = SyntaxError {
                    location,
                    rule: parent.kind().to_string(),
                };
                Diagnostics::push(db, diag.into());
            }
        }
    }

    Ast::new(db, tree, src.clone())
}

#[salsa::tracked]
#[derive(Debug, Clone)]
pub struct Ast {
    #[return_ref]
    pub tree: Tree,
    #[return_ref]
    pub src: Text,
}

impl Ast {
    pub fn root_node(self, db: &dyn Db) -> tree_sitter::Node<'_> {
        self.tree(db).root_node()
    }

    pub fn source_file(self, db: &dyn Db) -> crate::ast::SourceFile<'_> {
        let root = self.root_node(db);
        crate::ast::SourceFile::cast(root).unwrap()
    }
}
