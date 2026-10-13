# fix(core): follow a type a module imports and then exports

- **Date:** 2026-10-13 15:31
- **Commit:** ada26d1e
- **Area:** Type scan

## Problem

A barrel may bring a name in first and export it after:

```ts
// models/index.ts
import type { User } from './user';
import Model from './model';
export type { User as Member };
export default Model;
```

`import type { Member } from './models'` resolved to nothing, or, where the project declares one
`Member` elsewhere, to that unrelated type.

## Cause

The scan recorded what a module exports again only where the export names the module it comes from
(`export { User } from './user'`). An export of a name the module imported is the same thing written
in two statements, and was read as an export of a declaration the module does not have.

## Introduced

No single change: the link from an exported name to the import behind it has not been recorded by
any version of the scan.

## Fix

`collect_re_exports` reads a module's imports, and records `export { Local as Name }` and
`export
default Local` of an imported name as a re-export of what the import names in its module. An
exported namespace import names no type and is left out. Such an export brings the type alone, so
its generated functions are imported from the module that declares it.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_module_says_what_it_exports_again` reads the three forms and
`an_import_is_followed_through_the_modules_that_export_it_again` resolves a type and a default
through a module that imports them first, `pixi run test:conformance:update` (10 passed), with the
regenerated snapshots reviewed by the lines that changed, and the three groups on the branch's final
tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which `edge-names`
decodes `NameHanded` through the barrel.
