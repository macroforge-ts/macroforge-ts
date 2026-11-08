# fix(syn): read the decorators of a constructor parameter property

- **Date:** 2026-10-13 18:14
- **Commit:** e0c9c6a7
- **Area:** Class lowering

## Problem

```ts
/** @derive(Default, Encode, Decode) */
class Tagged {
    constructor(
        /** @default(9) */ public count: number = 7,
        /** @endec({ rename: "label" }) */
        public name: string = 'unnamed'
    ) {}
}
```

`Tagged.defaultValue().count` was `7`, and `name` was coded under `name`: a decorator on a parameter
property was read as none, without a word.

## Cause

A parameter with a modifier is lowered as the field it declares, and the field was given no
decorators: the comments above a parameter were never looked at.

## Introduced

`a526fc18` (2026-10-12, "fix(syn): lower every field a class or an object type declares"), on the
branch this one is stacked on, began lowering parameter properties, without their decorators.

## Fix

The decorators written between a parameter and the one before it, or the parenthesis that opens the
list, are the field's, as those above a property are. They are removed from the output with the
others.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed),
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `class_parameter_property_decorators` fixture, whose default is 9 and whose
key is `label`, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `edge-classes` defaults, encodes and decodes `ClassTagged` by its
decorators.
