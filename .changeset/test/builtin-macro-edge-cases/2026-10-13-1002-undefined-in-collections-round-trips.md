# fix(endec): bring an undefined held in a collection back as one

- **Date:** 2026-10-13 10:02
- **Commit:** 7a914a1c
- **Area:** Built-in Encode and Decode derives, collections

## Problem

A value holding `undefined` in a collection did not survive its own round trip:

- `list: (number | undefined)[]` holding `[1, undefined, 3]` encoded to `[1,null,3]`, and Decode
  refused it: "list[1]: expected number | undefined". A tuple and a `Set` failed the same way, as
  did an optional tuple element that is there and `undefined`.
- `held: Map<string, number | undefined>` holding an entry `a → undefined` decoded to a map without
  it, and a `Record` lost the key. A `Record<'a' | 'b', T | undefined>` then failed on the key that
  was dropped: "missing required field".

## Cause

JSON has no `undefined`: `JSON.stringify` writes one in an array as `null` and drops an object's key
that holds one. Decode checked each element against its type, which takes `undefined` and not
`null`, and nothing wrote the entry of a `Record` or a `Map` so that it would last.

## Introduced

5a7366fa (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on this
branch, made Decode check every element of a collection against its type, which is where the `null`
JSON writes became an error. The dropped entry of a `Record` and a `Map` is older: it dates from
dafd08ef (2025-12-07, "Add Serialize and Deserialize derive macros for JSON support"), which handed
them to `JSON.stringify` as they are.

## Fix

Where the type of an element, of an optional tuple element or of an entry's value takes `undefined`
and not `null`, Decode reads a `null` as `undefined`, and Encode writes the `undefined` value of a
`Record` or `Map` entry as `null`, so the entry stays. A type that takes `null` itself keeps it:
there an `undefined` comes back as `null`, which JSON cannot tell from it. A `null` where the type
takes neither is refused as before. Whether a type takes `null` is asked through a member in
parentheses as well, so `(string | null)?` keeps its `null`.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`an_undefined_in_a_collection_is_coded_as_null` covers an array, a record, a map, an optional tuple
element and the types that take `null` themselves, `pixi run test:conformance:update` (10 passed),
with the regenerated snapshots reviewed by the lines that changed, and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`edge-specials` round-trips nine values holding `undefined`, seven of which had failed to decode or
come back unequal.
