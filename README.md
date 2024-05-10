# wit-lsp

[![Continuous Integration](https://github.com/Michael-F-Bryan/wit-lsp/actions/workflows/ci.yml/badge.svg)](https://github.com/Michael-F-Bryan/wit-lsp/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/Michael-F-Bryan/wit-lsp/graph/badge.svg?token=0HHP8EX5UY)](https://codecov.io/gh/Michael-F-Bryan/wit-lsp)

([API Docs][api-docs] | [Code Coverage][coverage] | [Error Code Index][error-codes])

An alternative Language Server implementation for [WIT][wit].

## Features

- [x] Basic Syntax Highlighting
- [x] Code Folding
- [x] Show Syntax Tree
- [x] Selection range
- [x] Diagnostic reporting
- [ ] Semantic Syntax Highlighting
- [x] Semantic Autocomplete
- [ ] Jump to Definition
- [ ] Jump to Declaration
- [ ] Hover
- [ ] View Memory Layout
- [ ] Show Dependency Tree
- [ ] Find All References
- [ ] Workspace Symbol
- [ ] Assists
  - [x] Extract type alias
  - [ ] Generate documentation template
  - [ ] Autoimport
  - [ ] Rename
  - [ ] Inline type alias
  - [ ] Merge imports
  - [ ] Sort items

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE.md) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT.md) or
   <http://opensource.org/licenses/MIT>)

at your option.

It is recommended to always use [`cargo crev`][crev] to verify the
trustworthiness of each of your dependencies, including this one.

> **Note:** The language configuration, code snippets, and TextMate grammar for
> the VS Code extension have been copied from
> [`bytecodealliance/vscode-wit`][vscode-wit] and are made available under the
> same Apache-2.0 license.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

The intent of this crate is to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help in analysing and fixing them.

[api-docs]: https://michael-f-bryan.github.io/wit-lsp/crate-docs
[coverage]: https://michael-f-bryan.github.io/wit-lsp/coverage
[crev]: https://github.com/crev-dev/cargo-crev
[error-codes]: https://michael-f-bryan.github.io/wit-lsp/error-codes.html
[vscode-wit]: https://github.com/bytecodealliance/vscode-wit
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
