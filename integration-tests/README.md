# Integration Tests

This directory contains the integration test suite for this repository.

Basically, it's a bunch of valid `*.wit` files collected from all over the place
that we throw at the compiler.

## Test Discovery

The `integration-tests` crate has a custom test harness which will scan this
directory for test cases and automatically test them. That means running the
tests is as simple as `cargo test` (or `cargo nextest run` for the cool kids).

You can skip a test case by adding a `_` to the start of the filename.

### Compile-Pass Tests

Any tests placed in to the `compile-pass/` directory should follow either of the
following formats:

- `*.wit` files that are raw wit files that should be executed by the code
  generator.
- wit package in it's own directory which must contain a `wit` subdirectory with
  `*.wit` files and deps in it.

A compile-pass test asks the `wit-compiler` crate to run a full analysis over
the file(s), failing if any diagnostics are emitted.

## Licensing

To help bootstrap this test suite, a large number of `*.wit` files were copied
from [`bytecodealliance/wit-bindgen`][wit-bindgen] under the Apache-2.0 license.

[wit-bindgen]: https://github.com/bytecodealliance/wit-bindgen
