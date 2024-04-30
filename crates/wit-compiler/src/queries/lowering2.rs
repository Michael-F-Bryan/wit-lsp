use crate::{
    access::{InterfaceIndex, WorldIndex},
    ast::{AstNode, HasAttr},
    diagnostics::{Diagnostics, Location, MultiplePackageDocs},
    hir,
    queries::{
        metadata::{InterfaceMetadata, WorldMetadata},
        Package, Workspace,
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
    _ws: Workspace,
    pkg: Package,
    _item: WorldMetadata,
) -> Option<hir::World> {
    todo!();
}

#[salsa::tracked]
#[tracing::instrument(level = "debug", skip_all, fields(dir = %pkg.dir(db).raw_path(db)))]
pub(crate) fn lower_interface(
    db: &dyn Db,
    _ws: Workspace,
    pkg: Package,
    _item: InterfaceMetadata,
) -> Option<hir::Interface> {
    todo!();
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum TopLevelItem {
    World(WorldIndex, hir::World),
    Interface(InterfaceIndex, hir::Interface),
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
