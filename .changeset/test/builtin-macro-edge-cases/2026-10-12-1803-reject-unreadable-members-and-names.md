# fix(core)!: reject the members and module names a built-in derive cannot read

- **Date:** 2026-10-12 18:03
- **Commit:** 84120912
- **Area:** Lowering, expansion host, shared derive helpers

## Problem

A type with a member under a computed key (`[KEY]: number`) or an index signature
(`[key: string]: unknown`) derived as though the member were not there. `Clone` returned a value
without it, `PartialEq` and `Hash` ignored it, and `Encode` left it out of the JSON, all without a
diagnostic.

A module that declared or imported something of its own named like a global, such as
`interface Record` or `class Map`, broke the derives in it in two ways. A field typed `Map` was read
as the global map and coded as one, and the generated code's own `Record<string, unknown>` and
`new Map()` resolved to the module's declaration.

## Cause

Lowering turned a property into a field only when its key gave a name, and skipped index signatures,
so no derive ever saw such a member. The derives classify a field's type by its name and write code
against global names, and nothing checked that the module left those names alone.

## Introduced

af0a8b4f (2026-09-28, "Remove the swc backend") wrote the oxc lowering that keeps a property only
when its key is a static identifier. a526fc18 (2026-10-12, "fix(syn): lower every field a class or
an object type declares") widened that to quoted and numeric keys and still dropped computed ones.
The by-name reading of field types dates from 65c22961 (2026-04-07, "Syncing monorepo").

## Fix

A field carries how its key is written (`MemberKey`: named, `#private`, or unnamed), and lowering
keeps a computed key and an index signature as an unnamed member under its bracketed source. A
built-in derive on a class or an interface with such a member, own or inherited, and on an alias
with one in its object type or in an inline member of its union or intersection, is an expansion
error that names the member. A type the registry reaches through an intersection gives no field list
when it has one. The `#private` check reads the key kind rather than the name's first character, so
a quoted `'#tag'` is a name.

A module with a built-in derive that declares or imports a type the derives read by name (`Array`,
`Map`, `Set`, `Date`, `Record`, `Partial`, `Pick`, `Omit`, `Required`, `Readonly` and the like), or
a value the generated code calls (`Array`, `Map`, `Set`, `Date`, `Object`, `JSON`, `Number`,
`String`, `Error` and the like), is an expansion error at that declaration. An interface or a type
alias declares no value, so `interface Number` is left alone, and a macro named in an `import macro`
comment is neither.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (161 passed),
`cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground suites
selected by `deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering`
(all passed), `pixi run test:conformance:update` (10 passed), with the regenerated snapshots
reviewed by the shape of each changed line with the `unnamed_members`, `shadowed_globals` and
`shadowed_globals_without_derive` fixtures, and `pixi run test:rust` (workspace and
`tooling/testground/macro` passed).
