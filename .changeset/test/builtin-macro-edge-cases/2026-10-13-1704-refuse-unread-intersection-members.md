# fix(endec)!: refuse a union member that is an intersection no coder reads

- **Date:** 2026-10-13 17:04
- **Commit:** 6ee5b243
- **Area:** Encode and Decode of union aliases

## Problem

```ts
/** @derive(Encode, Decode) */
export type Shape = ({ kind: 'account' } & Account) | ({ kind: 'owner' } & Owner);
```

expanded without a word and coded neither member: the union's tag is `__type` unless
`@endec({ tag: "kind" })` says otherwise, and a member tagged by another field was read as no member
at all. Decoding `{ kind: 'account', id: 'a' }` threw "value does not match any union member", the
shape test answered `false`, and the encoder wrote the value as it was, dates and all. The same went
for `(Account & { extra: number }) | Owner` and for any intersection of two named types.

Where a member was read, parts of it were not: of `{ __type: 'both' } & Account & Owner` the last
type was taken and the other dropped, the fields beside the tag in
`{ kind: 'a'; note: string } & Account` were never decoded, and `{ kind: string } & Account` was
told by the tag `string`.

## Cause

Both derives took an intersection member only where they found the tag in an object part and a named
type beside it, and went on to the next member otherwise. Neither asked whether the two were all the
member is.

## Introduced

`9fdd7585` (2026-04-08, "Syncing monorepo") carries the search for a tag and a type among an
intersection's parts, with nothing for a member where it finds neither.

## Fix

`tagged_member` reads a member as `{ tag: 'name' } & Type` and nothing else: two parts, an object
type holding the union's tag alone with a string literal for its type, and one named type, in either
order. Any other intersection is an error at the derive, from Encode and from Decode, that says
which of these it lacks and how to code it: name the member with an alias, derive on that, and use
the alias in the union.

**Breaking:** a union with such a member, which expanded and then failed for the member's values at
runtime, no longer expands.

## Verified

Probed four unions through the CLI before the fix, each of which expanded with a member left out.
Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `union_intersection_member_unread` fixture, which reports its five unions and
reads the sixth with the tag on either side, and in which no snapshot of either testground changed,
and the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
