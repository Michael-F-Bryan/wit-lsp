# Changelog

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

## Change Log

All notable changes to the "wit-language-server" will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.
