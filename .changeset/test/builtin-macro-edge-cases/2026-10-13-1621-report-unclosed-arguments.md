# fix(core)!: report a decorator whose arguments never close

- **Date:** 2026-10-13 16:21
- **Commit:** 9681f05f
- **Area:** JSDoc directives, expansion host

## Problem

```ts
/** @derive(Default, Decode) */
export interface Settings {
    /** @default(5 */
    retries: number;
    /** @endec({ rename: "label" */
    name: string;
}
```

expanded without a word: `retries` defaulted to `0` and `name` kept its own key. A parenthesis left
open made the whole decorator vanish, and the derive went on as if it had never been written.
`/** @derive(Debug */` above a type generated nothing, as silently.

## Cause

The directive reader stops at an argument list whose `)` it cannot find, and returned what it had
read up to there. Nothing told the reader's caller that a directive had been started.

## Introduced

`65c22961` (2026-04-07, "Syncing monorepo") carries the directive reader, which has dropped an
unclosed directive since. `c729f98c` (2026-09-22, "Release 0.3.0") rewrote the search for the
closing parenthesis and kept the silent return.

## Fix

The reader names the directives whose arguments never close (`unclosed_directives`), limited to the
names a macro reads, so a tag of prose is not one. The host reports each as an error at its comment:
an `@derive(` anywhere, and any other in the comments of a type a derive is on, those of its members
included. A comment on a type no derive is on is left alone.

**Breaking:** a file with such a decorator, which expanded before as if the decorator were not
there, now fails to expand until the parenthesis is closed.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`a_directive_whose_arguments_never_close_is_named` covers six unclosed forms and four that are none,
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `decorator_arguments_unclosed` fixture, which reports its six and leaves the
comment of a type no derive is on alone, and in which no snapshot of the testground changed, and the
three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
