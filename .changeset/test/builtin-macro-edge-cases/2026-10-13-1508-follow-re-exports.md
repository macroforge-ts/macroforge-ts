# fix(core): follow a type through the modules that export it again

- **Date:** 2026-10-13 15:08
- **Commit:** 64969f15
- **Area:** Type scan, type registry, expansion host imports

## Problem

A type imported through a barrel was found only by its name, when the project declares that name
once, and not at all when the barrel gives it another:

```ts
// models/index.ts
export * from './user';
export { default as Model, User as Member } from './user';
```

`import type { Member, Model } from './models'` resolved to nothing, and the generated code imported
`memberDecodeWithContext` and `modelDecodeWithContext` from `./models`, which exports neither.

## Cause

A barrel declares nothing, so the registry, which holds declarations, knew nothing of it. What a
module exports again from another was not recorded anywhere.

## Introduced

No single change: re-exports have not been recorded by any version of the scan.

## Fix

The scan records what each module exports again (`export { A as B } from`,
`export { default as
A } from`, `export * from`) and links it to the declarations it leads to, round
by round until nothing is carried further, so a chain of barrels, barrels that export each other,
and barrels that reach one module by many ways are each read once a round. A registry entry carries
the names other modules give it (`TypeRegistryEntry::re_exported`), in the wire form too, and an
import resolves through them.

A barrel that exports all of a module (`export *`, all the way) brings the type's generated
functions with it. One that names the type brings the type alone, so the functions are imported from
the module that declares it.

The first version of the linker kept only the answers asked at the top, and read a module again for
each way it was reached: a commit security review found that this grows with the number of ways, not
of modules. The fixup replaces it with the round-by-round pass.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed),
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`an_import_is_followed_through_the_modules_that_export_it_again` scans a project of barrels and
`barrels_that_reach_a_module_by_many_ways_are_linked_once` links forty layers of two,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `edge-names` decodes three types held through a barrel.
