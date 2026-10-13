# fix(endec)!: refuse the numbers and types a value cannot be coded back from

- **Date:** 2026-10-12 22:08
- **Commit:** 8090684c
- **Area:** Built-in Encode and Decode derives, endec runtime

## Problem

- `decode` accepted `NaN` and the infinities for a number, and JSON text spells one: `1e999` parses
  to `Infinity`. `encode` throws for such a number, so input from outside could decode and then fail
  every attempt to encode it again.
- A field or an alias of a type the derives do not read was decoded with a cast and no check: a
  function, a `symbol`, a mapped or conditional type, `keyof`, `typeof`, an indexed access type,
  `object`, a template literal type. `` `id-${number}` `` took a number or an object, and `Decode`
  on a generic alias of such a type did not type-check.
- A `$Newtype<T>` over anything but a primitive, and an intersection alias with a member whose
  fields are not known, were coded the same way: `$Newtype<Date>` decoded a string and called it a
  `Date`.

## Cause

The number checks tested `typeof` alone, and the decode entries parsed text with `JSON.parse`. A
type the coder did not recognise was taken to be its own encoding.

## Introduced

4b004dd9 (2026-10-12, "fix(endec)!: refuse the keys, numbers and untyped fields an encoding cannot
keep") made `encode` throw for a number JSON cannot write and left `decode` accepting it. 5a7366fa
(2026-10-12, "fix(endec)!: code a value by the whole structure of its type") wrote `decodeNumber`
and the primitive checks. 1cc2f3c4 (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives
and shared foreign handlers") wrote the alias passthrough.

## Fix

A decoded number has to be finite: in a field, a collection, a tuple, a union, a primitive alias, a
decimal and a typed array. The decode entries parse text through `parsedText`, which refuses a
number literal too large to hold. It scans for the only literals that can be one, an exponent of
three digits or two hundred digits in a row, and parses with a reviver only then.

A value typed as a function, a symbol, `void` or `never`, or by a mapped, conditional, `keyof`,
`typeof` or indexed access type, is an expansion error under `Encode` and `Decode`, wherever in a
type it appears, with `encodeWith`, `decodeWith` and `skip` as the ways out. `keyof Name` and
`Name['field']` over a type the project declares are read as what they yield. `object` is checked to
be an object. A template literal type is checked through `spellsTemplate`: the fixed text in place,
a `${number}` or `${bigint}` hole spelling one in decimal, and a hole of literals holding one of
them. A hole runs to where the text after it first occurs, as TypeScript reads one, so the text is
walked once with no pattern to backtrack over. A template whose hole names any other type, or that
writes an escape or two holes side by side, is an expansion error. An alias of a template literal
type is coded as a field of it is.

A newtype is coded as the type it brands, whichever kind of body that is, since the brand exists in
the types alone. An intersection alias with a member whose fields are not known is an expansion
error under `Encode` and `Decode`, and the passthrough that carried such values is gone.

`decode` now fails for a non-finite number, and types that were carried unread no longer expand.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
whose `edge-specials` suite decodes non-finite numbers as values and as overflowing JSON literals
and whose `edge-exotics` suite decodes wrong text for each template literal type,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line with the `endec_uncodable_types` fixture, the foreign-type fixtures regenerated
with `INSTA_UPDATE=always` (1 passed) and `pixi run test:rust` (workspace and
`tooling/testground/macro` passed).
