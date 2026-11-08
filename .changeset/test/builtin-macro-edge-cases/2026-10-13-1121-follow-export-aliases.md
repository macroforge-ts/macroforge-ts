# fix(core): find a type by the name its module exports it under

- **Date:** 2026-10-13 11:21
- **Commit:** 139d711b
- **Area:** Type scan, type registry, expansion host imports

## Problem

A derived type holding a type its module exports under another name than it is declared by did not
type-check:

```ts
// models.ts
interface Late {
    note: string;
}
export type { Late as Shown };
export default class Model {
    title: string = '';
}

// page.ts
import type Chosen from './models';
import type { Shown } from './models';
```

"Module './models' has no exported member 'shownDecodeWithContext'", and
"'chosenDecodeWithContext'": the functions are `lateDecodeWithContext` and `modelDecodeWithContext`.

## Cause

The registry knew a type by the name it is declared under, and nothing recorded what a module
exports it as. An import of `Shown`, or a default import, resolved to no type, so the generated code
named the type's functions after the import and imported them under that name.

## Introduced

No single change: export aliases have not been recorded by any version of the scan.

## Fix

The scan records, for each declaration, the other names its module exports it under: `default` for a
default export (`export default class`, `export default interface`, `export default
Name`) and the
alias of `export { Name as Alias }`. A registry entry carries them
(`TypeRegistryEntry::exported_as`), in the registry's wire form too. An import resolves to the
declaration its module exports under the imported name, and the functions of a type the registry
resolves are imported under the name the type is declared by.

A default export that a barrel re-exports under a name, `export { default as B } from './b'`, is not
followed: the barrel declares nothing, and its re-exports are not recorded.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`an_import_resolves_by_the_name_its_module_exports_a_type_as` covers an aliased export, one renamed
again on import and a default import, `cargo test -p macroforge_ts --features test-macros --lib`
(756 passed), whose `a_module_exports_a_declaration_under_other_names` covers what the scan
collects, `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by
the lines that changed, and the three groups on the branch's final tree: `pixi run test:rust` (the
workspace and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13
validator files of 638 steps and 236 browser tests in Chromium, all passed) and
`pixi run test:packages` (the six packages passed), in which `edge-names` encodes, decodes and
defaults a type holding a default export imported as `Chosen` and a type exported as `NameShown`,
and `exact-types`, `tsc` and `strict-lib` type-check it.
