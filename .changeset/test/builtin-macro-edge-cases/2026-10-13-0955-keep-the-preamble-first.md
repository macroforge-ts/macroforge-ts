# fix(core): keep a file's preamble above the code a macro adds

- **Date:** 2026-10-13 09:55
- **Commit:** 0806fc4b
- **Area:** Expansion host, source mapping

## Problem

A file that opens with something that has to come first lost it to the imports a derive adds:

- `#!/usr/bin/env node` ended up on the line below the imports, where it is a syntax error.
- `'use client';` followed an import, and a string after an import is an expression, not a
  directive.
- `/// <reference types="node" />` and `// @ts-nocheck` stood below the imports, where TypeScript
  does not read them.
- A byte order mark sat in the middle of the output.

## Cause

The block of generated imports was put in front of the expanded code, at its first byte, whatever
the file opened with, and the source map moved everything by the block's length. A call macro's
imports and runtime and the `MF_LOG` header were inserted at the first byte the same way.

## Introduced

65c22961 (2026-04-07, "Syncing monorepo") is the earliest commit in this repository holding the
prepend; the files have opened with the imports since.

## Fix

`preamble_end` finds where a file's preamble ends: its byte order mark, a `#!` line, the directives
that open the module, and the comments read only at the top of a file (`///` directives,
`// @ts-check` and `// @ts-nocheck`, a `@jsx` pragma), with the comments standing before any of
them. A comment for the first declaration, and `// @ts-ignore` for the line below it, are not part
of it. The imports, a call macro's imports and runtime, and the `MF_LOG` header go right after it.
`SourceMapping::with_insertion` replaces `with_prefix`: what lies before the insertion stays, what
lies after moves, and a stretch across it is split.

## Verified

Expanded a file opening with each of `#!`, `'use client'`, `/// <reference>` with `// @ts-nocheck`,
a license comment and a byte order mark: the imports follow each preamble, and stay first after the
license comment. Ran `cargo test -p macroforge_ts_syn --lib` (178 passed), whose `preamble` tests
cover the four kinds and `an_insertion_splits_the_stretch_it_lands_in` the source map,
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `file_preamble` fixture, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
