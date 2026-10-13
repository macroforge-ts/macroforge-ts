# fix(endec)!: refuse a type named through a namespace import

- **Date:** 2026-10-13 10:11
- **Commit:** feba1f5a
- **Area:** Built-in Encode and Decode derives

## Problem

A field typed through a namespace import was neither coded nor checked:

```ts
import * as models from './models';

/** @derive(Encode, Decode) */
export interface Held {
    user: models.User;
}
```

Decode wrote `instance.user = __raw_user` with a cast: any value passed, and a `Date` field of
`User` came back as the text JSON holds under a type that says `Date`. Encode handed the value to
`JSON.stringify` as it stood. Nothing was reported.

## Cause

A type's coders are found by its name, and `models.User` is no name the registry holds: it was
classified as an unknown type, which is carried as it is.

## Introduced

5a7366fa (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on this
branch, wrote the coder that carries a type it does not know as it is. A name through a namespace
has not been followed by any version before it either.

## Fix

Encode and Decode report a field whose type, or a member or element of it, is named through a
namespace the file imports: "`models.User` names a type through the namespace `models`, and a type's
coders are found by its own name; import it by name, as in `import type { User } from
"./models"`".
A foreign type configured under a dotted name is coded by its handlers as before.

For a type two names deep, `models.Clock.Instant`, the hint named an import that cannot be written,
`import type { Clock.Instant }`. The fixup names what the module exports, `Clock`, and says to write
`Clock.Instant`.

Breaking: a type that held one expanded before, with the value uncoded and unchecked, and is now an
error.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `endec_namespace_type` fixture, which names a type through a type-only and
through a value namespace, bare, in an array and in a union, and the three groups on the branch's
final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
