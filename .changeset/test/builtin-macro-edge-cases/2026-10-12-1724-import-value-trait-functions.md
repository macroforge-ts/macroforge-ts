# fix(core): import a derived type's value-trait functions from its module

- **Date:** 2026-10-12 17:24
- **Commit:** 4076610d
- **Area:** Expansion host, generated imports

## Problem

A field holding a type from another module, under `Debug`, `Clone`, `PartialEq`, `Hash`,
`PartialOrd` or `Ord`, generated a call to that type's function without importing it:
`items: Item[]` with `Item` imported from `./item` expanded to `value.items.map(itemClone)` and
failed with "Cannot find name 'itemClone'".

## Cause

The names the host imports alongside an imported type were the `Encode`, `Decode` and `Default`
functions only.

## Introduced

1cc2f3c4 (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") made the value traits call a field type's derived function. The list of names the host
imports, from 194f9c6f (2026-02-27, "Overhaul import system with unified ImportRegistry and simplify
config loading"), was left with the `Encode`, `Decode` and `Default` names.

## Fix

The list also carries `{name}ToString`, `{name}Clone`, `{name}Equals`, `{name}HashCode`,
`{name}PartialCompare` and `{name}Compare`. A name is imported only when the generated code uses it.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed)
and `pixi run test:rust` (workspace and `tooling/testground/macro` passed).
