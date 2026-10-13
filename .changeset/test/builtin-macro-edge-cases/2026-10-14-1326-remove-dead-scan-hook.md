# refactor(vite-plugin): remove the scan-cache hook the WebAssembly engine ignores

- **Date:** 2026-10-14 13:26
- **Commit:** 96e45d80
- **Area:** `@macroforge/vite-plugin`

## Problem

`handleHotUpdate` forwarded each changed file to the engine's scan cache, which the WebAssembly
engine the plugin loads does not keep, so the hook did nothing.

## Fix

The hook is removed. The plugin reads the registry the CLI writes and reloads every module when it
changes, which is how a change to one file reaches the expansions that read it.

## Verified

Ran `pixi run test:packages` (the six packages passed), and the three groups on the branch's final
tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
