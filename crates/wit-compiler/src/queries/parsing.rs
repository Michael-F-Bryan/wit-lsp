use std::fmt::Display;

use im::OrdMap;
use tree_sitter::Parser;

use crate::{ast::AstNode, traverse::Order, Db, Text};

#[salsa::tracked]
pub fn parse(db: &dyn Db, ws: Workspace, path: Text) -> Option<Ast> {
    let files = ws.files(db);
    let src = files.get(&path)?;

    Some(Ast::new(db, Tree::parse(src)))
}

impl Tree {
    pub fn parse(src: &str) -> Tree {
        let mut parser = Parser::new();
        let lang = tree_sitter_wit::language();
        parser
            .set_language(&lang)
            .expect("The tree-sitter dependencies should be in sync");

        // TODO: Use incremental parsing if we've already parsed this file before
        parser
            .parse(src.as_bytes(), None)
            .expect("We don't use cancellation")
            .into()
    }
}

#[salsa::input]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Workspace {
    pub root: Text,
    pub files: OrdMap<Text, Text>,
}

impl Workspace {
    /// Update a file's contents.
    pub fn update(&self, db: &mut dyn Db, path: impl Into<Text>, text: impl Into<Text>) {
        let mut files = self.files(db);
        files.insert(path.into(), text.into());
        self.set_files(db).to(files);
    }

    /// Get the debug representation of this [`Workspace`].
    pub fn debug(&self, db: &dyn Db) -> impl std::fmt::Debug {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Workspace {
            root: Text,
            files: OrdMap<Text, Text>,
        }

        Workspace {
            root: self.root(db),
            files: self.files(db),
        }
    }
}

#[salsa::tracked]
#[derive(Debug, Clone)]
pub struct Ast {
    #[return_ref]
    pub tree: Tree,
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

/// A wrapper around [`tree_sitter::Tree`] that is comparable.
#[derive(Debug, Clone)]
pub struct Tree(pub tree_sitter::Tree);

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root_node())
    }
}

impl From<tree_sitter::Tree> for Tree {
    fn from(value: tree_sitter::Tree) -> Self {
        Tree(value)
    }
}

impl From<Tree> for tree_sitter::Tree {
    fn from(value: Tree) -> Self {
        value.0
    }
}

impl std::ops::Deref for Tree {
    type Target = tree_sitter::Tree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        if *self.language() != *other.language() {
            // They use different languages
            return false;
        }

        if self.included_ranges() != other.included_ranges() {
            // The trees spans different ranges
            return false;
        }

        // Looks like we need to walk the tree
        let mut left = crate::traverse::tree(&self.0, Order::Pre);
        let mut right = crate::traverse::tree(other, Order::Pre);

        loop {
            match (left.next(), right.next()) {
                (Some(lhs), Some(rhs)) => {
                    if lhs.range() != rhs.range() || lhs.kind_id() != rhs.kind_id() {
                        return false;
                    }
                }
                (None, Some(_)) | (Some(_), None) => return false,
                (None, None) => return true,
            }
        }
    }
}

impl Eq for Tree {}
