use super::Attribute;
use tree_sitter::Node;

pub trait AstNode<'tree>: 'tree {
    const NAME: &'static str;

    fn cast(node: Node<'tree>) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> Node<'tree>;
}

pub trait HasIdent {
    fn identifier(&self) -> Option<Node<'_>>;
}

pub trait HasAttr {
    fn attributes(&self) -> impl Iterator<Item = Attribute<'_>> + '_;
}

pub trait NodeExt<'tree> {}
