# fix(tooling): regenerate the derived docs when bumping the version

- **Date:** 2026-10-25 20:36
- **Commit:** c4f1cc57, d587deb3 (the regenerated docs)
- **Area:** `mf bump`, the MCP docs

## Problem

The merge request pipeline's `verify --check` failed after the 0.6.0 bump: seven MCP docs still
named 0.5.0, in their version line and in the Rust setup page's `macroforge_ts` dependency.

## Cause

`mf bump` regenerated the API data, which records the version, and not the READMEs and MCP docs
derived from it, which record it too.

## Introduced

`c729f98c` (2026-09-22, "Release 0.3.0") gave `mf bump` its docs step, regenerating the API data
alone.

## Fix

`mf bump` also regenerates the derived docs, as `mf docs all` does: the READMEs, the website build
and the MCP docs extracted from it. The seven pages are regenerated at 0.6.0.

## Verified

`cargo test -p mf bump` passed, 3 of 3. After `pixi run build:rust`, `pixi run docs:all` and
`pixi run docs:mcp` changed exactly the seven pages the pipeline reported, in their version alone.
