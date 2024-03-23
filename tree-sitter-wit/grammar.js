module.exports = grammar({
    name: "wit",

    extras: $ => [/[\s\n\t]/, $.comment],

    rules: {
        program: $ => repeat(choice(
        )),
        comment: $ => choice($._block_comment, $._slash_comment),
        _block_comment: $ => /\/\*(?:\*|[^*])*\*\//,
        _slash_comment: $ => /\/\/[^\n]*/,
    }
});
