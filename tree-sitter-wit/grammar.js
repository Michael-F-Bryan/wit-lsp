module.exports = grammar({
    name: "wit",
    word: $ => $.identifier,
    extras: $ => [/[\s\n\t]/, $._slash_comment, $._block_comment],

    rules: {
        source_file: $ => seq(optional($.package_decl), repeat($.top_level_item)),
        package_decl: $ => seq(
            "package",
            $.fully_qualified_package_name,
            ";",
        ),
        fully_qualified_package_name: $ => seq(
            field("package", $.package_name),
            prec(1, token.immediate(":")),
            field("path", $.package_path),
            optional(seq(token.immediate("@"), field("version", $.semver)),
            )
        ),
        package_name: $ => prec.left(seq($.identifier, repeat(seq(token.immediate(":"), $.identifier)))),
        package_path: $ => seq($.identifier, repeat(seq(token.immediate("/"), $.identifier))),
        top_level_item: $ => choice(),

        semver: $ => /(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?/,
        identifier: $ => /\w[\w\d_-]*/,
        _block_comment: $ => /\/\*(?:\*|[^*])*\*\//,
        _slash_comment: $ => /\/\/[^\n]*/,
    }
});
