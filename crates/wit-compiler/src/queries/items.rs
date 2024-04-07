#![allow(clippy::too_many_arguments)]

use either::Either;
use im::{ordmap::Entry, OrdMap, Vector};
use tree_sitter::{Node, Point, Range};

use crate::{
    access::{
        ConstructorPtr, EnumIndex, EnumPtr, FlagsIndex, FlagsPtr, FuncItemIndex, FunctionPtr,
        GetAstNode, Index, InterfaceIndex, InterfacePtr, MethodPtr, Pointer, RawIndex, RecordIndex,
        RecordPtr, ResourceIndex, ResourceMethodIndex, ResourcePtr, StaticMethodPtr,
        StaticResourceMethodIndex, TypeAliasIndex, TypeAliasPtr, VariantIndex, VariantPtr,
        WorldIndex, WorldPtr,
    },
    ast::{self, AstNode, HasIdent},
    diagnostics::{Diagnostic, Diagnostics, Location},
    queries::SourceFile,
    Db, Text,
};

/// Parse a file and walk its AST to find the names of all the items it contains.
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(file = %file.path(db)))]
pub fn file_items(db: &dyn Db, file: SourceFile) -> Items {
    let ast = crate::queries::parse(db, file);
    let src = ast.src(db);
    let root = ast.source_file(db);

    let mut names = NameTable::new(db, file);
    let mut worlds = Vector::new();
    let mut worlds_by_name = OrdMap::new();
    let mut interfaces = Vector::new();
    let mut interfaces_by_name = OrdMap::new();

    for top_level_item in root.iter_top_level_items() {
        let node = top_level_item.syntax();

        if let Some(world) = top_level_item.world_item() {
            if let Some((name, world)) = walk_world(db, world, src, file) {
                if names.insert(name.clone(), node) {
                    let ix = WorldIndex::from_raw(RawIndex::new(worlds.len()));
                    worlds.push_back(world);
                    worlds_by_name.insert(name, ix);
                }
            }
        } else if let Some(interface) = top_level_item.interface_item() {
            if let Some((name, interface)) = walk_interface(db, interface, src, file) {
                if names.insert(name.clone(), node) {
                    let ix = InterfaceIndex::from_raw(RawIndex::new(worlds.len()));
                    interfaces.push_back(interface);
                    interfaces_by_name.insert(name, ix);
                }
            }
        }
    }

    Items::new(db, worlds_by_name, worlds, interfaces_by_name, interfaces)
}

fn walk_world(
    db: &dyn Db,
    world: ast::WorldItem<'_>,
    src: &str,
    file: SourceFile,
) -> Option<(Text, WorldMetadata)> {
    let name = Text::from(world.identifier(src)?);
    let location = WorldPtr::for_node(world);

    let mut names = NameTable::new(db, file);

    let mut builder = TypesBuilder::new(src);

    for item in world.iter_items() {
        if let Some(typedef) = item.typedef_item() {
            builder.process(typedef, &mut names);
        }
    }

    let meta = WorldMetadata::new(db, name.clone(), location, builder.finish());

    Some((name, meta))
}

fn walk_interface(
    db: &dyn Db,
    interface: ast::InterfaceItem<'_>,
    src: &str,
    file: SourceFile,
) -> Option<(Text, InterfaceMetadata)> {
    let name = Text::from(interface.identifier(src)?);
    let location = InterfacePtr::for_node(interface);

    let mut names = NameTable::new(db, file);

    let mut builder = TypesBuilder::new(src);

    for item in interface.iter_items() {
        if let Some(f) = item.func_item() {
            builder.process_function(f, &mut names);
        } else if let Some(typedef) = item.typedef_item() {
            builder.process(typedef, &mut names);
        }
    }

    let meta = InterfaceMetadata::new(db, name.clone(), location, builder.finish());

    Some((name, meta))
}

#[derive(Debug, Default)]
struct TypesBuilder<'a> {
    src: &'a str,
    enums_by_name: OrdMap<Text, EnumIndex>,
    enums: Vector<EnumPtr>,
    flags_by_name: OrdMap<Text, FlagsIndex>,
    flags: Vector<FlagsPtr>,
    functions_by_name: OrdMap<Text, FuncItemIndex>,
    functions: Vector<FunctionPtr>,
    records_by_name: OrdMap<Text, RecordIndex>,
    records: Vector<RecordPtr>,
    resources_by_name: OrdMap<Text, ResourceIndex>,
    resources: Vector<ResourceMetadata>,
    typedefs_by_name: OrdMap<Text, TypeAliasIndex>,
    typedefs: Vector<TypeAliasPtr>,
    variants_by_name: OrdMap<Text, VariantIndex>,
    variants: Vector<VariantPtr>,
}

