module.exports = grammar({
    name: "wit",

    extras: $ => [/[\s\n\t]/, $.comment],

    rules: {
        program: $ => repeat(choice($.comment)),
        comment: $ => /\/\/[^\n]*/,
    }
});
