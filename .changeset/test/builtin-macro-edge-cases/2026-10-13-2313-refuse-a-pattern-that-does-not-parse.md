# fix(endec): refuse a pattern that is no regular expression

- **Date:** 2026-10-13 23:13
- **Commit:** 63a9a5b3
- **Area:** The `pattern` validator

## Problem

```ts
/** @derive(Decode) */
export interface Code {
    /** @decode({ validate: [{ pattern: "(" }] }) */ value: string;
}
```

The decoder held `/(/`, a syntax error in the generated code. A pattern holding a line break gave a
literal broken across lines.

## Cause

The pattern was written between two slashes with only `/` escaped, and never parsed.

## Introduced

`d0953080` (2026-04-11, "Harden declarative macro system for production") wrote `regex_literal` this
way.

## Fix

A line break is written as its escape, and a pattern that does not parse as a regular expression is
an expansion error that says why.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose
`a_pattern_is_written_as_a_literal_that_parses` covers a slash, an escaped slash, the four line
breaks and three patterns that do not parse, and
`a_pattern_that_is_no_regular_expression_is_refused` the validator,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `validator_pattern_unparsed` fixture, and the three groups on the branch's
final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
