# fix(core): read a generated name outside ASCII as one identifier

- **Date:** 2026-10-13 16:13
- **Commit:** 17964c7e
- **Area:** Expansion host

## Problem

A type named with letters outside ASCII, such as `Ünïcode` or `ΑΒΓ`, or with a `$`, got its
functions but not the object that groups them: no `export const Ünïcode = { clone, decode, … }`. A
text holding `ünïcodeClone` was also taken to name `codeClone`, so a file that imports a type `Code`
beside it imported `codeClone` for a call that is not there.

## Cause

The host read identifiers as ASCII letters, digits and `_`. The scan for the functions a derive
generated stopped a name at its first other letter, and the list of identifiers a text holds split
it there.

## Introduced

`9ad2d98a` (2025-12-14, "Add naming module and switch defaultValue calls to naming-style functions")
wrote the identifier test over ASCII, and `1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with
Rust-shaped derives and shared foreign handlers") the function scan with a test of its own.

## Fix

`is_ident_char` takes a letter or digit of any script, `_` and `$`, and the function scan and the
export check use it in place of their own tests.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_function_named_outside_ascii_is_read_whole` covers three names,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `type_names_outside_ascii` fixture, whose two types each get their object, and
the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed). A file calling `$dollarClone` itself expands with the call left alone.
