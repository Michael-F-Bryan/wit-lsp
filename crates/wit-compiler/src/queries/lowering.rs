use im::{OrdMap, Vector};
use tree_sitter::Node;

use crate::{
    ast::{self, AstNode, HasAttr, HasIdent},
    diagnostics::{Diagnostic, Diagnostics, Location},
    hir,
    queries::{SourceFile, Workspace},
    Db, Text,
};

/// Parse a file and lower it from its [`crate::ast`] representation to the
/// [`crate::hir`] representation.
///
/// Nodes that contain syntactic or semantic errors will be ignored and a
/// corresponding [`Diagnostic`] will be emitted.
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(file = %file.path(db)))]
pub fn lower(db: &dyn Db, _ws: Workspace, file: SourceFile) -> Items {
    let ast = crate::queries::parse(db, file);
    let src = file.contents(db);
    let root = ast.source_file(db);

    let mut ctx = Context::new(db, &src, file);
    ctx.process_file(root);

    ctx.finish(db)
}

struct Context<'a> {
    db: &'a dyn Db,
    src: &'a str,
    file: SourceFile,
    interfaces: OrdMap<Text, hir::Interface>,
    worlds: OrdMap<Text, hir::World>,
    names: OrdMap<Text, NamedItem<'a>>,
}

impl<'a> Context<'a> {
    pub fn new(db: &'a dyn Db, src: &'a str, file: SourceFile) -> Self {
        Context {
            db,
            file,
            interfaces: OrdMap::default(),
            names: OrdMap::new(),
            src,
            worlds: OrdMap::default(),
        }
    }

    fn process_file(&mut self, root: ast::SourceFile<'a>) {
        for item in root.iter_top_level_items().flat_map(named_item) {
            let name = item
                .ident()
                .syntax()
                .utf8_text(self.src.as_bytes())
                .expect("syntax error");

            match item {
                NamedItem::World(node) => {
                    if let Some(world) = self.lower_world(node) {
                        self.push_world(name, world, node);
                    }
                }
                NamedItem::Interface(node) => {
                    if let Some(world) = self.lower_interface(node) {
                        self.push_interface(name, world, node);
                    }
                }
                NamedItem::Use(_) => todo!(),
            }
        }
    }

    fn push_interface(
        &mut self,
        name: &str,
        interface: hir::Interface,
        node: ast::InterfaceItem<'a>,
    ) {
        let name = Text::from(name);

        if self.insert_named(self.db, name.clone(), node.into()) {
            self.interfaces.insert(name, interface);
        }
    }

    fn push_world(&mut self, name: &str, world: hir::World, node: ast::WorldItem<'a>) {
        let name = Text::from(name);

        if self.insert_named(self.db, name.clone(), node.into()) {
            self.worlds.insert(name, world);
        }
    }

    fn insert_named(&mut self, db: &dyn Db, name: Text, node: NamedItem<'a>) -> bool {
        match self.names.entry(name) {
            im::ordmap::Entry::Occupied(entry) => {
                let name = entry.key().clone();
                let original_node = *entry.get();
                drop(entry);
                let location = self.location(db, node.node());
                let original_definition = self.location(db, original_node.node());

                let diag = Diagnostic::duplicate_name(name, location, original_definition);
                Diagnostics::push(db, diag);
                false
            }
            im::ordmap::Entry::Vacant(entry) => {
                entry.insert(node);
                true
            }
        }
    }

    fn location(&self, db: &dyn Db, node: Node<'_>) -> Location {
        Location::new(self.file.path(db), node.range())
    }

    fn finish(self, db: &dyn Db) -> Items {
        let Context {
            interfaces, worlds, ..
        } = self;
        Items::new(db, interfaces, worlds)
    }

    fn lower_world(&mut self, world: ast::WorldItem<'_>) -> Option<hir::World> {
        let docs = world.docs(self.src);
        let items = world
            .iter_items()
            .filter_map(|item| self.lower_world_item(item))
            .collect();

        Some(hir::World::new(self.db, docs, items))
    }

    fn lower_world_item(&mut self, item: ast::WorldItems<'_>) -> Option<hir::WorldItem> {
        if let Some(export) = item.export_item() {
            self.lower_export_item(export).map(hir::WorldItem::Export)
        } else if let Some(import) = item.import_item() {
            self.lower_import_item(import).map(hir::WorldItem::Import)
        } else if let Some(include) = item.include_item() {
            self.lower_include_item(include)
                .map(hir::WorldItem::Include)
        } else {
            None
        }
    }

