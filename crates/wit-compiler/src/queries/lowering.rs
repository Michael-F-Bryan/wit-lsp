use im::{OrdMap, Vector};

use crate::{
    access::{
        AnyFuncItemIndex, EnumIndex, FlagsIndex, FuncItemIndex, GetAstNode, GetByIndex,
        InterfaceIndex, RecordIndex, ResourceIndex, ScopeIndex, TypeAliasIndex, VariantIndex,
        WorldIndex,
    },
    ast::{self, AstNode, HasAttr, HasIdent},
    diagnostics::{Diagnostic, Diagnostics, Location},
    hir,
    queries::{items::NameTable, SourceFile, Workspace},
    Db, Text,
};

/// Parse a file and lower it from its [`crate::ast`] representation to the
/// [`crate::hir`] representation.
///
/// Nodes that contain syntactic or semantic errors will be ignored and a
/// corresponding [`Diagnostic`] will be emitted.
#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(file = %file.path(db).raw_path(db)))]
pub fn lower(db: &dyn Db, _ws: Workspace, file: SourceFile) -> hir::Package {
    let ast = crate::queries::parse(db, file);
    let root = ast.source_file(db);
    let src = ast.src(db);

    let items = crate::queries::file_items(db, file);

    let mut worlds = OrdMap::new();
    let mut interfaces = OrdMap::new();

    for index in items.worlds_by_name(db).values().copied() {
        let world = lower_world(db, file, index);
        worlds.insert(index, world);
    }

    for index in items.interfaces_by_name(db).values().copied() {
        let interface = lower_interface(db, file, index);
        interfaces.insert(index, interface);
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
pub(crate) fn lower_world(db: &dyn Db, file: SourceFile, index: WorldIndex) -> hir::World {
    let ast = crate::queries::parse(db, file);
    let tree = ast.tree(db);
    let items = crate::queries::file_items(db, file);

    let meta = items.get_by_index(db, index);
    let node = meta.location(db).ast_node(tree);

    let src = ast.src(db);
    let name = meta.name(db);

    let mut world_items = Vector::new();

    for item in node.iter_items() {
        if let Some(item) = lower_world_item(db, item) {
            world_items.push_back(item);
        }
    }

    hir::World {
        name,
        docs: node.docs(src),
        items: world_items,
    }
}

fn lower_world_item(_db: &dyn Db, item: ast::WorldItems<'_>) -> Option<hir::WorldItem> {
    if let Some(_ty) = item.typedef_item() {
        todo!()
    } else if let Some(_export) = item.export_item() {
        todo!()
    } else if let Some(_import) = item.import_item() {
        todo!()
    } else if let Some(_include) = item.include_item() {
        todo!()
    } else if let Some(_use) = item.use_item() {
        todo!()
    } else {
        None
    }
}

#[salsa::tracked]
pub(crate) fn lower_interface(
    db: &dyn Db,
    file: SourceFile,
    index: InterfaceIndex,
) -> hir::Interface {
    let ast = crate::queries::parse(db, file);
    let tree = ast.tree(db);
    let items = crate::queries::file_items(db, file);

    let meta = items.get_by_index(db, index);
    let node = meta.location(db).ast_node(tree);
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

    let name = meta.name(db);
    let docs = node.docs(ast.src(db));
    hir::Interface { name, docs, items }
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
    let node = ptr.ast_node(tree);
    let name = node.identifier(src)?.into();
    let mut names = NameTable::new(db, file);

    let mut cases = Vector::new();

    for c in node.iter_cases() {
        if let Some(case) = lower_enum_case(c, src) {
            if names.insert(case.name.clone(), c.syntax()) {
                cases.push_back(case);
            }
        }
    }

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
    let node = ptr.ast_node(tree);
    let name = node.identifier(src)?.into();
    let mut names = NameTable::new(db, file);

    let mut cases = Vector::new();

    for c in node.iter_cases() {
        if let Some(case) = lower_flags_case(c, src) {
            if names.insert(case.name.clone(), c.syntax()) {
                cases.push_back(case);
            }
        }
    }

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
    let node = file.get_by_index(db, (interface, index)).ast_node(tree);
    let src = ast.src(db);

    let name = node.identifier(src)?.into();
    let func_type = node.ty()?;

    let mut params = Vector::new();

    for param in func_type.params()?.iter_params() {
        let param = lower_param(db, file, interface.into(), param)?;
        params.push_back(param);
    }

    let return_value = func_type
        .result_opt()
        .and_then(|r| lower_return_value(db, file, interface.into(), r));

    Some(hir::FuncItem {
        index: AnyFuncItemIndex::TopLevel(interface, index),
        name,
        docs: node.docs(src),
        params,
        return_value,
    })
}

fn lower_param(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    param: ast::NamedType<'_>,
) -> Option<hir::Parameter> {
    let src = file.contents(db);
    let name = param.identifier(src)?.into();
    let ty = param.ty()?;
    let ty = resolve_type(db, file, scope, ty)?;

    Some(hir::Parameter { name, ty })
}

fn lower_return_value(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    ret: ast::ResultList<'_>,
) -> Option<hir::ReturnValue> {
    let src = file.contents(db);

    if let Some(ty) = ret.ty_opt() {
        let ty = resolve_type(db, file, scope, ty)?;
        Some(hir::ReturnValue::Single(ty))
    } else if let Some(list) = ret.named_result_list_opt() {
        let mut return_types = OrdMap::new();

        for pair in list.iter_named_types() {
            let name = Text::from(pair.identifier(src)?);
            let ty_node = pair.ty()?;
            let ty = resolve_type(db, file, scope, ty_node).unwrap_or(hir::Type::Error);

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
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    index: RecordIndex,
) -> Option<hir::Record> {
    let ast = crate::queries::parse(db, file);
    let ptr = match scope {
        ScopeIndex::Interface(interface) => file.get_by_index(db, (interface, index)),
        ScopeIndex::World(world) => file.get_by_index(db, (world, index)),
    };
    let tree = ast.tree(db);
    let src = ast.src(db);

    let node = ptr.ast_node(tree);
    let name = node.identifier(src)?.into();

    let mut names = NameTable::new(db, file);
    let mut fields = Vector::new();

    for field in node.iter_fields() {
        if let Some(f) = lower_field(db, file, scope, field) {
            if names.insert(f.name.clone(), field.syntax()) {
                fields.push_back(f);
            }
        }
    }

    Some(hir::Record {
        name,
        docs: node.docs(src),
        index,
        fields,
    })
}

fn lower_field(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    node: ast::RecordField<'_>,
) -> Option<hir::RecordField> {
    let src = file.contents(db);
    let name = node.identifier(src)?.into();
    let docs = node.docs(src);
    let ty = node.ty()?;
    let ty = resolve_type(db, file, scope, ty)?;

    Some(hir::RecordField { name, docs, ty })
}

#[salsa::tracked]
pub(crate) fn lower_resource(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    index: ResourceIndex,
) -> Option<hir::Resource> {
    let ast = crate::queries::parse(db, file);
    let meta = match scope {
        ScopeIndex::Interface(interface) => file.get_by_index(db, (interface, index)),
        ScopeIndex::World(world) => file.get_by_index(db, (world, index)),
    };
    let tree = ast.tree(db);
    let src = ast.src(db);

    let node = meta.location.ast_node(tree);
    let name = node.identifier(src)?.into();
    let docs = node.docs(src);

    let methods = meta
        .iter_methods()
        .filter_map(|(_, ix, ptr)| {
            let node = ptr.ast_node(tree);
            let index = AnyFuncItemIndex::Method(scope, index, ix);
            lower_method(db, file, index, node, src)
        })
        .collect();
    let static_methods = meta
        .iter_static_methods()
        .filter_map(|(_, ix, ptr)| {
            let node = ptr.ast_node(tree);
            let index = AnyFuncItemIndex::StaticMethod(scope, index, ix);
            lower_static_method(db, file, index, node, src)
        })
        .collect();

    let constructor = meta
        .constructor
        .map(|c| c.ast_node(tree))
        .map(|c| lower_constructor(db, file, scope, c));

    Some(hir::Resource {
        constructor,
        docs,
        index,
        methods,
        static_methods,
        name,
    })
}

fn lower_method(
    db: &dyn Db,
    file: SourceFile,
    index: AnyFuncItemIndex,
    node: ast::FuncItem<'_>,
    src: &str,
) -> Option<hir::ResourceMethod> {
    let name = node.identifier(src)?.into();
    let ty = node.ty()?;
    let docs = node.docs(src);

    let (params, return_value) = lower_func_type(db, file, index.scope(), ty)?;

    Some(hir::ResourceMethod(hir::FuncItem {
        name,
        index,
        docs,
        params,
        return_value,
    }))
}

fn lower_func_type(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    node: ast::FuncType<'_>,
) -> Option<(Vector<hir::Parameter>, Option<hir::ReturnValue>)> {
    let mut params = Vector::new();
    for param in node.params()?.iter_params() {
        let param = lower_param(db, file, scope, param)?;
        params.push_back(param);
    }
    let return_value = node
        .result_opt()
        .and_then(|r| lower_return_value(db, file, scope, r));

    Some((params, return_value))
}

fn lower_static_method(
    db: &dyn Db,
    file: SourceFile,
    index: AnyFuncItemIndex,
    node: ast::StaticMethod<'_>,
    src: &str,
) -> Option<hir::StaticResourceMethod> {
    let name = node.identifier(src)?.into();
    let ty = node.func_type()?;
    let docs = node.docs(src);

    let (params, return_value) = lower_func_type(db, file, index.scope(), ty)?;

    Some(hir::StaticResourceMethod(hir::FuncItem {
        name,
        index,
        docs,
        params,
        return_value,
    }))
}

fn lower_constructor(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    node: ast::ResourceConstructor<'_>,
) -> hir::Constructor {
    let src = file.contents(db);
    let docs = node.docs(src);

    let params = node
        .params()
        .into_iter()
        .flat_map(|params| params.iter_params())
        .filter_map(|p| lower_param(db, file, scope, p))
        .collect();

    hir::Constructor { docs, params }
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
    let node = ptr.ast_node(tree);
    let name = node.identifier(src)?.into();

    let ty = node.ty()?;
    let ty = resolve_type(db, file, scope, ty)?;

    Some(hir::TypeAlias {
        name,
        docs: node.docs(src),
        index,
        ty,
    })
}

#[salsa::tracked]
pub(crate) fn lower_variant(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    index: VariantIndex,
) -> Option<hir::Variant> {
    let ast = crate::queries::parse(db, file);
    let ptr = match scope {
        ScopeIndex::Interface(interface) => file.get_by_index(db, (interface, index)),
        ScopeIndex::World(world) => file.get_by_index(db, (world, index)),
    };
    let src = ast.src(db);

    let tree = ast.tree(db);
    let node = ptr.ast_node(tree);
    let name = node.identifier(src)?.into();
    let docs = node.docs(src);

    let mut names = NameTable::new(db, file);

    let mut cases = Vector::new();

    for c in node.iter_cases() {
        if let Some(case) = lower_variant_case(db, file, scope, c) {
            if names.insert(case.name.clone(), c.syntax()) {
                cases.push_back(case);
            }
        }
    }

    Some(hir::Variant {
        docs,
        name,
        index,
        cases,
    })
}

fn lower_variant_case(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    node: ast::VariantCase<'_>,
) -> Option<hir::VariantCase> {
    let src = file.contents(db);
    let name = node.identifier(src)?.into();
    let docs = node.docs(src);

    let ty = if let Some(ty) = node.ty_opt() {
        Some(resolve_type(db, file, scope, ty)?)
    } else {
        None
    };

    Some(hir::VariantCase { name, docs, ty })
}

fn resolve_type(
    db: &dyn Db,
    file: SourceFile,
    scope: ScopeIndex,
    ty: ast::Ty<'_>,
) -> Option<hir::Type> {
    let resolver = TypeResolver::new(db, file, scope);
    resolver.resolve_type(ty)
}

struct TypeResolver<'a> {
    db: &'a dyn Db,
    src: &'a str,
    file: SourceFile,
    scope: ScopeIndex,
}

impl<'a> TypeResolver<'a> {
    fn new(db: &'a dyn Db, file: SourceFile, scope: ScopeIndex) -> Self {
        let src = file.contents(db).as_str();
        TypeResolver {
            db,
            src,
            file,
            scope,
        }
    }

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
        user_defined_type: ast::UserDefinedType<'_>,
    ) -> Option<hir::Type> {
        let name = user_defined_type.identifier(self.src)?;
        let name = Text::from(name);

        match crate::queries::resolve_name(self.db, self.file, self.scope, name.clone()) {
            Some(reference) => Some(hir::Type::UserDefinedType(reference)),
            None => {
                let diag = Diagnostic::unknown_name(
                    self.file.path(self.db),
                    name,
                    user_defined_type.range(),
                );
                Diagnostics::push(self.db, diag);
                Some(hir::Type::Error)
            }
        }
    }

    fn resolve_list(&self, list: ast::List<'_>) -> Option<hir::Type> {
        let element_type = list.ty()?;
        let element_type = self.resolve_type(element_type)?;
        Some(hir::Type::List(Box::new(element_type)))
    }

    fn resolve_handle(&self, handle: ast::Handle<'_>) -> Option<hir::Type> {
        let (ty, borrowed) = if let Some(h) = handle.borrowed_handle() {
            (h.user_defined_type()?, true)
        } else if let Some(h) = handle.owned_handle() {
            (h.user_defined_type()?, false)
        } else {
            return None;
        };

        let ty = self
            .resolve_user_defined_type(ty)
            .unwrap_or(hir::Type::Error);

        Some(hir::Type::Handle {
            borrowed,
            ty: Box::new(ty),
        })
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
    use crate::{diagnostics::Diagnostics, Compiler, FilePath};

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
                        FilePath::new(&db, format!("{}.wit", stringify!($name)).into()),
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
                        FilePath::new(&db, format!("{}.wit", stringify!($name)).into()),
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
        empty_record: "interface i { record empty {} }",
        record_with_one_field: "interface i { record foo { field: string } }",
        record_with_kitchen_sink: "interface i {
            /// A very important record.
            record foo {
                /// The first field.
                first: string,
                /// The second field.
                second: u32,
                /// The third field.
                third: list<bool>,
             }
        }",
        empty_resource: "interface i { resource empty; }",
        empty_resource_with_braces: "interface i { resource empty {} }",
        resource_with_parameterless_constructor: "interface i { resource r { constructor(); } }",
        resource_with_constructor: "interface i { resource r { constructor(arg1: string, arg2: bool); } }",
        resource_with_method: "interface i { resource r { method: func(arg1: string, arg2: bool) -> u32; } }",
        resource_with_static_method: "interface i { resource r { method: static func(arg1: string, arg2: bool) -> u32; } }",
        refer_to_user_defined_type: "interface i {
            record r {}
            type x = list<r>;
        }",
        #[ignore]
        refer_to_user_defined_type_from_other_interface: "
            interface first { record r {} }
            interface second {
                use first.{r};
                type x = r;
            }
        ",

        empty_variant: "interface i { variant v {} }",
        varant_with_one_field: "interface i { variant v { field } }",
        varant_with_multiple_fields_and_payloads: "interface i {
            /// A variant.
            variant v {
                /// An integer.
                first(u32),
                /// A string.
                second(string),
                /// An empty variant.
                third,
            }
        }",

        empty_world: "world empty {}",
        #[ignore]
        world_with_function_export: "world console { export run: func(); }",
        #[ignore]
        world_with_interface_export: "world console { export run: interface {} }",
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
        duplicate_identifiers_within_interface: "interface i { record foo {} variant foo {} }",
        duplicate_record_fields: "interface i { record r { field: u32, field: u32 } }",
        duplicate_variant_cases: "interface i { variant v { var(float32), var } }",
        duplicate_enum_cases: "interface i { enum e { field, field } }",
        duplicate_flags_cases: "interface i { flags f { field, field } }",
        duplicate_resource_methods: "interface i {
            resource r {
                method: func();
                method: static func();
            }
        }",
        resource_with_multiple_constructors: "interface i { resource r { constructor(); constructor(); }}",
    }
}
