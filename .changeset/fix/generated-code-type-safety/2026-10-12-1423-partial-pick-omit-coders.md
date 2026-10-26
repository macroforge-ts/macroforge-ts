# feat(endec): code a Partial, Pick or Omit of a type by the fields it has

- **Date:** 2026-10-12 14:23
- **Commit:** ee8e29b5, 7c530618 (fixup), c68d91df (fixup), 19cd9d83 (fixup)
- **Area:** Built-in Encode and Decode derives, endec runtime, testground

## Problem

A field typed `Partial<T>`, `Pick<T, K>` or `Omit<T, K>` could not be coded:

- the generated call passed the partial value to `T`'s encoder, a type error;
- encoding a partial value that lacks a `Date` or collection field threw;
- decoding rejected any partial value with "missing required field".

An optional field typed `T | null` threw while encoding when it was absent.

## Cause

These wrapper types were treated like `Required<T>` and `Readonly<T>`, whose values are whole, and
coded through `T`'s own encoder and decoder, which read every field of `T`.

The nullable field's statements tested for `null` alone, whether or not the field was optional.

## Introduced

dafd08ef (2025-12-07), `Add Serialize and Deserialize derive macros for JSON support`, which
classified every wrapper type as its inner type.

## Fix

A coder for the wrapped shape lives beside `T`, since only `T`'s file has the coders of its fields
in scope. A selection of `T`'s fields is the shape: `{ only: [...] }` for a `Pick`,
`{ except: [...] }` for an `Omit`, and none for a `Partial`.

- `{name}EncodePartialWithContext(value: Partial<T>, ctx, fields?)` holds the encoder's body. It
  writes a field when the value has it and the selection keeps it. `{name}EncodeWithContext` runs it
  without a selection.
- `{name}DecodePartialWithContext(value, ctx, fields?)` shares the body of `decodeWithContext`,
  which takes a requirement. A field the selection lacks is not read, even when the input has it.
  One it keeps is required as `T` requires it, except under `Partial`, where none is, and no default
  is filled in for a field the call does not require.
- The endec runtime gains `FieldSelection`, `FieldRequirement` and `fieldRequirement`.

A wrapper field calls the partial coders, and so does a nullable or undefined union of one.
`Required`, `Readonly` and `NonNullable` keep the whole coders. A `Pick` or `Omit` whose keys are
not string literals is an expansion error, since nothing then says which fields a value has.

An optional nullable field is encoded only when present.

## Fixed in review

The first version, ee8e29b5 alone, took the selection only as which fields are required. A review of
the commit found two holes, which the fixup closes:

- **Fields outside the shape.** Decoding an `Omit<User, 'isAdmin'>` from input carrying `isAdmin`
  kept it, and a whole `User`, which is assignable to an `Omit<User, 'passwordHash'>`, was encoded
  with the omitted field. The selection now decides which fields are read and written at all.
- **References.** A partially decoded value was registered under its `__id`, so a field typed as the
  whole `T` could refer to it and get a value that skipped the required-field checks. A value
  decoded by selection is no longer registered and may not be a reference itself, since the object
  referred to would carry the fields the selection leaves out. One encoded by a `Pick` or `Omit`
  selection takes no part in cycle detection for the same reason.

A reference is still not checked against the type of the field holding it: that was so before this
change and is left for its own.

## What changes for users

Every object-shaped type deriving `Encode` or `Decode` exports the two partial functions, and its
companion object carries them. The static `decodeWithContext` of a class has a second overload
taking the requirement.

## Verified

- The vanilla testground's `shape-tests.ts` gains `Partial`, `Pick`, `Omit` and `Readonly` fields
  over an interface and a class. Expanded, it has no type errors on TypeScript's own library or on
  the stricter one, and the probe of every derive over uncovered shapes is down from 20 errors to
  none.
- Five new tests in the `shapes` suite (12 of 12 pass): the fields a partial value has are encoded,
  a round trip keeps them and decodes a `Date` among them, a `Pick`, an `Omit` and a `Readonly` each
  report the field they keep when it is missing, a whole value coded as a `Pick` or `Omit` loses the
  fields outside it in both directions, and a partially decoded value is neither registered nor
  accepted as a reference.
- The core crate's unit tests pass, 692, with a new one for the wrapper classification. Two existing
  ones read the encoder's body out of `{name}EncodeWithContext` and now read it out of the partial
  function.
- `pixi run test:conformance` and `foreign_type_fixtures` pass with their snapshots updated: each
  object decoder's required-key and field statements gain the requirement, each encoder's fields are
  written when present, and each type gains the two functions.
- `pixi run verify` on the finished branch first stopped on one `endec` test, which asserted the
  text `const __id = ctx.register(value)` that the review fix changed. With the assertion updated it
  passes whole: diagnostics with no errors or warnings, the JSR publish check, the Rust workspace,
  the six package suites, 24 Deno suites, the validator tests, the browser suites (126 vanilla, 109
  svelte), the extensions and the docs. The formatting it applied and the MCP docs it regenerated
  are the last two fixups.
