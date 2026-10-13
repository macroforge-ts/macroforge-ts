# fix(core): read a spread value as a use of its parameter

- **Date:** 2026-10-13 23:11
- **Commit:** 87d2fc4c
- **Area:** Parameters of generated functions

## Problem

```ts
/** @derive(Debug, Clone) */
export type Tags = readonly string[];
```

The generated clone of `Tags` was `function tagsClone(_value: Tags): Tags { return [...value]; }`,
which does not type-check: "Cannot find name 'value'. Did you mean '_value'?".

## Cause

A generated function names a parameter it never reads with a leading `_`, and whether it reads one
was decided by a scan that took any `.` before the name for a property access. The last dot of a
spread is no such dot.

## Introduced

`5a7366fa` (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on the
branch this one is stacked on, added the scan.

## Fix

A name after `...` is read; a name after a single `.` or `?.` is still a property.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose
`a_binding_is_read_only_as_a_word_of_its_own` and `an_unread_parameter_is_named_as_one` cover a
spread, `?.` and a property, `pixi run test:conformance:update` (10 passed), with the regenerated
snapshots reviewed by the lines that changed with the `type_alias_readonly_array` fixture, and the
three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed), in which `edge-aliases` codes, clones, compares and hashes `AliasReadonlyStrings`
and `exact-types` type-checks it.
