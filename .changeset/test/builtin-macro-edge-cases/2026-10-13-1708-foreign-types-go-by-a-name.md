# fix(core)!: refuse a foreign type whose name is no name

- **Date:** 2026-10-13 17:08
- **Commit:** 96fce39c
- **Area:** Config loading

## Problem

A `foreignTypes` key is any string, and nothing checked it. The handlers of a foreign type are
exported from the expanded config under names made of its own, `__foreign__` and the name in camel
case, and the derives parse that name where they call a handler. A key with type arguments,
`'Map<string, Decimal>'`, made a name that does not parse, at a place that expects it to: the
expander panicked. A key with a hyphen, `'my-type'`, made one that parses as a subtraction.

## Cause

The key was taken as the type's name as written, and the name was made into an identifier without
asking whether it could be one.

## Introduced

`1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") began exporting each handler under a name made of the type's, where a handler had been
its expression written in place.

## Fix

A key is read as a name where the config is: identifiers joined by dots, in any script. Any other is
a config error that names the key and says how a foreign type is named.

**Breaking:** a config with such a key, which loaded and then failed in a derive, no longer loads.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_foreign_type_goes_by_a_name` reads four names and refuses six keys, and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`foreign-types` and the svelte testground load their configs as before.
