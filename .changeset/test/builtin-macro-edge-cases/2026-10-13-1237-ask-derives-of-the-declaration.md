# fix(core): ask what a type derives by the declaration its file's imports lead to

- **Date:** 2026-10-13 12:37
- **Commit:** 7cdadbe4
- **Area:** Value traits, Default, Encode and Decode, type registry lookups

## Problem

For a field whose type is imported under another name, the derives took the type for one that
derives nothing:

- `PartialEq`, `Hash`, `Clone` and `Debug` compared, hashed, copied and printed it structurally, not
  through its own functions, so a field its own derive leaves out, `@partialEq({ skip: true
  })`,
  was compared all the same.
- `Default` refused `Pick<Piece, 'key'>`, for `import type { Part as Piece }`, as "a selection of
  fields that are not known here".

## Cause

Whether a type derives a macro was asked of the registry by the name the file holds the type under
(`type_has_derive(registry, name, ...)`), which is no name the registry knows for a type imported
under another. The check `Default` makes of a `Pick` or a `Required` resolved its names with no file
at all.

## Introduced

65c22961 (2026-04-07, "Syncing monorepo") is the earliest commit in this repository holding
`type_has_derive` by name. That a renamed import resolves to a declaration at all dates from this
branch, which is when the two answers came apart.

## Fix

`declared_type_has_derive` asks the declaration the file's imports lead to, and falls back on the
name where they lead nowhere. The value traits ask the declaration a field's type was resolved to.
`Default`, `Encode`, `Decode` and a class's check of its superclass use the file they are expanding
in, and `undefaultable` takes it.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which the value traits of the types holding a renamed import call its functions
where they called the structural helpers, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which `edge-names`
finds two values equal, and of one hash, that differ in a field their type leaves out of its
equality, held under the name the type is exported as.
