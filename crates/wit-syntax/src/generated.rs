//! Automatically generated code. DO NOT EDIT!
/// Keywords used by the WIT language.
pub const KEYWORDS: &[&str] = &[
    "as",
    "bool",
    "borrow",
    "char",
    "constructor",
    "enum",
    "export",
    "f32",
    "f64",
    "flags",
    "float32",
    "float64",
    "func",
    "import",
    "include",
    "interface",
    "list",
    "option",
    "own",
    "package",
    "record",
    "resource",
    "result",
    "s16",
    "s32",
    "s64",
    "s8",
    "static",
    "string",
    "tuple",
    "type",
    "u16",
    "u32",
    "u64",
    "u8",
    "use",
    "variant",
    "with",
    "world",
];
/// Symbols and punctuation used by the WIT language.
pub const PUNCTUATION: &[&str] = &[
    " ",
    "(",
    ")",
    ",",
    "->",
    ".",
    "/",
    "///",
    ":",
    ";",
    "<",
    "=",
    ">",
    "@",
    "_",
    "{",
    "}",
];
///The `attribute` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Attribute<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Attribute<'tree> {
    pub fn doc_comment(self) -> Option<DocComment<'tree>> {
        super::children(self.0)
            .filter_map(<DocComment as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for Attribute<'tree> {
    const NAME: &'static str = "attribute";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Attribute(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `borrowed_handle` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BorrowedHandle<'tree>(tree_sitter::Node<'tree>);
impl<'tree> BorrowedHandle<'tree> {
    pub fn id(self) -> Option<Id<'tree>> {
        super::children(self.0).filter_map(<Id as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for BorrowedHandle<'tree> {
    const NAME: &'static str = "borrowed_handle";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(BorrowedHandle(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `builtin` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Builtin<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Builtin<'tree> {}
impl<'tree> super::AstNode<'tree> for Builtin<'tree> {
    const NAME: &'static str = "builtin";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Builtin(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `doc_comment` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DocComment<'tree>(tree_sitter::Node<'tree>);
impl<'tree> DocComment<'tree> {
    pub fn docs(self) -> Option<Docs<'tree>> {
        super::children(self.0).filter_map(<Docs as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for DocComment<'tree> {
    const NAME: &'static str = "doc_comment";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(DocComment(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `enum_body` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EnumBody<'tree>(tree_sitter::Node<'tree>);
impl<'tree> EnumBody<'tree> {
    pub fn iter_enum_cases(&self) -> impl Iterator<Item = EnumCase<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("enum_cases", &mut cursor)
            .filter_map(<EnumCase as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
}
impl<'tree> super::AstNode<'tree> for EnumBody<'tree> {
    const NAME: &'static str = "enum_body";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(EnumBody(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `enum_case` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EnumCase<'tree>(tree_sitter::Node<'tree>);
impl<'tree> EnumCase<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
}
impl super::HasIdent for EnumCase<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for EnumCase<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for EnumCase<'tree> {
    const NAME: &'static str = "enum_case";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(EnumCase(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `enum_items` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EnumItems<'tree>(tree_sitter::Node<'tree>);
impl<'tree> EnumItems<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
    pub fn enum_body(self) -> Option<EnumBody<'tree>> {
        super::children(self.0).filter_map(<EnumBody as super::AstNode<'_>>::cast).next()
    }
}
impl super::HasIdent for EnumItems<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for EnumItems<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for EnumItems<'tree> {
    const NAME: &'static str = "enum_items";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(EnumItems(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `export_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExportItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExportItem<'tree> {
    pub fn name_opt(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
    pub fn extern_type(self) -> Option<ExternType<'tree>> {
        super::children(self.0)
            .filter_map(<ExternType as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn use_path(self) -> Option<UsePath<'tree>> {
        super::children(self.0).filter_map(<UsePath as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for ExportItem<'tree> {
    const NAME: &'static str = "export_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ExportItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `extern_type` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExternType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExternType<'tree> {
    pub fn func_type(self) -> Option<FuncType<'tree>> {
        super::children(self.0).filter_map(<FuncType as super::AstNode<'_>>::cast).next()
    }
    pub fn interface_body(self) -> Option<InterfaceBody<'tree>> {
        super::children(self.0)
            .filter_map(<InterfaceBody as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for ExternType<'tree> {
    const NAME: &'static str = "extern_type";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ExternType(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `flags_body` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlagsBody<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FlagsBody<'tree> {
    pub fn iter_flags_fields(&self) -> impl Iterator<Item = FlagsCase<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("flags_fields", &mut cursor)
            .filter_map(<FlagsCase as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
}
impl<'tree> super::AstNode<'tree> for FlagsBody<'tree> {
    const NAME: &'static str = "flags_body";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(FlagsBody(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `flags_case` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlagsCase<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FlagsCase<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
}
impl super::HasIdent for FlagsCase<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for FlagsCase<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for FlagsCase<'tree> {
    const NAME: &'static str = "flags_case";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(FlagsCase(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `flags_items` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlagsItems<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FlagsItems<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn body(&self) -> Option<FlagsBody<'tree>> {
        self.0.child_by_field_name("body").and_then(<FlagsBody as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
}
impl super::HasIdent for FlagsItems<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for FlagsItems<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for FlagsItems<'tree> {
    const NAME: &'static str = "flags_items";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(FlagsItems(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `fully_qualified_use_path` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FullyQualifiedUsePath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FullyQualifiedUsePath<'tree> {
    pub fn iter_ids(self) -> impl Iterator<Item = Id<'tree>> {
        super::children(self.0).filter_map(<Id as super::AstNode<'_>>::cast)
    }
    pub fn iter_valid_semvers(self) -> impl Iterator<Item = ValidSemver<'tree>> {
        super::children(self.0).filter_map(<ValidSemver as super::AstNode<'_>>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for FullyQualifiedUsePath<'tree> {
    const NAME: &'static str = "fully_qualified_use_path";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(FullyQualifiedUsePath(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `func_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FuncItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FuncItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
    pub fn func_type(self) -> Option<FuncType<'tree>> {
        super::children(self.0).filter_map(<FuncType as super::AstNode<'_>>::cast).next()
    }
}
impl super::HasIdent for FuncItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for FuncItem<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for FuncItem<'tree> {
    const NAME: &'static str = "func_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(FuncItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `func_type` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FuncType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FuncType<'tree> {
    pub fn param_list(&self) -> Option<ParamList<'tree>> {
        self.0
            .child_by_field_name("param_list")
            .and_then(<ParamList as super::AstNode>::cast)
    }
    pub fn result_list_opt(&self) -> Option<ResultList<'tree>> {
        self.0
            .child_by_field_name("result_list")
            .and_then(<ResultList as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for FuncType<'tree> {
    const NAME: &'static str = "func_type";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(FuncType(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `handle` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Handle<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Handle<'tree> {
    pub fn borrowed_handle(self) -> Option<BorrowedHandle<'tree>> {
        super::children(self.0)
            .filter_map(<BorrowedHandle as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn owned_handle(self) -> Option<OwnedHandle<'tree>> {
        super::children(self.0)
            .filter_map(<OwnedHandle as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for Handle<'tree> {
    const NAME: &'static str = "handle";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Handle(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `import_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ImportItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ImportItem<'tree> {
    pub fn name_opt(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
    pub fn extern_type(self) -> Option<ExternType<'tree>> {
        super::children(self.0)
            .filter_map(<ExternType as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn use_path(self) -> Option<UsePath<'tree>> {
        super::children(self.0).filter_map(<UsePath as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for ImportItem<'tree> {
    const NAME: &'static str = "import_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ImportItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `include_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IncludeItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> IncludeItem<'tree> {
    pub fn include_names_body_opt(&self) -> Option<IncludeNamesBody<'tree>> {
        self.0
            .child_by_field_name("include_names_body")
            .and_then(<IncludeNamesBody as super::AstNode>::cast)
    }
    pub fn use_path(&self) -> Option<UsePath<'tree>> {
        self.0
            .child_by_field_name("use_path")
            .and_then(<UsePath as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for IncludeItem<'tree> {
    const NAME: &'static str = "include_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(IncludeItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `include_names_body` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IncludeNamesBody<'tree>(tree_sitter::Node<'tree>);
impl<'tree> IncludeNamesBody<'tree> {
    pub fn include_names_list(&self) -> Option<IncludeNamesList<'tree>> {
        self.0
            .child_by_field_name("include_names_list")
            .and_then(<IncludeNamesList as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for IncludeNamesBody<'tree> {
    const NAME: &'static str = "include_names_body";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(IncludeNamesBody(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `include_names_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IncludeNamesItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> IncludeNamesItem<'tree> {
    pub fn alias(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("alias").and_then(<Id as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
}
impl super::HasIdent for IncludeNamesItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::AstNode<'tree> for IncludeNamesItem<'tree> {
    const NAME: &'static str = "include_names_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(IncludeNamesItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `include_names_list` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IncludeNamesList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> IncludeNamesList<'tree> {
    pub fn iter_include_names_items(
        &self,
    ) -> impl Iterator<Item = IncludeNamesItem<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("include_names_item", &mut cursor)
            .filter_map(<IncludeNamesItem as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
}
impl<'tree> super::AstNode<'tree> for IncludeNamesList<'tree> {
    const NAME: &'static str = "include_names_list";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(IncludeNamesList(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `interface_body` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InterfaceBody<'tree>(tree_sitter::Node<'tree>);
impl<'tree> InterfaceBody<'tree> {
    pub fn iter_interface_items(&self) -> impl Iterator<Item = InterfaceItems<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("interface_items", &mut cursor)
            .filter_map(<InterfaceItems as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
}
impl<'tree> super::AstNode<'tree> for InterfaceBody<'tree> {
    const NAME: &'static str = "interface_body";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(InterfaceBody(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `interface_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InterfaceItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> InterfaceItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn body(&self) -> Option<InterfaceBody<'tree>> {
        self.0
            .child_by_field_name("body")
            .and_then(<InterfaceBody as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
}
impl super::HasIdent for InterfaceItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for InterfaceItem<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for InterfaceItem<'tree> {
    const NAME: &'static str = "interface_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(InterfaceItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `interface_items` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InterfaceItems<'tree>(tree_sitter::Node<'tree>);
impl<'tree> InterfaceItems<'tree> {
    pub fn func_opt(&self) -> Option<FuncItem<'tree>> {
        self.0.child_by_field_name("func").and_then(<FuncItem as super::AstNode>::cast)
    }
    pub fn typedef_opt(&self) -> Option<TypedefItem<'tree>> {
        self.0
            .child_by_field_name("typedef")
            .and_then(<TypedefItem as super::AstNode>::cast)
    }
    pub fn use_opt(&self) -> Option<UseItem<'tree>> {
        self.0.child_by_field_name("use").and_then(<UseItem as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for InterfaceItems<'tree> {
    const NAME: &'static str = "interface_items";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(InterfaceItems(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `list` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct List<'tree>(tree_sitter::Node<'tree>);
impl<'tree> List<'tree> {
    pub fn ty(self) -> Option<Ty<'tree>> {
        super::children(self.0).filter_map(<Ty as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for List<'tree> {
    const NAME: &'static str = "list";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(List(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `named_type` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NamedType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> NamedType<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
    pub fn type_(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("type").and_then(<Ty as super::AstNode>::cast)
    }
}
impl super::HasIdent for NamedType<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for NamedType<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for NamedType<'tree> {
    const NAME: &'static str = "named_type";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(NamedType(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `named_type_list` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NamedTypeList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> NamedTypeList<'tree> {
    pub fn iter_named_types(self) -> impl Iterator<Item = NamedType<'tree>> {
        super::children(self.0).filter_map(<NamedType as super::AstNode<'_>>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for NamedTypeList<'tree> {
    const NAME: &'static str = "named_type_list";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(NamedTypeList(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `option` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Option_<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Option_<'tree> {
    pub fn ty(self) -> Option<Ty<'tree>> {
        super::children(self.0).filter_map(<Ty as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for Option_<'tree> {
    const NAME: &'static str = "option";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Option_(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `owned_handle` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OwnedHandle<'tree>(tree_sitter::Node<'tree>);
impl<'tree> OwnedHandle<'tree> {
    pub fn id(self) -> Option<Id<'tree>> {
        super::children(self.0).filter_map(<Id as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for OwnedHandle<'tree> {
    const NAME: &'static str = "owned_handle";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(OwnedHandle(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `package_decl` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PackageDecl<'tree>(tree_sitter::Node<'tree>);
impl<'tree> PackageDecl<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn package_name(self) -> Option<PackageName<'tree>> {
        super::children(self.0)
            .filter_map(<PackageName as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::HasAttr<'tree> for PackageDecl<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for PackageDecl<'tree> {
    const NAME: &'static str = "package_decl";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(PackageDecl(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `package_name` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PackageName<'tree>(tree_sitter::Node<'tree>);
impl<'tree> PackageName<'tree> {
    pub fn iter_ids(self) -> impl Iterator<Item = Id<'tree>> {
        super::children(self.0).filter_map(<Id as super::AstNode<'_>>::cast)
    }
    pub fn iter_valid_semvers(self) -> impl Iterator<Item = ValidSemver<'tree>> {
        super::children(self.0).filter_map(<ValidSemver as super::AstNode<'_>>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for PackageName<'tree> {
    const NAME: &'static str = "package_name";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(PackageName(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `param_list` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParamList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ParamList<'tree> {
    pub fn named_type_list_opt(self) -> Option<NamedTypeList<'tree>> {
        super::children(self.0)
            .filter_map(<NamedTypeList as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for ParamList<'tree> {
    const NAME: &'static str = "param_list";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ParamList(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `record_body` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RecordBody<'tree>(tree_sitter::Node<'tree>);
impl<'tree> RecordBody<'tree> {
    pub fn iter_record_fields(&self) -> impl Iterator<Item = RecordField<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("record_fields", &mut cursor)
            .filter_map(<RecordField as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
}
impl<'tree> super::AstNode<'tree> for RecordBody<'tree> {
    const NAME: &'static str = "record_body";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(RecordBody(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `record_field` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RecordField<'tree>(tree_sitter::Node<'tree>);
impl<'tree> RecordField<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
    pub fn type_(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("type").and_then(<Ty as super::AstNode>::cast)
    }
}
impl super::HasIdent for RecordField<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for RecordField<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for RecordField<'tree> {
    const NAME: &'static str = "record_field";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(RecordField(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `record_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RecordItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> RecordItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn body(&self) -> Option<RecordBody<'tree>> {
        self.0.child_by_field_name("body").and_then(<RecordBody as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
}
impl super::HasIdent for RecordItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for RecordItem<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for RecordItem<'tree> {
    const NAME: &'static str = "record_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(RecordItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `resource_body` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ResourceBody<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ResourceBody<'tree> {
    pub fn iter_resource_methods(self) -> impl Iterator<Item = ResourceMethod<'tree>> {
        super::children(self.0).filter_map(<ResourceMethod as super::AstNode<'_>>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for ResourceBody<'tree> {
    const NAME: &'static str = "resource_body";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ResourceBody(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `resource_constructor` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ResourceConstructor<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ResourceConstructor<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn param_list(self) -> Option<ParamList<'tree>> {
        super::children(self.0)
            .filter_map(<ParamList as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::HasAttr<'tree> for ResourceConstructor<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for ResourceConstructor<'tree> {
    const NAME: &'static str = "resource_constructor";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ResourceConstructor(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `resource_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ResourceItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ResourceItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
    pub fn resource_body_opt(&self) -> Option<ResourceBody<'tree>> {
        self.0
            .child_by_field_name("resource_body")
            .and_then(<ResourceBody as super::AstNode>::cast)
    }
}
impl super::HasIdent for ResourceItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for ResourceItem<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for ResourceItem<'tree> {
    const NAME: &'static str = "resource_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ResourceItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `resource_method` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ResourceMethod<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ResourceMethod<'tree> {
    pub fn func_item(self) -> Option<FuncItem<'tree>> {
        super::children(self.0).filter_map(<FuncItem as super::AstNode<'_>>::cast).next()
    }
    pub fn resource_constructor(self) -> Option<ResourceConstructor<'tree>> {
        super::children(self.0)
            .filter_map(<ResourceConstructor as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn static_resource_method(self) -> Option<StaticResourceMethod<'tree>> {
        super::children(self.0)
            .filter_map(<StaticResourceMethod as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for ResourceMethod<'tree> {
    const NAME: &'static str = "resource_method";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ResourceMethod(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `result` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Result_<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Result_<'tree> {
    pub fn err_opt(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("err").and_then(<Ty as super::AstNode>::cast)
    }
    pub fn ok_opt(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("ok").and_then(<Ty as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for Result_<'tree> {
    const NAME: &'static str = "result";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Result_(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `result_list` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ResultList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ResultList<'tree> {
    pub fn named_type_list_opt(self) -> Option<NamedTypeList<'tree>> {
        super::children(self.0)
            .filter_map(<NamedTypeList as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn ty_opt(self) -> Option<Ty<'tree>> {
        super::children(self.0).filter_map(<Ty as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for ResultList<'tree> {
    const NAME: &'static str = "result_list";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ResultList(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `source_file` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SourceFile<'tree>(tree_sitter::Node<'tree>);
impl<'tree> SourceFile<'tree> {
    pub fn decl_opt(&self) -> Option<PackageDecl<'tree>> {
        self.0
            .child_by_field_name("decl")
            .and_then(<PackageDecl as super::AstNode>::cast)
    }
    pub fn iter_top_level_items(self) -> impl Iterator<Item = TopLevelItem<'tree>> {
        super::children(self.0).filter_map(<TopLevelItem as super::AstNode<'_>>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for SourceFile<'tree> {
    const NAME: &'static str = "source_file";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(SourceFile(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `static_resource_method` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StaticResourceMethod<'tree>(tree_sitter::Node<'tree>);
impl<'tree> StaticResourceMethod<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
    pub fn func_type(self) -> Option<FuncType<'tree>> {
        super::children(self.0).filter_map(<FuncType as super::AstNode<'_>>::cast).next()
    }
}
impl super::HasIdent for StaticResourceMethod<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for StaticResourceMethod<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for StaticResourceMethod<'tree> {
    const NAME: &'static str = "static_resource_method";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(StaticResourceMethod(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `top_level_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TopLevelItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TopLevelItem<'tree> {
    pub fn interface_item(self) -> Option<InterfaceItem<'tree>> {
        super::children(self.0)
            .filter_map(<InterfaceItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn toplevel_use_item(self) -> Option<ToplevelUseItem<'tree>> {
        super::children(self.0)
            .filter_map(<ToplevelUseItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn world_item(self) -> Option<WorldItem<'tree>> {
        super::children(self.0)
            .filter_map(<WorldItem as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for TopLevelItem<'tree> {
    const NAME: &'static str = "top_level_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(TopLevelItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `toplevel_use_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ToplevelUseItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ToplevelUseItem<'tree> {
    pub fn iter_alias(&self) -> impl Iterator<Item = Id<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("alias", &mut cursor)
            .filter_map(<Id as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn path(&self) -> Option<UsePath<'tree>> {
        self.0.child_by_field_name("path").and_then(<UsePath as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for ToplevelUseItem<'tree> {
    const NAME: &'static str = "toplevel_use_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ToplevelUseItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `tuple` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Tuple<'tree> {
    pub fn tuple_list_opt(self) -> Option<TupleList<'tree>> {
        super::children(self.0)
            .filter_map(<TupleList as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for Tuple<'tree> {
    const NAME: &'static str = "tuple";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Tuple(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `tuple_list` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TupleList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TupleList<'tree> {
    pub fn iter_tys(self) -> impl Iterator<Item = Ty<'tree>> {
        super::children(self.0).filter_map(<Ty as super::AstNode<'_>>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for TupleList<'tree> {
    const NAME: &'static str = "tuple_list";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(TupleList(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `ty` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ty<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Ty<'tree> {
    pub fn builtin(self) -> Option<Builtin<'tree>> {
        super::children(self.0).filter_map(<Builtin as super::AstNode<'_>>::cast).next()
    }
    pub fn handle(self) -> Option<Handle<'tree>> {
        super::children(self.0).filter_map(<Handle as super::AstNode<'_>>::cast).next()
    }
    pub fn id(self) -> Option<Id<'tree>> {
        super::children(self.0).filter_map(<Id as super::AstNode<'_>>::cast).next()
    }
    pub fn list(self) -> Option<List<'tree>> {
        super::children(self.0).filter_map(<List as super::AstNode<'_>>::cast).next()
    }
    pub fn option(self) -> Option<Option_<'tree>> {
        super::children(self.0).filter_map(<Option_ as super::AstNode<'_>>::cast).next()
    }
    pub fn result(self) -> Option<Result_<'tree>> {
        super::children(self.0).filter_map(<Result_ as super::AstNode<'_>>::cast).next()
    }
    pub fn tuple(self) -> Option<Tuple<'tree>> {
        super::children(self.0).filter_map(<Tuple as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for Ty<'tree> {
    const NAME: &'static str = "ty";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Ty(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `type_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TypeItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TypeItem<'tree> {
    pub fn alias(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("alias").and_then(<Id as super::AstNode>::cast)
    }
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn type_(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("type").and_then(<Ty as super::AstNode>::cast)
    }
}
impl<'tree> super::HasAttr<'tree> for TypeItem<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for TypeItem<'tree> {
    const NAME: &'static str = "type_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(TypeItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `typedef_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TypedefItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TypedefItem<'tree> {
    pub fn enum_items(self) -> Option<EnumItems<'tree>> {
        super::children(self.0)
            .filter_map(<EnumItems as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn flags_items(self) -> Option<FlagsItems<'tree>> {
        super::children(self.0)
            .filter_map(<FlagsItems as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn record_item(self) -> Option<RecordItem<'tree>> {
        super::children(self.0)
            .filter_map(<RecordItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn resource_item(self) -> Option<ResourceItem<'tree>> {
        super::children(self.0)
            .filter_map(<ResourceItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn type_item(self) -> Option<TypeItem<'tree>> {
        super::children(self.0).filter_map(<TypeItem as super::AstNode<'_>>::cast).next()
    }
    pub fn variant_items(self) -> Option<VariantItems<'tree>> {
        super::children(self.0)
            .filter_map(<VariantItems as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for TypedefItem<'tree> {
    const NAME: &'static str = "typedef_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(TypedefItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `use_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UseItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UseItem<'tree> {
    pub fn names(&self) -> Option<UseNamesList<'tree>> {
        self.0
            .child_by_field_name("names")
            .and_then(<UseNamesList as super::AstNode>::cast)
    }
    pub fn path(&self) -> Option<UsePath<'tree>> {
        self.0.child_by_field_name("path").and_then(<UsePath as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for UseItem<'tree> {
    const NAME: &'static str = "use_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(UseItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `use_names_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UseNamesItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UseNamesItem<'tree> {
    pub fn alias_opt(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("alias").and_then(<Id as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
}
impl super::HasIdent for UseNamesItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::AstNode<'tree> for UseNamesItem<'tree> {
    const NAME: &'static str = "use_names_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(UseNamesItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `use_names_list` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UseNamesList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UseNamesList<'tree> {
    pub fn iter_use_names_items(&self) -> impl Iterator<Item = UseNamesItem<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("use_names_item", &mut cursor)
            .filter_map(<UseNamesItem as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
}
impl<'tree> super::AstNode<'tree> for UseNamesList<'tree> {
    const NAME: &'static str = "use_names_list";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(UseNamesList(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `use_path` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UsePath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UsePath<'tree> {
    pub fn fully_qualified_use_path(self) -> Option<FullyQualifiedUsePath<'tree>> {
        super::children(self.0)
            .filter_map(<FullyQualifiedUsePath as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn id(self) -> Option<Id<'tree>> {
        super::children(self.0).filter_map(<Id as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for UsePath<'tree> {
    const NAME: &'static str = "use_path";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(UsePath(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `variant_body` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VariantBody<'tree>(tree_sitter::Node<'tree>);
impl<'tree> VariantBody<'tree> {
    pub fn iter_variant_cases(&self) -> impl Iterator<Item = VariantCase<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("variant_cases", &mut cursor)
            .filter_map(<VariantCase as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
}
impl<'tree> super::AstNode<'tree> for VariantBody<'tree> {
    const NAME: &'static str = "variant_body";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(VariantBody(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `variant_case` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VariantCase<'tree>(tree_sitter::Node<'tree>);
impl<'tree> VariantCase<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
    pub fn type_opt(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("type").and_then(<Ty as super::AstNode>::cast)
    }
}
impl super::HasIdent for VariantCase<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for VariantCase<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for VariantCase<'tree> {
    const NAME: &'static str = "variant_case";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(VariantCase(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `variant_items` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VariantItems<'tree>(tree_sitter::Node<'tree>);
impl<'tree> VariantItems<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn body(&self) -> Option<VariantBody<'tree>> {
        self.0
            .child_by_field_name("body")
            .and_then(<VariantBody as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
}
impl super::HasIdent for VariantItems<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for VariantItems<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for VariantItems<'tree> {
    const NAME: &'static str = "variant_items";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(VariantItems(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `world_body` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WorldBody<'tree>(tree_sitter::Node<'tree>);
impl<'tree> WorldBody<'tree> {
    pub fn iter_world_items(&self) -> impl Iterator<Item = WorldItems<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("world_items", &mut cursor)
            .filter_map(<WorldItems as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
}
impl<'tree> super::AstNode<'tree> for WorldBody<'tree> {
    const NAME: &'static str = "world_body";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(WorldBody(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `world_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WorldItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> WorldItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn body(&self) -> Option<WorldBody<'tree>> {
        self.0.child_by_field_name("body").and_then(<WorldBody as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Id<'tree>> {
        self.0.child_by_field_name("name").and_then(<Id as super::AstNode>::cast)
    }
}
impl super::HasIdent for WorldItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?;
        let raw = node.0.utf8_text(src.as_bytes()).unwrap();
        Some(crate::ident(raw))
    }
}
impl<'tree> super::HasAttr<'tree> for WorldItem<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for WorldItem<'tree> {
    const NAME: &'static str = "world_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(WorldItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `world_items` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WorldItems<'tree>(tree_sitter::Node<'tree>);
impl<'tree> WorldItems<'tree> {
    pub fn export_item_opt(&self) -> Option<ExportItem<'tree>> {
        self.0
            .child_by_field_name("export_item")
            .and_then(<ExportItem as super::AstNode>::cast)
    }
    pub fn import_item_opt(&self) -> Option<ImportItem<'tree>> {
        self.0
            .child_by_field_name("import_item")
            .and_then(<ImportItem as super::AstNode>::cast)
    }
    pub fn include_item_opt(&self) -> Option<IncludeItem<'tree>> {
        self.0
            .child_by_field_name("include_item")
            .and_then(<IncludeItem as super::AstNode>::cast)
    }
    pub fn typedef_item_opt(&self) -> Option<TypedefItem<'tree>> {
        self.0
            .child_by_field_name("typedef_item")
            .and_then(<TypedefItem as super::AstNode>::cast)
    }
    pub fn use_item_opt(&self) -> Option<UseItem<'tree>> {
        self.0
            .child_by_field_name("use_item")
            .and_then(<UseItem as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for WorldItems<'tree> {
    const NAME: &'static str = "world_items";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(WorldItems(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `comment` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Comment<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Comment<'tree> {}
impl<'tree> super::AstNode<'tree> for Comment<'tree> {
    const NAME: &'static str = "comment";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Comment(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `docs` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Docs<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Docs<'tree> {}
impl<'tree> super::AstNode<'tree> for Docs<'tree> {
    const NAME: &'static str = "docs";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Docs(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `id` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Id<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Id<'tree> {}
impl<'tree> super::AstNode<'tree> for Id<'tree> {
    const NAME: &'static str = "id";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Id(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `valid_semver` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ValidSemver<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ValidSemver<'tree> {}
impl<'tree> super::AstNode<'tree> for ValidSemver<'tree> {
    const NAME: &'static str = "valid_semver";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ValidSemver(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
