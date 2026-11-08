# chore(testground): record the peer dependencies the apps resolve

- **Date:** 2026-10-30 17:39
- **Commit:** 3a1011a61
- **Area:** testground lockfiles (svelte, vanilla, tests)

## Problem

Every testground run rewrote the svelte, vanilla and tests apps' `deno.lock`, leaving the tree dirty
after a run.

## Cause

The runner prepares each app with `deno install --node-modules-dir`, which records the peer each
package resolved, such as svelte with acorn and vite with picomatch. The committed lockfiles lacked
those peers. They were written with the same deno, 2.9.3; what made that install leave the peers out
is not known.

## Introduced

3ca86021a (2026-10-25) last wrote the three lockfiles. Its subject:

> feat(core)!: move to TypeScript 6 and generate code that type-checks without lib anys

## Fix

The three lockfiles as the runner's install writes them. Package versions are unchanged.

## Verified

The lockfiles are the output of the testground runner's own app preparation on this branch; every
resolved package version matches the committed one.
