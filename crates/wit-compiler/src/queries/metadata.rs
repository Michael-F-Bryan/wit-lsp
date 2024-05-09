use std::{any::TypeId, collections::HashMap};

use ast::HasSource;
use im::{OrdMap, Vector};
use salsa::DebugWithDb;
use tree_sitter::Point;

use crate::{
    access::{self, *},
    ast::{self, AstNode, HasIdent as _},
    diagnostics::{Diagnostics, DuplicateName, Location, MultipleConstructors},
    queries::{FilePath, Package, SourceFile},
    Db, Text,
};

/// Find all the top-level items exposed by a [`Package`].
///
/// If there are any duplicate items, a [`DuplicateName`] diagnostic will be
/// emitted.
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub fn package_items(db: &dyn Db, pkg: Package) -> PackageMetadata {
    let mut items: HashMap<Ident, TopLevelItemMetadata> = HashMap::new();

    for file in pkg.files(db) {
        for item in file_items(db, file) {
            push_item(db, &mut items, item);
        }
    }

    let mut worlds = OrdMap::new();
    let mut interfaces = OrdMap::new();

    for item in items.values().copied() {
        match item {
            TopLevelItemMetadata::World(w) => {
                let index = w.index(db);
                worlds.insert(index, w);
            }
            TopLevelItemMetadata::Interface(i) => {
                let index = i.index(db);
                interfaces.insert(index, i);
            }
        }
    }

    PackageMetadata::new(db, items.into_iter().collect(), worlds, interfaces)
}

/// Find all the top-level items in a single file.
///
/// If there are any duplicate items, a [`DuplicateName`] diagnostic will be
/// emitted.
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(file = %file.path(db).raw_path(db)))]
pub fn file_items(db: &dyn Db, file: SourceFile) -> Vector<TopLevelItemMetadata> {
    let ast = crate::queries::parse(db, file);
    let mut ctx = Context {
        db,
        file,
        path: file.path(db),
        src: ast.src(db),
        next_indices: OrdMap::new(),
    };
    let mut items = HashMap::new();

    let source_file = ast.source_file(db);

    for node in source_file.iter_top_level_items() {
        if let Some(item) = lower_top_level_item(&mut ctx, node) {
            push_item(ctx.db, &mut items, item);
        }
    }

    items.into_values().collect()
}

fn lower_top_level_item(
    ctx: &mut Context<'_>,
    node: ast::TopLevelItem<'_>,
) -> Option<TopLevelItemMetadata> {
    if let Some(node) = node.interface_item() {
        let item = interface_metadata(ctx, node)?;
        Some(TopLevelItemMetadata::Interface(item))
    } else if let Some(node) = node.world_item() {
        let item = world_metadata(ctx, node)?;
        Some(TopLevelItemMetadata::World(item))
    } else {
        None
    }
}

#[salsa::tracked]
pub struct PackageMetadata {
    pub items_by_name: OrdMap<Ident, TopLevelItemMetadata>,
    pub worlds: OrdMap<WorldIndex, WorldMetadata>,
    pub interfaces: OrdMap<InterfaceIndex, InterfaceMetadata>,
}

impl PackageMetadata {
    pub fn enclosing_item(self, db: &dyn Db, point: Point) -> Option<ScopeIndex> {
        self.items_by_name(db)
            .into_iter()
            .map(|(_, item)| item)
            .find_map(|meta| {
                let location = meta.definition(db);
                if location.contains(point) {
                    Some(meta.index(db))
                } else {
                    None
                }
            })
    }

    pub fn get_world(self, db: &dyn Db, index: WorldIndex) -> WorldMetadata {
        self.worlds(db).get(&index).copied().unwrap()
    }

    pub fn get_interface(self, db: &dyn Db, index: InterfaceIndex) -> InterfaceMetadata {
        self.interfaces(db).get(&index).copied().unwrap()
    }
}

impl GetByIndex<access::World> for PackageMetadata {
    type Metadata = WorldMetadata;

    fn try_get_by_index(&self, db: &dyn Db, index: WorldIndex) -> Option<WorldMetadata> {
        self.worlds(db).get(&index).copied()
    }
}

impl GetByIndex<access::Interface> for PackageMetadata {
    type Metadata = InterfaceMetadata;

    fn try_get_by_index(&self, db: &dyn Db, index: InterfaceIndex) -> Option<InterfaceMetadata> {
        self.interfaces(db).get(&index).copied()
    }
}

