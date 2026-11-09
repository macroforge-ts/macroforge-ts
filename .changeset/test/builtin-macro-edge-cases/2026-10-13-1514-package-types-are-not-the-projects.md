# fix(core)!: take a type imported from a package for none of the project's

- **Date:** 2026-10-13 15:14
- **Commit:** 9d26e6f4
- **Area:** Type registry lookups

## Problem

A type imported from a package was read as the project's own type of that name, when the project
declares one: with `import type { User } from 'some-auth'` and one `interface User` somewhere in the
project, a derive on a type holding the imported `User` read the project's fields and derives for
it. A commit security review flagged it.

## Cause

`TypeRegistry::resolve_in_file` returned a name the project declares once before it looked at the
file's import of that name, so the module the import names played no part. Only a name declared more
than once was resolved by its import.

## Introduced

e65d408c (2026-09-27, "svelte-package: re-expand only the modules a change affects") holds the
lookup that takes an unambiguous name before the file's imports.

## Fix

A name imported by a specifier that is not relative resolves to a module of the project whose path
ends as the specifier does, the longest such end first: `$lib/types/user`, `@/types/user` and
`~types/user` each name `src/lib/types/user.ts`, and `$lib` names `src/lib/index.ts`. The module's
own declarations, the names it exports them under, and what it exports again are all asked. An end
two modules share names neither, and a specifier no module's path ends in names a package, whose
types are none of the project's.

A relative import that leads to no scanned module, and a name the file does not import, are still
found by name: the first may pass through a module the scan does not read, and the second may be
declared for every module.

Breaking: an alias whose name is no part of the path it stands for (`@m` for `src/models`) no longer
finds the types of a barrel there. The types of its files are found by their file names.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`a_name_imported_from_a_package_is_no_type_of_the_project` and
`resolve_in_file_takes_the_longest_ending_and_refuses_one_two_modules_share` cover the specifiers,
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `edge-names` holds a type by a `paths` specifier.
