# fix(cli): ship the foreign-type handlers under svelte-package 3

- **Date:** 2026-10-30 00:20
- **Commit:** 857014039
- **Area:** `macroforge svelte-package` (the CLI's packager wrapper and fs shim)

## Problem

A library packaged with `macroforge svelte-package` under `@sveltejs/package` 3 shipped every module
with a foreign type importing the handlers from a path outside the package: `dist/types/meeting.js`
read `../../../.macroforge/config/handlers.js`, which resolves above the package root. Importing
such a module failed with "not found", and `dist/__macroforge/config/` was never written.

## Cause

`@sveltejs/package` 3 reads the project manifest's `#` imports and rewrites each import of one into
a path relative to the source file in the input directory. `#macroforge/config` maps to
`.macroforge/config/`, outside that directory, so the path it computes is wrong from the output
directory. The CLI's post-pass, which copies the expanded config into the output and points each
import of `#macroforge/config` at the copy, then found no import of the specifier left to rewrite.

## Introduced

`1cc2f3c4b` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") added the post-pass on the premise that the packager leaves `#macroforge/config` alone,
which `@sveltejs/package` 2 did. `@sveltejs/package` 3.0.0-next.3 (2026-07-17, released as 3.0.0 on
2026-10-01) began resolving the manifest's `#` imports as aliases.

## Fix

The wrapper installs `globalThis.__macroforgeManifest`, and the fs shim serves the packager the
project manifest without its `#macroforge/config` entry, so the specifier reaches the post-pass
intact. TypeScript reads the real manifest through `ts.sys`, so `.d.ts` emit still resolves the
handlers. The testground library moves to `@sveltejs/package` 3.0.1 and gains a module with a
foreign type.

## Verified

`deno test` of `tooling/testground/tests/packaging.test.mjs` with this checkout's CLI: 19 passed,
among them the new "a foreign type reaches the handlers shipped with the package", which packages
the library, finds `dist/types/meeting.js` importing `../__macroforge/config/handlers.js` and the
handlers shipped beside it, and decodes and encodes a `DateTime` through the packaged module. The
same test against the released 0.5.0 CLI failed: the module imported
`../../../.macroforge/config/handlers.js`. `pixi run diagnostics --no-fix`: no errors or warnings.
