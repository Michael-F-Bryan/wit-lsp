//! Automatically generated code. DO NOT EDIT!
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
///The `block_comment` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BlockComment<'tree>(tree_sitter::Node<'tree>);
impl<'tree> BlockComment<'tree> {}
impl<'tree> super::AstNode<'tree> for BlockComment<'tree> {
    const NAME: &'static str = "block_comment";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(BlockComment(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `borrowed_handle` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BorrowedHandle<'tree>(tree_sitter::Node<'tree>);
impl<'tree> BorrowedHandle<'tree> {
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for BorrowedHandle<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
///The `builtins` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Builtins<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Builtins<'tree> {}
impl<'tree> super::AstNode<'tree> for Builtins<'tree> {
    const NAME: &'static str = "builtins";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Builtins(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `doc_comment` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DocComment<'tree>(tree_sitter::Node<'tree>);
impl<'tree> DocComment<'tree> {
    pub fn docs(&self) -> Option<Docs<'tree>> {
        self.0.child_by_field_name("docs").and_then(<Docs as super::AstNode>::cast)
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
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for EnumCase<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
///The `enum_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EnumItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> EnumItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn iter_cases(&self) -> impl Iterator<Item = EnumCase<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("cases", &mut cursor)
            .filter_map(<EnumCase as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for EnumItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
    }
}
impl<'tree> super::HasAttr<'tree> for EnumItem<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for EnumItem<'tree> {
    const NAME: &'static str = "enum_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(EnumItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `export_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExportItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExportItem<'tree> {
    pub fn exported_item(self) -> Option<ExportedItem<'tree>> {
        super::children(self.0)
            .filter_map(<ExportedItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn exported_path(self) -> Option<ExportedPath<'tree>> {
        super::children(self.0)
            .filter_map(<ExportedPath as super::AstNode<'_>>::cast)
            .next()
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
///The `exported_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExportedItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExportedItem<'tree> {
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn extern_type(self) -> Option<ExternType<'tree>> {
        super::children(self.0)
            .filter_map(<ExternType as super::AstNode<'_>>::cast)
            .next()
    }
}
impl super::HasIdent for ExportedItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
    }
}
impl<'tree> super::AstNode<'tree> for ExportedItem<'tree> {
    const NAME: &'static str = "exported_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ExportedItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `exported_path` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExportedPath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExportedPath<'tree> {
    pub fn fully_qualified_use_path(self) -> Option<FullyQualifiedUsePath<'tree>> {
        super::children(self.0)
            .filter_map(<FullyQualifiedUsePath as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for ExportedPath<'tree> {
    const NAME: &'static str = "exported_path";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ExportedPath(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `extern_type` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExternType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExternType<'tree> {
    pub fn iter_func_types(self) -> impl Iterator<Item = FuncType<'tree>> {
        super::children(self.0).filter_map(<FuncType as super::AstNode<'_>>::cast)
    }
    pub fn iter_interface_items(self) -> impl Iterator<Item = InterfaceItems<'tree>> {
        super::children(self.0).filter_map(<InterfaceItems as super::AstNode<'_>>::cast)
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
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for FlagsCase<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
///The `flags_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlagsItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FlagsItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn iter_cases(&self) -> impl Iterator<Item = FlagsCase<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("cases", &mut cursor)
            .filter_map(<FlagsCase as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for FlagsItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
    }
}
impl<'tree> super::HasAttr<'tree> for FlagsItem<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for FlagsItem<'tree> {
    const NAME: &'static str = "flags_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(FlagsItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `fully_qualified_package_name` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FullyQualifiedPackageName<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FullyQualifiedPackageName<'tree> {
    pub fn package(&self) -> Option<PackageName<'tree>> {
        self.0
            .child_by_field_name("package")
            .and_then(<PackageName as super::AstNode>::cast)
    }
    pub fn path(&self) -> Option<PackagePath<'tree>> {
        self.0
            .child_by_field_name("path")
            .and_then(<PackagePath as super::AstNode>::cast)
    }
    pub fn version_opt(&self) -> Option<Semver<'tree>> {
        self.0.child_by_field_name("version").and_then(<Semver as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for FullyQualifiedPackageName<'tree> {
    const NAME: &'static str = "fully_qualified_package_name";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME {
            Some(FullyQualifiedPackageName(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `fully_qualified_use_path` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FullyQualifiedUsePath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FullyQualifiedUsePath<'tree> {
    pub fn package(&self) -> Option<PackageName<'tree>> {
        self.0
            .child_by_field_name("package")
            .and_then(<PackageName as super::AstNode>::cast)
    }
    pub fn path(&self) -> Option<PackagePath<'tree>> {
        self.0
            .child_by_field_name("path")
            .and_then(<PackagePath as super::AstNode>::cast)
    }
    pub fn version_opt(&self) -> Option<Semver<'tree>> {
        self.0.child_by_field_name("version").and_then(<Semver as super::AstNode>::cast)
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
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn ty(&self) -> Option<FuncType<'tree>> {
        self.0.child_by_field_name("ty").and_then(<FuncType as super::AstNode>::cast)
    }
}
impl super::HasIdent for FuncItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
    pub fn params(&self) -> Option<ParamList<'tree>> {
        self.0
            .child_by_field_name("params")
            .and_then(<ParamList as super::AstNode>::cast)
    }
    pub fn result_opt(&self) -> Option<ResultList<'tree>> {
        self.0
            .child_by_field_name("result")
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
    pub fn imported_item(self) -> Option<ImportedItem<'tree>> {
        super::children(self.0)
            .filter_map(<ImportedItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn imported_path(self) -> Option<ImportedPath<'tree>> {
        super::children(self.0)
            .filter_map(<ImportedPath as super::AstNode<'_>>::cast)
            .next()
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
///The `imported_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ImportedItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ImportedItem<'tree> {
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn extern_type(self) -> Option<ExternType<'tree>> {
        super::children(self.0)
            .filter_map(<ExternType as super::AstNode<'_>>::cast)
            .next()
    }
}
impl super::HasIdent for ImportedItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
    }
}
impl<'tree> super::AstNode<'tree> for ImportedItem<'tree> {
    const NAME: &'static str = "imported_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ImportedItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `imported_path` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ImportedPath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ImportedPath<'tree> {
    pub fn use_path(self) -> Option<UsePath<'tree>> {
        super::children(self.0).filter_map(<UsePath as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for ImportedPath<'tree> {
    const NAME: &'static str = "imported_path";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(ImportedPath(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `include_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IncludeItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> IncludeItem<'tree> {
    pub fn names_opt(&self) -> Option<IncludeNamesList<'tree>> {
        self.0
            .child_by_field_name("names")
            .and_then(<IncludeNamesList as super::AstNode>::cast)
    }
    pub fn path(&self) -> Option<UsePath<'tree>> {
        self.0.child_by_field_name("path").and_then(<UsePath as super::AstNode>::cast)
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
///The `include_names_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IncludeNamesItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> IncludeNamesItem<'tree> {
    pub fn alias(&self) -> Option<Identifier<'tree>> {
        self.0
            .child_by_field_name("alias")
            .and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for IncludeNamesItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
        self,
    ) -> impl Iterator<Item = IncludeNamesItem<'tree>> {
        super::children(self.0)
            .filter_map(<IncludeNamesItem as super::AstNode<'_>>::cast)
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
    pub fn iter_items(&self) -> impl Iterator<Item = InterfaceItems<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("items", &mut cursor)
            .filter_map(<InterfaceItems as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for InterfaceItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
    pub fn func_item(self) -> Option<FuncItem<'tree>> {
        super::children(self.0).filter_map(<FuncItem as super::AstNode<'_>>::cast).next()
    }
    pub fn typedef_item(self) -> Option<TypedefItem<'tree>> {
        super::children(self.0)
            .filter_map(<TypedefItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn use_item(self) -> Option<UseItem<'tree>> {
        super::children(self.0).filter_map(<UseItem as super::AstNode<'_>>::cast).next()
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
///The `local_use_path` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LocalUsePath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> LocalUsePath<'tree> {
    pub fn identifier(self) -> Option<Identifier<'tree>> {
        super::children(self.0)
            .filter_map(<Identifier as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for LocalUsePath<'tree> {
    const NAME: &'static str = "local_use_path";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(LocalUsePath(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `named_type` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NamedType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> NamedType<'tree> {
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn ty(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("ty").and_then(<Ty as super::AstNode>::cast)
    }
}
impl super::HasIdent for NamedType<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for OwnedHandle<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
    pub fn fully_qualified_package_name(
        self,
    ) -> Option<FullyQualifiedPackageName<'tree>> {
        super::children(self.0)
            .filter_map(<FullyQualifiedPackageName as super::AstNode<'_>>::cast)
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
    pub fn identifier(self) -> Option<Identifier<'tree>> {
        super::children(self.0)
            .filter_map(<Identifier as super::AstNode<'_>>::cast)
            .next()
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
///The `package_path` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PackagePath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> PackagePath<'tree> {
    pub fn iter_identifiers(self) -> impl Iterator<Item = Identifier<'tree>> {
        super::children(self.0).filter_map(<Identifier as super::AstNode<'_>>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for PackagePath<'tree> {
    const NAME: &'static str = "package_path";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(PackagePath(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `param_list` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParamList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ParamList<'tree> {
    pub fn iter_params(&self) -> impl Iterator<Item = NamedType<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("params", &mut cursor)
            .filter_map(<NamedType as super::AstNode>::cast)
            .collect();
        children.into_iter()
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
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn ty(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("ty").and_then(<Ty as super::AstNode>::cast)
    }
}
impl super::HasIdent for RecordField<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
    pub fn iter_fields(&self) -> impl Iterator<Item = RecordField<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("fields", &mut cursor)
            .filter_map(<RecordField as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for RecordItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
    pub fn params(&self) -> Option<ParamList<'tree>> {
        self.0
            .child_by_field_name("params")
            .and_then(<ParamList as super::AstNode>::cast)
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
    pub fn iter_methods(&self) -> impl Iterator<Item = ResourceMethod<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("methods", &mut cursor)
            .filter_map(<ResourceMethod as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for ResourceItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
    pub fn static_method(self) -> Option<StaticMethod<'tree>> {
        super::children(self.0)
            .filter_map(<StaticMethod as super::AstNode<'_>>::cast)
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
    pub fn iter_named_types(self) -> impl Iterator<Item = NamedType<'tree>> {
        super::children(self.0).filter_map(<NamedType as super::AstNode<'_>>::cast)
    }
    pub fn iter_tys(self) -> impl Iterator<Item = Ty<'tree>> {
        super::children(self.0).filter_map(<Ty as super::AstNode<'_>>::cast)
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
    pub fn package_opt(&self) -> Option<PackageDecl<'tree>> {
        self.0
            .child_by_field_name("package")
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
///The `static_method` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StaticMethod<'tree>(tree_sitter::Node<'tree>);
impl<'tree> StaticMethod<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn func_type(self) -> Option<FuncType<'tree>> {
        super::children(self.0).filter_map(<FuncType as super::AstNode<'_>>::cast).next()
    }
}
impl super::HasIdent for StaticMethod<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
    }
}
impl<'tree> super::HasAttr<'tree> for StaticMethod<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for StaticMethod<'tree> {
    const NAME: &'static str = "static_method";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(StaticMethod(node)) } else { None }
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
    pub fn top_level_use_item(self) -> Option<TopLevelUseItem<'tree>> {
        super::children(self.0)
            .filter_map(<TopLevelUseItem as super::AstNode<'_>>::cast)
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
///The `top_level_use_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TopLevelUseItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TopLevelUseItem<'tree> {
    pub fn alias_opt(&self) -> Option<Identifier<'tree>> {
        self.0
            .child_by_field_name("alias")
            .and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn use_path(self) -> Option<UsePath<'tree>> {
        super::children(self.0).filter_map(<UsePath as super::AstNode<'_>>::cast).next()
    }
}
impl<'tree> super::AstNode<'tree> for TopLevelUseItem<'tree> {
    const NAME: &'static str = "top_level_use_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(TopLevelUseItem(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `tuple` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Tuple<'tree> {
    pub fn iter_tys(self) -> impl Iterator<Item = Ty<'tree>> {
        super::children(self.0).filter_map(<Ty as super::AstNode<'_>>::cast)
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
///The `ty` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ty<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Ty<'tree> {
    pub fn builtins(self) -> Option<Builtins<'tree>> {
        super::children(self.0).filter_map(<Builtins as super::AstNode<'_>>::cast).next()
    }
    pub fn handle(self) -> Option<Handle<'tree>> {
        super::children(self.0).filter_map(<Handle as super::AstNode<'_>>::cast).next()
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
    pub fn user_defined_type(self) -> Option<UserDefinedType<'tree>> {
        super::children(self.0)
            .filter_map(<UserDefinedType as super::AstNode<'_>>::cast)
            .next()
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
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn ty(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("ty").and_then(<Ty as super::AstNode>::cast)
    }
}
impl super::HasIdent for TypeItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
    pub fn enum_item(self) -> Option<EnumItem<'tree>> {
        super::children(self.0).filter_map(<EnumItem as super::AstNode<'_>>::cast).next()
    }
    pub fn flags_item(self) -> Option<FlagsItem<'tree>> {
        super::children(self.0)
            .filter_map(<FlagsItem as super::AstNode<'_>>::cast)
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
    pub fn variant_item(self) -> Option<VariantItem<'tree>> {
        super::children(self.0)
            .filter_map(<VariantItem as super::AstNode<'_>>::cast)
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
    pub fn path(&self) -> Option<UsePath<'tree>> {
        self.0.child_by_field_name("path").and_then(<UsePath as super::AstNode>::cast)
    }
    pub fn iter_use_names_items(self) -> impl Iterator<Item = UseNamesItem<'tree>> {
        super::children(self.0).filter_map(<UseNamesItem as super::AstNode<'_>>::cast)
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
    pub fn alias_opt(&self) -> Option<Identifier<'tree>> {
        self.0
            .child_by_field_name("alias")
            .and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for UseNamesItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
///The `use_path` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UsePath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UsePath<'tree> {
    pub fn fully_qualified_use_path(self) -> Option<FullyQualifiedUsePath<'tree>> {
        super::children(self.0)
            .filter_map(<FullyQualifiedUsePath as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn local_use_path(self) -> Option<LocalUsePath<'tree>> {
        super::children(self.0)
            .filter_map(<LocalUsePath as super::AstNode<'_>>::cast)
            .next()
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
///The `user_defined_type` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UserDefinedType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UserDefinedType<'tree> {
    pub fn identifier(self) -> Option<Identifier<'tree>> {
        super::children(self.0)
            .filter_map(<Identifier as super::AstNode<'_>>::cast)
            .next()
    }
}
impl<'tree> super::AstNode<'tree> for UserDefinedType<'tree> {
    const NAME: &'static str = "user_defined_type";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(UserDefinedType(node)) } else { None }
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
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn ty_opt(&self) -> Option<Ty<'tree>> {
        self.0.child_by_field_name("ty").and_then(<Ty as super::AstNode>::cast)
    }
}
impl super::HasIdent for VariantCase<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
///The `variant_item` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VariantItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> VariantItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("attributes", &mut cursor)
            .filter_map(<Attribute as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn iter_cases(&self) -> impl Iterator<Item = VariantCase<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("cases", &mut cursor)
            .filter_map(<VariantCase as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for VariantItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
    }
}
impl<'tree> super::HasAttr<'tree> for VariantItem<'tree> {
    fn attributes(self) -> impl Iterator<Item = Attribute<'tree>> + 'tree {
        self.iter_attributes()
    }
}
impl<'tree> super::AstNode<'tree> for VariantItem<'tree> {
    const NAME: &'static str = "variant_item";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(VariantItem(node)) } else { None }
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
    pub fn iter_items(&self) -> impl Iterator<Item = WorldItems<'tree>> {
        let mut cursor = self.0.walk();
        let children: Vec<_> = self
            .0
            .children_by_field_name("items", &mut cursor)
            .filter_map(<WorldItems as super::AstNode>::cast)
            .collect();
        children.into_iter()
    }
    pub fn name(&self) -> Option<Identifier<'tree>> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
}
impl super::HasIdent for WorldItem<'_> {
    fn identifier(self, src: &str) -> Option<&str> {
        let node = self.name()?.0;
        let ident = node.utf8_text(src.as_bytes()).unwrap();
        Some(ident)
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
    pub fn export_item(self) -> Option<ExportItem<'tree>> {
        super::children(self.0)
            .filter_map(<ExportItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn import_item(self) -> Option<ImportItem<'tree>> {
        super::children(self.0)
            .filter_map(<ImportItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn include_item(self) -> Option<IncludeItem<'tree>> {
        super::children(self.0)
            .filter_map(<IncludeItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn typedef_item(self) -> Option<TypedefItem<'tree>> {
        super::children(self.0)
            .filter_map(<TypedefItem as super::AstNode<'_>>::cast)
            .next()
    }
    pub fn use_item(self) -> Option<UseItem<'tree>> {
        super::children(self.0).filter_map(<UseItem as super::AstNode<'_>>::cast).next()
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
///The `identifier` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Identifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Identifier<'tree> {}
impl<'tree> super::AstNode<'tree> for Identifier<'tree> {
    const NAME: &'static str = "identifier";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Identifier(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `semver` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Semver<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Semver<'tree> {}
impl<'tree> super::AstNode<'tree> for Semver<'tree> {
    const NAME: &'static str = "semver";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(Semver(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `slash_comment` node.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SlashComment<'tree>(tree_sitter::Node<'tree>);
impl<'tree> SlashComment<'tree> {}
impl<'tree> super::AstNode<'tree> for SlashComment<'tree> {
    const NAME: &'static str = "slash_comment";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(SlashComment(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
