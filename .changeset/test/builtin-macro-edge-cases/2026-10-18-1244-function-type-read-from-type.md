# fix(core): read a function type from the type, not from an arrow in its text

- **Date:** 2026-10-18 12:44
- **Commit:** 2f7a2968
- **Area:** `macroforge` built-in derives

## Problem

Default wrote a default that does not parse for a union whose first member is a function in
parentheses, `((value: bigint) => string) | string[]`, where it refuses a field typed by a function.
A union holding a grouped function, or a string literal holding `=>`, was read as a function type by
the value traits, the alias form of Default and the union check of the coders.

## Cause

`has_known_default` asked whether the whole type is a function, which a union is not, and the other
checks looked for `=>` anywhere in the text.

## Introduced

`1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") added the check of `classify_value_type`; `5b6d9d6f` (2026-10-12, "fix(core): default a
Required, a Pick and an object type written in place by their fields") the one of an alias's
default; `e6845936` (2026-10-12, "fix(endec)!: code what a field's type arguments, Required and
shared values left uncoded") the one of `untold_member`.

## Fix

Each check reads the type with `is_function_type`, and a union has the default of the member it
defaults to.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose Default test
refuses `((value: bigint) => string) | string[]` as a function field and defaults
`string[] | (() => string)` and a literal `'=>'`. Found by expanding a corpus of 120 generated field
types under each derive set.
