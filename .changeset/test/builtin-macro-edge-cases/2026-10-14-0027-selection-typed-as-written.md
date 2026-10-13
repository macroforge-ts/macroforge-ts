# fix(endec): type a decoded selection of a type's fields as written

- **Date:** 2026-10-14 00:27
- **Commit:** 54f8253f
- **Area:** Decode of `Pick`, `Omit`, `Partial` and `Required` fields

## Problem

```ts
/** @derive(Decode) */
export interface Patches {
    ids: Pick<Item, 'id'>[];
    wholes: Required<Options>[];
}
```

The decoder gave `ids` an array of `Partial<Item>` and `wholes` an array of `Options`. Typed by its
fields, the value the decoder fills in does not type-check: "Type 'Partial<Item>[]' is not
assignable to type 'Pick<Item, "id">[]'".

## Cause

A selection of a type's fields is decoded by the type's partial decoder, or by its whole decoder
after the keys `Required` demands are checked, and the call took the result type that decoder
declares. The value the fields are decoded onto is untyped, which hid the difference.

## Introduced

`662c47a8` (2026-10-12, "fix(endec)!: code what a field's type arguments, Required and shared values
left uncoded"), on the branch this one is stacked on, decoded a selection in a collection through
those decoders.

## Fix

The decoded selection is typed as written, with the reference it may still be: the fields the
decoder is asked to keep and require make the value that selection.

## Verified

Typed the value a decoder fills in by its fields and type-checked the vanilla testground: the
`Pick`, `Omit` and `Required` fields of `partial-shapes` failed before the fix. Ran
`cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose
`a_partial_pick_or_omit_is_coded_by_the_fields_it_has` and
`a_user_type_is_coded_by_its_own_functions` pin the typed calls, `pixi run test:conformance:update`
(10 passed), with the regenerated snapshots reviewed by the lines that changed, in which only the
calls decoding a selection changed, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