    fn lower_include_item(&mut self, item: ast::IncludeItem<'_>) -> Option<hir::Include> {
        let _path = item.path()?;

        todo!()
    }

    fn lower_import_item(&mut self, item: ast::ImportItem<'_>) -> Option<hir::ExposableItem> {
        dbg!(item.syntax().to_sexp());

        if let Some(_path) = item.imported_item() {
            todo!()
        } else if let Some(_item) = item.imported_path() {
            todo!()
        } else {
            None
        }
    }

    fn lower_export_item(&mut self, item: ast::ExportItem<'_>) -> Option<hir::ExposableItem> {
        if let Some(_path) = item.exported_item() {
            todo!()
        } else if let Some(_item) = item.exported_item() {
            todo!()
        } else {
            None
        }
    }

    fn lower_interface(&mut self, interface: ast::InterfaceItem<'_>) -> Option<hir::Interface> {
        let docs = interface.docs(self.src);

        let items = interface
            .iter_items()
            .filter_map(|item| self.lower_interface_item(item))
            .collect();

        Some(hir::Interface::new(self.db, docs, items))
    }

    fn lower_interface_item(
        &mut self,
        item: ast::InterfaceItems<'_>,
    ) -> Option<hir::InterfaceItem> {
        if let Some(node) = item.func_item() {
            self.lower_func_item(node).map(hir::InterfaceItem::Func)
        } else if let Some(node) = item.typedef_item() {
            self.lower_type_definition(node)
        } else if let Some(node) = item.use_item() {
            todo!()
        } else {
            None
        }
    }

    fn lower_type_definition(&self, node: ast::TypedefItem<'_>) -> Option<hir::InterfaceItem> {
        todo!()
    }

    fn lower_func_item(&self, node: ast::FuncItem<'_>) -> Option<hir::FuncItem> {
        let docs = node.docs(self.src);
        let name = node
            .identifier()?
            .syntax()
            .utf8_text(self.src.as_bytes())
            .unwrap()
            .into();

        let ty = node.ty()?;
        let params = self.lower_params(ty.params()?)?;
        let return_value = ty
            .result_opt()
            .and_then(|node| self.lower_return_type(node));

        Some(hir::FuncItem {
            docs,
            name,
            params,
            return_value,
        })
    }

    fn lower_params(&self, ty: ast::ParamList<'_>) -> Option<im::Vector<hir::Parameter>> {
        let mut params = Vector::new();

        for param in ty.iter_params() {
            let name = param
                .name()?
                .syntax()
                .utf8_text(self.src.as_bytes())
                .expect("syntax error")
                .into();
            let ty = self.lower_type(param.ty()?)?;
            params.push_back(hir::Parameter { name, ty });
        }

        Some(params)
    }

    fn lower_return_type(&self, node: ast::ResultList<'_>) -> Option<hir::ReturnValue> {
        todo!()
    }

    fn lower_type(&self, ty: ast::Ty<'_>) -> Option<hir::Type> {
        if let Some(builtin) = ty.builtins() {
            self.lower_builtin(builtin)
        } else if let Some(handle) = ty.handle() {
            self.lower_handle(handle)
        } else if let Some(list) = ty.list() {
            self.lower_list(list)
        } else if let Some(option) = ty.option() {
            self.lower_option(option)
        } else if let Some(result) = ty.result() {
            self.lower_result(result)
        } else if let Some(tuple) = ty.tuple() {
            self.lower_tuple(tuple)
        } else if let Some(user_defined_type) = ty.user_defined_type() {
            self.lower_user_defined_type(user_defined_type)
        } else {
            None
        }
    }

