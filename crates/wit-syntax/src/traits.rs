use crate::Text;

use super::Attribute;
use tree_sitter::{Node, Range};

pub trait AstNode<'tree>: 'tree {
    const NAME: &'static str;

    fn cast(node: Node<'tree>) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> Node<'tree>;

    fn range(&self) -> Range {
        self.syntax().range()
    }
}

pub trait HasSource {
    /// Given the contents of a file, extract the text that corresponds to this
    /// [`AstNode`].
    fn utf8_text(self, file_contents: &str) -> &str;
}

impl<'tree, A> HasSource for A
where
    A: AstNode<'tree>,
{
    fn utf8_text(self, file_contents: &str) -> &str {
        self.syntax()
            .utf8_text(file_contents.as_bytes())
            .expect("unreachable")
    }
}

pub trait HasIdent {
    fn identifier(self, src: &str) -> Option<&str>;
}

impl<A> HasIdent for &A
where
    A: HasIdent + Clone,
{
    fn identifier(self, src: &str) -> Option<&str> {
        self.clone().identifier(src)
    }
}

pub trait HasAttr<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree;

    fn docs(self, src: &str) -> Option<Text>
    where
        Self: Sized,
    {
        let mut docs = String::new();

        for attr in self.attributes() {
            if let Some(doc_comment) = attr.doc_comment().and_then(|d| d.docs()) {
                if !docs.is_empty() {
                    docs.push('\n');
                }
                let doc_comment = doc_comment.syntax().utf8_text(src.as_bytes()).unwrap();
                docs.push_str(doc_comment);
            }
        }

        if docs.is_empty() {
            None
        } else {
            Some(docs.into())
        }
    }
}

impl<'tree, A> HasAttr<'tree> for &A
where
    A: HasAttr<'tree> + Clone,
{
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        A::clone(self).attributes()
    }
}

pub trait NodeExt<'tree> {}
