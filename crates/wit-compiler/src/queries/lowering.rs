use im::{OrdMap, Vector};

use crate::{
    ast::{self, AstNode, HasAttr, HasIdent},
    diagnostics::{Diagnostic, Diagnostics, Location},
    hir,
    pointer::{
        EnumIndex, FlagsIndex, FuncItemIndex, GetByIndex, InterfaceIndex, Pointer, RecordIndex,
        ResourceIndex, ScopeIndex, TypeAliasIndex, VariantIndex, WorldIndex,
    },
    queries::{SourceFile, Workspace},
    Db, Text, Tree,
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
    let root = ast.source_file(db);
    let src = ast.src(db);

    let items = crate::queries::file_items(db, file);

    let mut worlds = OrdMap::new();
    let mut interfaces = OrdMap::new();

    for index in items.worlds_by_name(db).values().copied() {
        if let Some(world) = lower_world(db, file, index) {
            worlds.insert(index, world);
        }
    }

    for index in items.interfaces_by_name(db).values().copied() {
        if let Some(interface) = lower_interface(db, file, index) {
            interfaces.insert(index, interface);
        }
    }

    hir::Package {
        decl: root.package_opt().and_then(|d| lower_package_decl(d, src)),
        interfaces,
        worlds,
    }
}

fn lower_package_decl(node: ast::PackageDecl, src: &str) -> Option<hir::PackageDeclaration> {
    let docs = node.docs(src);

    let package_name = node.fully_qualified_package_name()?;
    let package = package_name.package()?.identifier()?.value(src).into();
    let path = package_name
        .path()?
        .iter_identifiers()
        .map(|p| p.value(src).into())
        .collect();
    let version = package_name.version_opt().map(|s| s.value(src).into());

    Some(hir::PackageDeclaration {
        docs,
        package,
        path,
        version,
    })
}

#[salsa::tracked]
pub(crate) fn lower_world(db: &dyn Db, file: SourceFile, index: WorldIndex) -> Option<hir::World> {
    let ast = crate::queries::parse(db, file);
    let tree = ast.tree(db);
    let items = crate::queries::file_items(db, file);

    let meta = items.get_by_index(db, index);
    let _node = meta.location(db).lookup(tree);
    let _item_definitions = meta.items(db);

    todo!();
}

#[salsa::tracked]
pub(crate) fn lower_interface(
    db: &dyn Db,
    file: SourceFile,
    index: InterfaceIndex,
) -> Option<hir::Interface> {
    let ast = crate::queries::parse(db, file);
    let tree = ast.tree(db);
    let items = crate::queries::file_items(db, file);

    let meta = items.get_by_index(db, index);
    let node = meta.location(db).lookup(tree);
    let item_definitions = meta.items(db);

    let mut items = Vector::new();
    let scope = ScopeIndex::Interface(index);

    for ix in item_definitions.iter_enums() {
        if let Some(item) = lower_enum(db, file, scope, ix) {
            items.push_back(item.into());
        }
    }

    for ix in item_definitions.iter_flags() {
        if let Some(item) = lower_flags(db, file, scope, ix) {
            items.push_back(item.into());
        }
    }

    for ix in item_definitions.iter_functions() {
        if let Some(item) = lower_func_item(db, file, index, ix) {
            items.push_back(item.into());
        }
    }

    for ix in item_definitions.iter_records() {
        if let Some(item) = lower_record(db, file, scope, ix) {
            items.push_back(item.into());
        }
    }

    for ix in item_definitions.iter_resources() {
        if let Some(item) = lower_resource(db, file, scope, ix) {
            items.push_back(item.into());
        }
    }

    for ix in item_definitions.iter_typedefs() {
        if let Some(item) = lower_type_alias(db, file, scope, ix) {
            items.push_back(item.into());
        }
    }

    for ix in item_definitions.iter_variants() {
        if let Some(item) = lower_variant(db, file, scope, ix) {
            items.push_back(item.into());
        }
    }

    Some(hir::Interface {
        name: meta.name(db),
        docs: node.docs(ast.src(db)),
        items,
    })
}

#[salsa::tracked]
pub(crate) fn lower_enum(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    index: EnumIndex,
) -> Option<hir::Enum> {
    let ast = crate::queries::parse(db, file);
    let ptr = match scope {
        ScopeIndex::Interface(interface) => file.get_by_index(db, (interface, index)),
        ScopeIndex::World(world) => file.get_by_index(db, (world, index)),
    };
    let src = ast.src(db);

    let tree = ast.tree(db);
    let node = ptr.lookup(tree);
    let name = node.identifier(src)?.into();

    let cases = node
        .iter_cases()
        .filter_map(|case| lower_enum_case(case, src))
        .collect();

    Some(hir::Enum {
        name,
        docs: node.docs(src),
        index,
        cases,
    })
}

fn lower_enum_case(case: ast::EnumCase<'_>, src: &str) -> Option<hir::EnumCase> {
    Some(hir::EnumCase {
        docs: case.docs(src),
        name: case.identifier(src)?.into(),
    })
}

