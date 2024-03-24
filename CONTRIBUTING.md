# Contributing

## Project Architecture

This project contains several releasable components,

- `tree-sitter-wit` - a [Tree Sitter][tree-sitter] parser for [the WIT format][wit]
- `crates/wit-compiler` - an incremental compiler for the WIT language
- `crates/wit-language-server` - the actual [Language Server][ls] implementation
- `plugins/vscode` - an extension for VS Code which uses the language server

It also contains a `crates/xtask` crate for running useful internal commands.

## Design Goals

### Goal 1: Fast Build Times

A clean debug build of the entire workspace shouldn't take any longer than 30
seconds and the entire CI workflow should finish within 10 minutes.

This isn't actually too difficult to achieve as long as you follow some
guidelines:

- Don't add dependencies unless you absolutely need to
- Trim out unnecessary features
- Periodically use `cargo clean && cargo build --timings` to see where compile
  time is spent
- Don't use dependencies that pull in half of crates.io

The rationale behind this is simple - [**a short edit-compile-test cycle is a
force multiplier**][fast-rust-builds]. If you have fast compile times then
developers can recompile and re-run the test suite after every change.

On the other hand, if CI takes 30 minutes to complete, developers will avoid
your project like the plague because getting even the most trivial changes
merged becomes a multi-hour chore.

To help this, we have [a GitHub Action][workflow-timer] which will post comments
on each PR to let you know how much your changes have affected CI times.

## Testing the VS Code Extension

You can test the VS Code extension is by using the *"Run Extension"* launch
command found in the *"Run and Debug"* panel down the side.

This should automatically compile the extension and open a new VS Code window
with the extension installed.

## Releasing

This repository uses [Release Please][release-please] to automate a lot of the
work around creating releases.

Every time a commit following the [Conventional Commit Style][conv] is merged
into `main`, the [`release-please.yml`](.github/workflows/release-please.yml)
workflow will run and update the "Release PR" to reflect the new changes.

For commits that just fix bugs (i.e. the message starts with `"fix: "`), the
associated crate will receive a changelog entry and a patch version bump.
Similarly, adding a new feature (i.e. `"feat:"`) does a minor version bump and
adding breaking changes (i.e. `"fix!:"` or `"feat!:"`) will result in a major
version bump.

When the release PR is merged, the updated changelog and bumped version number
will be merged into the `main` branch, the `release-please.yml` workflow will
automatically generate GitHub Releases, and CI will publish the package to NPM.

TL;DR:

1. Use [Conventional Commit Messages][conv] whenever you make a noteworthy change
2. Merge the release PR when ready to release
3. Let the automation do everything else

[conv]: https://www.conventionalcommits.org/en/v1.0.0/
[ls]: https://code.visualstudio.com/api/language-extensions/language-server-extension-guide
[release-please]: https://github.com/googleapis/release-please
[tree-sitter]: https://tree-sitter.github.io/tree-sitter/
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[workflow-timer]: https://github.com/Michael-F-Bryan/workflow-timer
