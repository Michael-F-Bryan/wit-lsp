# Changelog

All notable changes to `wit-compiler` will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## [0.2.0](https://github.com/Michael-F-Bryan/wit-lsp/compare/wit-compiler-v0.1.0...wit-compiler-v0.2.0) (2024-04-11)


### Features

* Added a query that extracts the names and pointers for all items in a file ([59318aa](https://github.com/Michael-F-Bryan/wit-lsp/commit/59318aab20371a8ab7bbb9800c3035edea9e1fc3))
* Automatically implement `HasIdent` and `HasAttr` for strongly-typed AST nodes ([70cfd37](https://github.com/Michael-F-Bryan/wit-lsp/commit/70cfd37f3a0e488b6dd19d2f80e088f702aff313))
* Created a `GetByIndex` trait which makes it easier to navigate the HIR ([f823461](https://github.com/Michael-F-Bryan/wit-lsp/commit/f8234610d14ab2faff04f0348fc52cb034fe4255))
* Implemented a `hover_info()` query ([f823461](https://github.com/Michael-F-Bryan/wit-lsp/commit/f8234610d14ab2faff04f0348fc52cb034fe4255))
* Instrument all queries using tracing ([396c89b](https://github.com/Michael-F-Bryan/wit-lsp/commit/396c89b70cd03f87a1d165edb8fc20fc370b88c9))
* Parsing now generates syntax error diagnistics ([c41877e](https://github.com/Michael-F-Bryan/wit-lsp/commit/c41877e65f6a359d9b2dc61f45ce349ae5b81dd7))
* Syntax errors are emitted while parsing ([c6003b3](https://github.com/Michael-F-Bryan/wit-lsp/commit/c6003b3bcc06d23550d784198c63eb9f2efd8f6b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * tree-sitter-wit bumped from 0.1.0 to 0.2.0

## 0.1.0 (2024-03-25)


### Features

* Added queries for syntax highlighting ([91fef15](https://github.com/Michael-F-Bryan/wit-lsp/commit/91fef1530e437ed78112ba736fe80f5f83d7cad5))
* Created a `parse()` query and `Ast` wrapper around a Tree Sitter tree ([0257819](https://github.com/Michael-F-Bryan/wit-lsp/commit/025781938787c83b1bb8ae3ddc8b2f35bba85c20))
* Created an abstraction for a workspace ([0257819](https://github.com/Michael-F-Bryan/wit-lsp/commit/025781938787c83b1bb8ae3ddc8b2f35bba85c20))
* Generated strongly-typed AST nodes ([4a4005d](https://github.com/Michael-F-Bryan/wit-lsp/commit/4a4005d873aafd7649250a60d090ddc5e2212ffa))
* Implemented block comment parsing ([0794f11](https://github.com/Michael-F-Bryan/wit-lsp/commit/0794f11175734f39ac8dfe77177eceeb41ebe35d))
* Implemented selection range ([950b19c](https://github.com/Michael-F-Bryan/wit-lsp/commit/950b19c83ad56a8d6e678b4425d7a4a3bac96ead))


### Bug Fixes

* We can now correctly parse all functions, resources, and types ([6e98229](https://github.com/Michael-F-Bryan/wit-lsp/commit/6e982299086c58d119f19e7cffd3b8fef4e78635))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * tree-sitter-wit bumped from 0.0.0 to 0.1.0
