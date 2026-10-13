# fix(core): default a parenthesised type as the type in it

- **Date:** 2026-10-13 09:30
- **Commit:** 754b96d4
- **Area:** Built-in Default derive, nullable types

## Problem

`@derive(Default)` on a type with a field written in parentheses, `value: (number | null)`, failed
with "invalid default expression for 'value': Parse("Unexpected token")". The same type without the
parentheses defaults to `null`.

## Cause

The default of a type was chosen from its text as written. A union in parentheses has no member at
its top level, so it was not seen as nullable, fell through every case, and was defaulted by a call
to a function named after the whole text, which is no expression. `is_nullable_type` read the text
the same way.

## Introduced

1b9ff6a1 (2025-12-08, "Add built-in derive macros for PartialEq, Hash, Ord, PartialOrd, and
Default") wrote the default of a type from its text and never set grouping parentheses aside.

## Fix

The parentheses that only group a type are set aside before its default is chosen and before it is
asked whether it is nullable. The helper that does so, which the endec coder already had, moved to
the shared type utilities.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_grouped_type_defaults_as_the_type_in_it` covers one and two pairs of parentheses, an expansion of
a field typed `(number | null)` and of one nested 32 pairs deep, which default to `null`, and the
three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
