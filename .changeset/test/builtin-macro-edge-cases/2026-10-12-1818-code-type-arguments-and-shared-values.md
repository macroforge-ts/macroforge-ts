# fix(endec)!: code what a field's type arguments, Required and shared values left uncoded

- **Date:** 2026-10-12 18:18
- **Commit:** 662c47a8
- **Area:** `Encode` and `Decode` generators, `@macroforge/core/endec` runtime

## Problem

- A field written with a generic type's arguments, `page: Page<User>` or `at: Box<Date>`, coded the
  values of the type parameter as they were. `Box<Date>` decoded to a box holding text where its
  type said `Date`, and `Page<User>` to unchecked plain objects where a class was declared.
- `Required<T>` decoded as `T`, so a value typed as having every field could come back without the
  ones `T` leaves optional.
- A field written under `__id`, `__ref` or the type's tag key was overwritten by the encoding's own
  use of that key, or overwrote it, and decoded as the wrong value.
- A required field missing from the input was reported twice: once as missing, once as having the
  wrong type.
- An object type written in place with an index signature, `{ [key: string]: Date }`, passed through
  uncoded.
- `encode(value)` without `keepMetadata` wrote a value held twice as `{ "__ref": n }` at its second
  place while leaving out the `__id` the reference names, so its own output failed to decode with
  "Unresolved reference".
- A nullable field of a derived type named in lowercase, `maybe: point2D | null`, was passed through
  unchecked.
- A union alias's object member without the union's tag, as in
  `{ ok: true; at: Date } | { ok: false; reason: string }`, was copied key by key: a `Date` stayed
  text, a `Set` an array, and nothing was checked. Encoding passed it through the same way.
- A `Date` field took any value: `5`, `true` and `null` were assigned as they were, and text that
  spells no date became an `Invalid Date`. A date in a collection took such text too.
- A `format: "decimal"` field took whatever `Number()` coerces: `""`, `null` and `true` decoded to
  `0`, `0` and `1`.
- A hole in an array or a tuple was skipped, leaving `undefined` where an element was declared.
- The built-in decoders of `bigint`, `URL`, `URLSearchParams`, `RegExp`, `Error`, `ArrayBuffer` and
  the typed arrays handed whatever they were given to a constructor that coerces: `true` decoded to
  `1n`, `[]` to `0n`, and `5` to a typed array of five zeroes.
- `decode(encode(value))` failed for a type that takes a string: an alias of `string` read the JSON
  text back with its quotes, and a literal union, a string enum and a union with a string member
  refused it, because a string input was taken for the value and never for JSON text.
- A union carried as it is, `tags: string | string[]`, was not checked at all.
- The top-level `boxEncode` and `boxDecode` of a generic type had no way to be told how its
  parameter is coded.
- `Encode` on an alias of a `Pick` or an `Omit`, `type Summary = Pick<User, 'id'>`, generated an
  encoder that read `ctx` without taking it, and threw a `ReferenceError` when called.
- A field typed `Box<Date> | 'none'` tested the literal with `typeof value === "'none'"`.
- A field typed as a union of types written in place, `shape: Circle | Square` or
  `when: Date | number`, was passed through: a class member stayed a plain object and a `Date`
  stayed text. A generic union alias held with arguments was written out into such a union and met
  the same end.

## Cause

A type parameter has no coder to call, and the coders of a generic type had no way to be told one,
so every value of a parameter was passed through. `Required` was classified with `Readonly` as
leaving its inner type's coding unchanged. Nothing compared a field's JSON key with the keys the
encoding writes itself. The decoder checked for a required key, then decoded the key's value whether
or not it was there. Leaving out the metadata removed two keys at every depth and nothing else,
references included. The nullable path asked for a type's name by its capital letter alone.

## Introduced

