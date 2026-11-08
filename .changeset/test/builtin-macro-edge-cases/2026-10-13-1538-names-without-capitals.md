# fix(endec)!: code a field typed by a name without a leading capital like any other type

- **Date:** 2026-10-13 15:38
- **Commit:** ccecf542
- **Area:** Encode and Decode field types

## Problem

```ts
export interface plain {
    id: string;
}

/** @derive(Encode, Decode) */
export interface Holder {
    held: plain;
    all: plain[];
}
```

`Holder`'s decoder read `instance.held = __raw_held` and `instance.all = __raw_all as plain[]`:
whatever the input held went into the value unchecked, and the encoder wrote the field as it was.
With `Plain` in place of `plain` both called the type's coders. The same held for `_Hidden`,
`$Dollar` and `my_type`, and for a type that does derive the coders wherever the registry did not
hold it: a file expanded on its own, a Svelte component, a file newer than the scan.

## Cause

`TypeCategory::classify` took a bare name for a type the project declares only when it began with a
capital letter. Any other name was `Unknown`, which a field carries as it is. Three places then
asked the registry whether an unknown word was a deriving type after all, which covered a scanned
project and nothing else.

## Introduced

`dafd08ef` (2025-12-07, "Add Serialize and Deserialize derive macros for JSON support") wrote the
classifier with the capital-letter test. `2a0d26f5` (2026-10-12, "fix(endec)!: code a value by the
whole structure of its type") added the registry lookup beside it for a type named in lowercase.

## Fix

A name that is no keyword of the language (`any`, `unknown`, `never`, `void`, `object`, `symbol`,
`true`, `false`, `this`) and no number names a type, whatever letter it starts with. The lookups
that stood in for that, `TypeScope::derived` and its arms in the encoder, the decoder, the flatten
check and the union check, are gone.

**Breaking:** a field typed by such a name whose type derives no coder now calls coders that do not
exist, as one named with a capital does, where it was carried unchecked before.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_name_is_a_type_whatever_it_starts_with` covers eight names and twelve keywords and literals,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `type_names_without_capitals` fixture, expanded without a registry, in which
every field is decoded through its type's function, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
