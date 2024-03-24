function punctuated(rule, separator) {
    return seq(rule, repeat(seq(separator, rule)));
}

function cases(rule) {
    return optional(
        seq(
            punctuated(rule, ","),
            optional(","),
        ),
    );
}

module.exports = grammar({
    name: "wit",
    word: $ => $.identifier,
    extras: $ => [/[\s\n\t]/, $.slash_comment, $.block_comment],
    conficts: $ => [
        [$.package_name],
    ],

    rules: {
        source_file: $ => seq(
            optional(field("package", $.package_decl)),
            repeat($.top_level_item),
        ),

        package_decl: $ => seq(
            field("attributes", repeat($.attribute)),
            "package",
            $.fully_qualified_package_name,
            ";",
        ),
        fully_qualified_package_name: $ => seq(
            field("package", $.package_name),
            token.immediate(":"),
            field("path", $.package_path),
            optional(seq(token.immediate("@"), field("version", $.semver))),
        ),
        // FIXME: Allow package names with multiple segments separated by ":"
        // without causing an ambiguity in "package really:nested:module;", where
        // that last "module" is actually the $.package_path
        package_name: $ => $.identifier,
        package_path: $ => prec.left(punctuated($.identifier, "/")),

        top_level_item: $ => choice($.top_level_use_item, $.world_item, $.interface_item),

        top_level_use_item: $ => seq(
            "use",
            $.use_path,
            optional(seq("as", field("alias", $.identifier))),
            ";",
        ),
        use_path: $ => choice($.identifier, $._use_path),
        _use_path: $ => seq(
            field("package", $.package_name),
            token.immediate(":"),
            field("path", $.package_path),
            optional(seq(token.immediate("@"), field("version", $.semver))),
        ),

        world_item: $ => seq("world", field("name", $.identifier), "{", repeat($.world_items), "}"),
        world_items: $ => choice(
            $.export_item, $.import_item, $.use_item, $.typedef_item, $.include_item,
        ),
        export_item: $ => choice($.exported_item, $.exported_path),
        exported_item: $ => seq("export", field("name", $.identifier), ":", $.extern_type),
        exported_path: $ => seq("export", $._use_path, ";"),
        import_item: $ => choice($.imported_item, $.imported_path),
        imported_item: $ => seq("import", field("name", $.identifier), ":", $.extern_type),
        imported_path: $ => seq("import", $.use_path, ";"),

        extern_type: $ => choice(
            seq($.func_type, ";"),
            seq("interface", "{", repeat($.interface_items), "}"),
        ),

        use_item: $ => seq(
            "use",
            field("path", $.use_path),
            token.immediate("."),
            token.immediate("{"),
            $._use_names_list,
            token.immediate("}"),
            ";",
        ),
        _use_names_list: $ => punctuated($.use_names_item, ","),
        use_names_item: $ => seq(
            field("name", $.identifier),
            optional(seq("as", field("alias", $.identifier))),
        ),

        interface_item: $ => seq(
            field("attributes", repeat($.attribute)),
            "interface",
            field("name", $.identifier),
            "{",
            repeat($.interface_items),
            "}",
        ),
        interface_items: $ => choice($.typedef_item, $.use_item, $.func_item),
        typedef_item: $ => choice(
            $.resource_item,
            $.variant_item,
            $.record_item,
            $.flags_item,
            $.enum_item,
            $.type_item,
        ),
        func_item: $ => seq(
            field("attributes", repeat($.attribute)),
            field("name", $.identifier),
            ":",
            field("ty", $.func_type),
            ";",
        ),
        func_type: $ => seq(
            "func",
            field("params", $.param_list),
            optional(
                seq(
                    "->",
                    field("result", $.result_list),
                )
            ),
        ),
        param_list: $ => seq("(", optional($._param_list_inner), ")"),
        _param_list_inner: $ => seq(punctuated($._named_type_list, ","), optional(",")),
        result_list: $ => choice($.ty, seq("(", optional($._named_type_list), ")")),
        _named_type_list: $ => prec.left(punctuated($.named_type, ",")),
        named_type: $ => seq(
            field("name", $.identifier),
            ":",
            field("ty", $.ty),
        ),

        include_item: $ => seq(
            "include",
            field("path", $.use_path),
            choice(
                ";",
                seq("with", "{", field("names", $.include_names_list), "}")
            ),
        ),
        include_names_list: $ => punctuated($.include_names_item, ","),
        include_names_item: $ => seq(
            field("name", $.identifier),
            "as",
            field("alias", $.identifier),
        ),

        resource_item: $ => seq(
            field("attributes", repeat($.attribute)),
            "resource",
            field("name", $.identifier),
            choice(
                ";",
                seq("{", repeat($.resource_method), "}")
            ),
        ),
        resource_method: $ => choice(
            $.func_item,
            $.static_method,
            $.resource_constructor,
        ),
        static_method: $ => seq(
            field("attributes", repeat($.attribute)),
            field("name", $.identifier),
            ":",
            "static",
            $.func_type,
            ";",
        ),
        resource_constructor: $ => seq(
            field("attributes", repeat($.attribute)),
            "constructor",
            field("params", $.param_list),
            ";",
        ),

        variant_item: $ => seq(
            field("attributes", repeat($.attribute)),
            "variant",
            field("name", $.identifier),
            "{",
            cases($.variant_case),
            "}",
        ),
        variant_case: $ => seq(
            field("attributes", repeat($.attribute)),
            field("name", $.identifier),
            optional(
                seq(
                    "(",
                    field("ty", $.ty),
                    ")",
                ),
            ),
        ),

        record_item: $ => seq(
            field("attributes", repeat($.attribute)),
            "record",
            field("name", $.identifier),
            "{",
            cases($.record_field),
            "}",
        ),
        record_field: $ => seq(
            field("attributes", repeat($.attribute)),
            field("name", $.identifier),
            ":",
            field("ty", $.ty),
        ),

        flags_item: $ => seq(
            field("attributes", repeat($.attribute)),
            "flags",
            field("name", $.identifier),
            "{",
            cases($.flags_field),
            "}",
        ),
        flags_field: $ => seq(
            field("attributes", repeat($.attribute)),
            field("name", $.identifier),
        ),

        enum_item: $ => seq(
            field("attributes", repeat($.attribute)),
            "enum",
            field("name", $.identifier),
            "{",
            cases($.enum_field),
            "}",
        ),
        enum_field: $ => seq(
            field("attributes", repeat($.attribute)),
            field("name", $.identifier),
        ),

        type_item: $ => seq(
            field("attributes", repeat($.attribute)),
            "type",
            field("name", $.identifier),
            "=",
            field("ty", $.ty),
            ";",
        ),

        ty: $ => choice($._builtins, $.tuple, $.list, $.option, $.result, $.identifier, $.handle),
        _builtins: $ => choice(
            "u8", "u16", "u32", "u64", "s8", "s16", "s32", "s64", "float32", "float64", "char", "bool", "string"
        ),
        tuple: $ => seq("tuple", "<", cases($.ty), ">"),
        list: $ => seq("list", "<", $.ty, ">"),
        option: $ => seq("option", "<", $.ty, ">"),
        result: $ => seq("result", optional($._result_list)),
        _result_list: $ => seq(
            "<",
            choice("_", field("ok", $.ty)),
            optional(
                seq(",", field("err", $.ty)),
            ),
            ">",
        ),

        handle: $ => choice($.borrowed_handle, $.owned_handle),
        borrowed_handle: $ => seq("borrow", "<", field("name", $.identifier), ">"),
        owned_handle: $ => seq("own", "<", field("name", $.identifier), ">"),

        attribute: $ => choice($.doc_comment),

        semver: $ => /(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?/,
        identifier: $ => /%?\w[\w\d_-]*/,
        doc_comment: $ => seq("///", /[ \t\v]*/, field("doc", /[^n]*/)),
        block_comment: $ => seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/"),
        slash_comment: $ => /\/\/[^\n]*/,
    },
});
