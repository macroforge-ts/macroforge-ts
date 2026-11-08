# fix(core): read a number literal type as TypeScript writes one

- **Date:** 2026-10-13 12:37
- **Commit:** fa4d3214
- **Area:** Every built-in derive, literal types

## Problem

`@derive(Default)` on a field typed `1_000 | 2_000` failed: "invalid default expression for
'grouped': Parse("Invalid characters after number")". A type named `Inf`, `NaN` or `Infinity` was
read as a number literal.

## Cause

A literal type was a number when Rust's float parser took its text. That parser does not know a
numeric separator, a hexadecimal, binary or octal literal, or a bigint with a sign or a separator,
and it does take `inf`, `nan` and `infinity`. Ten places asked it.

## Introduced

77e77192 (2025-12-16, "Support literal types in Default macro for correct defaults") recognised a
number literal type by parsing it as a float; the later places followed it.

## Fix

`number_literal` reads a literal as TypeScript writes one, `-1`, `1_000`, `0x10`, `0b11`, `1e3`,
`1n`, and says whether it is a number or a bigint. The ten places use it: the default of a literal
type, the keys of a record, the tests a union's members are told by, the kind a validator is checked
against, and the key of a `Map`.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`a_number_literal_is_read_as_typescript_writes_one` covers sixteen spellings and fifteen texts that
are no number, and which found `_1` read as one before the reader checked the text as written;
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed);
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `literal_number_forms` fixture, which derives every macro over the six
spellings and holds a type named `Inf`; and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