    fn lower_builtin(&self, builtin: ast::Builtins<'_>) -> Option<hir::Type> {
        let node = builtin.syntax();
        let name = node.utf8_text(self.src.as_bytes()).expect("syntax error");

        match name {
            "u8" => Some(hir::Type::Builtin(hir::Builtin::U8)),
            "u16" => Some(hir::Type::Builtin(hir::Builtin::U16)),
            "u32" => Some(hir::Type::Builtin(hir::Builtin::U32)),
            "u64" => Some(hir::Type::Builtin(hir::Builtin::U64)),
            "s8" => Some(hir::Type::Builtin(hir::Builtin::I8)),
            "s16" => Some(hir::Type::Builtin(hir::Builtin::I16)),
            "s32" => Some(hir::Type::Builtin(hir::Builtin::I32)),
            "s64" => Some(hir::Type::Builtin(hir::Builtin::I64)),
            "float32" => Some(hir::Type::Builtin(hir::Builtin::Float32)),
            "float64" => Some(hir::Type::Builtin(hir::Builtin::Float64)),
            "char" => Some(hir::Type::Builtin(hir::Builtin::Char)),
            "bool" => Some(hir::Type::Builtin(hir::Builtin::Boolean)),
            "string" => Some(hir::Type::Builtin(hir::Builtin::String)),
            other => {
                unreachable!("Unknown builtin type, \"{other}\" at {}", node.to_sexp())
            }
        }
    }

    fn lower_handle(&self, handle: ast::Handle<'_>) -> Option<hir::Type> {
        todo!()
    }

    fn lower_list(&self, list: ast::List<'_>) -> Option<hir::Type> {
        todo!()
    }

    fn lower_option(&self, option: ast::Option_<'_>) -> Option<hir::Type> {
        todo!()
    }

    fn lower_result(&self, result: ast::Result_<'_>) -> Option<hir::Type> {
        todo!()
    }

    fn lower_tuple(&self, tuple: ast::Tuple<'_>) -> Option<hir::Type> {
        todo!()
    }

    fn lower_user_defined_type(
        &self,
        user_defined_type: ast::UserDefinedType<'_>,
    ) -> Option<hir::Type> {
        todo!()
    }
}

fn named_item(item: ast::TopLevelItem<'_>) -> Option<NamedItem<'_>> {
    let node = item.syntax();

    if node.has_error() {
        return None;
    }

    if let Some(interface) = item.interface_item() {
        Some(NamedItem::Interface(interface))
    } else if let Some(world) = item.world_item() {
        Some(NamedItem::World(world))
    } else if let Some(import) = item.top_level_use_item() {
        Some(NamedItem::Use(import))
    } else {
        unreachable!("Unknown top level item node: {}", node.kind());
    }
}

#[salsa::tracked]
pub struct Items {
    pub interfaces: OrdMap<Text, hir::Interface>,
    pub worlds: OrdMap<Text, hir::World>,
}

#[derive(Debug, Copy, Clone)]
enum NamedItem<'a> {
    World(ast::WorldItem<'a>),
    Interface(ast::InterfaceItem<'a>),
    Use(ast::TopLevelUseItem<'a>),
}

impl<'a> NamedItem<'a> {
    fn node(self) -> tree_sitter::Node<'a> {
        match self {
            NamedItem::World(world) => world.syntax(),
            NamedItem::Interface(interface) => interface.syntax(),
            NamedItem::Use(node) => node.syntax(),
        }
    }

    fn ident(self) -> ast::Identifier<'a> {
        match self {
            NamedItem::World(w) => w.name().expect("syntax error"),
            NamedItem::Interface(i) => i.name().expect("syntax error"),
            NamedItem::Use(u) => {
                if let Some(ident) = u.alias_opt() {
                    return ident;
                }

                let use_path = u.use_path().expect("syntax error");
                if let Some(p) = use_path.fully_qualified_use_path() {
                    p.package()
                        .and_then(|pkg| pkg.identifier())
                        .expect("syntax error")
                } else if let Some(p) = use_path.local_use_path() {
                    p.identifier().expect("syntax error")
                } else {
                    unreachable!("syntax error")
                }
            }
        }
    }
}

impl<'a> From<ast::WorldItem<'a>> for NamedItem<'a> {
    fn from(value: ast::WorldItem<'a>) -> Self {
        NamedItem::World(value)
    }
}

impl<'a> From<ast::InterfaceItem<'a>> for NamedItem<'a> {
    fn from(value: ast::InterfaceItem<'a>) -> Self {
        NamedItem::Interface(value)
    }
}

#[cfg(test)]
mod tests {
    use im::Vector;

    use crate::Compiler;

    use super::*;

