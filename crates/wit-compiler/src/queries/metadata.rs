use std::collections::HashMap;

use im::Vector;
use salsa::DebugWithDb;

use crate::{
    access::{
        ConstructorPtr, EnumCasePtr, EnumPtr, FlagsCasePtr, FlagsPtr, FunctionPtr, InterfacePtr,
        MethodPtr, Pointer, RecordFieldPtr, RecordPtr, ResourcePtr, StaticMethodPtr, TypeAliasPtr,
        VariantCasePtr, VariantPtr, WorldPtr,
    },
    ast::{self, AstNode, HasIdent},
    diagnostics::{Diagnostics, DuplicateName, Location, MultipleConstructors},
    hir,
    queries::{Package, SourceFile, Workspace},
    Db, Text,
};

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub fn lower_package(db: &dyn Db, _ws: Workspace, pkg: Package) -> hir::Package {
    let _items = package_items(db, pkg);
    todo!();
}

/// Find all the top-level items exposed by a [`Package`].
///
/// If there are any duplicate items, a [`DuplicateName`] diagnostic will be
/// emitted.
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub fn package_items(db: &dyn Db, pkg: Package) -> Vector<TopLevelItemMetadata> {
    let mut items: HashMap<Ident, TopLevelItemMetadata> = HashMap::new();

    for file in pkg.files(db) {
        for item in file_items(db, file) {
            push_item(db, &mut items, item);
        }
    }

    items.into_values().collect()
}

/// Find all the top-level items in a single file.
///
/// If there are any duplicate items, a [`DuplicateName`] diagnostic will be
/// emitted.
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(file = %file.path(db).raw_path(db)))]
pub fn file_items(db: &dyn Db, file: SourceFile) -> Vector<TopLevelItemMetadata> {
    let ast = crate::queries::parse(db, file);
    let ctx = Context {
        db,
        file,
        src: ast.src(db),
    };
    let mut items = HashMap::new();

    for node in ast.source_file(db).iter_top_level_items() {
        if let Some(item) = top_level_item_metadata(ctx, node) {
            push_item(ctx.db, &mut items, item);
        }
    }

    items.into_values().collect()
}

#[derive(Copy, Clone)]
struct Context<'db> {
    db: &'db dyn Db,
    file: SourceFile,
    src: &'db str,
}

fn top_level_item_metadata(
    ctx: Context<'_>,
    node: ast::TopLevelItem<'_>,
) -> Option<TopLevelItemMetadata> {
    if let Some(interface) = node.interface_item() {
        interface_metadata(ctx, interface).map(TopLevelItemMetadata::Interface)
    } else if let Some(world) = node.world_item() {
        world_metadata(ctx, world).map(TopLevelItemMetadata::World)
    } else {
        None
    }
}

fn interface_metadata(ctx: Context<'_>, node: ast::InterfaceItem<'_>) -> Option<InterfaceMetadata> {
    let name = node.identifier(ctx.src)?;
    let name = Ident::new(ctx.db, name.into());
    let mut items = HashMap::new();

    for child in node.iter_items() {
        if let Some(item) = interface_item_metadata(ctx, child) {
            push_item(ctx.db, &mut items, item);
        }
    }

    Some(InterfaceMetadata::new(
        ctx.db,
        name,
        ctx.file,
        InterfacePtr::for_node(node),
        items.into_values().collect(),
    ))
}

fn world_metadata(ctx: Context<'_>, node: ast::WorldItem<'_>) -> Option<WorldMetadata> {
    let name = node.identifier(ctx.src)?;
    let name = Ident::new(ctx.db, name.into());

    let mut definitions = HashMap::new();

    for child in node.iter_items() {
        if let Some(item) = child.typedef_item().and_then(|n| typedef_metadata(ctx, n)) {
            push_item(ctx.db, &mut definitions, item);
        }
    }

    Some(WorldMetadata::new(
        ctx.db,
        name,
        ctx.file,
        WorldPtr::for_node(node),
        definitions.into_values().collect(),
    ))
}

fn interface_item_metadata(
    ctx: Context<'_>,
    node: ast::InterfaceItems<'_>,
) -> Option<InterfaceItemMetadata> {
    if let Some(func) = node.func_item() {
        simple_item_metadata(ctx, func, FuncItemMetadata::new).map(InterfaceItemMetadata::Func)
    } else if let Some(typedef) = node.typedef_item() {
        typedef_metadata(ctx, typedef).map(InterfaceItemMetadata::Type)
    } else {
        None
    }
}

