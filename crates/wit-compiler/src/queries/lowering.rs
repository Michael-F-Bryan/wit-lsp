use im::{OrdMap, Vector};

use crate::{
    access::{AnyFuncItemIndex, InterfaceIndex, NodeKind, Pointer, WorldIndex},
    ast::{self, AstNode, HasAttr, HasIdent as _},
    diagnostics::{Diagnostics, DuplicateName, Location, MultiplePackageDocs},
    hir,
    queries::{
        metadata::{
            ConstructorMetadata, EnumMetadata, FlagsMetadata, FuncItemMetadata, HasDefinition,
            HasIdent, Ident, InterfaceItemMetadata, InterfaceMetadata, MethodMetadata,
            RecordMetadata, ResourceMetadata, StaticMethodMetadata, TypeAliasMetadata,
            TypeDefinitionMetadata, VariantCaseMetadata, VariantMetadata, WorldMetadata,
        },
        Ast, FilePath, Package, SourceFile, Workspace,
    },
    Db, Text,
};

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub fn lower_package(db: &dyn Db, ws: Workspace, pkg: Package) -> hir::Package {
    let mut lowered = hir::Package {
        id: pkg.id(db),
        ..Default::default()
    };

    let meta = crate::queries::package_items(db, pkg);

    for (ix, world) in meta.worlds(db) {
        if let Some(world) = lower_world(db, ws, pkg, world) {
            lowered.worlds.insert(ix, world);
        }
    }

    for (ix, interface) in meta.interfaces(db) {
        if let Some(interface) = lower_interface(db, ws, pkg, interface) {
            lowered.interfaces.insert(ix, interface);
        }
    }

    lowered
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub(crate) fn lower_package_docs(db: &dyn Db, pkg: Package) -> Option<Text> {
    let mut all_docs = pkg.files(db).into_iter().filter_map(|f| {
        let tree = crate::queries::parse(db, f);
        let src = f.contents(db);
        let decl = tree.source_file(db).package_opt()?;
        let docs = decl.docs(src)?;
        let location = Location::new(f.path(db), decl.range());
        Some((location, docs))
    });

    let (original_definition, docs) = all_docs.next()?;

    for (loc, _) in all_docs {
        let diag = MultiplePackageDocs {
            second_location: loc,
            original_definition,
        };
        Diagnostics::push(db, diag.into());
    }

    Some(docs)
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub(crate) fn lower_world(
    db: &dyn Db,
    ws: Workspace,
    pkg: Package,
    meta: WorldMetadata,
) -> Option<hir::World> {
    let ptr = meta.ptr(db);
    let file = ws.file(db, ptr.file());
    let ast = crate::queries::parse(db, file);
    let node = ast.get(db, ptr);

    let type_definitions = meta
        .definitions(db)
        .into_iter()
        .filter_map(|item| lower_type_definition(db, ws, item))
        .collect();

    Some(hir::World {
        docs: node.docs(file.contents(db)),
        name: meta.name(db),
        type_definitions,
    })
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub(crate) fn lower_interface(
    db: &dyn Db,
    ws: Workspace,
    pkg: Package,
    meta: InterfaceMetadata,
) -> Option<hir::Interface> {
    let ptr = meta.ptr(db);
    let file = ws.file(db, ptr.file());
    let ast = crate::queries::parse(db, file);
    let node = ast.get(db, ptr);

    let items = meta
        .items(db)
        .into_iter()
        .filter_map(|item| lower_interface_item(db, ws, item))
        .collect();

    Some(hir::Interface {
        docs: node.docs(file.contents(db)),
        name: meta.name(db),
        items,
    })
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all)]
pub(crate) fn lower_interface_item(
    db: &dyn Db,
    ws: Workspace,
    meta: InterfaceItemMetadata,
) -> Option<hir::InterfaceItem> {
    match meta {
        InterfaceItemMetadata::Func(f) => lower_func_definition(db, ws, f).map(Into::into),
        InterfaceItemMetadata::Type(t) => lower_type_definition(db, ws, t).map(Into::into),
    }
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(name = %meta.name(db).raw(db)))]
pub(crate) fn lower_func_definition(
    db: &dyn Db,
    ws: Workspace,
    meta: FuncItemMetadata,
) -> Option<hir::FuncItem> {
    let ptr = meta.ptr(db);
    let ctx = Context::new(db, ws, ptr);
    let node = ctx.get(ptr);

    let ty = node.ty()?;
    let (params, return_value) = lower_func_ty(ctx, ty)?;

    Some(hir::FuncItem {
        name: meta.name(db),
        index: AnyFuncItemIndex::Standalone(meta.index(db)),
        docs: node.docs(ctx.src),
        params,
        return_value,
    })
}

#[derive(Copy, Clone)]
struct Context<'db> {
    db: &'db dyn Db,
    _ws: Workspace,
    file: SourceFile,
    src: &'db str,
    ast: Ast,
}