#[derive(Clone)]
struct Context<'db> {
    db: &'db dyn Db,
    path: FilePath,
    file: SourceFile,
    src: &'db str,
    next_indices: OrdMap<TypeId, RawIndex>,
}

impl<'db> Context<'db> {
    /// Allocate a unique [`Index`] for a particular [`NodeKind`].
    fn index<K: NodeKind + 'static>(&mut self) -> Index<K> {
        let next_ix = self
            .next_indices
            .entry(TypeId::of::<K>())
            .or_insert(RawIndex::ZERO);

        let index = Index::new(self.path, *next_ix);
        *next_ix = next_ix.next();

        index
    }
}

fn interface_metadata(
    ctx: &mut Context<'_>,
    node: ast::InterfaceItem<'_>,
) -> Option<InterfaceMetadata> {
    let name = node.identifier(ctx.src)?;
    let name = Ident::new(ctx.db, name.into());
    let mut items = HashMap::new();

    for child in node.body()?.iter_interface_items() {
        if let Some(item) = interface_item_metadata(ctx, child) {
            push_item(ctx.db, &mut items, item);
        }
    }

    Some(InterfaceMetadata::new(
        ctx.db,
        name,
        ctx.index(),
        InterfacePtr::for_node(ctx.path, node),
        items.into_values().collect(),
    ))
}

fn world_metadata(ctx: &mut Context<'_>, node: ast::WorldItem<'_>) -> Option<WorldMetadata> {
    let name = node.identifier(ctx.src)?;
    let name = Ident::new(ctx.db, name.into());

    let mut definitions = HashMap::new();

    let mut imports = Vector::new();
    let mut named_imports: HashMap<Ident, ImportMetadata> = HashMap::new();
    let mut exports = Vector::new();
    let mut named_exports: HashMap<Ident, ExportMetadata> = HashMap::new();

    for child in node.body()?.iter_world_items() {
        if let Some(item) = child
            .typedef_item_opt()
            .and_then(|n| typedef_metadata(ctx, n))
        {
            push_item(ctx.db, &mut definitions, item);
        } else if let Some(item) = child
            .export_item_opt()
            .and_then(|n| export_metadata(ctx, n))
        {
            push_with_optional_name(
                ctx.db,
                &mut exports,
                &mut named_exports,
                item,
                item.name(ctx.db),
            );
        } else if let Some(item) = child
            .import_item_opt()
            .and_then(|n| import_metadata(ctx, n))
        {
            push_with_optional_name(
                ctx.db,
                &mut imports,
                &mut named_imports,
                item,
                item.name(ctx.db),
            );
        }
    }

    Some(WorldMetadata::new(
        ctx.db,
        name,
        ctx.index(),
        WorldPtr::for_node(ctx.path, node),
        definitions.into_values().collect(),
        exports,
        imports,
    ))
}

fn push_with_optional_name<T>(
    db: &dyn Db,
    values: &mut Vector<T>,
    named_values: &mut HashMap<Ident, T>,
    value: T,
    ident: Option<Ident>,
) where
    T: HasDefinition + Clone,
{
    let Some(ident) = ident else {
        values.push_back(value);
        return;
    };

    match named_values.entry(ident) {
        std::collections::hash_map::Entry::Occupied(entry) => {
            let diag = DuplicateName {
                name: ident.raw(db).into(),
                location: value.definition(db),
                original_definition: entry.get().definition(db),
            };
            Diagnostics::push(db, diag.into());
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            values.push_back(value.clone());
            entry.insert(value);
        }
    }
}

fn export_metadata(ctx: &mut Context, node: ast::ExportItem<'_>) -> Option<ExportMetadata> {
    let ident = node
        .name_opt()
        .map(|n| Ident::new(ctx.db, n.utf8_text(ctx.src).into()));

    Some(ExportMetadata::new(
        ctx.db,
        ident,
        ctx.index(),
        Pointer::for_node(ctx.path, node),
    ))
}

fn import_metadata(ctx: &mut Context, node: ast::ImportItem<'_>) -> Option<ImportMetadata> {
    let ident = node
        .name_opt()
        .map(|n| Ident::new(ctx.db, n.utf8_text(ctx.src).into()));

    Some(ImportMetadata::new(
        ctx.db,
        ident,
        ctx.index(),
        Pointer::for_node(ctx.path, node),
    ))
}

