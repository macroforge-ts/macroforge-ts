# fix(endec): type the value a decoder fills in by its fields

- **Date:** 2026-10-14 12:36
- **Commit:** c29a76d7
- **Area:** Decode of objects, classes and tuple and alias views

## Problem

The value a decoder decodes fields onto was declared `const instance: any`, so nothing checked that
a field was given a value of its type. A field of a configured foreign type, `Option<string>` in the
Svelte testground, was given the `Option<unknown>` its handler returns.

## Cause

The decoders fill a value in field by field and return it as the type, and the value was left
untyped for it: an object's fields are optional until decoded, and a class instance's `readonly` and
`private` fields can be written by no type over it.

## Introduced

`1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") declared the value `any`.

## Fix

The value is typed by its fields, each optional and writable, and a field named like a member of
`Object.prototype` is `unknown` there, as the value inherits that member until the field is written.
A class instance is made from the prototype as an `object` and narrowed with `instanceof` once its
fields are in. A foreign handler's result is typed as the type written, which its config says it
decodes.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which every decoder types the value it fills in by the fields it can be written
through, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed), in which the exact-types and strict-lib testground suites type-check the decoders
of the edge cases.
