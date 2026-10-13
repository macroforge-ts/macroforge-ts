# fix(core): let a file's own declaration answer for its name, scanned or not

- **Date:** 2026-10-13 18:00
- **Commit:** de62444c
- **Area:** Type registry, expansion host

## Problem

In a file the scan has not read, a Svelte component, a file under a directory the scan skips or one
newer than the scan, a type the file declares under a name the project declares once elsewhere was
taken for the project's type:

```ts
// the project declares one EdgeItem, which derives Clone, PartialEq and Hash
export interface EdgeItem {
    other: boolean;
}

/** @derive(Clone, PartialEq, Hash) */
export interface LocalHolder {
    held: EdgeItem;
}
```

`LocalHolder`'s functions called `edgeItemClone`, `edgeItemEquals` and `edgeItemHashCode`, the
functions of the project's type, which this file neither has nor imports.

## Cause

The registry holds what the scan read. For a name the calling file does not import it answers with
the one type the project declares under it, and a file the scan did not read has no entry of its own
to answer with first.

## Introduced

`65c22961` (2026-04-07, "Syncing monorepo") carries the lookup of a name the project declares once,
whatever file asks.

## Fix

The expander tells the registry the names the file it is expanding declares
(`TypeRegistry::declaring`), and a name among them resolves to that file's own entry and no other:
its entry where the scan read the file, by either spelling of its path, and none where it did not. A
type without an entry is coded as a file expanded on its own codes it.

## Verified

Probed a file under `dist`, which the scan skips, through the CLI before the fix. Ran
`cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`a_name_a_file_declares_is_not_another_files_type` covers an unscanned file, another file and a
scanned one under either spelling of its path,
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which no snapshot changed, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which the `cli`
suite expands such a file and finds none of the project type's functions called.