impl<'a> TypesBuilder<'a> {
    fn new(src: &'a str) -> Self {
        TypesBuilder {
            src,
            ..Default::default()
        }
    }

    fn process(&mut self, typedef: ast::TypedefItem<'a>, names: &mut NameTable<'_, 'a>) {
        if let Some(e) = typedef.enum_item() {
            insert(e, self.src, names, &mut self.enums, &mut self.enums_by_name);
        } else if let Some(v) = typedef.variant_item() {
            insert(
                v,
                self.src,
                names,
                &mut self.variants,
                &mut self.variants_by_name,
            );
        } else if let Some(r) = typedef.record_item() {
            insert(
                r,
                self.src,
                names,
                &mut self.records,
                &mut self.records_by_name,
            );
        } else if let Some(r) = typedef.resource_item() {
            let Some(name) = r.identifier(self.src) else {
                return;
            };

            let meta = process_resource(names.db, names.file, r, name);
            let name = meta.name.clone();

            if names.insert(name.clone(), r.syntax()) {
                let index = Index::from_raw(RawIndex::new(self.resources.len()));
                self.resources.push_back(meta);
                self.resources_by_name.insert(name, index);
            }
        } else if let Some(f) = typedef.flags_item() {
            insert(f, self.src, names, &mut self.flags, &mut self.flags_by_name);
        } else if let Some(t) = typedef.type_item() {
            insert(
                t,
                self.src,
                names,
                &mut self.typedefs,
                &mut self.typedefs_by_name,
            );
        }
    }

    fn finish(self) -> ItemDefinitionMetadata {
        let TypesBuilder {
            enums_by_name,
            enums,
            functions,
            functions_by_name,
            flags_by_name,
            flags,
            records_by_name,
            records,
            resources_by_name,
            resources,
            typedefs_by_name,
            typedefs,
            variants_by_name,
            variants,
            ..
        } = self;
        ItemDefinitionMetadata {
            enums_by_name,
            enums,
            flags_by_name,
            flags,
            functions_by_name,
            functions,
            records_by_name,
            records,
            resources_by_name,
            resources,
            typedefs_by_name,
            typedefs,
            variants_by_name,
            variants,
        }
    }

    fn process_function(&mut self, f: ast::FuncItem<'a>, names: &mut NameTable<'_, 'a>) {
        insert(
            f,
            self.src,
            names,
            &mut self.functions,
            &mut self.functions_by_name,
        );
    }
}

fn process_resource(
    db: &dyn Db,
    file: SourceFile,
    node: ast::ResourceItem<'_>,
    name: &str,
) -> ResourceMetadata {
    let location = ResourcePtr::for_node(node);
    let name = Text::from(name);
    let src = file.contents(db);
    let mut constructor: Option<ConstructorPtr> = None;

    let mut names = NameTable::new(db, file);

    let mut methods = Vector::new();
    let mut static_methods = Vector::new();
    let mut methods_by_name = OrdMap::new();
    let mut static_methods_by_name = OrdMap::new();

    for m in node.iter_methods() {
        if let Some(c) = m.resource_constructor() {
            if let Some(previous) = constructor {
                let location = Location::new(file.path(db), c.range());
                let original_location = Location::new(file.path(db), previous.range());
                let diag = Diagnostic::multiple_constructors(location, original_location);
                Diagnostics::push(db, diag);
                continue;
            }

            constructor = Some(ConstructorPtr::for_node(c));
        } else if let Some(f) = m.func_item() {
            if let Some(name) = f.identifier(src) {
                let name = Text::from(name);
                if names.insert(name.clone(), f.syntax()) {
                    let index = ResourceMethodIndex::from_raw(RawIndex::new(methods.len()));
                    methods_by_name.insert(name, index);
                    methods.push_back(MethodPtr::for_node(f));
                }
            }
        } else if let Some(s) = m.static_method() {
            if let Some(name) = s.identifier(src) {
                let name = Text::from(name);
                if names.insert(name.clone(), m.syntax()) {
                    let index = StaticResourceMethodIndex::from_raw(RawIndex::new(methods.len()));
                    static_methods_by_name.insert(name, index);
                    static_methods.push_back(StaticMethodPtr::for_node(s));
                }
            }
        }
    }

    ResourceMetadata {
        name,
        location,
        constructor,
        methods,
        methods_by_name,
        static_methods,
        static_methods_by_name,
    }
}

