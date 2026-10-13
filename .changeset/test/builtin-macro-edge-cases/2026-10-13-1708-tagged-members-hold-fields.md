# fix(endec)!: refuse a tagged union member over a type with no fields

- **Date:** 2026-10-13 17:08
- **Commit:** 200d01c6
- **Area:** Encode and Decode of union aliases

## Problem

A union member `{ kind: 'when' } & Date`, or one over a foreign type or a primitive, decoded as
`{ ...value, kind: 'when' }`: the input handed back with the tag written on it, unchecked and
undecoded, as a value of the union.

## Cause

The decoder took a tagged member's type for one with a decoder of its own unless it was a primitive,
a `Date`, a type parameter or a foreign type, and for those it had the spread and nothing else.

## Introduced

`9fdd7585` (2026-04-08, "Syncing monorepo") carries the spread of the input for every tagged member.
`c5c98dc2` (2026-07-20, "fix(serde): dispatch union-alias (de)serialization for plain-object
variants") sent the members with a decoder through it and left the spread for the rest.

## Fix

A tag stands beside the fields of a type that has fields of its own. `tagged_member` refuses a
member whose type is a primitive, a `Date`, a collection or a foreign type, with an error from
Encode and from Decode. A type parameter is still taken, and decodes through the decoder handed in
for it.

**Breaking:** a union with such a member no longer expands.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `NoFields` union of the `union_intersection_member_unread` fixture, refused
for its `Date`, and in which no snapshot of either testground changed, and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
