# fix(core)!: refuse a derive where the code it generates cannot go

- **Date:** 2026-10-13 11:14
- **Commit:** b5e1beda
- **Area:** Expansion host, class IR

## Problem

A built-in derive generated code where none can be written:

- In a JavaScript file (`.js`, `.mjs`) the output held TypeScript,
  `static toString(value: Point):
  string`.
- In a declaration file (`.d.ts`) an ambient class got method bodies and an interface got functions
  with bodies.
- On a `declare class` in a module, expansion failed with an internal error: "the type patches of
  ... do not parse at line 5: `static clone(value: Ambient): Ambient {`: An implementation cannot be
  declared in ambient contexts".

## Cause

Nothing asked what kind of file, or of class, a derive was written in. The class IR did not say
whether a class is declared with `declare`, as the enum IR does for an enum.

## Introduced

No single change: the three have not been refused by any version. a4c380d3 (2026-10-12, "fix(core)!:
cover the fields, enums and names the built-in derives misread"), on this branch, added the check
for a `declare enum` and left the class's out.

## Fix

A built-in derive on a type in a declaration file or a JavaScript file, and on a `declare
class`, is
an expansion error naming the type, and the type is not expanded. `ClassIR` gains `is_declare`.

Breaking: these expanded before, to code that is not valid where it was written.

## Verified

Expanded a derive in a `.js`, an `.mjs` and a `.d.ts` file and on a `declare class`: each is
refused, where the first three wrote invalid code and the last failed with an internal error. Ran
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `ambient_class` fixture, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which the `cli`
suite expands each file kind and reads the error.