fn insert<'tree, Ast, Index, Ptr>(
    node: Ast,
    src: &str,
    names: &mut NameTable<'_, 'tree>,
    items: &mut Vector<Ptr>,
    by_name: &mut OrdMap<Text, Index>,
) where
    Ast: AstNode<'tree> + HasIdent + Copy,
    Index: crate::access::Index,
    Ptr: crate::access::Pointer<Node<'tree> = Ast> + Copy,
{
    let Some(name) = node.identifier(src) else {
        return;
    };

    let ptr = Ptr::for_node(node);

    let name = Text::from(name);

    if names.insert(name.clone(), node.syntax()) {
        let index = Index::from_raw(RawIndex::new(items.len()));
        items.push_back(ptr);
        by_name.insert(name, index);
    }
}

pub(crate) struct NameTable<'db, 'tree> {
    db: &'db dyn Db,
    file: SourceFile,
    names: OrdMap<Text, Node<'tree>>,
}

impl<'db, 'tree> NameTable<'db, 'tree> {
    pub fn new(db: &'db dyn Db, file: SourceFile) -> Self {
        NameTable {
            db,
            file,
            names: OrdMap::new(),
        }
    }

    pub fn insert(&mut self, name: Text, node: Node<'tree>) -> bool {
        match self.names.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(node);
                true
            }
            Entry::Occupied(entry) => {
                let diag = Diagnostic::duplicate_name(
                    entry.key().clone(),
                    Location::new(self.file.path(self.db), node.range()),
                    Location::new(self.file.path(self.db), entry.get().range()),
                );
                Diagnostics::push(self.db, diag);
                false
            }
        }
    }
}

/// Metadata about all items in a file.
#[salsa::tracked]
pub struct Items {
    pub worlds_by_name: OrdMap<Text, WorldIndex>,
    pub worlds: Vector<WorldMetadata>,
    pub interfaces_by_name: OrdMap<Text, InterfaceIndex>,
    pub interfaces: Vector<InterfaceMetadata>,
}

impl Items {
    pub fn iter_worlds(
        &self,
        db: &dyn Db,
    ) -> impl Iterator<Item = (Text, WorldIndex, WorldMetadata)> {
        let worlds = self.worlds(db);

        self.worlds_by_name(db)
            .into_iter()
            .map(move |(name, ix)| (name, ix, worlds[ix.raw().as_usize()]))
    }

    pub fn iter_interfaces(
        &self,
        db: &dyn Db,
    ) -> impl Iterator<Item = (Text, InterfaceIndex, InterfaceMetadata)> {
        let interfaces = self.interfaces(db);

        self.interfaces_by_name(db)
            .into_iter()
            .map(move |(name, ix)| (name, ix, interfaces[ix.raw().as_usize()]))
    }
}

#[salsa::tracked]
impl Items {
    pub fn get_world(&self, db: &dyn Db, index: WorldIndex) -> WorldMetadata {
        let worlds = self.worlds(db);
        worlds[index.raw().as_usize()]
    }

    pub fn get_interface(&self, db: &dyn Db, index: InterfaceIndex) -> InterfaceMetadata {
        let interfaces = self.interfaces(db);
        interfaces[index.raw().as_usize()]
    }

    pub fn enclosing_item(
        &self,
        db: &dyn Db,
        point: Point,
    ) -> Option<Either<WorldIndex, InterfaceIndex>> {
        for (i, meta) in self.worlds(db).into_iter().enumerate() {
            if range_contains(meta.location(db).range(), point) {
                return Some(Either::Left(WorldIndex::from_raw(RawIndex::new(i))));
            }
        }

        for (i, meta) in self.interfaces(db).into_iter().enumerate() {
            if range_contains(meta.location(db).range(), point) {
                return Some(Either::Right(InterfaceIndex::from_raw(RawIndex::new(i))));
            }
        }

        None
    }
}

