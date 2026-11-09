# fix(core): import a project type's functions by whatever path it is imported

- **Date:** 2026-10-13 11:14
- **Commit:** 75e93641
- **Area:** Expansion host imports

## Problem

A derived type holding a type imported through a `paths` alias or by a path from the project root
did not type-check:

```ts
import type { ZzItem } from '@zz/source';

/** @derive(Encode, Decode, Default) */
export interface User {
    held: ZzItem;
}
```

"Cannot find name 'zzItemDecodeWithContext'", and the same for each function of `ZzItem` the
generated code calls. The same import written `./source`, `./source.js`, `../source` or as a
directory worked.

## Cause

The host imports another type's functions only from a module whose specifier starts with `.` or `$`,
to leave a package's types alone, which have no such functions. A `paths` alias such as
`@/types/user` reads like a package, so nothing was imported for it.

## Introduced

9ad2d98a (2025-12-14, "Add naming module and switch defaultValue calls to naming-style functions")
added the import of another type's functions with the limit to relative specifiers, marked "for now"
in its comment.

## Fix

The host tells the import step which of the file's imported types the registry holds. For those the
functions are imported from the module as the file writes it, whatever the specifier looks like. A
type the registry does not hold, imported by a specifier that is neither relative nor `$`-prefixed,
is still left alone.

## Verified

Type-checked six ways of writing the path of an import with `macroforge tsc`: a `.js` extension, a
directory, an explicit `index` and `../` pass; a `paths` alias and a path from the project root
failed before the fix. Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
whose `a_project_type_under_a_path_alias_has_its_functions_imported` covers the alias and
`skips_non_relative_module_specifiers` a package, `pixi run test:conformance:update` (10 passed),
with the regenerated snapshots reviewed by the lines that changed, and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`exact-types`, `tsc` and `strict-lib` type-check `name-paths.ts`, which imports `EdgeItem` by a path
from the project root.
