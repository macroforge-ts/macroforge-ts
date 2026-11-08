# fix(syn): split and substitute type text where TypeScript reads it

- **Date:** 2026-10-14 14:33
- **Commit:** d83708c1
- **Area:** `macroforge_ts_syn`

## Problem

Type text was split and rewritten in places TypeScript does not read it:

- `'\\' | '/'` was no union: the quote after an escaped backslash was read as escaped, so the rest
  of the text stayed inside the string.
- `() => string | number` and `T extends U ? 'a' | 'b' : never` were split into union members, and
  `A | B & C` into intersection members.
- A generic alias's parameter was substituted where it names no type: a key (`{ T: string }`), a
  tuple label, a parameter's name, a qualified name's member (`NS.T`). It was left in place in a
  template literal type (`` `key-${T}` ``), whose field the derives then refused, and was replaced
  inside a name that only holds it (`ÜT`), as identifiers were read as ASCII.

## Cause

Each scanner of type text kept its own string state, the splitter's with "the letter before is no
backslash" as its escape rule, and the splitter took a `|` or `&` at the top level as an operator
wherever it stood. The substitution tokenizer read a template literal as one string, did not look at
what stood around an identifier, and took identifiers to be ASCII.

## Introduced

`c5693462` (2026-04-27, "fix(serde): resolve foreign-type body imports against config, never guess")
added the top-level splitter and the substitution tokenizer.

## Fix

One scanner of string literals (`scan::Quoting`) and one tokenizer (`scan::tokens`), which reads the
types in a template's `${}`, serve every reader of type text. A union or intersection is not split
where a function's or conditional's last part takes the operator, nor an intersection where a `|`
binds looser, and an identifier is substituted only where it names a type.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (195 passed), whose properties split generated unions,
intersections and tuples into the members they were written from, and substitute and list a type's
names only where a type stands, and whose regression tests pin each case above;
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which the Encode error for `() => Circle | null` names the whole function where it
had named `() => Circle`; and the `edge-scanned` and `exact-types` testground suites, which the
WebAssembly engine built before the fix failed with "`` `key-${T}` `` is a template literal type
whose `${T}` takes text a derive cannot check" and which pass after it.
