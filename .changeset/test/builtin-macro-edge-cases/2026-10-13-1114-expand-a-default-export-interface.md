# fix(core): expand an interface that is a default export

- **Date:** 2026-10-13 11:14
- **Commit:** 043663c7
- **Area:** Lowering, expansion host

## Problem

`@derive` above `export default interface Shape` generated nothing, and above
`export default
class {` reported "this `@derive` is on a type declared inside a namespace or a
block", which it is not.

## Cause

Lowering read a default export when it is a class or a function, and passed an interface by, so the
derive stood above nothing a derive expands. The error for such a derive told a type that was not
lowered only as one nested in a namespace or a block.

## Introduced

65c22961 (2026-04-07, "Syncing monorepo") is the earliest commit in this repository holding the
lowering of a default-exported class without the interface beside it. The message dates from
da8f0fda (2026-10-13, "fix(core)!: report an @derive that expands nothing or reads part of a type"),
on this branch.

## Fix

A default-exported interface is lowered like any other, and counts among the names a module exports.
An `@derive` on a class without a name says that the class has none for its generated functions to
take.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed),
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `default_export_interface` and `unnamed_default_class` fixtures, a type-check
of a default-exported interface beside the constant of its name, and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