fn typedef_metadata(
    ctx: Context<'_>,
    node: ast::TypedefItem<'_>,
) -> Option<TypeDefinitionMetadata> {
    if let Some(node) = node.enum_item() {
        enum_metadata(ctx, node).map(TypeDefinitionMetadata::Enum)
    } else if let Some(node) = node.flags_item() {
        flags_metadata(ctx, node).map(TypeDefinitionMetadata::Flags)
    } else if let Some(node) = node.record_item() {
        record_metadata(ctx, node).map(TypeDefinitionMetadata::Record)
    } else if let Some(node) = node.resource_item() {
        resource_metadata(ctx, node).map(TypeDefinitionMetadata::Resource)
    } else if let Some(node) = node.type_item() {
        type_alias_metadata(ctx, node).map(TypeDefinitionMetadata::Alias)
    } else if let Some(node) = node.variant_item() {
        variant_metadata(ctx, node).map(TypeDefinitionMetadata::Variant)
    } else {
        None
    }
}

fn enum_metadata(ctx: Context<'_>, node: ast::EnumItem<'_>) -> Option<EnumMetadata> {
    let name = node.identifier(ctx.src)?;
    let cases = node
        .iter_cases()
        .filter_map(|case| simple_item_metadata(ctx, case, EnumCaseMetadata::new))
        .collect();

    Some(EnumMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.file,
        EnumPtr::for_node(node),
        cases,
    ))
}

fn flags_metadata(ctx: Context<'_>, node: ast::FlagsItem<'_>) -> Option<FlagsMetadata> {
    let name = node.identifier(ctx.src)?;
    let cases = node
        .iter_cases()
        .filter_map(|case| simple_item_metadata(ctx, case, FlagsCaseMetadata::new))
        .collect();

    Some(FlagsMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.file,
        FlagsPtr::for_node(node),
        cases,
    ))
}

fn record_metadata(ctx: Context<'_>, node: ast::RecordItem<'_>) -> Option<RecordMetadata> {
    let name = node.identifier(ctx.src)?;
    let cases = node
        .iter_fields()
        .filter_map(|case| simple_item_metadata(ctx, case, FieldMetadata::new))
        .collect();

    Some(RecordMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.file,
        RecordPtr::for_node(node),
        cases,
    ))
}

fn variant_metadata(ctx: Context<'_>, node: ast::VariantItem<'_>) -> Option<VariantMetadata> {
    let name = node.identifier(ctx.src)?;
    let cases = node
        .iter_cases()
        .filter_map(|case| simple_item_metadata(ctx, case, VariantCaseMetadata::new))
        .collect();

    Some(VariantMetadata::new(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.file,
        VariantPtr::for_node(node),
        cases,
    ))
}

fn resource_metadata(ctx: Context<'_>, node: ast::ResourceItem<'_>) -> Option<ResourceMetadata> {
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

    impl HasName for AnyMethod {
        fn name(&self, db: &dyn Db) -> Ident {
            match self {
                AnyMethod::Method(m) => m.name(db),
                AnyMethod::Static(s) => s.name(db),
            }
        }
    }

    let mut constructor: Option<ConstructorMetadata> = None;
    let mut all_methods: HashMap<Ident, AnyMethod> = HashMap::new();

    for method in node.iter_methods() {
        if let Some(c) = method.resource_constructor() {
            let c = ConstructorMetadata::new(ctx.db, ctx.file, ConstructorPtr::for_node(c));

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
        } else if let Some(method) = method.static_method() {
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
        ctx.file,
        ResourcePtr::for_node(node),
        constructor,
        methods,
        static_methods,
    ))
}

fn type_alias_metadata(ctx: Context<'_>, node: ast::TypeItem<'_>) -> Option<TypeAliasMetadata> {
    simple_item_metadata(ctx, node, TypeAliasMetadata::new)
}

fn simple_item_metadata<'db, N, Ptr, Meta>(
    ctx: Context<'db>,
    node: N,
    constructor: impl FnOnce(&dyn Db, Ident, SourceFile, Ptr) -> Meta,
) -> Option<Meta>
where
    N: AstNode<'db> + HasIdent + Copy,
    Ptr: Pointer<Node<'db> = N>,
{
    let name = node.identifier(ctx.src)?;

    Some(constructor(
        ctx.db,
        Ident::new(ctx.db, name.into()),
        ctx.file,
        Ptr::for_node(node),
    ))
}

/// Something that has a [`Location`] that points to its definition in the
/// source code.
pub trait HasDefinition {
    fn definition(&self, db: &dyn Db) -> Location;
}

pub trait HasName {
    fn name(&self, db: &dyn Db) -> Ident;
}

macro_rules! impl_common {
    ($name:ty) => {
        impl HasDefinition for $name {
            fn definition(&self, db: &dyn Db) -> Location {
                Location::new(self.file(db).path(db), self.ptr(db).range())
            }
        }

        impl HasName for $name {
            fn name(&self, db: &dyn Db) -> Ident {
                <$name>::name(*self, db)
            }
        }
    };
}

