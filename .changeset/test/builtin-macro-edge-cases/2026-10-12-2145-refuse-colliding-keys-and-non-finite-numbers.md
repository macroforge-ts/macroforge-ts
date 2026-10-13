# fix(endec)!: refuse the keys, numbers and untyped fields an encoding cannot keep

- **Date:** 2026-10-12 21:45
- **Commit:** 4b004dd9
- **Area:** Built-in Encode and Decode derives, endec runtime

## Problem

- Two fields coded under one JSON key, by `rename`, `renameAll` or a flattened type's own fields,
  overwrote each other on encode, and decode gave both the one value.
- Decode read keys through the prototype chain. `{}` satisfied a required field named `constructor`
  or `toString`, and an input built with `Object.create(base)` decoded `base`'s members.
- `encode` wrote `NaN` and the infinities as `null`, which decodes as a missing number or fails.
- `Encode` on a type with a field named `constructor`, `toString`, `valueOf` or `hasOwnProperty` did
  not type-check: `Type 'string' is not assignable to type 'Function'`.
- A class field whose type the declaration leaves unsaid (`items = []`) was written and read back as
  it came, whatever it held.

## Cause

Nothing compared the keys a container's fields are coded under. The decoders tested a key with `in`
and read it off the raw input. `JSON.stringify` writes a non-finite number as `null`. The encoder
wrote `result.name`, which TypeScript types as the member every object has when `name` is one of
`Object.prototype`'s. A field typed `any` for want of an annotation was coded as `any`.

## Introduced

dafd08ef (2025-12-07, "Add Serialize and Deserialize derive macros for JSON support") brought
`rename` and `flatten`, the `in` tests and the `JSON.stringify` of whatever a number field holds.
ab35d909 (2025-12-08, "Bump macroforge version to 0.1.12 and add enum/type alias support") wrote the
encoder's `result.name`. af0a8b4f (2026-09-28, "Remove the swc backend") types an unannotated field
`any`.

## Fix

A field coded under a key another field of its container has, a flattened type's keys included, is
an expansion error naming both fields. Decoding reads the members the input holds itself
(`ownMembers`, `holds`), in the generated object and class decoders, the shape tests of untagged
unions, and the runtime's `decodeObject` and `requireKeys`. `encode` throws a `TypeError` naming the
number JSON cannot represent. The encoder writes a field by its key, `result["name"]`, which the
result's index signature types. A class field left untyped that `Encode` or `Decode` would code by
its type is an expansion error asking for the annotation; one coded by `encodeWith` or `decodeWith`,
or skipped, is left alone.

`encode` of a value holding `NaN` or an infinity now throws, and decode ignores inherited members.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
whose `edge-specials` and `edge-names` suites decode inputs with inherited members and code fields
named like `Object.prototype`'s, `pixi run test:conformance:update` (10 passed), with the
regenerated snapshots reviewed by the shape of each changed line with the `endec_key_collisions` and
`endec_untyped_field` fixtures, the foreign-type fixtures regenerated with `INSTA_UPDATE=always` (1
passed) and `pixi run test:rust` (workspace and `tooling/testground/macro` passed).
