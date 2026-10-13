# fix(endec): refuse a Map or a Set where an object's members are read

- **Date:** 2026-10-13 12:19
- **Commit:** 628e7a82
- **Area:** Endec runtime, `decode`

## Problem

A `Map` instance handed to `decode` where a `Map`, a `Record` or an object is expected decoded
without an error to an empty one: `lookup: Map<string, number>` given `new Map([['a', 1]])` came
back as an empty map. A `Set` given for a record did the same.

## Cause

The decoders of a map, a record and an object read the input's own enumerable members. A `Map` and a
`Set` are objects that keep what they hold elsewhere, so they have none, and were read as empty
objects.

## Introduced

5a7366fa (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on this
branch, added the runtime decoders with the test that takes any object that is not an array.

## Fix

`decodeMap`, `decodeRecord`, `decodeObject` and `ownMembers`, through which a type's own fields are
read, refuse a `Map` and a `Set`: "expected an object". A `Set` field already refused a `Set`, as it
reads an array. What `decode` takes for a map is still the object `encode` writes.

## Verified

Handed `decode` eight values that never were JSON: an already decoded value, a `Date`, a `bigint`, a
class instance, a frozen and a null-prototype object decode, a `Set` for a set is refused, and a
`Map` for a map decoded to an empty one before the fix. Ran the three groups on the branch's final
tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`edge-hostile` hands a `Map` and a `Set` in for a map, a record, an object, a set and the root, and
expects each refused.
