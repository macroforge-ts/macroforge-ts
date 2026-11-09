# fix(core)!: name a type's functions in one place, and keep the underscores its name opens with

- **Date:** 2026-10-13 16:13
- **Commit:** 38a847fd
- **Area:** Derive naming, expansion host

## Problem

A type whose name opens with an underscore got functions named without it and with a capital:
`_Hidden` gave `HiddenClone` and `HiddenDecode`. A type named `_` gave functions named `Clone`,
`Decode` and `DefaultValue`, and one named `_1` gave `1Clone`, which is no name: the expansion
failed with "the type patches do not parse".

## Cause

Some fifty places each built `{name in camel case}{Suffix}` with their own call to the case
converter, which takes an underscore for a word boundary and drops it. With the first word empty,
the second kept its capital.

## Introduced

No single change: the per-site conversion goes back to `d2502dd7` (2025-12-17, "refactor(macros):
update builtin derive macro implementations") and was copied with each derive after it.

## Fix

`function_prefix(type_name)` in `builtin::derive::common` is the one place a type's functions get
their prefix: the name in camel case, behind the underscores it opens with. The derives, the host's
function imports, its check for two types naming the same functions, and the convenience object all
call it. `standalone_fn_name` and `TypeNames::function` are built on it.

**Breaking:** the functions of a type whose name opens with an underscore are renamed: `_Hidden`
gives `_hiddenClone` where it gave `HiddenClone`.

## Verified

Probed twenty names through the CLI before the fix. Ran
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_types_functions_are_named_by_its_name_in_camel_case` covers thirteen names,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `type_names_unusual` fixture, in which no existing snapshot changed, and the
three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
