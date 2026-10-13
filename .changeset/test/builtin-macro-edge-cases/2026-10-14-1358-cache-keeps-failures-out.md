# fix(cli): keep a file whose expansion fails out of the cache and fail the build

- **Date:** 2026-10-14 13:58
- **Commit:** 5ffaef30
- **Area:** `macroforge` CLI

## Problem

`macroforge cache` and `macroforge refresh` exited 0 when a file failed to expand, and wrote what
the expansion produced to the cache with a manifest entry. The Vite plugin then served that partial
expansion as the file's code with no error, and a later `macroforge cache` skipped the file, its
source unchanged, so the error was never reported again. The testground's helper imported such a
file without the functions the failing derive should have added.

## Cause

`warm_cache` printed a file's errors and recorded its expansion as it recorded a clean one, and
returned no sign of the failure, so the one-shot commands had nothing to fail on. `watch` did the
same in two further copies of the recording code.

## Introduced

`96b1e424` (2026-03-03, "Add watch and cache CLI commands") added the cache and recorded every
expansion in it whatever its diagnostics. `594b8de4` (2026-08-11, "feat(cli): expand macros through
svelte-package, safely under concurrency") made the expansion carry its errors and refused them for
`svelte-package`, while the cache went on printing and recording them.

## Fix

One function records an expansion in the cache for `cache`, `refresh` and `watch`: a failed
expansion is printed and taken out of the cache with any earlier code of the file, so a plugin
reading the cache expands the file itself and meets the error. `cache` and `refresh` exit non-zero
naming how many files failed; `watch` reports them and goes on. Its re-expansion after a config
change now runs the cache warmer instead of a copy of it.

## Verified

Ran the CLI integration test that caches a project, breaks one file and caches twice more: both runs
exit non-zero naming the error, and the file has no cache entry or code while the clean file keeps
its own. Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed) and the three
groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
