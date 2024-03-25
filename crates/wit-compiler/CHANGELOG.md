# Changelog

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