#[salsa::tracked]
pub struct PackageMetadata {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TopLevelItemMetadata {
    World(WorldMetadata),
    Interface(InterfaceMetadata),
}

impl HasDefinition for TopLevelItemMetadata {
    fn definition(&self, db: &dyn Db) -> Location {
        match self {
            TopLevelItemMetadata::World(w) => w.definition(db),
            TopLevelItemMetadata::Interface(i) => i.definition(db),
        }
    }
}

impl HasName for TopLevelItemMetadata {
    fn name(&self, db: &dyn Db) -> Ident {
        match self {
            TopLevelItemMetadata::World(w) => w.name(db),
            TopLevelItemMetadata::Interface(i) => i.name(db),
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
    #[id]
    pub name: Ident,
    pub file: SourceFile,
    pub ptr: WorldPtr,
    pub definitions: Vector<TypeDefinitionMetadata>,
}

impl_common!(WorldMetadata);

#[salsa::tracked]
pub struct InterfaceMetadata {
    #[id]
    pub name: Ident,
    pub file: SourceFile,
    pub ptr: InterfacePtr,
    pub items: Vector<InterfaceItemMetadata>,
}

impl_common!(InterfaceMetadata);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InterfaceItemMetadata {
    Func(FuncItemMetadata),
    Type(TypeDefinitionMetadata),
}

impl HasDefinition for InterfaceItemMetadata {
    fn definition(&self, db: &dyn Db) -> Location {
        match self {
            InterfaceItemMetadata::Func(item) => item.definition(db),
            InterfaceItemMetadata::Type(item) => item.definition(db),
        }
    }
}

impl HasName for InterfaceItemMetadata {
    fn name(&self, db: &dyn Db) -> Ident {
        match self {
            InterfaceItemMetadata::Func(item) => item.name(db),
            InterfaceItemMetadata::Type(item) => item.name(db),
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

impl HasName for TypeDefinitionMetadata {
    fn name(&self, db: &dyn Db) -> Ident {
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
    #[id]
    pub name: Ident,
    pub file: SourceFile,
    pub ptr: RecordPtr,
    pub fields: Vector<FieldMetadata>,
}

impl_common!(RecordMetadata);

#[salsa::tracked]
pub struct ResourceMetadata {
    #[id]
    pub name: Ident,
    pub file: SourceFile,
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
    ($( $name:ident => $ptr:ty ),* $(,)?) => {
        $(
            // Note: We use paste to force the macro substitutions to be
            // evaluated before #[salsa::tracked].
            paste::paste! {
                #[salsa::tracked]
                pub struct [< $name >] {
                    #[id]
                    pub name: Ident,
                    pub file: SourceFile,
                    pub ptr: [< $ptr >],
                }
            }

            impl_common!($name);
        )*
    };
}

simple_metadata! {
    FuncItemMetadata => FunctionPtr,
    TypeAliasMetadata => TypeAliasPtr,
    FieldMetadata => RecordFieldPtr,
    MethodMetadata => MethodPtr,
    StaticMethodMetadata => StaticMethodPtr,
}

macro_rules! enum_like_metadata {
    ($( $name:ident => ($ptr:ty, $case:ty) ),* $(,)?) => {
        $(
            // Note: We use paste to force the macro substitutions to be
            // evaluated before #[salsa::tracked].
            paste::paste! {
                #[salsa::tracked]
                pub struct [< $name Metadata >] {
                    #[id]
                    pub name: Ident,
                    pub file: SourceFile,
                    pub ptr: [< $ptr >],
                    pub cases: Vector<[< $name CaseMetadata >]>,
                }

                impl_common!([< $name Metadata >]);

                #[salsa::tracked]
                pub struct [< $name CaseMetadata >] {
                    #[id]
                    pub name: Ident,
                    pub file: SourceFile,
                    pub ptr: [< $case >],
                }

                impl_common!([< $name CaseMetadata >]);
            }
        )*
    };
}

enum_like_metadata! {
    Variant => (VariantPtr, VariantCasePtr),
    Enum => (EnumPtr, EnumCasePtr),
    Flags => (FlagsPtr, FlagsCasePtr),
}

/// An interned identifier.
#[salsa::interned]
pub struct Ident {
    #[return_ref]
    pub raw: Text,
}

fn push_item<T>(db: &dyn Db, items: &mut HashMap<Ident, T>, item: T)
where
    T: HasName + HasDefinition,
{
    let name = item.name(db);

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
    }
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
            T: HasDefinition + HasName + 'static,
        {
            let name = item.name(db).raw(db);
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
            T: HasDefinition + HasName + Copy + 'static,
            C: HasDefinition + HasName + Copy + 'static,
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
        let children = meta
            .definitions(db)
            .iter()
            .map(|&meta| type_definition_node(db, meta))
            .collect();

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
    }
}