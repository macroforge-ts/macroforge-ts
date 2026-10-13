# fix(core): import and resolve a type named in lowercase like any other

- **Date:** 2026-10-12 18:03
- **Commit:** 945d75c4
- **Area:** Expansion host, generic alias resolution

## Problem

A derived type named in lowercase, such as `point2D`, held as a field from another module, expanded
to calls of `point2DEncodeWithContext`, `point2DClone` and the rest with no import for them. A
generic alias named in lowercase was not expanded where a field used it with arguments.

## Cause

The host imported a type's generated functions only for an imported name that began with an ASCII
capital, and the generic alias resolver tried the registry only for such a name. Both tests stood in
for "is this a user type", which the registry and the generated code already answer.

## Introduced

65c22961 (2026-04-07, "Syncing monorepo") carries the capital-letter test on imported names.
c5693462 (2026-04-27, "fix(serde): resolve foreign-type body imports against config, never guess")
carries the one in the alias resolver.

## Fix

Both tests are gone. A function is imported when the generated code names it, and an alias is
expanded when the registry holds it as one.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (161 passed), the 25 testground suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
whose `edge-names` suite holds `point2D` from another module, and `pixi run test:rust` (workspace
and `tooling/testground/macro` passed).
