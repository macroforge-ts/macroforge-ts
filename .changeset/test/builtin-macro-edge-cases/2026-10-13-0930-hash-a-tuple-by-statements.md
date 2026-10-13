# fix(core): hash a tuple alias a statement an element

- **Date:** 2026-10-13 09:30
- **Commit:** 8958f935
- **Area:** Built-in Hash derive, tuple aliases

## Problem

`@derive(Hash)` on a tuple alias of 5000 elements aborted the expander with a stack overflow. No
other derive did on the same type.

## Cause

The hash of a tuple alias was written as one expression, `((1 * 31 + a) | 0) * 31 + b ...`, that
nests once for each element. Parsing an expression 5000 levels deep recursed past the stack.

## Introduced

1cc2f3c4 (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") added the hash of a tuple alias as a fold into one expression.

## Fix

Each element takes a statement of its own, `hash = (hash * 31 + (...)) | 0;`, as the fields of an
interface do. The hash is the same number as before.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_tuple_hashes_a_statement_an_element` builds and parses the hash of 5000 elements,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `type_alias_tuple_hash` fixture, an expansion of a 5000-element tuple alias
under `Hash`, which exits 0 after aborting, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
