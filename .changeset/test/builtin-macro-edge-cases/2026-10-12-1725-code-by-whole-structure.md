# fix(endec)!: code a value by the whole structure of its type

- **Date:** 2026-10-12 17:25
- **Commit:** 5a7366fa
- **Area:** Built-in Encode and Decode derives, endec runtime

## Problem

`Encode` and `Decode` handled one level of a field's type. The edge-case sweep found, at run time:

1. A collection in a collection was not coded: `Map<string, Set<string>>` decoded to a map of
   arrays, `Date[][]` to strings, `Record<string, Item[]>` to raw objects. A tuple or an object type
   written in place, `{ at: Date }`, was passed through the same way, and so was a `Partial`, `Pick`
   or `Omit` inside a collection.
2. An enum in a collection threw while decoding ("undefined is not iterable").
3. A child pointing back at its parent through an array, and a value held twice where one holder is
   a collection, did not decode ("Unresolved reference"). `freeze` did not reach elements.
4. A `Map` keyed by a number, an enum or a `Date` decoded with string keys.
5. Elements were not checked: `tags: [1, 2]` decoded as `string[]`.
6. A `{ "__ref": n }` stood for whatever object was registered as `n`, of any type, and a value
   shaped like the decoder's pending marker was taken for one.
7. A `Partial<T>` shared with a whole `T` encoded as a `__ref` the decoder then refused, and a
   `Pick` or `Omit` of a value that holds itself overflowed the stack.
8. `@endec({ default: true })` set the field to `true`. `skipDecoding` with a default left the field
   unset. A `T | undefined` primitive skipped its validators.
9. `renameAll: "kebab-case"` failed to expand (`result.first-name`).
10. A flattened field: decoding registered it under the enclosing object's `__id`, an optional or
    partial one was required, a flattened `Record` took every key, and `denyUnknownFields` rejected
    its keys.
11. A tagged union alias: an object variant's fields were copied without decoding (a `Date` stayed
    text, a missing field became `null`), and a union of interfaces under `tag`/`content` or
    `externallyTagged` encoded a plain object untagged.
12. An alias of a collection or of `Date` reported any non-null value as having its shape, so a
    union with one as a member matched nothing, and `Date`'s alias parsed an ISO string as JSON.
13. `User['id']`, `keyof User` and a union member such as `Item[]` produced function names that do
    not parse. A derived type named in lowercase was not treated as one.
14. A class's `private` fields could not be encoded, a subclass's constructor lacked `super`, a
    class with a constructor of its own could not derive `Decode`, and an optional field was
    assigned `undefined` under `exactOptionalPropertyTypes`.

## Cause

The field generators were templates keyed by the outermost type, each with a fixed set of element
cases, and the decoder ran each element through its type's public `decode`, which opens a context of
its own. References were looked up by number alone.

## Introduced

dafd08ef (2025-12-07, "Add Serialize and Deserialize derive macros for JSON support") introduced the
per-field templates and the `default` option parsing. 65c22961 (2026-04-07, "Syncing monorepo") is
the first commit here with the union variant copy and the flatten decoding. 29e380d4 (2025-12-14,
"Add functionNamingStyle config to control generated function names") has the `value != null` shape
check. The partial coders of 3ca86021 (2026-10-25, squashed from "feat(endec): code a Partial, Pick
or Omit of a type by the fields it has") registered a `Partial<T>` like a whole value, which is
item 7. 6168c207 (2025-12-08, "Reject @Derive(Deserialize) on classes with custom constructors")
refused a class with a constructor of its own, though decoding never calls one.

## Fix

One recursive coder (`endec/coder.rs`) writes the expression that encodes or decodes a value from
its type: collections through their members at any depth, tuples element by element, an object type
written in place member by member, `null` and `undefined` passed by, a user type or a `Partial`,
`Pick` or `Omit` of one through that type's functions (`endec/nested.rs`), a foreign type through
its handler. Decoding goes through new runtime functions (`decodeArray`, `decodeSet`, `decodeMap`,
`decodeRecord`, `decodeTuple`, `decodeObject` and leaf decoders) that share the caller's context,
report a member's errors under its path (`grid[0][1]`, `byName.k.id`), and settle references ahead
of their object through the new `DecodeContext.defer`.

- A reference is held to the type its object was registered as (`register(id, instance, kind)`), and
  pending markers are recognised by identity.
- A `Partial`, `Pick` or `Omit` is never registered or referred to. One that holds itself throws a
  `TypeError`.
- A `Map` key is read back as its type: a number, bigint, boolean, `Date`, enum or newtype. A key
  type with no text form is an expansion error.
- `default` and `default: true` are the flag, `default: false` none. A field skipped on decode takes
  its default.
- A flattened type is coded without an identity, only when its keys are there if it may be absent,
  and its keys are known to `denyUnknownFields` and to a flattened `Record` beside it.
- A union member that is a user type is found by its tag, else by its shape, in every tagging mode.
  An object variant's fields are decoded by their types.
- A class's encoders are static methods, its decoder builds the instance untyped, and its
  constructor is synthesized only when the class has none, calling `super`.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line, the foreign-type fixtures regenerated with `INSTA_UPDATE=always` (1 passed)
and `pixi run test:rust` (workspace and `tooling/testground/macro` passed).
