# wit-lsp

[![Continuous Integration](https://github.com/Michael-F-Bryan/wit-lsp/actions/workflows/ci.yml/badge.svg)](https://github.com/Michael-F-Bryan/wit-lsp/actions/workflows/ci.yml)

([API Docs][api-docs])

An alternative Language Server implementation for [WIT][wit].

## Features

- [x] Basic Syntax Highlighting
- [x] Code Folding
- [x] Show Syntax Tree
- [x] Selection range
- [ ] Semantic Syntax Highlighting
- [ ] Semantic Autocomplete
- [ ] Jump to Definition
- [ ] Jump to Declaration
- [ ] Hover
- [ ] View Memory Layout
- [ ] Show Dependency Tree
- [ ] Find All References
- [ ] Workspace Symbol
- [ ] Assists
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

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

The intent of this crate is to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help in analysing and fixing them.

[api-docs]: https://michael-f-bryan.github.io/wit-lsp
[crev]: https://github.com/crev-dev/cargo-crev
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
