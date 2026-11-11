# fix(endec)!: tell a union's foreign member apart in Encode by its hasShape

- **Date:** 2026-11-09 10:55
- **Commit:** 4951ccb4
- **Area:** endec `Encode` union generator

## Problem

`Encode` of a generic union with a foreign member, `type Ref<Target> = RecordId | Target`, handed a
value of the foreign type to the coder passed in for `Target`. Encoding a `RecordId` as a `Target`
threw `Cannot read properties of undefined`.

## Cause

A foreign member got no coder of its own: the named-member coders call `{name}EncodeWithContext`,
which a foreign type does not have, so they left it out. A value no coder took was written as it
was, which happened to work through the type's `toJSON`. The coder for a union's one type parameter
takes every value no other member does, and counted a foreign member as told apart, though nothing
told it.

## Introduced

`cfbe66610` on `main` (2026-11-09), the squash of `test/builtin-macro-edge-cases`. Its commit
`2a0d26f5` (2026-10-12, "fix(endec)!: code a value by the whole structure of its type") left foreign
members out of the named-member coders, and `e6845936` (2026-10-12, "fix(endec)!: code what a
field's type arguments, Required and shared values left uncoded") added the type parameter's coder.

## Fix

A foreign member is a coder of its own, told by the type's `hasShape` handler and encoded by its
`encode` handler (written as it is without one), tried before the type parameter's coder. A foreign
member whose type declares no `hasShape` is an expansion error, as nothing tells its values from the
other members'.

## Verified

`cargo test -p macroforge_ts --features test-macros --lib`: 777 passed, two new among them, one for
the coder and its order and one for the error.