fn interface_item_metadata(
    ctx: &mut Context<'_>,
    node: ast::InterfaceItems<'_>,
) -> Option<InterfaceItemMetadata> {
    if let Some(func) = node.func_opt() {
        simple_item_metadata(ctx, func, FuncItemMetadata::new).map(InterfaceItemMetadata::Func)
    } else if let Some(typedef) = node.typedef_opt() {
        typedef_metadata(ctx, typedef).map(InterfaceItemMetadata::Type)
    } else {
        None
    }
}

fn typedef_metadata(
    ctx: &mut Context<'_>,
    node: ast::TypedefItem<'_>,
) -> Option<TypeDefinitionMetadata> {
    if let Some(node) = node.enum_items() {
        enum_metadata(ctx, node).map(TypeDefinitionMetadata::Enum)
    } else if let Some(node) = node.flags_items() {
        flags_metadata(ctx, node).map(TypeDefinitionMetadata::Flags)
    } else if let Some(node) = node.record_item() {
        record_metadata(ctx, node).map(TypeDefinitionMetadata::Record)
    } else if let Some(node) = node.resource_item() {
        resource_metadata(ctx, node).map(TypeDefinitionMetadata::Resource)
    } else if let Some(node) = node.type_item() {
        type_alias_metadata(ctx, node).map(TypeDefinitionMetadata::Alias)
    } else if let Some(node) = node.variant_items() {
        variant_metadata(ctx, node).map(TypeDefinitionMetadata::Variant)
    } else {
        None
    }
}

fn enum_metadata(ctx: &mut Context<'_>, node: ast::EnumItems<'_>) -> Option<EnumMetadata> {
    let name = node.identifier(ctx.src)?;
    let cases = node
        .enum_body()?
        .iter_enum_cases()
        .filter_map(|case| simple_item_metadata(ctx, case, EnumCaseMetadata::new))
        .collect();

    Some(EnumMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.index(),
        EnumPtr::for_node(ctx.path, node),
        cases,
    ))
}

fn flags_metadata(ctx: &mut Context<'_>, node: ast::FlagsItems<'_>) -> Option<FlagsMetadata> {
    let name = node.identifier(ctx.src)?;
    let cases = node
        .body()?
        .iter_flags_fields()
        .filter_map(|case| simple_item_metadata(ctx, case, FlagsCaseMetadata::new))
        .collect();

    Some(FlagsMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.index(),
        FlagsPtr::for_node(ctx.path, node),
        cases,
    ))
}

fn record_metadata(ctx: &mut Context<'_>, node: ast::RecordItem<'_>) -> Option<RecordMetadata> {
    let name = node.identifier(ctx.src)?;
    let cases = node
        .body()?
        .iter_record_fields()
        .filter_map(|case| simple_item_metadata(ctx, case, FieldMetadata::new))
        .collect();

    Some(RecordMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.index(),
        RecordPtr::for_node(ctx.path, node),
        cases,
    ))
}

fn variant_metadata(ctx: &mut Context<'_>, node: ast::VariantItems<'_>) -> Option<VariantMetadata> {
    let name = node.identifier(ctx.src)?;
    let cases = node
        .body()?
        .iter_variant_cases()
        .filter_map(|case| simple_item_metadata(ctx, case, VariantCaseMetadata::new))
        .collect();

    Some(VariantMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.index(),
        VariantPtr::for_node(ctx.path, node),
        cases,
    ))
}

