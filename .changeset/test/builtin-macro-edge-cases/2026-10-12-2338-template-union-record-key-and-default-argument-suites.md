# test(testground): cover template unions, record keys, defaulted arguments and indexed accesses

- **Date:** 2026-10-12 23:38
- **Commit:** a6e37e3c
- **Area:** Testground fixtures and suites, conformance snapshots

## Problem

No fixture held a union alias of template literal types, a template literal type inside a
collection, a record over anything but `string`, a generic type used with its default type
arguments, a newtype over a non-primitive under `Default`, or an indexed access to a type its file
does not import.

## Fix

`exotic-shapes.ts` gains `ExoticLabels`, `ExoticTexts` and `ExoticKeyed`, and `edge-exotics` decodes
wrong text and wrong keys for each. `generic-shapes.ts` gains `GenericDefaults` and a holder that
uses it bare, with one argument and with both, and `edge-generics` decodes a value of the wrong type
for each defaulted parameter. `edge-newtypes` reads the default of a newtype over a date, an array
and an object type. `operator-shapes.ts` imports `OperatorItem` alone and reads
`OperatorItem['owner']`, and `edge-operators` round-trips it and decodes a wrong owner.
`keyed-shapes.ts` holds records keyed by an enum and by literals, a partial record, collections of
enums and template literals, a class with inferred fields under each decorator option, and
`Partial`, `Pick`, `Omit` and `Required` over them, with `edge-keyed` decoding wrong input for each.
`exotic-shapes.ts` gains an interface declaring its fields by `get` and `set` signatures and an
object type alias with a method.

## Verified

Ran the 25 testground suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed)
and `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
shape of each changed line.
