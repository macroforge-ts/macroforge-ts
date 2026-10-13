# fix(core)!: read template literals, record keys, defaulted arguments and indexed accesses by type

- **Date:** 2026-10-12 23:38
- **Commit:** a8cdc70f
- **Area:** Built-in Hash, Default, Encode and Decode derives, endec runtime

## Problem

- `Decode` on a union alias with a template literal member, `` `a-${number}` | 'plain' ``, called a
  decoder named after the template, and the expansion did not parse.
- `Decode` never looked at the keys of a `Record`. `Record<'a' | 'b', number>` took `{ c: 1 }`, and
  `{}` without either key.
- A generic type used with a type argument left to its default, `Box` for `Box<T = string>`, was
  handed no coder for it, so the values of `T` were carried unchecked and unencoded.
- A field typed `Item['owner']` generated calls to the coders of the type `owner` has, and a
  reference to that type, with no import when the file imports `Item` alone. The expansion did not
  type-check: `Cannot find name 'ownerDecodeWithContext'`.
- A wrapper over a type written in place, `Partial<Record<'x' | 'y', number>>` or
  `Readonly<Record<string, Date>>`, was carried as it came: nothing in it was checked or coded.
- `Hash` read `'a'[]` and `` `k-${number}`[] `` as strings and called `.split` on the array.
- `Default` on a newtype over a date, an array or a tuple, and on an alias of a template literal
  type, asked for `@default(value)`, and a default written for a newtype of an object type did not
  type-check.

## Cause

A template literal type was no kind of type the coders knew, so a union alias took it for a named
type. `decodeRecord` was handed a decoder for the values alone. The coders of a nested generic type
were built from the arguments a field writes. The type an indexed access yields was read off the
other module's declaration and written into this one as it stood. A type was read as a string type
when its text started with a quote, which an array of a literal does. The alias default looked at a
newtype's body and not at the type under its brand.

## Introduced

f716dc5d (2025-12-14, "Add union type deserialization docs and field validation methods") sorts a
union alias's members into primitives and named types. 5a7366fa (2026-10-12, "fix(endec)!: code a
value by the whole structure of its type") wrote `decodeRecord` without a key. 662c47a8 (2026-10-12,
"fix(endec)!: code what a field's type arguments, Required and shared values left uncoded") built a
nested type's coders from its written arguments. This branch's own indexed access support, in the
commit before this one, read the yielded type without its imports. 1cc2f3c4 (2026-10-08,
"feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign handlers") wrote the
value-type classification.

## Fix

A template literal type is a primitive-like type wherever a type is classified, so a union alias, a
map key and an array element check it by its text, and one a derive cannot read is reported on the
alias. `decodeRecord` takes a check of each key where the key type is narrower than `string`, and
the keys a record over a finite set of keys has to have: a union of literals, or an enum. A type
argument a field leaves out is written as its default before the coders are built, with the
parameters before it filled in. The types an indexed access yields are brought in from the module
that declares them, as generated type imports, and their functions are imported from there the way
those of a type the file imports itself are. A type that module does not export, or whose name this
file already uses, is an expansion error naming it. A wrapper over a type written in place is coded
as the type it comes to: `Readonly<T>` and `NonNullable<T>` as `T`, and a `Partial` of a record over
a finite set of keys as an object whose members may be missing. A wrapper that picks members out of
a type written in place is an expansion error. A literal is quoted from end to end, so `'a'[]` is an
array. A newtype defaults as the type it brands, a template literal alias by its text, and a default
written for a newtype is cast to its brand.

`Decode` now refuses a record with a key its key type does not take, or without a key a finite key
type demands.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (161 passed),
`cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground suites
selected by `deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering`
(all passed), whose `edge-exotics`, `edge-generics` and `edge-newtypes` suites decode wrong text,
wrong keys and wrong default-argument values and read the newtype defaults,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line, the foreign-type fixtures regenerated with `INSTA_UPDATE=always` (1 passed)
and `pixi run test:rust` (workspace and `tooling/testground/macro` passed).