fn resource_metadata(
    ctx: &mut Context<'_>,
    node: ast::ResourceItem<'_>,
) -> Option<ResourceMetadata> {
    let name = node.identifier(ctx.src)?;

    enum AnyMethod {
        Method(MethodMetadata),
        Static(StaticMethodMetadata),
    }

    impl HasDefinition for AnyMethod {
        fn definition(&self, db: &dyn Db) -> Location {
            match self {
                AnyMethod::Method(m) => m.definition(db),
                AnyMethod::Static(s) => s.definition(db),
            }
        }
    }

    impl HasIdent for AnyMethod {
        fn ident(&self, db: &dyn Db) -> Ident {
            match self {
                AnyMethod::Method(m) => m.name(db),
                AnyMethod::Static(s) => s.name(db),
            }
        }
    }

    let mut constructor: Option<ConstructorMetadata> = None;
    let mut all_methods: HashMap<Ident, AnyMethod> = HashMap::new();

    for method in node
        .resource_body_opt()
        .into_iter()
        .flat_map(|body| body.iter_resource_methods())
    {
        if let Some(c) = method.resource_constructor() {
            let c =
                ConstructorMetadata::new(ctx.db, ctx.file, ConstructorPtr::for_node(ctx.path, c));

            if let Some(previous_constructor) = &constructor {
                let diag = MultipleConstructors {
                    location: c.definition(ctx.db),
                    original_definition: previous_constructor.definition(ctx.db),
                };
                Diagnostics::push(ctx.db, diag.into());
            } else {
                constructor = Some(c);
            }
        } else if let Some(method) = method.func_item() {
            if let Some(item) = simple_item_metadata(ctx, method, MethodMetadata::new) {
                push_item(ctx.db, &mut all_methods, AnyMethod::Method(item));
            }
        } else if let Some(method) = method.static_resource_method() {
            if let Some(item) = simple_item_metadata(ctx, method, StaticMethodMetadata::new) {
                push_item(ctx.db, &mut all_methods, AnyMethod::Static(item));
            }
        }
    }

    let mut methods = Vector::new();
    let mut static_methods = Vector::new();

    for m in all_methods.into_values() {
        match m {
            AnyMethod::Method(m) => methods.push_back(m),
            AnyMethod::Static(s) => static_methods.push_back(s),
        }
    }

    Some(ResourceMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.index(),
        ResourcePtr::for_node(ctx.path, node),
        constructor,
        methods,
        static_methods,
    ))
}

fn type_alias_metadata(
    ctx: &mut Context<'_>,
    node: ast::TypeItem<'_>,
) -> Option<TypeAliasMetadata> {
    let name = node.alias()?.utf8_text(ctx.src);

    Some(TypeAliasMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.index(),
        Pointer::for_node(ctx.path, node),
    ))
}

fn simple_item_metadata<'tree, 'db: 'tree, K>(
    ctx: &mut Context<'db>,
    node: K::Ast<'tree>,
    constructor: impl FnOnce(&dyn Db, Ident, Index<K>, Pointer<K>) -> K::Metadata,
) -> Option<K::Metadata>
where
    K: NodeKind + 'static,
    K::Ast<'tree>: AstNode<'tree> + crate::ast::HasIdent + Copy,
{
    let name = node.identifier(ctx.src)?;

    Some(constructor(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.index(),
        Pointer::for_node(ctx.path, node),
    ))
}

/// Something that has a [`Location`] that points to its definition in the
/// source code.
pub trait HasDefinition {
    fn definition(&self, db: &dyn Db) -> Location;
}

pub trait HasIdent {
    fn ident(&self, db: &dyn Db) -> Ident;
}

pub(crate) trait GetByIndex<K> {
    type Metadata;

    fn try_get_by_index(&self, db: &dyn Db, index: Index<K>) -> Option<Self::Metadata>;

    fn get_by_index(&self, db: &dyn Db, index: Index<K>) -> Self::Metadata {
        match self.try_get_by_index(db, index) {
            Some(value) => value,
            None => panic!("Lookup failed: {index:?}"),
        }
    }
}

pub trait HasIndex {
    /// Get the type's [`Index`].
    fn any_index(&self, db: &dyn Db) -> AnyIndex;
}

