# fix(cli): re-expand a package when the CLI itself was rebuilt

- **Date:** 2026-10-11 07:22
- **Commit:** 6aa6de8b
- **Area:** CLI, `svelte-package`

## Problem

After a CLI built from a checkout replaced the installed one, `macroforge svelte-package` printed
"Reused all 751 expanded module(s)" and published the previous build's expansions. A derive fix in
the new CLI never reached the package.

## Cause

The package state identified the engine that expanded a tree by its crate version. A build from a
checkout changes the built-in derives under an unchanged version, so every recorded input still
matched.

## Introduced

594b8de4 (2026-08-11), `feat(cli): expand macros through svelte-package, safely under concurrency`.
It wrote the package state with the version as the engine's identity. 94687047 (2026-09-23),
`Release 0.3.1`, gave the expand cache a fingerprint of the CLI binary and left the package state
without one.

## Fix

The package state records the same CLI fingerprint, and a different one both triggers a run and
re-expands every artifact.

## Verified

New test `test_a_rebuilt_cli_re_expands_every_artifact`; the five re-expansion tests pass.
`pixi run test:rust` passed and clippy is clean. After the CLI was rebuilt and installed again,
dealdraft's types package printed "Re-expanded all 751 macro module(s): the macroforge CLI was
rebuilt" and published the new decoders.
