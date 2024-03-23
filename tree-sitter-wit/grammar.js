module.exports = grammar({
    name: "wit",
    word: $ => $.identifier,
    extras: $ => [/[\s\n\t]/, $.slash_comment, $.block_comment],
    conficts: $ => [[$.package_name]],

    rules: {
        source_file: $ => seq(optional($.package_decl), repeat($.top_level_item)),
        package_decl: $ => seq(
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
        package_path: $ => seq($.identifier, repeat(seq(token.immediate("/"), $.identifier))),
        top_level_item: $ => choice($.top_level_use_item),

        top_level_use_item: $ => seq(
            "use",
            $.use_path,
            optional(seq("as", field("alias", $.identifier))),
            ";",
        ),
        use_path: $ => choice(
            $.identifier,
            seq(
                field("package", $.package_name),
                token.immediate(":"),
                field("path", $.package_path),
                optional(seq(token.immediate("@"), field("version", $.semver))),
            ),
        ),

        semver: $ => /(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?/,
        identifier: $ => /\w[\w\d_-]*/,
        block_comment: $ => /\/\*(?:\*|[^*])*\*\//,
        slash_comment: $ => /\/\/[^\n]*/,
    },
});
