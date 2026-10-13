# fix(endec)!: default a field Decode skips, and flatten only a type with fields

- **Date:** 2026-10-13 07:03
- **Commit:** 3f5ac228
- **Area:** Built-in Encode and Decode derives, endec runtime

## Problem

- A required field under `skip` or `skipDecoding` was left off the decoded value, which its type
  says has it: `@endec({ skip: true }) password: string` decoded to an object with no `password`,
  and a class instance to one whose field was `undefined`.
- `flatten` on a type with no fields decoded to the enclosing object itself: a `string`, an array, a
  `Map`, a `Partial<T>`, an object type written in place or a type parameter was assigned the whole
  input object, unchecked. Encoding spread a string's characters or an array's indices into the
  parent.
- A flattened `T | null` whose fields were not in the input was left off the value instead of being
  `null`.
- Two flattened `Record`s on one type each took every key no other field reads, so each failed on
  the other's values.
- `format: "decimal"` on a field that is no `number` wrote a string with `String` and decoded it to
  a number, into a field typed `string`.
- A `default` of the wrong type was assigned without a type error, since the decoded instance is
  built untyped: `@endec({ default: 5 })` on a `string`.
- The built-in `bigint` decoder took any whole number, among them one past the safe integers, whose
  digits JSON parsing had already rounded.
- `decode(input, { freeze: true })` froze the objects it built and left their arrays, tuples and
  records open.

## Cause

A field `Decode` does not read was dropped from the fields it builds unless it had a `default`. The
flattened-field decoder fell back to assigning the input object for any type it had no case for, and
nothing checked the type a field flattens or formats. A default was written straight into the
assignment. Only the objects of derived types were tracked for freezing.

## Introduced

dafd08ef (2025-12-07, "Add Serialize and Deserialize derive macros for JSON support") wrote `skip`
to leave the field out and `flatten`'s fallback to the whole object. 773e843b (2025-12-11, "Add
serde runtime helpers and update to v0.1.30") tracked derived objects alone for freezing. 65c22961
(2026-04-07, "Syncing monorepo") added `format: "decimal"` for any primitive. 662c47a8 (2026-10-12,
"fix(endec)!: code what a field's type arguments, Required and shared values left uncoded"), on this
branch, made the `bigint` decoder take a whole number of any size.

## Fix

A field `Decode` never reads takes its `default`, or else its type's own default, as a `default`
flag gives it; only an optional field is left out. Every default is written
`(value) satisfies FieldType`, so TypeScript checks it.

`flatten` on a type that is not a class, an interface, an object type alias or a `Record`, which may
be `null` or optional, is an expansion error under both derives, and the decoder's fallback is gone.
A flattened type that may be `null` is `null` when its fields are not there. A second flattened
`Record` on one type is an expansion error. `format: "decimal"` on a field that is not `number` is
an expansion error.

The `bigint` decoder takes a number only when it is a safe integer. The runtime's array, tuple,
record and object decoders track what they build, so `freeze` freezes it; a `Map` and a `Set` have
no frozen form.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (734 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line with the `endec_flatten_misuse` and `endec_flattened_records` fixtures, the
foreign-type fixtures regenerated with `INSTA_UPDATE=always` (1 passed), `pixi run test:testground`
(43 Deno suites, 13 validator files of 638 steps and 236 browser tests in Chromium, all passed),
whose `edge-directives` suite decodes skipped fields and an absent flattened type and whose
`edge-hostile` suite decodes each misspelled `bigint` and checks what `freeze` freezes, and
`pixi run test:rust` (workspace and `tooling/testground/macro` passed).
