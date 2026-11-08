# fix(core)!: report an @derive that expands nothing or reads part of a type

- **Date:** 2026-10-13 07:03
- **Commit:** baa44619
- **Area:** Expansion host

## Problem

- A declaration with two `@derive` directives expanded one of them: of `/** @derive(Debug) */`
  stacked on `/** @derive(Clone) */`, or the two written on two lines of one comment, only one
  generated anything.
- An `@derive` that stood above nothing a derive expands generated nothing and said nothing: above a
  `const` or a namespace, on a member, above a type with a line comment in between, and on a type
  declared inside a namespace or a function body.
- A derive on an interface, a class or an enum that the module declares again read one declaration.
  `Decode` built a value without the other declaration's members, and `PartialEq` and `Hash` left
  them out.

## Cause

Collecting a declaration's derive target returned at the first `@derive` among its decorators. Only
declarations at the top level of a module are lowered, and nothing looked at the `@derive` comments
no lowered declaration owns. A target is one declaration's fields, and TypeScript makes one type of
every declaration of a name.

## Introduced

41397f41 (2025-11-24, "implemented macro.toml") wrote the collection of a class's derives, which
stopped at the first. 59c4ef58 (2025-12-01, "Switch to /** @derive(...) */ JSDoc macro syntax")
moved the directive into a comment, where one that attaches to nothing is no longer a syntax error.

## Fix

A declaration's target names the macros of every `@derive` it has, each once. A JSDoc holding an
`@derive` that is neither a lowered declaration's decorator nor among the comments stacked directly
above one is an expansion error, which says so when the comment stands on a type declared inside a
namespace or a block. A class, an interface or an enum with a built-in derive that the module
declares more than once is an expansion error naming how many times.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (734 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line with the `unattached_derive` and `merged_declarations` fixtures,
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed), whose `edge-directives` suite calls what each of a declaration's two
`@derive` directives generates, and `pixi run test:rust` (workspace and `tooling/testground/macro`
passed).