fn range_contains(range: Range, point: Point) -> bool {
    range.start_point <= point && point <= range.end_point
}

#[salsa::tracked]
pub struct WorldMetadata {
    pub name: Text,
    pub location: WorldPtr,
    pub items: ItemDefinitionMetadata,
}

#[salsa::tracked]
pub struct InterfaceMetadata {
    pub name: Text,
    pub location: InterfacePtr,
    pub items: ItemDefinitionMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemDefinitionMetadata {
    pub enums_by_name: OrdMap<Text, EnumIndex>,
    pub enums: Vector<EnumPtr>,
    pub flags_by_name: OrdMap<Text, FlagsIndex>,
    pub flags: Vector<FlagsPtr>,
    pub functions_by_name: OrdMap<Text, FuncItemIndex>,
    pub functions: Vector<FunctionPtr>,
    pub records_by_name: OrdMap<Text, RecordIndex>,
    pub records: Vector<RecordPtr>,
    pub resources_by_name: OrdMap<Text, ResourceIndex>,
    pub resources: Vector<ResourceMetadata>,
    pub typedefs_by_name: OrdMap<Text, TypeAliasIndex>,
    pub typedefs: Vector<TypeAliasPtr>,
    pub variants_by_name: OrdMap<Text, VariantIndex>,
    pub variants: Vector<VariantPtr>,
}

impl ItemDefinitionMetadata {
    pub fn iter_enums(&self) -> impl Iterator<Item = EnumIndex> + '_ {
        iter(&self.enums)
    }

    pub fn iter_flags(&self) -> impl Iterator<Item = FlagsIndex> + '_ {
        iter(&self.flags)
    }

    pub fn iter_functions(&self) -> impl Iterator<Item = FuncItemIndex> + '_ {
        iter(&self.functions)
    }

    pub fn iter_records(&self) -> impl Iterator<Item = RecordIndex> + '_ {
        iter(&self.records)
    }

    pub fn iter_resources(&self) -> impl Iterator<Item = ResourceIndex> + '_ {
        iter(&self.resources)
    }

    pub fn iter_typedefs(&self) -> impl Iterator<Item = TypeAliasIndex> + '_ {
        iter(&self.typedefs)
    }

    pub fn iter_variants(&self) -> impl Iterator<Item = VariantIndex> + '_ {
        iter(&self.variants)
    }

    pub fn names(&self) -> impl Iterator<Item = &Text> {
        self.enums_by_name
            .keys()
            .chain(self.flags_by_name.keys())
            .chain(self.functions_by_name.keys())
            .chain(self.records_by_name.keys())
            .chain(self.resources_by_name.keys())
            .chain(self.typedefs_by_name.keys())
            .chain(self.variants_by_name.keys())
    }
}

fn iter<Ix, Meta>(items: &Vector<Meta>) -> impl Iterator<Item = Ix> + '_
where
    Ix: Copy + Index,
    Meta: Clone,
{
    (0..items.len()).map(|ix| Ix::from_raw(RawIndex::new(ix)))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceMetadata {
    pub name: Text,
    pub location: ResourcePtr,
    pub constructor: Option<ConstructorPtr>,
    pub methods_by_name: OrdMap<Text, ResourceMethodIndex>,
    pub methods: Vector<MethodPtr>,
    pub static_methods_by_name: OrdMap<Text, StaticResourceMethodIndex>,
    pub static_methods: Vector<StaticMethodPtr>,
}

impl ResourceMetadata {
    pub fn iter_methods(&self) -> impl Iterator<Item = (&Text, ResourceMethodIndex, MethodPtr)> {
        self.methods_by_name
            .iter()
            .map(|(name, ix)| (name, *ix, self.methods[ix.raw().as_usize()]))
    }

    pub fn iter_static_methods(
        &self,
    ) -> impl Iterator<Item = (&Text, StaticResourceMethodIndex, StaticMethodPtr)> {
        self.static_methods_by_name
            .iter()
            .map(|(name, ix)| (name, *ix, self.static_methods[ix.raw().as_usize()]))
    }
}

impl GetAstNode for ResourceMetadata {
    type Node<'tree> = ast::ResourceItem<'tree>;

    fn ast_node(self, tree: &crate::Tree) -> Self::Node<'_> {
        self.location.ast_node(tree)
    }
}
