# fix(core): leave out generated lines that can decide nothing

- **Date:** 2026-10-13 16:25
- **Commit:** 08e66c5c
- **Area:** Decode shape tests and union decoders, PartialEq

## Problem

Three emitters wrote statements whose outcome was already settled:

```ts
export function maybeHasShape(value: unknown): boolean {
    if (value === null) return true;
    return true;
}

let __matchCount = 0;
if (__matchCount === 1) return true;

export function idEquals(a: Id, b: Id): boolean {
    if (a === b) return true;
    return a === b;
}
```

The first is the shape test of a union with a type parameter, the second that of a union none of
whose members is told by its shape, with a matching `const __shapeMatches: Array<string> = []` in
its decoder, and the third the equality of an alias of a primitive, or of an empty tuple.

## Cause

Each template wrote its checks for every union and its shortcut for every alias, and left the loop
that fills them in empty where there was nothing to fill in.

## Introduced

`65c22961` (2026-04-07, "Syncing monorepo") carries the union templates. `1cc2f3c4` (2026-10-08,
"feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign handlers") added the alias
equality with the shortcut ahead of every body.

## Fix

A union with a type parameter tests its shape as `return true`, which is what its checks came to. A
union with no member told by its shape reads its tag and nothing else, in the shape test and in the
decoder. A shape test that needs one match returns at the first instead of counting. The equality of
an alias writes the identity shortcut only ahead of a comparison that does more than it.

Two of the bodies cut down no longer read a parameter, the equality of an empty tuple and the shape
test of a union with a type parameter, which `noUnusedParameters` refuses. The first run of the
type-check suites on a rebuilt engine found it, and the fixup names each with an underscore, as the
other derives name theirs.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which 89 counting lines became 87 early returns, 17 identity shortcuts and the
empty match blocks went, and no other line changed, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`exact-types`, `strict-lib` and `tsc` type-check the result on a rebuilt engine.
