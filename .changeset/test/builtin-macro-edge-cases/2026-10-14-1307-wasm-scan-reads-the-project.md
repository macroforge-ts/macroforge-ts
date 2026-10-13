# fix(core): scan a project through Node's fs in the WebAssembly engine

- **Date:** 2026-10-14 13:07
- **Commit:** 5a734dc3
- **Area:** `scanProjectSync` of `@macroforge/core`

## Problem

`scanProjectSync` of the engine the integrations load returned an empty type registry for any
project, with a warning in its diagnostics, so an expansion given that registry knew no type
declared in another file.

## Cause

The scan walked the project with the `ignore` crate, which reads the filesystem through `std::fs`,
and the WebAssembly build has none: every entry failed to read. The files it would have found were
read the same way.

## Introduced

`65c22961` (2026-04-07, "Syncing monorepo") exported the scan from the WebAssembly build over the
native walk.

## Fix

The WebAssembly build walks the project through Node's `fs`, as it already reads files and resolves
modules: in path order, with hidden entries and the configured directories left out and, inside a
git repository, what each directory's `.gitignore` names, as the native walk does. The files are
read the same way.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), a check of the
WebAssembly build, and the three groups on the branch's final tree: `pixi run test:rust` (the
workspace and `tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites, 13
validator files of 638 steps and 236 browser tests in Chromium, all passed) and
`pixi run test:packages` (the six packages passed), in which `scan-project` scans a project of a
nested file, a gitignored file, a hidden directory and `node_modules` with the engine the
integrations load, and finds the two types it should and none of the three it should not.