impl<'db> Context<'db> {
    fn new<K>(db: &'db dyn Db, ws: Workspace, ptr: Pointer<K>) -> Self {
        let file = ws.file(db, ptr.file());
        let src = file.contents(db);
        let ast = crate::queries::parse(db, file);

        Context {
            db,
            _ws: ws,
            file,
            ast,
            src,
        }
    }

    fn path(self) -> FilePath {
        self.file.path(self.db)
    }

    fn get<K: NodeKind>(self, ptr: Pointer<K>) -> K::Ast<'db> {
        self.ast.get(self.db, ptr)
    }
}

fn lower_func_ty(
    ctx: Context<'_>,
    ty: ast::FuncType<'_>,
) -> Option<(Vector<hir::Parameter>, Option<hir::ReturnValue>)> {
    let params = ty.params()?;
    let params = lower_params(ctx, params)?;

    let return_value = ty.result_opt().and_then(|ret| lower_return_value(ctx, ret));

    Some((params, return_value))
}

fn lower_return_value(ctx: Context<'_>, node: ast::ResultList<'_>) -> Option<hir::ReturnValue> {
    let ty = if let Some(result_list) = node.named_result_list_opt() {
        lower_named_result_list(ctx, result_list)
            .unwrap_or(hir::ReturnValue::Single(hir::Type::Error))
    } else if let Some(ty) = node.ty_opt() {
        let ty = lower_type(ctx, ty).unwrap_or(hir::Type::Error);
        hir::ReturnValue::Single(ty)
    } else {
        // Syntax error - let's say the return type is an error. It's
        // not 100% correct, but at least we can make progress analysing
        // this function.
        hir::ReturnValue::Single(hir::Type::Error)
    };

    Some(ty)
}

fn lower_params(ctx: Context<'_>, node: ast::ParamList<'_>) -> Option<Vector<hir::Parameter>> {
    let mut params: Vector<(Location, hir::Parameter)> = Vector::new();

    for param in node.iter_params() {
        let location = Location::new(ctx.path(), param.range());

        let Some(param) = lower_param(ctx, param) else {
            continue;
        };

        if let Some((original_definition, _)) = params.iter().find(|(_, p)| p.name == param.name) {
            let diag = DuplicateName {
                name: param.name.raw(ctx.db).clone(),
                location,
                original_definition: *original_definition,
            };
            Diagnostics::push(ctx.db, diag.into());
        } else {
            params.push_back((location, param));
        }
    }

    Some(params.into_iter().map(|(_, p)| p).collect())
}

