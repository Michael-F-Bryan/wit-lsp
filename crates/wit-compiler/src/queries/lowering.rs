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
pub fn lower(db: &dyn Db, _ws: Workspace, file: SourceFile) -> hir::Package {
    let ast = crate::queries::parse(db, file);
    let src = file.contents(db);
    let root = ast.source_file(db);

    let mut ctx = State::new(db, src, file);
    ctx.process_file(root);

    ctx.finish()
}

/// A big bundle of state used while lowering.
struct State<'a> {
    db: &'a dyn Db,
    src: &'a str,
    file: SourceFile,
    decl: Option<hir::PackageDeclaration>,
    interfaces: OrdMap<Text, hir::Interface>,
    worlds: OrdMap<Text, hir::World>,
    names: OrdMap<Text, NamedItem<'a>>,
}

impl<'a> State<'a> {
    pub fn new(db: &'a dyn Db, src: &'a str, file: SourceFile) -> Self {
        State {
            db,
            file,
            decl: None,
            interfaces: OrdMap::default(),
            names: OrdMap::new(),
            src,
            worlds: OrdMap::default(),
        }
    }

    fn process_file(&mut self, root: ast::SourceFile<'a>) {
        self.decl = self.lower_package_decl(root);

        for item in root.iter_top_level_items().flat_map(named_item) {
            let name = item.ident().value(self.src);

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

    fn finish(self) -> hir::Package {
        let State {
            interfaces,
            worlds,
            decl,
            ..
        } = self;

        hir::Package {
            decl,
            interfaces: interfaces.into_iter().map(|(_, i)| i).collect(),
            worlds: worlds.into_iter().map(|(_, w)| w).collect(),
        }
    }

    fn lower_world(&mut self, world: ast::WorldItem<'_>) -> Option<hir::World> {
        let docs = world.docs(self.src);
        let name = world.identifier(self.src)?.into();
        let items = world
            .iter_items()
            .filter_map(|item| self.lower_world_item(item))
            .collect();

        Some(hir::World { name, docs, items })
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
        let name = interface.identifier(self.src)?.into();

        let items = interface
            .iter_items()
            .filter_map(|item| self.lower_interface_item(item))
            .collect();

        Some(hir::Interface { name, docs, items })
    }

    fn lower_interface_item(
        &mut self,
        item: ast::InterfaceItems<'_>,
    ) -> Option<hir::InterfaceItem> {
        if let Some(node) = item.func_item() {
            self.lower_func_item(node).map(hir::InterfaceItem::Func)
        } else if let Some(node) = item.typedef_item() {
            self.lower_type_definition(node)
        } else if let Some(_node) = item.use_item() {
            todo!()
        } else {
            None
        }
    }

    fn lower_type_definition(&self, node: ast::TypedefItem<'_>) -> Option<hir::InterfaceItem> {
        if let Some(type_item) = node.type_item() {
            self.lower_type_item(type_item)
        } else {
            None
        }
    }

    fn lower_func_item(&self, node: ast::FuncItem<'_>) -> Option<hir::FuncItem> {
        let docs = node.docs(self.src);
        let name = node.identifier(self.src)?.into();

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
            let name = param.name()?.value(self.src).into();
            let ty = self.lower_type(param.ty()?)?;
            params.push_back(hir::Parameter { name, ty });
        }

        Some(params)
    }

    fn lower_return_type(&self, _node: ast::ResultList<'_>) -> Option<hir::ReturnValue> {
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
        let name = builtin.value(self.src);

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
                unreachable!(
                    "Unknown builtin type, \"{other}\" at {}",
                    builtin.syntax().to_sexp()
                )
            }
        }
    }

    fn lower_handle(&self, handle: ast::Handle<'_>) -> Option<hir::Type> {
        if let Some(borrowed) = handle.borrowed_handle() {
            let _name = borrowed.name()?.value(self.src);
            todo!()
        } else if let Some(owned) = handle.owned_handle() {
            let _name = owned.name()?.value(self.src);
            todo!()
        } else {
            None
        }
    }

    fn lower_list(&self, list: ast::List<'_>) -> Option<hir::Type> {
        let element_type = list.ty().and_then(|ty| self.lower_type(ty))?;
        Some(hir::Type::List(Box::new(element_type)))
    }

    fn lower_option(&self, option: ast::Option_<'_>) -> Option<hir::Type> {
        let element_type = option.ty().and_then(|ty| self.lower_type(ty))?;
        Some(hir::Type::Option(Box::new(element_type)))
    }

    fn lower_result(&self, result: ast::Result_<'_>) -> Option<hir::Type> {
        let ok = result
            .ok_opt()
            .and_then(|ty| self.lower_type(ty))
            .map(Box::new);
        let err = result
            .err_opt()
            .and_then(|ty| self.lower_type(ty))
            .map(Box::new);
        Some(hir::Type::Result { ok, err })
    }

    fn lower_tuple(&self, tuple: ast::Tuple<'_>) -> Option<hir::Type> {
        let mut element_types = Vector::new();

        for ty in tuple.iter_tys() {
            element_types.push_back(self.lower_type(ty)?);
        }

        Some(hir::Type::Tuple(element_types))
    }

    fn lower_user_defined_type(
        &self,
        user_defined_type: ast::UserDefinedType<'_>,
    ) -> Option<hir::Type> {
        let text = user_defined_type.value(self.src);
        eprintln!("{:?} => {}", text, user_defined_type.syntax().to_sexp());
        todo!()
    }

    fn lower_type_item(&self, type_item: ast::TypeItem<'_>) -> Option<hir::InterfaceItem> {
        let docs = type_item.docs(self.src);
        let name = type_item.identifier(self.src)?.into();
        let ty = self.lower_type(type_item.ty()?)?;

        Some(hir::InterfaceItem::TypeAlias(hir::TypeAlias {
            docs,
            name,
            ty,
        }))
    }

    fn lower_package_decl(&self, root: ast::SourceFile<'_>) -> Option<hir::PackageDeclaration> {
        let node = root.package_opt()?;
        let docs = node.docs(self.src);

        let package_name = node.fully_qualified_package_name()?;
        let package = package_name.package()?.identifier()?.value(self.src).into();
        let path = package_name
            .path()?
            .iter_identifiers()
            .map(|p| p.value(self.src).into())
            .collect();
        let version = package_name.version_opt().map(|s| s.value(self.src).into());

        Some(hir::PackageDeclaration {
            docs,
            package,
            path,
            version,
        })
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
    use crate::Compiler;

    use super::*;

    macro_rules! lowering_tests {
        (
            $(
                $( #[$meta:meta] )*
                $name:ident : $contents:literal
            ),* $(,)?
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

                    let ast = crate::queries::parse(&db, file);
                    eprintln!("{}", ast.root_node(&db).to_sexp());

                    let got = super::lower(&db, ws, file);
                    let diags = super::lower::accumulated::<Diagnostics>(&db, ws, file);

                    assert!(diags.is_empty(), "{diags:#?}");

                    let mut settings = insta::Settings::clone_current();

                    #[derive(serde::Serialize)]
                    struct Info<'a> {
                        src: &'a str,
                        ast: String,
                    }
                    settings.set_info(&Info {
                        src: $contents,
                        ast: ast.root_node(&db).to_sexp(),
                    });
                    settings.set_omit_expression(true);

                    settings.bind(|| insta::assert_debug_snapshot!(got));
                }
            )*
        };
    }

    macro_rules! lowering_error_tests {
        (
            $(
                $( #[$meta:meta] )*
                $name:ident : $contents:literal
            ),* $(,)?
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

                    let ast = crate::queries::parse(&db, file);
                    let diags = super::lower::accumulated::<Diagnostics>(&db, ws, file);

                    assert_ne!(diags.len(), 0, "No diagnostics emitted");

                    #[derive(serde::Serialize)]
                    struct Info<'a> {
                        src: &'a str,
                        ast: String,
                    }

                    let mut settings = insta::Settings::clone_current();
                    settings.set_info(&Info {
                        src: $contents,
                        ast: ast.root_node(&db).to_sexp(),
                    });
                    settings.set_omit_expression(true);

                    settings.bind(|| insta::assert_debug_snapshot!(diags));
                }
            )*
        };
    }

    lowering_tests! {
        lower_an_empty_file: "",
        lower_package_with_docs: "/// This is a package.\npackage wasi:filesystem@1.2.3;",
        empty_interface: "interface empty {}" ,
        interface_with_func: "interface console { log: func(message: string); }",
        interface_with_builtin_type: "interface i { type x = u32; }",
        empty_tuple: "interface i { type x = tuple<>; }",
        tuple_with_single_element: "interface i { type x = tuple<string>; }",
        tuple_with_multiple_elements: "interface i { type x = tuple<string, bool, u32>; }",
        result_with_ok_and_error: "interface i { type x = result<bool, string>; }",
        result_with_empty_ok: "interface i { type x = result<_>; }",
        bare_result: "interface i { type x = result; }",
        result_with_just_error: "interface i { type x = result<_, string>; }",
        list: "interface i { type x = list<u32>; }",
        option: "interface i { type x = option<u32>; }",

        empty_world: "world empty {}",
        #[ignore]
        world_with_function_export: "world console { export run: func(); }",
        #[ignore]
        world_with_external_export: "world with-import {
            export wasi:filesystem/filesystem;
        }",
        #[ignore]
        world_with_external_import: "world with-import {
            import wasi:filesystem/filesystem;
        }"
    }

    lowering_error_tests! {
        syntax_errors_are_emitted: "#$",
        #[ignore]
        refer_to_unknown_type: "interface i { type x = this-does-not-exist; }",
        #[ignore]
        recursive_records_are_not_allowed: "interface i { record recursive { inner: recursive } }",
        #[ignore]
        reference_cycles_are_not_allowed: "interface i {
            record first { second: second }
            record second { first: first }
        }",
        #[ignore]
        duplicate_identifiers: "interface i { record foo {} variant foo {} }",
    }
}
