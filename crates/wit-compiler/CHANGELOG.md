# Changelog

All notable changes to `wit-compiler` will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## [1.0.0](https://github.com/Michael-F-Bryan/wit-lsp/compare/wit-compiler-v0.2.0...wit-compiler-v1.0.0) (2024-05-10)


### ⚠ BREAKING CHANGES

* Switched over to a different version of `tree-sitter-wit`
* Introduced a `NodeKind` abstraction and re-worked the entire `wit_compiler::access` module
* Made a `package_items()` query for listing information about all items defined in a package
* Converted `FilePath` to an interned type

### Features

* Added a `calculate_line_numbers()` query ([fcda3f2](https://github.com/Michael-F-Bryan/wit-lsp/commit/fcda3f26d29237616305becd061e64a18a3d1698))
* Added a convenient `wit_compiler::diagnostics::check_all()` function to make checking a workspace easier ([2ab531a](https://github.com/Michael-F-Bryan/wit-lsp/commit/2ab531acf6decd0fef1a5fb556944a0c547147fb))
* Added an adapter which lets you use a `Workspace` as a `codespan_reporting::files::Files` ([fcda3f2](https://github.com/Michael-F-Bryan/wit-lsp/commit/fcda3f26d29237616305becd061e64a18a3d1698))
* Converted `FilePath` to an interned type ([fcda3f2](https://github.com/Michael-F-Bryan/wit-lsp/commit/fcda3f26d29237616305becd061e64a18a3d1698))
* Introduced a `DiagnosticInfo` type with metadata about a diagnostic (error code, explanatory text, etc.) and a `wit_compiler::diagnostics::all()` function for listing all known diagnostics ([c712eb4](https://github.com/Michael-F-Bryan/wit-lsp/commit/c712eb41908dc30d85bcba9f1fde293f8135a079))
* Introduced a `IntoDiagnostic` trait that all `Diagnostic` types should implement ([c712eb4](https://github.com/Michael-F-Bryan/wit-lsp/commit/c712eb41908dc30d85bcba9f1fde293f8135a079))
* Introduced a `NodeKind` abstraction and re-worked the entire `wit_compiler::access` module ([22ab0c1](https://github.com/Michael-F-Bryan/wit-lsp/commit/22ab0c194698981f2571128866847c7f50ce9bb2))
* Introduced the concept of lints ([2ab531a](https://github.com/Michael-F-Bryan/wit-lsp/commit/2ab531acf6decd0fef1a5fb556944a0c547147fb))
* Made a `package_items()` query for listing information about all items defined in a package ([8d57e21](https://github.com/Michael-F-Bryan/wit-lsp/commit/8d57e219f9490600630f3b56030bc1873fa3470d))
* Switched over to a different version of `tree-sitter-wit` ([c97da9f](https://github.com/Michael-F-Bryan/wit-lsp/commit/c97da9fad65a76c22a6221812f4b128926d40143))
* The repo now contains a `diagnostics.json` file with metadata for all known diagnostics ([c712eb4](https://github.com/Michael-F-Bryan/wit-lsp/commit/c712eb41908dc30d85bcba9f1fde293f8135a079))


### Bug Fixes

* A `named_type` node (e.g. a function parameter or named return value) can now have attributes ([e0cf9c9](https://github.com/Michael-F-Bryan/wit-lsp/commit/e0cf9c927a559136b8d33baf6be5e82cf76e4dcd))
* The leading `%` from a raw identifier is now correctly ignored when looking up a name ([7ac76fe](https://github.com/Michael-F-Bryan/wit-lsp/commit/7ac76fe29d3dd21366044ed54491d2015aa7b447))

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
