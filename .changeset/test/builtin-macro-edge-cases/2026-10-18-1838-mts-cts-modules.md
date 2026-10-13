# fix(core): expand .mts and .cts modules everywhere a .ts one is

- **Date:** 2026-10-18 18:38
- **Commit:** c19adf55
- **Area:** `macroforge_ts` scan, CLI and expansion, `macroforge_ts_syn`, the Vite, Deno and
  TypeScript plugins, `@macroforge/shared`

## Problem

A derive in a `.mts` or `.cts` module was silently not expanded: the project scan did not read the
module, `macroforge expand --scan`, `cache`, `watch` and `svelte-package` passed it over, the Vite
and Deno plugins did not transform it, and the TypeScript plugin and the `tsc` and `svelte-check`
wrappers type-checked it unexpanded. A type held from one was imported from a specifier TypeScript
does not resolve. A library packaged with `svelte-package` shipped its `.tsx` modules unexpanded
too. A declaration file holding `export const LIMIT: number;` failed to parse in the scan and
dropped out of the registry.

## Cause

Each of those places kept its own list of TypeScript extensions, `.ts` and `.tsx`. A relative
specifier was matched to a module with its extension stripped, so `./user.mjs` met `user.ts` and an
extensionless one met `user.mts`. `svelte-package` copies the modules it does not compile with
`copyFileSync`, which the CLI's fs shim did not override. The scan built its own parse type, without
the declaration flag.

## Introduced

`a075f452` (2025-11-23, "Initial project setup with Rust and TypeScript integration") filtered the
Vite plugin's transform to `.ts` and `.tsx`; `5f160879` (2026-02-28, "Add project-wide type scanner,
type resolver, and expose derive_common") scanned `.ts` and `.tsx` alone; `594b8de4` (2026-08-11,
"feat(cli): expand macros through svelte-package, safely under concurrency") served expanded source
to the packager's reads alone and left `.tsx` out of its check; `cc18a03f` (2026-07-21,
"fix(packages): align public types and docs with actual behavior") gave the Deno plugin `.ts` and
`.tsx`; `cca06306` (2026-09-21, "feat!: publish the engine as @macroforge/core and root both
workspaces") parsed a declaration file with its flag everywhere but the scan.

## Fix

`macroforge_ts_syn::source_path` says what a file's name holds and how a specifier names it, and
`@macroforge/shared`'s `source-file` says the same for the plugins; every filter reads them. A
specifier names the module TypeScript resolves it to, a generated import names a `.mts` module as
`.mjs`, a type surface of one is a `.d.mts`, the fs shim writes the expanded source for a copy, and
the scan parses a file with `source_type::for_path`.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (195 passed), whose `source_path` tests read each kind
of file and specifier and `resolve_in_file` finds `user.mts` by `./user.mjs`;
`cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose tests scan a project
of every module kind and a declaration file holding `export const LIMIT: number;`, name a `.mts`
module's functions by `.mjs`, and resolve a macro import of `./esm.mjs`;
`cargo test -p macroforge_ts --features test-macros --bin macroforge` (83 passed);
`cargo test -p macroforge_ts --features test-macros --test cli` (22 passed), in which
`expand --scan`, `--types-out` and `cache` expand, declare and cache a `.mts` and a `.cts` module
and a `.ts` one importing both; `pixi run test:conformance:update` (10 passed), with the regenerated
snapshots reviewed by the lines that changed with the `module-kinds.mts` and `module-held.ts`
goldens; `pixi run diagnostics --no-fix` (no errors or warnings); and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`edge-module-kinds` runs the derives of a `.mts` module and of a `.ts` one holding its type,
`exact-types` and `tsc` type-check both, `packaging` packages a `.mts`, a `.cts` and a `.tsx` module
expanded, the Vite plugin transforms `.mts` and `.cts` ids, the Deno plugin expands a project of
each kind, and the TypeScript plugin reads a `.mts` and a `.cts` module as it reads a `.ts` one.