macro_rules! impl_common {
    ($name:ty) => {
        impl HasDefinition for $name {
            fn definition(&self, db: &dyn Db) -> Location {
                self.ptr(db).location()
            }
        }

        impl HasIdent for $name {
            fn ident(&self, db: &dyn Db) -> Ident {
                <$name>::name(*self, db)
            }
        }

        impl HasIndex for $name {
            fn any_index(&self, db: &dyn Db) -> AnyIndex {
                <$name>::index(*self, db).into()
            }
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TopLevelItemMetadata {
    World(WorldMetadata),
    Interface(InterfaceMetadata),
}

impl TopLevelItemMetadata {
    pub fn index(self, db: &dyn Db) -> ScopeIndex {
        match self {
            TopLevelItemMetadata::World(w) => w.index(db).into(),
            TopLevelItemMetadata::Interface(i) => i.index(db).into(),
        }
    }
}

impl HasDefinition for TopLevelItemMetadata {
    fn definition(&self, db: &dyn Db) -> Location {
        match self {
            TopLevelItemMetadata::World(w) => w.definition(db),
            TopLevelItemMetadata::Interface(i) => i.definition(db),
        }
    }
}

impl HasIdent for TopLevelItemMetadata {
    fn ident(&self, db: &dyn Db) -> Ident {
        match self {
            TopLevelItemMetadata::World(w) => w.name(db),
            TopLevelItemMetadata::Interface(i) => i.name(db),
        }
    }
}

impl HasIndex for TopLevelItemMetadata {
    fn any_index(&self, db: &dyn Db) -> AnyIndex {
        match self {
            TopLevelItemMetadata::World(w) => w.any_index(db),
            TopLevelItemMetadata::Interface(i) => i.any_index(db),
        }
    }
}

impl DebugWithDb<dyn Db> for TopLevelItemMetadata {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            TopLevelItemMetadata::World(w) => w.fmt(f, db, include_all_fields),
            TopLevelItemMetadata::Interface(i) => i.fmt(f, db, include_all_fields),
        }
    }
}

#[salsa::tracked]
pub struct WorldMetadata {
    pub name: Ident,
    #[id]
    pub index: WorldIndex,
    pub ptr: WorldPtr,
    pub definitions: Vector<TypeDefinitionMetadata>,
    pub exports: Vector<ExportMetadata>,
    pub imports: Vector<ImportMetadata>,
}

impl_common!(WorldMetadata);

#[salsa::tracked]
pub struct ExportMetadata {
    pub name: Option<Ident>,
    #[id]
    pub index: ExportIndex,
    pub ptr: ExportPtr,
}

impl HasDefinition for ExportMetadata {
    fn definition(&self, db: &dyn Db) -> Location {
        self.ptr(db).location()
    }
}

impl HasIndex for ExportMetadata {
    fn any_index(&self, db: &dyn Db) -> AnyIndex {
        self.index(db).into()
    }
}

#[salsa::tracked]
pub struct ImportMetadata {
    pub name: Option<Ident>,
    #[id]
    pub index: ExportIndex,
    pub ptr: ImportPtr,
}

impl HasDefinition for ImportMetadata {
    fn definition(&self, db: &dyn Db) -> Location {
        self.ptr(db).location()
    }
}

impl HasIndex for ImportMetadata {
    fn any_index(&self, db: &dyn Db) -> AnyIndex {
        self.index(db).into()
    }
}

#[salsa::tracked]
pub struct InterfaceMetadata {
    pub name: Ident,
    #[id]
    pub index: InterfaceIndex,
    pub ptr: InterfacePtr,
    pub items: Vector<InterfaceItemMetadata>,
}

impl_common!(InterfaceMetadata);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InterfaceItemMetadata {
    Func(FuncItemMetadata),
    Type(TypeDefinitionMetadata),
}

impl InterfaceItemMetadata {
    pub fn as_func(self) -> Option<FuncItemMetadata> {
        match self {
            Self::Func(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_type(self) -> Option<TypeDefinitionMetadata> {
        match self {
            Self::Type(v) => Some(v),
            _ => None,
        }
    }
}

impl HasDefinition for InterfaceItemMetadata {
    fn definition(&self, db: &dyn Db) -> Location {
        match self {
            InterfaceItemMetadata::Func(item) => item.definition(db),
            InterfaceItemMetadata::Type(item) => item.definition(db),
        }
    }
}

impl HasIdent for InterfaceItemMetadata {
    fn ident(&self, db: &dyn Db) -> Ident {
        match self {
            InterfaceItemMetadata::Func(item) => item.name(db),
            InterfaceItemMetadata::Type(item) => item.ident(db),
        }
    }
}

impl HasIndex for InterfaceItemMetadata {
    fn any_index(&self, db: &dyn Db) -> AnyIndex {
        match self {
            InterfaceItemMetadata::Func(f) => f.any_index(db),
            InterfaceItemMetadata::Type(t) => t.any_index(db),
        }
    }
}

impl DebugWithDb<dyn Db + '_> for InterfaceItemMetadata {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            InterfaceItemMetadata::Func(item) => item.fmt(f, db, include_all_fields),
            InterfaceItemMetadata::Type(item) => item.fmt(f, db, include_all_fields),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TypeDefinitionMetadata {
    Enum(EnumMetadata),
    Record(RecordMetadata),
    Resource(ResourceMetadata),
    Variant(VariantMetadata),
    Flags(FlagsMetadata),
    Alias(TypeAliasMetadata),
}

impl HasDefinition for TypeDefinitionMetadata {
    fn definition(&self, db: &dyn Db) -> Location {
        match self {
            TypeDefinitionMetadata::Enum(item) => item.definition(db),
            TypeDefinitionMetadata::Record(item) => item.definition(db),
            TypeDefinitionMetadata::Resource(item) => item.definition(db),
            TypeDefinitionMetadata::Variant(item) => item.definition(db),
            TypeDefinitionMetadata::Flags(item) => item.definition(db),
            TypeDefinitionMetadata::Alias(item) => item.definition(db),
        }
    }
}

impl HasIdent for TypeDefinitionMetadata {
    fn ident(&self, db: &dyn Db) -> Ident {
        match self {
            TypeDefinitionMetadata::Enum(item) => item.name(db),
            TypeDefinitionMetadata::Record(item) => item.name(db),
            TypeDefinitionMetadata::Resource(item) => item.name(db),
            TypeDefinitionMetadata::Variant(item) => item.name(db),
            TypeDefinitionMetadata::Flags(item) => item.name(db),
            TypeDefinitionMetadata::Alias(item) => item.name(db),
        }
    }
}

impl HasIndex for TypeDefinitionMetadata {
    fn any_index(&self, db: &dyn Db) -> AnyIndex {
        match self {
            TypeDefinitionMetadata::Enum(item) => item.any_index(db),
            TypeDefinitionMetadata::Record(item) => item.any_index(db),
            TypeDefinitionMetadata::Resource(item) => item.any_index(db),
            TypeDefinitionMetadata::Variant(item) => item.any_index(db),
            TypeDefinitionMetadata::Flags(item) => item.any_index(db),
            TypeDefinitionMetadata::Alias(item) => item.any_index(db),
        }
    }
}

impl DebugWithDb<dyn Db + '_> for TypeDefinitionMetadata {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            TypeDefinitionMetadata::Enum(item) => item.fmt(f, db, include_all_fields),
            TypeDefinitionMetadata::Record(item) => item.fmt(f, db, include_all_fields),
            TypeDefinitionMetadata::Resource(item) => item.fmt(f, db, include_all_fields),
            TypeDefinitionMetadata::Variant(item) => item.fmt(f, db, include_all_fields),
            TypeDefinitionMetadata::Flags(item) => item.fmt(f, db, include_all_fields),
            TypeDefinitionMetadata::Alias(item) => item.fmt(f, db, include_all_fields),
        }
    }
}

#[salsa::tracked]
pub struct RecordMetadata {
    pub name: Ident,
    #[id]
    pub index: RecordIndex,
    pub ptr: RecordPtr,
    pub fields: Vector<FieldMetadata>,
}

impl_common!(RecordMetadata);

#[salsa::tracked]
pub struct ResourceMetadata {
    pub name: Ident,
    #[id]
    pub index: ResourceIndex,
    pub ptr: ResourcePtr,
    pub constructor: Option<ConstructorMetadata>,
    pub methods: Vector<MethodMetadata>,
    pub static_methods: Vector<StaticMethodMetadata>,
}

impl_common!(ResourceMetadata);

#[salsa::tracked]
pub struct ConstructorMetadata {
    pub file: SourceFile,
    pub ptr: ConstructorPtr,
}

impl HasDefinition for ConstructorMetadata {
    fn definition(&self, db: &dyn Db) -> Location {
        Location::new(self.file(db).path(db), self.ptr(db).range())
    }
}

macro_rules! simple_metadata {
    ($( $name:ident => $kind:ty ),* $(,)?) => {
        $(
            // Note: We use paste to force the macro substitutions to be
            // evaluated before #[salsa::tracked].
            paste::paste! {
                #[salsa::tracked]
                pub struct [< $name >] {
                    pub name: Ident,
                    #[id]
                    pub index: [< $kind Index >],
                    pub ptr: [< $kind Ptr >],
                }
            }

            impl_common!($name);
        )*
    };
}

simple_metadata! {
    FuncItemMetadata => Function,
    TypeAliasMetadata => TypeAlias,
    FieldMetadata => RecordField,
    MethodMetadata => Method,
    StaticMethodMetadata => StaticMethod,
}

macro_rules! enum_like_metadata {
    ($( $name:ident => ($kind:ty, $case:ty) ),* $(,)?) => {
        $(
            // Note: We use paste to force the macro substitutions to be
            // evaluated before #[salsa::tracked].
            paste::paste! {
                #[salsa::tracked]
                pub struct [< $name Metadata >] {
                    pub name: Ident,
                    #[id]
                    pub index: [< $kind Index >],
                    pub ptr: [< $kind Ptr >],
                    pub cases: Vector<[< $name CaseMetadata >]>,
                }

                impl_common!([< $name Metadata >]);

                #[salsa::tracked]
                pub struct [< $name CaseMetadata >] {
                    pub name: Ident,
                    #[id]
                    pub index: [< $case Index >],
                    pub ptr: [< $case Ptr >],
                }

                impl_common!([< $name CaseMetadata >]);
            }
        )*
    };
}

enum_like_metadata! {
    Variant => (Variant, VariantCase),
    Enum => (Enum, EnumCase),
    Flags => (Flags, FlagsCase),
}

/// An interned identifier.
#[salsa::interned]
pub struct Ident {
    #[return_ref]
    pub raw: Text,
}

fn push_item<T>(db: &dyn Db, items: &mut HashMap<Ident, T>, item: T)
where
    T: HasIdent + HasDefinition,
{
    let name = item.ident(db);

    match items.entry(name) {
        std::collections::hash_map::Entry::Occupied(entry) => {
            let diag = DuplicateName {
                name: name.raw(db).clone(),
                location: item.definition(db),
                original_definition: entry.get().definition(db),
            };
            Diagnostics::push(db, diag.into());
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(item);
        }
    };
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::{queries::FilePath, Compiler};

    use super::*;

    #[allow(dead_code)] // we only need this for the Debug impl
    #[derive(Debug)]
    struct Node<'db> {
        name: &'db str,
        type_name: &'static str,
        file: &'db str,
        range: Range<usize>,
        children: Vec<Node<'db>>,
    }

    impl<'db> Node<'db> {
        fn common<T>(db: &'db dyn Db, item: &T) -> Self
        where
            T: HasDefinition + HasIdent + 'static,
        {
            let name = item.ident(db).raw(db);
            Node::new(db, item, name)
        }

        fn new<T>(db: &'db dyn Db, item: &T, name: &'db str) -> Self
        where
            T: HasDefinition + 'static,
        {
            let location = item.definition(db);
            let file = location.filename.raw_path(db);
            let tree_sitter::Range {
                start_byte,
                end_byte,
                ..
            } = location.range;

            Node {
                name,
                type_name: std::any::type_name::<T>(),
                file,
                range: start_byte..end_byte,
                children: Vec::new(),
            }
        }

        fn with_cases<T, C>(
            db: &'db dyn Db,
            item: &T,
            getter: impl Fn(T, &dyn Db) -> Vector<C>,
        ) -> Self
        where
            T: HasDefinition + HasIdent + Copy + 'static,
            C: HasDefinition + HasIdent + Copy + 'static,
        {
            let children = getter(*item, db)
                .iter()
                .map(|c| Node::common(db, c))
                .collect();

            Node {
                children,
                ..Node::common(db, item)
            }
        }
    }

    fn nodes<'db>(
        db: &'db dyn Db,
        top_level_items: &Vector<TopLevelItemMetadata>,
    ) -> Vec<Node<'db>> {
        top_level_items
            .iter()
            .map(|&n| top_level_node(db, n))
            .collect()
    }

    fn top_level_node(db: &dyn Db, n: TopLevelItemMetadata) -> Node<'_> {
        match n {
            TopLevelItemMetadata::World(w) => world_node(db, w),
            TopLevelItemMetadata::Interface(i) => interface_node(db, i),
        }
    }

    fn world_node(db: &dyn Db, meta: WorldMetadata) -> Node<'_> {
        let mut children = Vec::new();

        children.extend(
            meta.definitions(db)
                .iter()
                .map(|&meta| type_definition_node(db, meta)),
        );

        children.extend(meta.imports(db).iter().map(|meta| {
            Node::new(
                db,
                meta,
                meta.name(db)
                    .map(|id| id.raw(db).as_str())
                    .unwrap_or_default(),
            )
        }));

        children.extend(meta.exports(db).iter().map(|meta| {
            Node::new(
                db,
                meta,
                meta.name(db)
                    .map(|id| id.raw(db).as_str())
                    .unwrap_or_default(),
            )
        }));

        Node {
            children,
            ..Node::common(db, &meta)
        }
    }

    fn interface_node(db: &dyn Db, meta: InterfaceMetadata) -> Node<'_> {
        let children = meta
            .items(db)
            .iter()
            .map(|&meta| interface_item_node(db, meta))
            .collect();

        Node {
            children,
            ..Node::common(db, &meta)
        }
    }

    fn interface_item_node(db: &dyn Db, meta: InterfaceItemMetadata) -> Node<'_> {
        match meta {
            InterfaceItemMetadata::Func(meta) => Node::common(db, &meta),
            InterfaceItemMetadata::Type(meta) => type_definition_node(db, meta),
        }
    }

    fn type_definition_node(db: &dyn Db, meta: TypeDefinitionMetadata) -> Node<'_> {
        match &meta {
            TypeDefinitionMetadata::Enum(e) => Node::with_cases(db, e, EnumMetadata::cases),
            TypeDefinitionMetadata::Record(f) => Node::with_cases(db, f, RecordMetadata::fields),
            TypeDefinitionMetadata::Resource(r) => {
                let mut children = Vec::new();

                if let Some(c) = r.constructor(db) {
                    let item = &c;
                    let location = item.definition(db);
                    let file = location.filename.raw_path(db);
                    let tree_sitter::Range {
                        start_byte,
                        end_byte,
                        ..
                    } = location.range;

                    let node = Node {
                        name: "$constructor",
                        type_name: std::any::type_name_of_val(&c),
                        file,
                        range: start_byte..end_byte,
                        children: Vec::new(),
                    };
                    children.push(node);
                }
                children.extend(r.methods(db).into_iter().map(|m| Node::common(db, &m)));
                children.extend(
                    r.static_methods(db)
                        .into_iter()
                        .map(|m| Node::common(db, &m)),
                );

                Node {
                    children,
                    ..Node::common(db, &meta)
                }
            }
            TypeDefinitionMetadata::Variant(v) => Node::with_cases(db, v, VariantMetadata::cases),
            TypeDefinitionMetadata::Flags(f) => Node::with_cases(db, f, FlagsMetadata::cases),
            TypeDefinitionMetadata::Alias(a) => Node::common(db, a),
        }
    }

    macro_rules! item_metadata_tests {
        ($( $(#[$meta:meta])* $name:ident => $src:expr ),* $(,)?) => {
            $(
                #[test]
                $( #[$meta] )*
                fn $name() {
                    let src = $src;
                    let db = Compiler::default();
                    let path = FilePath::new(&db, concat!(stringify!($name), ".wit").into());
                    let file = SourceFile::new(&db, path, src.into());

                    let items = file_items(&db, file);
                    let diags = file_items::accumulated::<Diagnostics>(&db, file);

                    assert!(diags.is_empty(), "{diags:?}");

                    #[derive(serde::Serialize)]
                    struct Info<'a> {
                        src: &'a str,
                        ast: String,
                    }

                    let mut settings = insta::Settings::clone_current();
                    settings.set_info(&Info {
                        src,
                        ast: crate::queries::parse(&db, file).root_node(&db).to_sexp(),
                    });
                    settings.set_omit_expression(true);
                    settings.bind(|| insta::assert_debug_snapshot!(nodes(&db, &items)));
                }
            )*
        };
    }

    item_metadata_tests! {
        empty_interface => "interface empty {}",
        interface_function => "interface i { x: func(); }",
        interface_type_alias => "interface i { type x = string; }",
        interface_enum => "interface i { enum foo { first, second } }",
        interface_variant => "interface i { variant foo { first, second(u32) } }",
        interface_flags => "interface i { flags foo {first, second } }",
        interface_record => "interface i { record point { x: float64, y: float64 } }",

        interface_resource_empty => "interface i { resource foo {} }",
        interface_resource_with_constructor => "interface i { resource foo { constructor(); } }",
        interface_resource_with_method => "interface i { resource foo { method: func(); } }",
        interface_resource_with_static_method => "interface i { resource foo { method: static func(); } }",

        world_empty => "world empty {}",
        world_with_type_alias => "world w { type x = u32; }",
        #[ignore = "Parse error"]
        world_with_named_export => "world w { export run: func(); }",
        #[ignore = "Parse error"]
        world_with_named_import => "world w { import run: func(); }",
        world_with_external_export => "world w { export wasi:filesystem/filesystem; }",
        world_with_external_import => "world w { import wasi:filesystem/filesystem; }",
    }
}
