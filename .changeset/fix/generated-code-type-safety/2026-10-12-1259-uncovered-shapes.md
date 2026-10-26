# fix(core): generate type-correct, working code for eight uncovered shapes

- **Date:** 2026-10-12 12:59
- **Commit:** 3f381827, d75d4dd9 (fixup)
- **Area:** Built-in Debug, Default, Ord, PartialOrd, Encode and Decode derives, testground

## Problem

Field and alias shapes the testground never had produced generated code that fails type checking on
TypeScript's own library. A probe of every derive over such shapes reported 20 errors, and several
of them are wrong at run time as well:

1. Decoding `null` for a union such as `string | Item | null` failed, and a null element of
   `Array<Item | null>` threw while encoding.
2. A field typed `ReadonlyArray<T>`, `readonly T[]`, `ReadonlyMap` or `ReadonlySet` called functions
   that do not exist, such as `readonlyArrayEncodeWithContext`.
3. `Default` gave `null` for `T | undefined` and for `(T | null)[]`, `undefined` for a `ReadonlyMap`
   or `ReadonlySet`, and failed on a tuple field with "invalid default expression".
4. `Debug` of `Array<T | null>` passed `null` to `T`'s `toString`.
5. Decoding a union with a class member called `{name}HasShape`, which no class emitted.
6. `Ord` and `PartialOrd` on an enum called `localeCompare` on `never`.
7. A value with a field typed `T | undefined` set to `undefined` did not survive a round trip:
   "missing required field".
8. `Decode` and `Default` on a class with a `readonly` field assigned it outside the constructor.

## Cause

1. A union's primitive members are told apart by `typeof value === "<keyword>"`, emitted for `null`
   as well, and `typeof null` is `"object"`.
2. The endec classifier knew `Array<T>`, `T[]`, `Map` and `Set` by prefix and took every other
   capitalised name for a user type.
3. `is_nullable_type` looked for `|null` or `|undefined` anywhere in the type's text, so a
   collection of nullable elements counted as nullable, and every nullable type defaulted to `null`.
   A tuple fell through to "a user type deriving `Default`".
4. The element's `toString` was passed to `map` whatever the element type admitted.
5. Classes emit `hasShape` as a static method. The standalone functions that delegate to the statics
   covered `decode`, `decodeWithContext` and `is` only.
6. The enum comparison emitted a numeric and a string branch whatever the enum's values.
7. Only a field declared with `?` was exempt from the required-key check, while `Encode` leaves out
   any `undefined` value.
8. The instance was typed as the class itself, whose `readonly` fields only a constructor may
   assign.

## Introduced

1. 1cc2f3c4 (2026-10-08),
   `feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
   handlers`, for the
   field guard, and f716dc5d (2025-12-14),
   `Add union type deserialization docs
   and field validation methods`, for union aliases.
2. dafd08ef (2025-12-07), `Add Serialize and Deserialize derive macros for JSON support`.
3. f1d3d7a7 (2025-12-09), `Bump version to 0.1.24`, for the nullable check, and 752a6148
   (2025-12-14),
   `Remove unused @derive decorators from vanilla macros and demo validator forms to
   clean up code style`,
   for the user-type fallback a tuple reaches.
4. 1cc2f3c4 (2026-10-08), as above.
5. 7486244e (2025-12-14),
   `Update generated TypeScript and Svelte forms to use suffixed exported
   createForm functions and improve formatting`.
6. 1b9ff6a1 (2025-12-08),
   `Add built-in derive macros for PartialEq, Hash, Ord, PartialOrd, and
   Default`.
7. dafd08ef (2025-12-07), as above: the encoder has left `undefined` out since then.
8. 1cc2f3c4 (2026-10-08), as above, for `Default`. The decoder's instance predates the monorepo
   sync, 65c22961 (2026-04-07).

## Fix

1. One helper, `primitive_keyword_test`, tests a primitive keyword, by identity for `null` and
   `undefined`. The field guard and the three union decoders use it.
2. The endec classifier reads an array through `array_element_type`, as the other derives do, so
   `T[]`, `Array<T>` and their readonly forms are one category with the same element text, and
   `ReadonlyMap` and `ReadonlySet` are a map and a set.
3. `is_nullable_type` looks at the union's own members. A type with `null` defaults to `null`, one
   with `undefined` alone to `undefined`, a tuple to its elements' defaults, and a readonly map or
   set to an empty one.
4. A nullable element is rendered by `String`.
5. Classes emit a standalone `{name}HasShape`.
6. The comparison emits only the branches the enum's values need.
7. A field whose type admits `undefined` has no required key.
8. An instance of a class with a `readonly` field is filled in under its writable type.

## Verified

- New source `shape-tests.ts` in the vanilla testground holds these shapes. Expanded, it has no type
  errors on TypeScript's own library or on the stricter one. The probe went from 20 errors to 1, the
  `Partial<T>` field described below.
- New suite `shapes` (7 tests) runs the expansions: null elements and readonly collections
  round-trip, defaults, enum order, a union of an interface and a class, a readonly class field, and
  `null` in a mixed union. Two of them failed first, which is how items 3 (readonly map and set) and
  7 were found.
- The `tsc`, `decode-tsc`, `newtype-tsc`, `types-out-tsc`, `strict-lib`, `value-traits`, `endec` and
  `expansion` suites pass with the file in the project.
- The core crate's unit tests pass, 691, with new ones for the defaults, the classifier and the
  keyword test.
- `pixi run test:conformance` passes. In existing snapshots only the class `hasShape` function and
  the offsets after it moved.
- `pixi run test:all` on the finished branch: the Rust workspace, the six package suites, the 24
  Deno suites and the validator tests pass, and the browser suites pass, 126 vanilla and 109 svelte.
- `pixi run diagnostics` on the finished branch: no errors or warnings. The formatting it applied
  and the regenerated MCP docs are the fixup.

## Left for its own change

A field typed `Partial<T>`, `Pick<T, K>` or `Omit<T, K>` was the probe's last error. It needed
coders of its own, which ee8e29b5 adds.
