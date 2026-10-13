# fix(core): make the value traits and Default reach every field of a type

- **Date:** 2026-10-12 17:25
- **Commit:** bb74741f
- **Area:** Built-in Debug, Clone, PartialEq, Hash, PartialOrd, Ord and Default derives, structural
  runtime

## Problem

An edge-case sweep of the built-in derives found code that does not type-check, or is wrong at run
time, for:

1. A class with a `private` or `protected` field: the generated functions read it from outside the
   class (TS2341, TS2445). `Default` on a class with both a `readonly` and a `private` field could
   not assign either.
2. A subclass: only its own fields were cloned, compared, hashed and shown. An interface that
   `extends` another had the same gap.
3. A field keyed by a string or a number, once the lowering reports one: `value.first-name`.
4. A nullable or optional collection under `PartialEq` (`'b.items' is possibly 'undefined'`).
5. `Hash` of an array of a union such as `(string | number)[]`.
6. `Hash` of an enum with both string and number members returned a string for the string ones.
   `Ord` and `PartialOrd` on enums compared strings with `localeCompare`, and called a number and a
   string equal (`Ord`) or unordered (`PartialOrd`).
7. `Default` failed on a `readonly` tuple ("invalid default expression"), read an array of tuples
   `[A, B][]` as a tuple, gave `{}` for a `Record` over a finite set of keys, and `{}` for
   `Readonly<T>`.

## Cause

1. The function body lived in a standalone function beside the class, with a static method
   delegating to it, the reverse of what `Default` and `Decode` do.
2. A derive read `class.fields()` and `interface.fields()`, the type's own declarations.
3. Property access was always written `.name`.
4. The comparison of a collection runs in a callback, where the checker has forgotten that the outer
   `null` test passed.
5. `reduce((hash, item) => ..., 1)` picks the overload whose accumulator has the element type when
   `1` is assignable to it.
6. One branch assumed a string enum, the other returned `value as number`. The enum ordering was
   written for one kind of value.
7. The tuple test was "starts with `[` and ends with `]`", `readonly` was not stripped, and every
   utility type defaulted to `({})`.

## Introduced

1. 1bee0267 (2025-12-16, "Fix class macro code generation to properly emit standalone functions")
   put the body in the standalone function and left the static method delegating to it.
2. 1b9ff6a1 (2025-12-08, "Add built-in derive macros for PartialEq, Hash, Ord, PartialOrd, and
   Default") and the derives before it read a type's own fields. No derive ever followed `extends`.
3. Exposed by the lowering change on this branch.
4. 1cc2f3c4 (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
   handlers") compares a collection element by element in a callback that reads the other collection
   by its original, nullable, expression.
5. The same commit, which hashes an array with `reduce`.
6. bb3be3c2 (2025-12-08, "Support enums and type aliases in all built-in macros") for the hash,
   1b9ff6a1 for `localeCompare`.
7. 3ca86021 (2026-10-25, squashed from "fix(core): generate type-correct, working code for eight
   uncovered shapes") added the tuple default with that test.

## Fix

1. A class's derived function is written as the static method, and the standalone function runs it.
   `Clone` and `Default` build the instance through `instanceWith` and the new `instanceOf`, which
   take the fields as an object. `Default` writes them out under their declared types first, so a
   default of the wrong type still fails to type-check.
2. Every derive works over the fields an instance holds: a superclass's first, then the class's own,
   and likewise an interface's. What it cannot reach is an expansion error: a superclass or extended
   interface the project does not declare, a generic one, a `private` field of a superclass, a
   `#private` field, and, for the derives that build instances, a field set up by a Svelte rune.
3. `Member` renders `.name` or `["first-name"]`, and `property_key` the key of an object literal.
4. The two present collections are handed to a function of their own.
5. `reduce<number>`.
6. An enum member hashes as a field of its kind does. Enums order by `<` and `>`, numbers before
   strings, the same under `Ord` and `PartialOrd`.
7. One `tuple_body` tells a tuple from an array of tuples for every generator. A `Record` over
   literal keys, an alias of them or an enum defaults to each key with the value type's default, and
   `Readonly<T>` as `T`.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line and `pixi run test:rust` (workspace and `tooling/testground/macro` passed).
