# fix(core)!: read the names in an intersection, an alias chain and a base as their file imports them

- **Date:** 2026-10-13 15:31
- **Commit:** 60a80cbc
- **Area:** Derive field helpers, type registry, macro context

## Problem

Four lookups took a type by its bare name, wherever it was declared:

- The members of an intersection. `type Mixed = Item & { extra: string }` after
  `import type {
  EdgeItem as Item }` had no fields a derive could read, and with an unrelated
  `Item` declared elsewhere took that type's fields.
- The check for a circular alias, which found no cycle among aliases whose names two files declare,
  and so let a derive follow one.
- `MacroContextIR::import_specifier_for`, which for a name this file declares and another file
  declares too could answer with the other file's module, so generated code imported functions the
  file already has.
- The checks that a class or an interface does not extend itself, which took two types of one name
  in two files for one.

`declared_type_has_derive` also fell back to any type of the name when the file's imports led to
none, so a type imported from a package was taken to derive what a project type of its name derives.

## Cause

Each was written against `TypeRegistry::get` or the primary map, which hold one type a name, and
`get` answers nothing for a name declared twice.

## Introduced

- `04692eb4` (2026-04-29, "refactor(macroforge_ts): host, derive, and serde improvements") wrote the
  intersection flattening against `get`.
- `c729f98c` (2026-09-22, "Release 0.3.0") added the alias cycle check by name.
- `4667d955` (2026-04-29, "refactor(macroforge_ts_syn): refine type-registry resolution") wrote
  `import_specifier_for` against the primary map.
- `4f7ffa55` (2026-10-12, "fix(core): make the value traits and Default reach every field of a
  type") added the heritage walks, with their visited lists kept by name.
- `e9dcb729` (2026-10-13, "fix(core): ask what a type derives by the declaration its file's imports
  lead to") added `declared_type_has_derive` with the fallback by name.

## Fix

Each resolves a name through the file that writes it, and moves to the declaring file of what it
finds before reading on:

- `flatten_intersection_fields` and `get_effective_fields` take a `Declared` (the registry, a file
  and its imports) in place of the registry, and `fields_from_definition` takes the registry entry,
  whose file its names are read in. `derive_alias_fields(input, alias)` is the form a derive calls,
  beside `derive_class_fields` and `derive_interface_fields`.
- `TypeRegistry::alias_cycle` takes the file and its imports, and follows each alias in its own.
- `import_specifier_for` resolves the name as a file that does not import it knows it: its own
  declaration first, then the one the project declares.
- The cycle checks keep declarations, by file and name.
- `declared_type_has_derive` answers `false` for a name the imports lead to no declaration from.

**Breaking:** `flatten_intersection_fields`, `get_effective_fields`, `fields_from_definition` and
`TypeRegistry::alias_cycle` changed their parameters. The type-aware macros page lists
`derive_alias_fields` and `declared_type_has_derive` in place of the forms by name.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (178 passed), whose
`alias_cycle_reads_each_alias_in_its_own_file` and
`a_name_several_files_declare_is_this_files_own_or_none` cover the registry and the context,
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`an_intersection_member_is_the_type_its_file_imports` covers a renamed import and two types of one
name, `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
lines that changed, in which five snapshots gained the registry reads of aliases whose names several
files declare, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `edge-names` round-trips `NameMixed`, an intersection over a type
imported under another name.
