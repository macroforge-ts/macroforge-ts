# fix(endec)!: refuse a field whose type no coder reads

- **Date:** 2026-10-13 23:13
- **Commit:** 86ec95f4
- **Area:** Encode and Decode of fields of types they cannot code

## Problem

```ts
/** @derive(Encode, Decode) */
export interface Job {
    done: Promise<void>;
    run: Function;
    key: keyof Job;
    mask: A & B;
}
```

Encode wrote each such field as it was and Decode handed back the input unchecked, typed as the
field: a `Promise`, a `Function`, a boxed `String`, a `WeakMap`, an error class, a mapped,
conditional or `typeof` type, `keyof`, an indexed access, `this`, an inline `import()` type, a
utility type like `ReturnType` and an intersection of anything but a brand.

## Cause

The coders' final arm carried any type they did not recognise, so a type no runtime check can read
was treated like `unknown`.

## Introduced

`5a7366fa` (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on the
branch this one is stacked on, wrote that arm.

## Fix

Only `any`, `unknown` and types the coders check structurally are carried. Every other type is an
expansion error naming it and saying why it cannot be read, with an alias suggested. A brand
(`string & { readonly __brand: 'Id' }`) is read as its base type. The derive classifier and the
coders share one list of built-in types no coder reads.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose
`a_type_json_cannot_hold_or_a_derive_cannot_read_is_uncodable` and
`a_type_a_derive_reads_or_carries_is_left_alone` cover each kind of type refused and a brand,
`unknown`, a bare `Parameters` and `URL` read, `pixi run test:conformance:update` (10 passed), with
the regenerated snapshots reviewed by the lines that changed with the `endec_unchecked_types`
fixture, which reports all sixteen fields under `Encode` and under `Decode`, and in which no
snapshot of either testground changed for it, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
