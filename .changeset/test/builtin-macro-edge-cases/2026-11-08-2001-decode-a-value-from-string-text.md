# fix(endec)!: decode a value, and decode JSON text through fromString

- **Date:** 2026-11-08 20:01
- **Commit:** fbe08a0d
- **Area:** endec `Decode` generators, `@macroforge/core/endec` runtime, testground suites

## Problem

`decode` of a type that takes a string read a string as JSON text whenever it spelled a value the
type takes. A string value that happened to be JSON came back unwrapped: `nameDecode('"hello"')`
returned `hello`, so a caller decoding text a user typed, such as a code expression holding a string
literal, lost its quotes.

## Cause

`decode` took both kinds of input in one parameter and told them apart by guessing with
`textOrValue`, which cannot tell a value that starts and ends with a quote from encoded text.

## Introduced

`e6845936` (2026-10-12, "fix(endec)!: code what a field's type arguments, Required and shared values
left uncoded") added `textOrValue`, so that `decode(encode(value))` read back what was encoded for a
type that takes a string. Before it, a string was always the value.

## Fix

`decode` takes a value and never reads a string as text. Every type that derives `Decode` gets
`{name}FromString` (a static `fromString` on a class), which parses JSON text through the new
runtime `decodedText` and decodes the value; an enum's throws like its `decode`. `textOrValue` and
the check for whether a type takes a string are removed. The testground suites decode the text
`encode` writes through `fromString`, and assert that `decode('"quoted"')` keeps its quotes.

## Verified

`pixi run verify` passed: formatting, clippy, the Rust workspace (1501 tests), the packages, the 48
testground suites, the validator suites and the browser e2e, and regenerated the docs.
