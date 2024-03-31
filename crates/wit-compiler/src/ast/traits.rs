use super::Attribute;
use tree_sitter::Node;

pub trait AstNode<'tree>: 'tree {
    const NAME: &'static str;

    fn cast(node: Node<'tree>) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> Node<'tree>;
}

pub trait HasIdent<'tree> {
    fn identifier(self) -> Option<super::Identifier<'tree>>;
}

impl<'tree, A> HasIdent<'tree> for &A
where
    A: HasIdent<'tree> + Clone,
{
    fn identifier(self) -> Option<super::Identifier<'tree>> {
        self.clone().identifier()
    }
}

pub trait HasAttr<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree;
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
