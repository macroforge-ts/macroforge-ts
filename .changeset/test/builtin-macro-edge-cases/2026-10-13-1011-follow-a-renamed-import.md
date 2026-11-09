# fix(core): find a type imported under another name by the name it was exported as

- **Date:** 2026-10-13 10:11
- **Commit:** 0cba024a
- **Area:** Expansion host imports, type registry

## Problem

A derived type holding a type imported under another name did not type-check:

```ts
import type { ZzItem as Item } from './source';

/** @derive(Encode, Decode, Default) */
export interface User {
    held: Item;
}
```

"'./source' has no exported member named 'itemDecodeWithContext'. Did you mean
'zzItemDecodeWithContext'?", and the same for `itemEncodeWithContext` and `itemDefaultValue`.

## Cause

The generated code names another type's functions after the type as the file names it, and the host
imported each under that very name from the type's module, which exports them under the type's own
name.

`TypeRegistry::resolve_in_file` looked a name up across the whole project first and read the file's
imports only for a name declared more than once. So a renamed import resolved to nothing, or to an
unrelated type that the project declares under the local name.

## Introduced

65c22961 (2026-04-07, "Syncing monorepo") is the earliest commit in this repository holding the
import of a type's functions by the local name. e65d408c (2026-09-27, "svelte-package: re-expand
only the modules a change affects") holds the lookup that takes an unambiguous name before the
file's imports.

## Fix

The host hands the import step the name each import was exported under, and a function is imported
under that name as the one the generated code calls:
`import {
zzItemDecodeWithContext as itemDecodeWithContext }`. `resolve_in_file` asks for the
exported name of a renamed import, and no longer falls back to a type of the local name declared
elsewhere.

A type its module exports under another name, and a default import, are followed by a later change
on this branch.

## Verified

Type-checked seven ways of importing a derived type from another module with `macroforge tsc`: a
renamed import failed before the fix and passes after it; a default import, a barrel's `export *`
and a path with an extension pass; a renamed export and a renamed default re-export still fail. Ran
`cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`a_renamed_import_resolves_to_the_type_it_was_exported_as` covers the lookup,
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_renamed_import_brings_its_functions_under_the_exported_name` covers the imports,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `edge-names` encodes, decodes, clones and defaults a type holding
`EdgeItem` imported as `Item`.
