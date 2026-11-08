# fix(core): read a Windows path as the module it names

- **Date:** 2026-10-13 18:24
- **Commit:** 9ac9283e
- **Area:** Type registry

## Problem

The registry tells which module an import names by comparing paths, and split every path at `/`. A
path written with the separators of Windows, as the scan writes one there, was one segment: a
relative import resolved to no module, and its name was looked up as whatever the project declares
under it, so two types of one name in two files could not be told apart.

## Cause

`resolve_relative_module`, `file_path_is_module`, `file_path_ends_in_module` and the comparison of a
module's base name each assumed `/`.

## Introduced

`c729f98c` (2026-09-22, "Release 0.3.0") added the resolution of a relative import to its module,
over paths split at `/`.

## Fix

Each reads `\` as `/` before it compares, as does the comparison of two spellings of one file's
path.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`a_windows_path_names_the_module_a_slashed_one_does` resolves a relative import and an alias between
two types of one name over paths written with `\`,
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed), and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed). Nothing was run on
Windows.
