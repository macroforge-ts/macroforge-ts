# fix(core): record what a lookup through an export name or a module asked

- **Date:** 2026-10-13 18:33
- **Commit:** 447152e3
- **Area:** Type registry, expansion caches

## Problem

A build keeps a cached expansion for as long as every registry lookup it made still answers the
same. Two lookups read the registry without saying so:

- The search for a declaration a module exports under another name than it is declared by. A file
  that imported `Shown` from a module with no such type kept its cached expansion after the module
  gained `export type { Late as Shown }`.
- The check that a module holds types at all, which decides whether a name it does not export may
  still be the project's type of that name. Its answer changes with the module's first type, and no
  cached expansion heard of it.

## Cause

Both walk every entry of the registry directly. Recording them as a read of the whole registry would
have been true and useless: every file that imports a type from a package makes the first lookup,
and would be expanded again on any change to any type.

## Introduced

`364955cf` (2026-10-13, "fix(core): follow a type through the modules that export it again") added
the search by export name, and `64cfb4c5` (2026-10-13, "fix(core)!: take a name a module does not
export for no type declared elsewhere") the check on the module, both on this branch.

## Fix

Two reads say what was asked and no more. `RegistryRead::ExportedAs(name)` resolves to the
declarations exported under `name` that are declared under another, and `RegistryRead::Module` to
whether the module at a path holds types. Each is recorded where its walk runs, and `resolution_of`
answers both without recording.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`a_lookup_through_an_export_name_or_a_module_records_what_it_asked` checks that both reads are
recorded and that each resolution changes with what it stands for,
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed), the CLI's own tests (82
passed), `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by
the lines that changed, in which 106 lines naming an export read were added and no other line
changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
