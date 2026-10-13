# fix(core)!: take a name a module does not export for no type declared elsewhere

- **Date:** 2026-10-13 15:31
- **Commit:** 64cfb4c5
- **Area:** Type registry

## Problem

```ts
import type { User } from './other';
```

where `./other` declares types but no `User`, and one other module of the project declares a `User`,
resolved to that `User`. A derive then called its generated functions, and decided what the field
can do, by a type the file never names.

## Cause

`TypeRegistry::resolve_in_file` asked the module the import names first, and when that module
exported no type of the name it fell to the lookup for a name the project declares once.

## Introduced

`65c22961` (2026-04-07, "Syncing monorepo") brought the registry in with the lookup by a name
declared once ahead of the import. `364955cf` on this branch put the import's module first and kept
the lookup by name behind it.

## Fix

A relative import whose module the registry holds types of, declared there or exported again by it,
resolves to what that module exports under the name, or to nothing. A module the scan read no types
in is not known to export none, so a name imported from it is still looked up as the project
declares it.

**Breaking:** a file that relied on the wrong type being found, a derive on a field typed by a name
its module does not export, now treats the field's type as one the project does not declare.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`a_name_a_known_module_does_not_export_is_no_type_declared_elsewhere` covers a module that declares
types, one that only exports them again, and one the scan read none in,
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which no snapshot of the testground changed, and the three groups on the branch's
final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
