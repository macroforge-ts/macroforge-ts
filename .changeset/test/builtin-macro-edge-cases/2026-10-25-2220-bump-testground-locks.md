# fix(tooling): rebuild the packages when bumping so the testground locks follow

- **Date:** 2026-10-25 22:20
- **Commit:** d592198a
- **Area:** `mf bump`, `mf verify`, the testground lockfiles

## Problem

After the 0.6.0 bump, the lockfiles of the `library`, `svelte`, `tests` and `vanilla` testground
apps still recorded `@macroforge/core` and the other workspace packages at 0.5.0, and an install of
one of them recorded the new version in one entry and kept the old one in another.

## Cause

Each app links the built packages under `npm/`, not their sources, and its lock records the version
those builds carry. `mf bump` set the version in the sources and never rebuilt the packages or
reinstalled the apps.

## Introduced

`1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") linked the testground apps to the built packages under `npm/`, and `mf bump` did not
follow.

## Fix

`verify`'s package build is one function, `build_packages`: every package in dependency order, the
CLI, then the testground macro package and each app's install. `mf bump` runs it after the root
install, so the apps' locks record the new version.

## Verified

Ran `cargo test -p mf` and `cargo clippy -p mf --all-targets -- -D warnings` (no warnings), then
`target/debug/mf bump --version 0.6.0`, which rebuilt the packages, reinstalled the four apps and
left none of their lockfiles naming a workspace package at 0.5.0; the docs it regenerated were
unchanged.
