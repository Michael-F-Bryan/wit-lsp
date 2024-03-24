//! Automatically generated code. DO NOT EDIT!
///The `attribute` node.
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Attribute<'tree> {
    pub fn doc_comment(&self) -> Option<DocComment> {
        self.0
            .child_by_field_name("doc_comment")
            .and_then(<DocComment as super::AstNode>::cast)
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
///The `doc_comment` node.
#[derive(Debug, Clone, PartialEq)]
pub struct DocComment<'tree>(tree_sitter::Node<'tree>);
impl<'tree> DocComment<'tree> {}
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
///The `enum_field` node.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumField<'tree>(tree_sitter::Node<'tree>);
impl<'tree> EnumField<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn identifier(&self) -> Option<Identifier> {
        self.0
            .child_by_field_name("identifier")
            .and_then(<Identifier as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for EnumField<'tree> {
    const NAME: &'static str = "enum_field";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(EnumField(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `enum_item` node.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> EnumItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn iter_enum_fields(&self) -> impl Iterator<Item = EnumField> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct ExportItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExportItem<'tree> {
    pub fn exported_item(&self) -> Option<ExportedItem> {
        self.0
            .child_by_field_name("exported_item")
            .and_then(<ExportedItem as super::AstNode>::cast)
    }
    pub fn exported_path(&self) -> Option<ExportedPath> {
        self.0
            .child_by_field_name("exported_path")
            .and_then(<ExportedPath as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct ExportedItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExportedItem<'tree> {
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn extern_type(&self) -> Option<ExternType> {
        self.0
            .child_by_field_name("extern_type")
            .and_then(<ExternType as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct ExportedPath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExportedPath<'tree> {
    pub fn package(&self) -> Option<PackageName> {
        self.0
            .child_by_field_name("package")
            .and_then(<PackageName as super::AstNode>::cast)
    }
    pub fn path(&self) -> Option<PackagePath> {
        self.0
            .child_by_field_name("path")
            .and_then(<PackagePath as super::AstNode>::cast)
    }
    pub fn version_opt(&self) -> Option<Semver> {
        todo!()
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
#[derive(Debug, Clone, PartialEq)]
pub struct ExternType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ExternType<'tree> {
    pub fn iter_func_types(&self) -> impl Iterator<Item = FuncType> {
        Vec::new().into_iter()
    }
    pub fn iter_interface_items(&self) -> impl Iterator<Item = InterfaceItems> {
        Vec::new().into_iter()
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
///The `flags_field` node.
#[derive(Debug, Clone, PartialEq)]
pub struct FlagsField<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FlagsField<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn identifier(&self) -> Option<Identifier> {
        self.0
            .child_by_field_name("identifier")
            .and_then(<Identifier as super::AstNode>::cast)
    }
}
impl<'tree> super::AstNode<'tree> for FlagsField<'tree> {
    const NAME: &'static str = "flags_field";
    fn cast(node: tree_sitter::Node<'tree>) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == Self::NAME { Some(FlagsField(node)) } else { None }
    }
    fn syntax(&self) -> tree_sitter::Node<'tree> {
        self.0
    }
}
///The `flags_item` node.
#[derive(Debug, Clone, PartialEq)]
pub struct FlagsItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FlagsItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn iter_flags_fields(&self) -> impl Iterator<Item = FlagsField> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct FullyQualifiedPackageName<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FullyQualifiedPackageName<'tree> {
    pub fn package(&self) -> Option<PackageName> {
        self.0
            .child_by_field_name("package")
            .and_then(<PackageName as super::AstNode>::cast)
    }
    pub fn path(&self) -> Option<PackagePath> {
        self.0
            .child_by_field_name("path")
            .and_then(<PackagePath as super::AstNode>::cast)
    }
    pub fn version_opt(&self) -> Option<Semver> {
        todo!()
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
///The `func_item` node.
#[derive(Debug, Clone, PartialEq)]
pub struct FuncItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FuncItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn func_type(&self) -> Option<FuncType> {
        self.0
            .child_by_field_name("func_type")
            .and_then(<FuncType as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct FuncType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> FuncType<'tree> {
    pub fn result_opt(&self) -> Option<ResultList> {
        todo!()
    }
    pub fn param_list(&self) -> Option<ParamList> {
        self.0
            .child_by_field_name("param_list")
            .and_then(<ParamList as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct Handle<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Handle<'tree> {
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct ImportItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ImportItem<'tree> {
    pub fn imported_item(&self) -> Option<ImportedItem> {
        self.0
            .child_by_field_name("imported_item")
            .and_then(<ImportedItem as super::AstNode>::cast)
    }
    pub fn imported_path(&self) -> Option<ImportedPath> {
        self.0
            .child_by_field_name("imported_path")
            .and_then(<ImportedPath as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct ImportedItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ImportedItem<'tree> {
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn extern_type(&self) -> Option<ExternType> {
        self.0
            .child_by_field_name("extern_type")
            .and_then(<ExternType as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct ImportedPath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ImportedPath<'tree> {
    pub fn use_path(&self) -> Option<UsePath> {
        self.0
            .child_by_field_name("use_path")
            .and_then(<UsePath as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct IncludeItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> IncludeItem<'tree> {
    pub fn names_opt(&self) -> Option<IncludeNamesList> {
        todo!()
    }
    pub fn path(&self) -> Option<UsePath> {
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
#[derive(Debug, Clone, PartialEq)]
pub struct IncludeNamesItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> IncludeNamesItem<'tree> {
    pub fn alias(&self) -> Option<Identifier> {
        self.0
            .child_by_field_name("alias")
            .and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct IncludeNamesList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> IncludeNamesList<'tree> {
    pub fn iter_include_names_items(&self) -> impl Iterator<Item = IncludeNamesItem> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> InterfaceItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn iter_interface_items(&self) -> impl Iterator<Item = InterfaceItems> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceItems<'tree>(tree_sitter::Node<'tree>);
impl<'tree> InterfaceItems<'tree> {
    pub fn func_item(&self) -> Option<FuncItem> {
        self.0
            .child_by_field_name("func_item")
            .and_then(<FuncItem as super::AstNode>::cast)
    }
    pub fn typedef_item(&self) -> Option<TypedefItem> {
        self.0
            .child_by_field_name("typedef_item")
            .and_then(<TypedefItem as super::AstNode>::cast)
    }
    pub fn use_item(&self) -> Option<UseItem> {
        self.0
            .child_by_field_name("use_item")
            .and_then(<UseItem as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct List<'tree>(tree_sitter::Node<'tree>);
impl<'tree> List<'tree> {
    pub fn ty(&self) -> Option<Ty> {
        self.0.child_by_field_name("ty").and_then(<Ty as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct NamedType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> NamedType<'tree> {
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn ty(&self) -> Option<Ty> {
        self.0.child_by_field_name("ty").and_then(<Ty as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct NamedTypeList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> NamedTypeList<'tree> {
    pub fn iter_named_types(&self) -> impl Iterator<Item = NamedType> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct Option_<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Option_<'tree> {
    pub fn ty(&self) -> Option<Ty> {
        self.0.child_by_field_name("ty").and_then(<Ty as super::AstNode>::cast)
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
///The `package_decl` node.
#[derive(Debug, Clone, PartialEq)]
pub struct PackageDecl<'tree>(tree_sitter::Node<'tree>);
impl<'tree> PackageDecl<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn fully_qualified_package_name(&self) -> Option<FullyQualifiedPackageName> {
        self.0
            .child_by_field_name("fully_qualified_package_name")
            .and_then(<FullyQualifiedPackageName as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct PackageName<'tree>(tree_sitter::Node<'tree>);
impl<'tree> PackageName<'tree> {
    pub fn identifier(&self) -> Option<Identifier> {
        self.0
            .child_by_field_name("identifier")
            .and_then(<Identifier as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct PackagePath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> PackagePath<'tree> {
    pub fn iter_identifiers(&self) -> impl Iterator<Item = Identifier> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct ParamList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ParamList<'tree> {
    pub fn iter_named_type_lists(&self) -> impl Iterator<Item = NamedTypeList> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct RecordField<'tree>(tree_sitter::Node<'tree>);
impl<'tree> RecordField<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn ty(&self) -> Option<Ty> {
        self.0.child_by_field_name("ty").and_then(<Ty as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct RecordItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> RecordItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn iter_record_fields(&self) -> impl Iterator<Item = RecordField> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceConstructor<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ResourceConstructor<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn param_list(&self) -> Option<ParamList> {
        self.0
            .child_by_field_name("param_list")
            .and_then(<ParamList as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ResourceItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn iter_resource_methods(&self) -> impl Iterator<Item = ResourceMethod> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceMethod<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ResourceMethod<'tree> {
    pub fn func_item(&self) -> Option<FuncItem> {
        self.0
            .child_by_field_name("func_item")
            .and_then(<FuncItem as super::AstNode>::cast)
    }
    pub fn resource_constructor(&self) -> Option<ResourceConstructor> {
        self.0
            .child_by_field_name("resource_constructor")
            .and_then(<ResourceConstructor as super::AstNode>::cast)
    }
    pub fn static_method(&self) -> Option<StaticMethod> {
        self.0
            .child_by_field_name("static_method")
            .and_then(<StaticMethod as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct Result_<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Result_<'tree> {
    pub fn err_opt(&self) -> Option<Ty> {
        todo!()
    }
    pub fn ok_opt(&self) -> Option<Ty> {
        todo!()
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
#[derive(Debug, Clone, PartialEq)]
pub struct ResultList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> ResultList<'tree> {
    pub fn named_type_list_opt(&self) -> Option<NamedTypeList> {
        todo!()
    }
    pub fn ty_opt(&self) -> Option<Ty> {
        todo!()
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
#[derive(Debug, Clone, PartialEq)]
pub struct SourceFile<'tree>(tree_sitter::Node<'tree>);
impl<'tree> SourceFile<'tree> {
    pub fn package_opt(&self) -> Option<PackageDecl> {
        todo!()
    }
    pub fn iter_top_level_items(&self) -> impl Iterator<Item = TopLevelItem> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct StaticMethod<'tree>(tree_sitter::Node<'tree>);
impl<'tree> StaticMethod<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn func_type(&self) -> Option<FuncType> {
        self.0
            .child_by_field_name("func_type")
            .and_then(<FuncType as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TopLevelItem<'tree> {
    pub fn interface_item(&self) -> Option<InterfaceItem> {
        self.0
            .child_by_field_name("interface_item")
            .and_then(<InterfaceItem as super::AstNode>::cast)
    }
    pub fn top_level_use_item(&self) -> Option<TopLevelUseItem> {
        self.0
            .child_by_field_name("top_level_use_item")
            .and_then(<TopLevelUseItem as super::AstNode>::cast)
    }
    pub fn world_item(&self) -> Option<WorldItem> {
        self.0
            .child_by_field_name("world_item")
            .and_then(<WorldItem as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelUseItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TopLevelUseItem<'tree> {
    pub fn alias_opt(&self) -> Option<Identifier> {
        todo!()
    }
    pub fn use_path(&self) -> Option<UsePath> {
        self.0
            .child_by_field_name("use_path")
            .and_then(<UsePath as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct Tuple<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Tuple<'tree> {
    pub fn iter_tys(&self) -> impl Iterator<Item = Ty> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct Ty<'tree>(tree_sitter::Node<'tree>);
impl<'tree> Ty<'tree> {
    pub fn handle_opt(&self) -> Option<Handle> {
        todo!()
    }
    pub fn identifier_opt(&self) -> Option<Identifier> {
        todo!()
    }
    pub fn list_opt(&self) -> Option<List> {
        todo!()
    }
    pub fn option_opt(&self) -> Option<Option_> {
        todo!()
    }
    pub fn result_opt(&self) -> Option<Result_> {
        todo!()
    }
    pub fn tuple_opt(&self) -> Option<Tuple> {
        todo!()
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
#[derive(Debug, Clone, PartialEq)]
pub struct TypeItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TypeItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn ty(&self) -> Option<Ty> {
        self.0.child_by_field_name("ty").and_then(<Ty as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct TypedefItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TypedefItem<'tree> {
    pub fn enum_item(&self) -> Option<EnumItem> {
        self.0
            .child_by_field_name("enum_item")
            .and_then(<EnumItem as super::AstNode>::cast)
    }
    pub fn flags_item(&self) -> Option<FlagsItem> {
        self.0
            .child_by_field_name("flags_item")
            .and_then(<FlagsItem as super::AstNode>::cast)
    }
    pub fn record_item(&self) -> Option<RecordItem> {
        self.0
            .child_by_field_name("record_item")
            .and_then(<RecordItem as super::AstNode>::cast)
    }
    pub fn resource_item(&self) -> Option<ResourceItem> {
        self.0
            .child_by_field_name("resource_item")
            .and_then(<ResourceItem as super::AstNode>::cast)
    }
    pub fn type_item(&self) -> Option<TypeItem> {
        self.0
            .child_by_field_name("type_item")
            .and_then(<TypeItem as super::AstNode>::cast)
    }
    pub fn variant_item(&self) -> Option<VariantItem> {
        self.0
            .child_by_field_name("variant_item")
            .and_then(<VariantItem as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct UseItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UseItem<'tree> {
    pub fn path(&self) -> Option<UsePath> {
        self.0.child_by_field_name("path").and_then(<UsePath as super::AstNode>::cast)
    }
    pub fn use_names_list(&self) -> Option<UseNamesList> {
        self.0
            .child_by_field_name("use_names_list")
            .and_then(<UseNamesList as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct UseNamesItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UseNamesItem<'tree> {
    pub fn alias_opt(&self) -> Option<Identifier> {
        todo!()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
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
#[derive(Debug, Clone, PartialEq)]
pub struct UseNamesList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UseNamesList<'tree> {
    pub fn iter_use_names_items(&self) -> impl Iterator<Item = UseNamesItem> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct UsePath<'tree>(tree_sitter::Node<'tree>);
impl<'tree> UsePath<'tree> {
    pub fn package_opt(&self) -> Option<PackageName> {
        todo!()
    }
    pub fn path_opt(&self) -> Option<PackagePath> {
        todo!()
    }
    pub fn version_opt(&self) -> Option<Semver> {
        todo!()
    }
    pub fn identifier_opt(&self) -> Option<Identifier> {
        todo!()
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
///The `variant_case` node.
#[derive(Debug, Clone, PartialEq)]
pub struct VariantCase<'tree>(tree_sitter::Node<'tree>);
impl<'tree> VariantCase<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn payload_opt(&self) -> Option<Ty> {
        todo!()
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
#[derive(Debug, Clone, PartialEq)]
pub struct VariantItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> VariantItem<'tree> {
    pub fn iter_attributes(&self) -> impl Iterator<Item = Attribute> {
        Vec::new().into_iter()
    }
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn iter_variant_cases(&self) -> impl Iterator<Item = VariantCase> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct WorldItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> WorldItem<'tree> {
    pub fn name(&self) -> Option<Identifier> {
        self.0.child_by_field_name("name").and_then(<Identifier as super::AstNode>::cast)
    }
    pub fn iter_world_items(&self) -> impl Iterator<Item = WorldItems> {
        Vec::new().into_iter()
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
#[derive(Debug, Clone, PartialEq)]
pub struct WorldItems<'tree>(tree_sitter::Node<'tree>);
impl<'tree> WorldItems<'tree> {
    pub fn export_item(&self) -> Option<ExportItem> {
        self.0
            .child_by_field_name("export_item")
            .and_then(<ExportItem as super::AstNode>::cast)
    }
    pub fn import_item(&self) -> Option<ImportItem> {
        self.0
            .child_by_field_name("import_item")
            .and_then(<ImportItem as super::AstNode>::cast)
    }
    pub fn include_item(&self) -> Option<IncludeItem> {
        self.0
            .child_by_field_name("include_item")
            .and_then(<IncludeItem as super::AstNode>::cast)
    }
    pub fn typedef_item(&self) -> Option<TypedefItem> {
        self.0
            .child_by_field_name("typedef_item")
            .and_then(<TypedefItem as super::AstNode>::cast)
    }
    pub fn use_item(&self) -> Option<UseItem> {
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
///The `block_comment` node.
#[derive(Debug, Clone, PartialEq)]
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
///The `identifier` node.
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