fn lower_named_result_list(
    ctx: Context<'_>,
    result_list: ast::NamedResultList<'_>,
) -> Option<hir::ReturnValue> {
    let mut names: OrdMap<Ident, (Location, hir::Type)> = OrdMap::new();

    for ret in result_list.iter_named_types() {
        let Some(name) = ret.identifier(ctx.src) else {
            continue;
        };
        let name = Ident::new(ctx.db, name.into());
        let ty = ret
            .ty()
            .and_then(|ty| lower_type(ctx, ty))
            .unwrap_or(hir::Type::Error);

        let location = Location::new(ctx.path(), ret.range());

        match names.entry(name) {
            im::ordmap::Entry::Occupied(entry) => {
                let (original_definition, _) = entry.get();
                let diag = crate::diagnostics::DuplicateName {
                    name: name.raw(ctx.db).clone(),
                    location,
                    original_definition: *original_definition,
                };
                Diagnostics::push(ctx.db, diag.into());
            }
            im::ordmap::Entry::Vacant(entry) => {
                entry.insert((location, ty));
            }
        }
    }

    let names = names
        .into_iter()
        .map(|(k, (_, ty))| (k.raw(ctx.db).clone(), ty))
        .collect();
    Some(hir::ReturnValue::Named(names))
}

fn lower_param(ctx: Context<'_>, param: ast::NamedType<'_>) -> Option<hir::Parameter> {
    let name = param.identifier(ctx.src)?;
    let name = Ident::new(ctx.db, name.into());

    let ty = param
        .ty()
        .and_then(|ty| lower_type(ctx, ty))
        .unwrap_or(hir::Type::Error);

    Some(hir::Parameter {
        name,
        docs: param.docs(ctx.src),
        ty,
    })
}

