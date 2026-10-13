# fix(core): read a parameter as used only where generated code reads it

- **Date:** 2026-10-14 14:34
- **Commit:** 71a474ed
- **Area:** `macroforge_ts` built-in derives

## Problem

A generated function's parameter was read as used where its name stood in a single-quoted or
template string, or as an object key, and was then left named where it is unused, which
`noUnusedParameters` refuses. A use inside a template's `${}` was read only as text around it.

## Cause

`mentions` matched the name in the raw text, telling only a `"` before it and a `.` apart.

## Introduced

`018f5131` (2026-10-12, "fix(core)!: derive over literal unions, empty types and an interface's
methods as they are") added `mentions`.

## Fix

`mentions` reads the body with the shared tokenizer: strings and a template's text are text, the
code in its `${}` is code, and a key or a property is no use of a binding.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose property holds
`mentions` to the identifier references oxc finds in generated array literals, and whose test pins
quoted and template text, a key, a shorthand property, a conditional and a `${}`.
