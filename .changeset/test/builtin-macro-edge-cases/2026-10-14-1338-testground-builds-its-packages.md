# fix(scripts): build the packages the testground links before installing it

- **Date:** 2026-10-14 13:38
- **Commit:** c9954b65
- **Area:** `mf test testground`

## Problem

The testground suites ran the Svelte preprocessor, the Vite plugin, the TypeScript plugin and
`@macroforge/shared` as they were last built, not as the checkout holds them: a change to one of
them passed the testground without being exercised there.

## Cause

The testground apps link the built `npm/` packages, and a fresh install copies them, but the run
rebuilt only `core`, through `build:rust`, before installing.

## Introduced

`1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") linked the apps to the built packages.

## Fix

The run builds `shared`, the Svelte preprocessor, the TypeScript plugin and the Vite plugin before
it installs the apps.

## Verified

Rebuilt the four packages and reinstalled the testground, after which the Svelte testground ran the
preprocessor that fails a block on a macro error, where it had logged it. Ran the three groups on
the branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