pub(crate) fn lower_type_definition(
    db: &dyn Db,
    ws: Workspace,
    meta: TypeDefinitionMetadata,
) -> Option<hir::TypeDefinition> {
    match meta {
        TypeDefinitionMetadata::Enum(item) => lower_enum(db, ws, item).map(Into::into),
        TypeDefinitionMetadata::Record(item) => lower_record(db, ws, item).map(Into::into),
        TypeDefinitionMetadata::Resource(item) => lower_resource(db, ws, item).map(Into::into),
        TypeDefinitionMetadata::Variant(item) => lower_variant(db, ws, item).map(Into::into),
        TypeDefinitionMetadata::Flags(item) => lower_flags(db, ws, item).map(Into::into),
        TypeDefinitionMetadata::Alias(item) => lower_type_alias(db, ws, item).map(Into::into),
    }
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(name = %meta.name(db).raw(db)))]
pub(crate) fn lower_enum(db: &dyn Db, ws: Workspace, meta: EnumMetadata) -> Option<hir::Enum> {
    let ptr = meta.ptr(db);
    let file = ws.file(db, ptr.file());
    let src = file.contents(db);
    let ast = crate::queries::parse(db, file);
    let node = ast.get(db, ptr);

    let cases = cases(db, meta.cases(db), |c| hir::EnumCase {
        name: c.name(db),
        docs: ast.get(db, c.ptr(db)).docs(src),
    });

    Some(hir::Enum {
        name: meta.name(db),
        index: meta.index(db),
        docs: node.docs(src),
        cases,
    })
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(name = %meta.name(db).raw(db)))]
pub(crate) fn lower_record(
    db: &dyn Db,
    ws: Workspace,
    meta: RecordMetadata,
) -> Option<hir::Record> {
    let ptr = meta.ptr(db);
    let ctx = Context::new(db, ws, ptr);
    let node = ctx.get(ptr);

    let fields = cases(db, meta.fields(db), |f| {
        let node = ctx.get(f.ptr(db));
        hir::RecordField {
            name: f.name(db),
            docs: node.docs(ctx.src),
            ty: node
                .ty()
                .and_then(|ty| lower_type(ctx, ty))
                .unwrap_or(hir::Type::Error),
        }
    });

    Some(hir::Record {
        name: meta.name(db),
        index: meta.index(db),
        docs: node.docs(ctx.src),
        fields,
    })
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(name = %meta.name(db).raw(db)))]
pub(crate) fn lower_resource(
    db: &dyn Db,
    ws: Workspace,
    meta: ResourceMetadata,
) -> Option<hir::Resource> {
    let ptr = meta.ptr(db);
    let ctx = Context::new(db, ws, ptr);
    let node = ctx.get(ptr);

    let methods = meta
        .methods(db)
        .into_iter()
        .filter_map(|meta| lower_method(db, ws, meta))
        .collect();
    let static_methods = meta
        .static_methods(db)
        .into_iter()
        .filter_map(|meta| lower_static_method(db, ws, meta))
        .collect();

    let constructor = meta
        .constructor(db)
        .and_then(|meta| lower_constructor(db, ws, meta));

    Some(hir::Resource {
        name: meta.name(db),
        docs: node.docs(ctx.src),
        index: meta.index(db),
        constructor,
        methods,
        static_methods,
    })
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all)]
pub(crate) fn lower_constructor(
    db: &dyn Db,
    ws: Workspace,
    meta: ConstructorMetadata,
) -> Option<hir::Constructor> {
    let ptr = meta.ptr(db);
    let ctx = Context::new(db, ws, ptr);
    let node = ctx.get(ptr);

    let params = node.params()?;
    let params = lower_params(ctx, params)?;

    Some(hir::Constructor {
        docs: node.docs(ctx.src),
        params,
    })
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(name = %meta.name(db).raw(db)))]
pub(crate) fn lower_method(
    db: &dyn Db,
    ws: Workspace,
    meta: MethodMetadata,
) -> Option<hir::ResourceMethod> {
    let ptr = meta.ptr(db);
    let ctx = Context::new(db, ws, ptr);
    let node = ctx.get(ptr);

    let func_ty = node.ty()?;

    let params = func_ty.params()?;
    let params = lower_params(ctx, params)?;

    let return_value = func_ty
        .result_opt()
        .and_then(|return_value| lower_return_value(ctx, return_value));

    let item = hir::FuncItem {
        name: meta.name(db),
        docs: node.docs(ctx.src),
        index: AnyFuncItemIndex::Method(meta.index(db)),
        params,
        return_value,
    };

    Some(hir::ResourceMethod(item))
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(name = %meta.name(db).raw(db)))]
pub(crate) fn lower_static_method(
    db: &dyn Db,
    ws: Workspace,
    meta: StaticMethodMetadata,
) -> Option<hir::StaticResourceMethod> {
    let ptr = meta.ptr(db);
    let ctx = Context::new(db, ws, ptr);
    let node = ctx.get(ptr);

    let func_ty = node.ty()?;

    let params = func_ty.params()?;
    let params = lower_params(ctx, params)?;

    let return_value = func_ty
        .result_opt()
        .and_then(|return_value| lower_return_value(ctx, return_value));

    let item = hir::FuncItem {
        name: meta.name(db),
        docs: node.docs(ctx.src),
        index: AnyFuncItemIndex::StaticMethod(meta.index(db)),
        params,
        return_value,
    };

    Some(hir::StaticResourceMethod(item))
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(name = %meta.name(db).raw(db)))]
pub(crate) fn lower_variant(
    db: &dyn Db,
    ws: Workspace,
    meta: VariantMetadata,
) -> Option<hir::Variant> {
    let ptr = meta.ptr(db);
    let ctx = Context::new(db, ws, ptr);
    let node = ctx.get(ptr);

    let cases = cases(db, meta.cases(db), |c| lower_variant_case(ctx, c));

    Some(hir::Variant {
        name: meta.name(db),
        index: meta.index(db),
        docs: node.docs(ctx.src),
        cases,
    })
}