Generic parameters have passed through since 65c22961 (2026-04-07, "Syncing monorepo"), the origin
of the type-parameter handling that 1cc2f3c4 (2026-10-08, "feat(core)!: release 0.5.0 with
Rust-shaped derives and shared foreign handlers") kept. 3ca86021 (2026-10-25, squashed from
"feat(endec): code a Partial, Pick or Omit of a type by the fields it has") classified `Required` as
coding like its inner type. The reserved keys date from 773e843b (2025-12-11, "Add serde runtime
helpers and update to v0.1.30"), which introduced `__id` and `__ref` with no check on field keys.
The doubled error comes from 1cc2f3c4, whose decoder reads a required field without testing for its
key. Text without metadata has kept its references since 65c22961, where `keepMetadata` first
appears, and 3ca86021 (2026-10-25, squashed from "fix(core): generate Encode and class Clone code
that type-checks without lib `any`s") moved that into `encodedTextWithoutMetadata` unchanged.

## Fix

- `{name}EncodeWithContext`, `{name}EncodePartialWithContext`, `{name}DecodeWithContext` and
  `{name}DecodePartialWithContext` of a generic type take one optional `Encoder` or `Decoder` for
  each type parameter, after their other parameters. A value of a parameter goes through the coder
  handed in for it, wherever the type holds it, and is carried as it is without one. A field written
  with the type's arguments hands in a coder made from each. Classes, interfaces, object aliases,
  tuple and union aliases all take them.
- A union alias hands the coders on to a member written with type arguments, and a field typed
  `string | Box<Date>` to its user-type member.
- Without its metadata a value held in several places is written out at each of them, and a value
  that holds itself throws a `TypeError` that names `keepMetadata`. The encoders mark the references
  and encodings they write (`encodedReference`, `encodedAs`), so data shaped like a reference is
  never taken for one, and a union re-points a reference at the member as it tags it. Writing out is
  capped at a million copied members, past which the same error asks for `keepMetadata`: a value
  shared at every level of a deep graph grows without bound.
- A union's object member without a tag is told by the keys it has to have and the literals it
  holds, and is encoded and decoded field by field like a tagged one. A literal beside a type is
  told by equality.
- A nullable field of a lowercase derived type is decoded through that type.
- A union written in place with a member that has to be coded, and no primitive to tell it by, is an
  expansion error that names the member and the remedy, an alias of the union. A union of values
  that are their own coding is carried as before. A generic union alias that derives the coder is no
  longer written out where a field could not code the result, and is called by name with its
  arguments' coders.
- `Required<T>` decodes as `T` and reports each field `T` leaves optional that the input lacks,
  through the new `requireKeys` runtime helper. A reference standing for one is held to the same
  fields once the value it names has been read. A default still fills its field. A `Required` of a
  `Partial` has every field again, and a `Partial` of a `Required` none.
- A field of a class, an interface or an object alias coded under `__id`, `__ref` or the type's tag
  key is an expansion error.
- A required field is decoded only when its key is there.
- An object type written in place that is one index signature over strings, `{ [key: string]: V }`,
  is coded as the `Record<string, V>` it describes. One with a computed key, or an index signature
  beside named members, is an expansion error.
- A `Date` field takes a `Date` or text that spells one and reports anything else, as `decodeDate`
  now does for a date in a collection. A decimal takes a number or non-empty numeric text. The
  collection decoders walk an array by index, so a hole is the `undefined` element it reads as.
- A string handed to `decode` of a type that takes one is the JSON text of a value when it spells
  one the type takes, and the value itself otherwise (`textOrValue`), so what `encode` writes reads
  back and a bare `"small"` is still the member.
- A union of values that are their own encoding is checked to be one of its members, arrays and
  records to any depth.
- The top-level `{name}Encode` and `{name}Decode`, and a class's static `encode` and `decode`, take
  the coders of the type's parameters after their other parameters.
- Each built-in decoder takes the encoding its encoder writes and throws a `TypeError` for any other
  value, which is reported under the field.
- An alias's encoder takes the context whenever its body names it. It was judged to by the text
  `ctx)`, which a call that passes a selection after the context does not contain.
- A failure thrown by a decoder handed in for a type parameter, such as a foreign type's handler, is
  reported under the field it was decoding.

What this breaks for callers: the coders of a generic type have new trailing parameters, and a
hand-written `{name}EncodeWithContext` or `{name}DecodeWithContext` for a generic type is now called
with type arguments and coders where a field writes the type with arguments. `Page<User>` and the
like now decode and check the values of their parameters, so input that passed unchecked can fail,
as can input for a `Required<T>`. A field named `__id`, `__ref` or like the tag key no longer
expands without a `rename`. `encode(value)` of a value that holds itself throws where it wrote text
that could not be decoded. A bare string handed to `decode` that happens to be the JSON text of
another member, `"5"` for `string | number`, is now read as that member.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (161 passed),
`cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground suites
selected by `deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering`
(all passed), `pixi run test:conformance:update` (10 passed), with the regenerated snapshots
reviewed by the shape of each changed line, the foreign-type fixtures regenerated with
`INSTA_UPDATE=always` (1 passed) and `pixi run test:rust` (workspace and `tooling/testground/macro`
passed).
