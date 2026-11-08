# test(testground): cover colliding keys and names, special numbers, field kinds and enums

- **Date:** 2026-10-12 21:45
- **Commit:** 3d1d840a
- **Area:** Testground fixtures and suites, conformance fixtures and snapshots

## Problem

Nothing exercised the derives on `NaN`, the infinities and invalid dates, on inputs with inherited
members, on fields named like `Object.prototype`'s, on fields typed by their initializers or
declared with `accessor`, or on `const` enums and enums whose values are other members' names. No
fixture pinned the errors for colliding keys and names, `__proto__`, `declare` enums, `accessor`
fields a derive would build, or fields left untyped.

## Fix

`special-shapes.ts` with `edge-specials.test.mjs`, and new cases in `edge-names`, `edge-classes` and
`edge-enums` over `NamePrototype`, `ClassInferred`, `ClassAccessors`, `EnumConstant` and
`EnumCrossed`. Conformance error fixtures `endec_key_collisions`, `generated_name_collisions`,
`prototype_named_field`, `class_accessor_field_built`, `endec_untyped_field` and `declare_enum`. The
regenerated conformance snapshots carry the keyed writes, the own-member reads and the
member-by-value `Debug`.

## Verified

Ran the 25 testground suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed)
and `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
shape of each changed line.