fn lower_variant_case(ctx: Context<'_>, meta: VariantCaseMetadata) -> hir::VariantCase {
    let node = ctx.get(meta.ptr(ctx.db));
    let ty = node
        .ty_opt()
        .map(|ty| lower_type(ctx, ty).unwrap_or(hir::Type::Error));

    hir::VariantCase {
        name: meta.ident(ctx.db),
        docs: node.docs(ctx.src),
        ty,
    }
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(name = %meta.name(db).raw(db)))]
pub(crate) fn lower_flags(db: &dyn Db, ws: Workspace, meta: FlagsMetadata) -> Option<hir::Flags> {
    let ptr = meta.ptr(db);
    let file = ws.file(db, ptr.file());
    let src = file.contents(db);
    let ast = crate::queries::parse(db, file);
    let node = ast.get(db, ptr);

    let cases = cases(db, meta.cases(db), |c| hir::FlagsCase {
        name: c.name(db),
        docs: ast.get(db, c.ptr(db)).docs(src),
    });

    Some(hir::Flags {
        name: meta.name(db),
        index: meta.index(db),
        docs: node.docs(src),
        cases,
    })
}

fn cases<C, F, Ret>(db: &dyn Db, meta: impl IntoIterator<Item = C>, mut map: F) -> Vector<Ret>
where
    F: FnMut(C) -> Ret,
    C: HasIdent + HasDefinition + Clone,
    Ret: Clone,
{
    let mut cases: OrdMap<Ident, (Location, Ret)> = OrdMap::new();

    for case in meta {
        let name = case.ident(db);
        let location = case.definition(db);

        match cases.entry(name) {
            im::ordmap::Entry::Occupied(entry) => {
                let (original_definition, _) = entry.get();
                let diag = crate::diagnostics::DuplicateName {
                    name: name.raw(db).clone(),
                    location,
                    original_definition: *original_definition,
                };
                Diagnostics::push(db, diag.into());
            }
            im::ordmap::Entry::Vacant(entry) => {
                entry.insert((location, map(case)));
            }
        }
    }

    cases.into_iter().map(|(_, (_, v))| v).collect()
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(name = %meta.name(db).raw(db)))]
pub(crate) fn lower_type_alias(
    db: &dyn Db,
    ws: Workspace,
    meta: TypeAliasMetadata,
) -> Option<hir::TypeAlias> {
    let ptr = meta.ptr(db);
    let ctx = Context::new(db, ws, ptr);
    let node = ctx.get(ptr);

    let ty = node.ty()?;
    let ty = lower_type(ctx, ty).unwrap_or(hir::Type::Error);

    Some(hir::TypeAlias {
        name: meta.name(db),
        index: meta.index(db),
        docs: node.docs(ctx.src),
        ty,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum TopLevelItem {
    World(WorldIndex, hir::World),
    Interface(InterfaceIndex, hir::Interface),
}

/// Resolve a [`ast::Ty`] into a [`hir::Type`]. If resolution fails for whatever
/// reason, [`None`] is returned.
fn lower_type(ctx: Context<'_>, node: ast::Ty<'_>) -> Option<hir::Type> {
    if let Some(node) = node.builtins() {
        let builtin = resolve_builtin(ctx, node)?;
        return Some(hir::Type::Builtin(builtin));
    }

    if let Some(list) = node.list() {
        return resolve_list(ctx, list);
    }

    if let Some(result) = node.result() {
        return resolve_result(ctx, result);
    }

    if let Some(option) = node.option() {
        return resolve_option(ctx, option);
    }

    if let Some(handle) = node.handle() {
        return resolve_handle(ctx, handle);
    }

    if let Some(tuple) = node.tuple() {
        return resolve_tuple(ctx, tuple);
    }

    if let Some(ty) = node.user_defined_type() {
        return resolve_user_defined_type(ctx, ty);
    }

    None
}

fn resolve_handle(ctx: Context<'_>, handle: ast::Handle<'_>) -> Option<hir::Type> {
    let (ty, borrowed) = if let Some(node) = handle.borrowed_handle() {
        (node.user_defined_type()?, true)
    } else if let Some(node) = handle.owned_handle() {
        (node.user_defined_type()?, false)
    } else {
        return None;
    };

    let ty = resolve_user_defined_type(ctx, ty)?;

    Some(hir::Type::Handle {
        ty: Box::new(ty),
        borrowed,
    })
}

fn resolve_tuple(ctx: Context<'_>, tuple: ast::Tuple<'_>) -> Option<hir::Type> {
    let mut elements = Vector::new();

    for ty in tuple.iter_tys() {
        let ty = lower_type(ctx, ty)?;
        elements.push_back(ty);
    }

    Some(hir::Type::Tuple(elements))
}

fn resolve_user_defined_type(ctx: Context<'_>, ty: ast::UserDefinedType<'_>) -> Option<hir::Type> {
    let _ident = ty.identifier(ctx.src)?;
    todo!()
}

fn resolve_option(ctx: Context<'_>, option: ast::Option_<'_>) -> Option<hir::Type> {
    let ty = option.ty()?;
    let element = lower_type(ctx, ty)?;
    Some(hir::Type::Option(Box::new(element)))
}

fn resolve_result(ctx: Context<'_>, result: ast::Result_<'_>) -> Option<hir::Type> {
    let ok = match result.ok_opt() {
        Some(ty) => {
            let ty = lower_type(ctx, ty)?;
            Some(Box::new(ty))
        }
        None => None,
    };
    let err = match result.err_opt() {
        Some(ty) => {
            let ty = lower_type(ctx, ty)?;
            Some(Box::new(ty))
        }
        None => None,
    };

    Some(hir::Type::Result { ok, err })
}

fn resolve_list(ctx: Context<'_>, list: ast::List<'_>) -> Option<hir::Type> {
    let element = list.ty()?;
    let element = lower_type(ctx, element)?;
    Some(hir::Type::List(Box::new(element)))
}

fn resolve_builtin(ctx: Context<'_>, builtin: ast::Builtins<'_>) -> Option<hir::Builtin> {
    let name = builtin.value(ctx.src);

    match name {
        "u8" => Some(hir::Builtin::U8),
        "u16" => Some(hir::Builtin::U16),
        "u32" => Some(hir::Builtin::U32),
        "u64" => Some(hir::Builtin::U64),
        "s8" => Some(hir::Builtin::I8),
        "s16" => Some(hir::Builtin::I16),
        "s32" => Some(hir::Builtin::I32),
        "s64" => Some(hir::Builtin::I64),
        "float32" => Some(hir::Builtin::Float32),
        "float64" => Some(hir::Builtin::Float64),
        "char" => Some(hir::Builtin::Char),
        "bool" => Some(hir::Builtin::Boolean),
        "string" => Some(hir::Builtin::String),
        other => {
            unreachable!(
                "Unknown builtin type, \"{other}\" at {:#} ({:?})",
                builtin.syntax(),
                builtin.range(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        diagnostics::Diagnostics,
        queries::{FilePath, SourceFile},
        Compiler,
    };

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

                    let packages = crate::queries::workspace_packages(&db, ws);
                    let pkg = packages[0];
                    let got = super::lower_package(&db, ws, pkg);
                    let diags = super::lower_package::accumulated::<Diagnostics>(&db, ws, pkg);

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

                    let packages = crate::queries::workspace_packages(&db, ws);
                    let pkg = packages[0];
                    let diags = super::lower_package::accumulated::<Diagnostics>(&db, ws, pkg);

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
        #[ignore]
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
        varant_with_one_field: "interface i { variant v { case } }",
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
        world_defining_a_type: "world w { type x = u32; }",
        #[ignore = "Parse error"]
        world_with_function_import: "world console { import run: func(); }",
        #[ignore]
        world_with_interface_import: "world console { import run: interface {} }",
        #[ignore = "Parse error"]
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
        func_with_duplicate_parameters: "interface i { f: func(a: u32, a: u32); }",
        func_with_duplicate_named_return_values: "interface i { f: func() -> (a: u32, a: u32); }",
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
