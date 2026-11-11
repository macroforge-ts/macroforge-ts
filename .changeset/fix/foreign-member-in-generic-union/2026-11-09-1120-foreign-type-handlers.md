# feat(core)!: configure a foreign type's handler for every function the built-ins call on it

- **Date:** 2026-11-09 11:20
- **Commit:** dc3e6817
- **Area:** foreign type config, the value-trait derives, endec `Encode` union generator, docs

## Problem

A foreign type could configure `encode`, `decode`, `default` and `hasShape` only. `Clone`,
`PartialEq`, `Hash`, `Debug`, `PartialOrd` and `Ord` never read the config: a field of a foreign
type was copied, compared, hashed and printed by its structure, whatever the type's own notion of
those is. The union encoder told a foreign member apart by `hasShape`, which also takes the type's
wire forms, so a plain object shaped like an encoded `RecordId` would have been encoded as one.

## Cause

The config was written for the coders alone, and a value trait handled every type it did not find
deriving it as structural.

## Introduced

`1cc2f3c4b` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") gave the foreign handlers their four keys.

## Fix

A foreign type configures `is`, `clone`, `toString`, `equals`, `hashCode`, `partialCompare` and
`compare` beside the four it had, each exported from the expanded config like them. Each value trait
calls its handler on a field of the type, nullable or not, and on an array's elements under `Debug`;
a field whose type configures no handler the derive needs is an expansion error naming it. The union
encoder tells a foreign member apart by `is`; a built-in foreign type's `is` is its `instanceof` or
`typeof`. The config loader takes every handler key in one place. The testground configs and the
docs list the new handlers.

## Verified

`cargo test -p macroforge_ts --features test-macros --lib`: 780 passed, among them a test that every
handler key is read from a config, one that each value trait calls its handler on a plain and a
nullable field, one that each refuses a type without its handler, and the union encoder's tests now
on `is`.

`pixi run build:mf && pixi run verify` passed every step, with the testground conformance snapshots
regenerated for the new handlers and the union encoder's `is` test, and the MCP server's docs
regenerated from the handler docs.