    macro_rules! world_assertions {
        (
            $(
                $name:literal : {
                    $(docs: $docs:literal,)?
                    items: [$($items:expr),* $(,)?] $(,)?
                }
            ),*
            $(,)?
        ) => {
            |db: &dyn Db, mut worlds: OrdMap<Text, hir::World>| {
                $(
                    let world = worlds.remove($name)
                        .unwrap_or_else(|| panic!("expected a {} world in {worlds:#?}", stringify!($name)));
                    let mut expected_docs = None;
                    $(
                        expected_docs = Some($docs.into());
                    )*

                    assert_eq!(world.docs(db), expected_docs);
                    assert_eq!(world.items(db), Vector::from_iter([$($items),*]));
                )*

                assert!(worlds.is_empty(), "Leftover worlds: {worlds:#?}");
            }
        };
    }

    macro_rules! interface_assertions {
        (
            $(
                $interface_name:literal : {
                    $(docs: $docs:literal,)?
                    items: [$($items:expr),* $(,)?] $(,)?
                }
            ),*
            $(,)?
        ) => {
            |db: &dyn Db, mut interfaces: OrdMap<Text, hir::Interface>| {
                $(
                    let interface = interfaces.remove($interface_name)
                        .unwrap_or_else(|| panic!("expected a {} interface in {interfaces:#?}", stringify!($interface_name)));
                    let mut expected_docs = None;
                    $(
                        expected_docs = Some($docs.into());
                    )*

                    assert_eq!(interface.docs(db), expected_docs);
                    assert_eq!(interface.items(db), Vector::from_iter([$($items),*]));
                )*

                assert!(interfaces.is_empty(), "Leftover interfaces: {interfaces:#?}");
            }
        };
    }

    macro_rules! lowering_tests {
        (
            $(
                $( #[$meta:meta] )*
                $name:ident : $contents:literal => {
                    interfaces: { $($interface_value:tt)* },
                    worlds: { $($world_value:tt)* } $(,)?
                }
            ),*
            $(,)?
        ) => {
            $(
                #[test]
                #[allow(unused_mut, unused_variables)]
                $( #[$meta] )*
                fn $name() {
                    let db = Compiler::default();

                    let file = SourceFile::new(
                        &db,
                        format!("{}.wit", stringify!($name)).into(),
                        $contents.into(),
                    );
                    let ws = Workspace::new(&db, [(file.path(&db), file)].into_iter().collect());

                    let got = super::lower(&db, ws, file);
                    let diags = super::lower::accumulated::<Diagnostics>(&db, ws, file);

                    assert!(diags.is_empty(), "{diags:#?}");

                    let assert = interface_assertions!($($interface_value)*);
                    assert(&db, got.interfaces(&db));

                    let assert = world_assertions!($($world_value)*);
                    assert(&db, got.worlds(&db));
                }
            )*
        };
    }

    lowering_tests! {
        lower_an_empty_file: "" => { interfaces: {}, worlds: {} },
        empty_interface: "interface empty {}" => {
            interfaces: {
                "empty": {items: []},
            },
            worlds: {},
        },
        interface_with_func: "interface console { log: func(message: string); }" => {
            interfaces: {
                "console": {items: [
                    hir::InterfaceItem::Func(hir::FuncItem {
                        docs: None,
                        name: "log".into(),
                        params: vec![
                            hir::Parameter {
                                name: "message".into(),
                                ty: hir::Type::Builtin(hir::Builtin::String),
                            },
                        ].into(),
                        return_value: None,
                    }),
                ]},
            },
            worlds: {},
        },
        empty_world: "world empty {}" => {
            interfaces: { },
            worlds: {
                "empty": {items: []},
            },
        },
        #[ignore]
        world_with_function_export: "world console { export run: func(); }" => {
            interfaces: { },
            worlds: {
                "console": {items: [
                    hir::WorldItem::Export(hir::ExposableItem::Inline {
                        name: "run".into(),
                        value: hir::ExternType::Function(hir::FuncItem {
                            docs: None,
                            name: "run".into(),
                            params: Vector::new(),
                            return_value: None,
                        }),
                    }),
                ]},
            },
        },
        #[ignore]
        world_with_external_import: "world with-import {
            import wasi:filesystem/filesystem;
        }" => {
            interfaces: { },
            worlds: {
                "with-import": {items: [
                    hir::WorldItem::Import(hir::ExposableItem::Named(hir::Path {
                        namespace: Some("wasi".into()),
                        path: "filesystem/filesystem".into(),
                    }))
                ]},
            },
        },

    }
}