#[salsa::tracked]
pub(crate) fn lower_flags(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    index: FlagsIndex,
) -> Option<hir::Flags> {
    let ast = crate::queries::parse(db, file);
    let ptr = match scope {
        ScopeIndex::Interface(interface) => file.get_by_index(db, (interface, index)),
        ScopeIndex::World(world) => file.get_by_index(db, (world, index)),
    };
    let src = ast.src(db);

    let tree = ast.tree(db);
    let node = ptr.lookup(tree);
    let name = node.identifier(src)?.into();

    let cases = node
        .iter_cases()
        .filter_map(|case| lower_flags_case(case, src))
        .collect();

    Some(hir::Flags {
        name,
        docs: node.docs(src),
        index,
        cases,
    })
}

fn lower_flags_case(case: ast::FlagsCase<'_>, src: &str) -> Option<hir::FlagsCase> {
    Some(hir::FlagsCase {
        docs: case.docs(src),
        name: case.identifier(src)?.into(),
    })
}

#[salsa::tracked]
pub(crate) fn lower_func_item(
    db: &dyn Db,
    file: SourceFile,
    interface: InterfaceIndex,
    index: FuncItemIndex,
) -> Option<hir::FuncItem> {
    let ast = crate::queries::parse(db, file);
    let tree = ast.tree(db);
    let node = file.get_by_index(db, (interface, index)).lookup(tree);
    let src = ast.src(db);

    let name = node.identifier(src)?.into();
    let func_type = node.ty()?;

    let mut params = Vector::new();

    for param in func_type.params()?.iter_params() {
        let param = lower_param(db, src, interface.into(), tree, param)?;
        params.push_back(param);
    }

    let return_value = func_type
        .result_opt()
        .and_then(|r| lower_return_value(db, src, file, interface.into(), tree, r));

    Some(hir::FuncItem {
        index,
        name,
        docs: node.docs(src),
        params,
        return_value,
    })
}

fn lower_param(
    db: &dyn Db,
    src: &str,
    scope: ScopeIndex,
    tree: &Tree,
    param: ast::NamedType<'_>,
) -> Option<hir::Parameter> {
    let name = param.identifier(src)?.into();
    let ty = param.ty()?;
    let ty = resolve_type(db, src, scope, tree, ty)?;

    Some(hir::Parameter { name, ty })
}

fn lower_return_value(
    db: &dyn Db,
    src: &str,
    file: SourceFile,
    scope: ScopeIndex,
    tree: &Tree,
    ret: ast::ResultList<'_>,
) -> Option<hir::ReturnValue> {
    if let Some(ty) = ret.ty_opt() {
        let ty = resolve_type(db, src, scope, tree, ty)?;
        Some(hir::ReturnValue::Single(ty))
    } else if let Some(list) = ret.named_result_list_opt() {
        let mut return_types = OrdMap::new();

        for pair in list.iter_named_types() {
            let name = Text::from(pair.identifier(src)?);
            let ty_node = pair.ty()?;
            let ty = resolve_type(db, src, scope, tree, ty_node).unwrap_or(hir::Type::Error);

            match return_types.entry(name) {
                im::ordmap::Entry::Vacant(entry) => {
                    entry.insert((ty, ty_node.range()));
                }
                im::ordmap::Entry::Occupied(entry) => {
                    let path = file.path(db);
                    let location = Location::new(path, pair.range());
                    let original_definition = Location::new(path, entry.get().1);

                    let diag = Diagnostic::duplicate_name(
                        entry.key().clone(),
                        location,
                        original_definition,
                    );
                    Diagnostics::push(db, diag);
                }
            }
        }

        let return_types = return_types.into_iter().map(|(k, (v, _))| (k, v)).collect();
        Some(hir::ReturnValue::Named(return_types))
    } else {
        None
    }
}

#[salsa::tracked]
pub(crate) fn lower_record(
    _db: &dyn Db,
    _file: SourceFile,
    _scope: ScopeIndex,
    _ix: RecordIndex,
) -> Option<hir::Record> {
    todo!();
}

#[salsa::tracked]
pub(crate) fn lower_resource(
    _db: &dyn Db,
    _file: SourceFile,
    _scope: ScopeIndex,
    _ix: ResourceIndex,
) -> Option<hir::Resource> {
    todo!();
}

#[salsa::tracked]
pub(crate) fn lower_type_alias(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    index: TypeAliasIndex,
) -> Option<hir::TypeAlias> {
    let ast = crate::queries::parse(db, file);
    let ptr = match scope {
        ScopeIndex::Interface(interface) => file.get_by_index(db, (interface, index)),
        ScopeIndex::World(world) => file.get_by_index(db, (world, index)),
    };
    let src = ast.src(db);

    let tree = ast.tree(db);
    let node = ptr.lookup(tree);
    let name = node.identifier(src)?.into();

    let ty = node.ty()?;
    let ty = resolve_type(db, src, scope, tree, ty)?;

    Some(hir::TypeAlias {
        name,
        docs: node.docs(src),
        index,
        ty,
    })
}

