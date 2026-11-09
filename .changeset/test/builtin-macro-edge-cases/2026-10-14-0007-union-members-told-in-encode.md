# fix(endec): tell a union's members apart in Encode by their own fields

- **Date:** 2026-10-14 00:07
- **Commit:** 5b43dd16
- **Area:** Encode of union aliases

## Problem

```ts
/** @derive(Encode) */
export interface Cat {
    kind: 'cat';
    born: Date;
}

/** @derive(Encode) */
export type Pet = Cat | { kind: 'bird'; at: Date };
```

The encoder of `Pet` called `catHasShape`, which only `Decode` writes, so a union whose members
derive `Encode` alone did not type-check. It handed each member `value as any`, and type-checking it
took 24 seconds for a union of 100 members and over four minutes for 250.

## Cause

The union encoder told a named member by the shape test of its decoder, encoded it through a cast,
and tried the members one after another in one body, each test narrowing the value again after all
the ones before it.

## Introduced

`1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") wrote the dispatch.

## Fix

Each member is a coder of its own, tried in turn by `encodedMember`. A named member is told by its
tag, then by the keys and literals of its own fields, through a type predicate that narrows the
value to it, so its encoder takes it uncast. A member without fields is told by its `hasShape` when
it derives `Decode`, and is otherwise an expansion error; an enum member is written as it is.

## Verified

Expanded and type-checked a union of two interfaces deriving `Encode` alone before the fix: it
called `catHasShape` and `dogHasShape`, which nothing defines. Ran
`cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose
`a_named_member_is_told_by_its_own_fields_or_its_shape_test` covers an interface, an enum, an alias
with and without `Decode` and a type the registry does not hold, `pixi run test:conformance:update`
(10 passed), with the regenerated snapshots reviewed by the lines that changed with the
`union_encode_only` fixture, and the three groups on the branch's final tree: `pixi run test:rust`
(the workspace and `tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites,
13 validator files of 638 steps and 236 browser tests in Chromium, all passed) and
`pixi run test:packages` (the six packages passed), in which `edge-variants` encodes each member of
`VariantPet` with `keepMetadata` and finds the tag its own encoder writes, and `exact-types`
type-checks it.
