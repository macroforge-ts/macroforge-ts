# test(testground): cover unusual field types and the numbers decode refuses

- **Date:** 2026-10-12 22:08
- **Commit:** c3298abc
- **Area:** Testground fixtures and suites, conformance fixtures and snapshots

## Problem

No fixture held a template literal type, `object`, a lone `null`, a union of literals of several
kinds, an empty tuple, interface or class, or an interface with methods, and nothing decoded a
non-finite number or JSON text that spells one.

## Fix

`exotic-shapes.ts` with `edge-exotics.test.mjs`, a case in `edge-specials` that decodes `NaN`, the
infinities and overflowing literals as values and as text, and the conformance error fixture
`endec_uncodable_types`. `edge-newtypes` codes newtypes over a date, an array, an object type and a
tuple, and `endec_unread_intersection` pins the intersection error. The `class_accessor_field_built`
fixture now holds a class each for the derives that build an instance.

## Verified

Ran the 25 testground suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed)
and `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
shape of each changed line.