#[salsa::tracked]
pub(crate) fn lower_variant(
    _db: &dyn Db,
    _file: SourceFile,
    _scope: ScopeIndex,
    _ix: VariantIndex,
) -> Option<hir::Variant> {
    todo!();
}

fn resolve_type(
    _db: &dyn Db,
    src: &str,
    _scope: ScopeIndex,
    _tree: &Tree,
    ty: ast::Ty<'_>,
) -> Option<hir::Type> {
    let resolver = TypeResolver { src };
    resolver.resolve_type(ty)
}

struct TypeResolver<'a> {
    src: &'a str,
}

impl<'a> TypeResolver<'a> {
    fn resolve_type(&self, ty: ast::Ty<'_>) -> Option<hir::Type> {
        if let Some(builtin) = ty.builtins() {
            self.resolve_builtin(builtin)
        } else if let Some(handle) = ty.handle() {
            self.resolve_handle(handle)
        } else if let Some(list) = ty.list() {
            self.resolve_list(list)
        } else if let Some(option) = ty.option() {
            self.resolve_option(option)
        } else if let Some(result) = ty.result() {
            self.resolve_result(result)
        } else if let Some(tuple) = ty.tuple() {
            self.resolve_tuple(tuple)
        } else if let Some(user_defined_type) = ty.user_defined_type() {
            self.resolve_user_defined_type(user_defined_type)
        } else {
            None
        }
    }

    fn resolve_option(&self, option: ast::Option_<'_>) -> Option<hir::Type> {
        let element_type = option.ty()?;
        let element_type = self.resolve_type(element_type)?;
        Some(hir::Type::Option(Box::new(element_type)))
    }

    fn resolve_result(&self, result: ast::Result_<'_>) -> Option<hir::Type> {
        let ok = result.ok_opt().and_then(|ty| self.resolve_type(ty));
        let err = result.err_opt().and_then(|ty| self.resolve_type(ty));

        Some(hir::Type::Result {
            ok: ok.map(Box::new),
            err: err.map(Box::new),
        })
    }

    fn resolve_tuple(&self, tuple: ast::Tuple<'_>) -> Option<hir::Type> {
        let mut types = Vector::new();

        for ty in tuple.iter_tys() {
            let ty = self.resolve_type(ty).unwrap_or(hir::Type::Error);
            types.push_back(ty);
        }

        Some(hir::Type::Tuple(types))
    }

    fn resolve_user_defined_type(
        &self,
        _user_defined_type: ast::UserDefinedType<'_>,
    ) -> Option<hir::Type> {
        todo!()
    }

    fn resolve_list(&self, list: ast::List<'_>) -> Option<hir::Type> {
        let element_type = list.ty()?;
        let element_type = self.resolve_type(element_type)?;
        Some(hir::Type::List(Box::new(element_type)))
    }

    fn resolve_handle(&self, _handle: ast::Handle<'_>) -> Option<hir::Type> {
        todo!()
    }

    fn resolve_builtin(&self, builtin: ast::Builtins<'_>) -> Option<hir::Type> {
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
}

#[cfg(test)]
mod tests {
    use crate::{diagnostics::Diagnostics, Compiler};

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
        func_with_no_arguments: "interface i { f: func(); }",
        func_with_one_argument: "interface i { f: func(message: string); }",
        func_with_multiple_arguments: "interface i { f: func(first: u32, second: string, third: list<bool>); }",
        func_with_return_value: "interface i { f: func() -> bool; }",
        func_with_named_return_values: "interface i { f: func() -> (a: u32, b: option<string>); }",
        interface_with_builtin_type: "interface i { type x = u32; }",
        empty_enum: "interface i { enum empty {} }",
        enum_with_one_element: "interface i { enum foo { first } }",
        enum_with_multiple_elements: "interface i {
            /// This is an enum.
            enum foo {
                /// first case.
                first,
                /// Second case.
                /// and another
                /// doc-comment.
                second,
             }
         }",
        empty_flags: "interface i { flags empty {} }",
        flags_with_one_element: "interface i { flags foo { first } }",
        flags_with_multiple_elements: "interface i {
            /// This is a flags.
            flags foo {
                /// first case.
                first,
                /// Second case.
                /// and another
                /// doc-comment.
                second,
             }
         }",
        empty_tuple: "interface i { type x = tuple<>; }",
        tuple_with_single_element: "interface i { type x = tuple<string>; }",
        tuple_with_multiple_elements: "interface i { type x = tuple<string, bool, u32>; }",
        result_with_ok_and_error: "interface i { type x = result<bool, string>; }",
        result_with_empty_ok: "interface i { type x = result<_>; }",
        bare_result: "interface i { type x = result; }",
        result_with_just_error: "interface i { type x = result<_, string>; }",
        list: "interface i { type x = list<u32>; }",
        option: "interface i { type x = option<u32>; }",

        #[ignore]
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
