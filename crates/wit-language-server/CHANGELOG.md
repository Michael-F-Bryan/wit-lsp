# Changelog

All notable changes to the `wit-language-server` will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## [0.2.0](https://github.com/Michael-F-Bryan/wit-lsp/compare/wit-language-server-v0.1.0...wit-language-server-v0.2.0) (2024-04-11)


### Features

* Added a `wit-language-server -vV` flag which will print verbose version info, similar to `rustc -vV` ([bc46605](https://github.com/Michael-F-Bryan/wit-lsp/commit/bc4660541ef6e7b4a3d8f09c8549578f658e4e4d))
* Implemented realtime diagnostic reporting ([fc9b233](https://github.com/Michael-F-Bryan/wit-lsp/commit/fc9b233952980186e658f8e7aa55a99c6355bb6b))
* Implemented semantic code completion ([5926700](https://github.com/Michael-F-Bryan/wit-lsp/commit/59267009257d91822bf456a0d236d350ab62c531))
* Instrument all queries using tracing ([396c89b](https://github.com/Michael-F-Bryan/wit-lsp/commit/396c89b70cd03f87a1d165edb8fc20fc370b88c9))


### Bug Fixes

* Fixed a panic that would happen when calculating code folding for a file with comments ([26b054c](https://github.com/Michael-F-Bryan/wit-lsp/commit/26b054c5e99937c9c8f2dfc86781bc825123c47f))
* Panics will no longer cause the language server to crash ([d47c664](https://github.com/Michael-F-Bryan/wit-lsp/commit/d47c6646cdb771710750ad8f56028666fbac1758))
* The default log level has been reduced to avoid unnecessary output ([5ddcf7b](https://github.com/Michael-F-Bryan/wit-lsp/commit/5ddcf7b253ebf2b3342e779855e7dcbf13e0c68a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * tree-sitter-wit bumped from 0.1.0 to 0.2.0
    * wit-compiler bumped from 0.1.0 to 0.2.0

## 0.1.0 (2024-03-25)


### Features

* Added a middleware that logs each Language Server request ([5bb3a5c](https://github.com/Michael-F-Bryan/wit-lsp/commit/5bb3a5cac2f5f05f0a5275cea9d9035dc3d8cc19))
* Created a language server executable ([fc378bd](https://github.com/Michael-F-Bryan/wit-lsp/commit/fc378bd99d6f6b3f1f060ac389b487fdbeb1e5a1))
* Implemented selection range ([950b19c](https://github.com/Michael-F-Bryan/wit-lsp/commit/950b19c83ad56a8d6e678b4425d7a4a3bac96ead))
* The language server now uses file open/change/save events to keep track of the contents of the workspace ([7fc5f4a](https://github.com/Michael-F-Bryan/wit-lsp/commit/7fc5f4a6c8b5313a4dab8b17cc9f07b185ae9629))


### Bug Fixes

* Resolved a theoretical deadlock in the language server's state management ([26ac1ad](https://github.com/Michael-F-Bryan/wit-lsp/commit/26ac1addb7deaf1fbb88e78ab01b73fc15d79722))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * tree-sitter-wit bumped from 0.0.0 to 0.1.0
    * wit-compiler bumped from 0.0.0 to 0.1.0
