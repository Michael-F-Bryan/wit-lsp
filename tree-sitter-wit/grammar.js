module.exports = grammar({
    name: "WIT",

    extras: $ => [/[\s\n\t]/, $.comment],

    rules: {
        program: $ => repeat(choice($.comment)),
        comment: $ => /\/\/[^\n]*/,
    }
});
