# fix(core): say an error in its own words, not as a Rust value

- **Date:** 2026-10-13 16:16
- **Commit:** 3a9f7254
- **Area:** Derive diagnostics, expansion host, CLI

## Problem

A message that passed another error on printed it as Rust writes a value:

```
@derive(Default): invalid default expression for 'retries': Parse("Unexpected token")
Error: Patch("the type patches of src/probe.ts do not parse at line 4: …")
```

## Cause

Twenty-two messages in the derives, the patch collector, the dispatcher, the project expander and
the expander's lowering step formatted the error they wrap with `{:?}`. Each of those errors has a
display form. The scan did the same with the parser's list of diagnostics.

## Introduced

`65c22961` (2026-04-07, "Syncing monorepo") carries the derives' messages and the project
expander's. `e7538242` (2025-11-29, "Add dynamic module resolution for macros and manifest API")
wrote the patch collector's.

## Fix

Each prints the error's display form:
`invalid default expression for 'retries': parse error:
Unexpected token`. The project expander
hands a host error on as it is, with its source, where it wrapped the debug text of one. The scan
joins the parser's messages. Errors from JavaScript, which have no other form, are left as they
were.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `default_expression_unparsed` fixture, whose three messages end in
`parse error: Unexpected token`, and the `endec_option_values` one, whose message changed from
`Parse("...")` to the same form, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
