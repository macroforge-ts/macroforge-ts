# fix(core)!: read a decorator's arguments as written

- **Date:** 2026-10-13 07:03
- **Commit:** c7415b9b
- **Area:** JSDoc directive parser, decorator option readers, expansion host

## Problem

- A bracket in a string ended or swallowed a directive: `@default(":(")` and
  `@endec({ default: "(" })` were dropped whole, with whatever directives followed them.
- A word in a value was read as an option: `@endec({ rename: "skip" })` skipped the field, and
  `@debug({ rename: "skip" })` hid it.
- A value of the wrong kind was taken for a yes: `@hash({ skip: 1 })`, `@partialEq({ skip: null })`
  and `@ord({ skip: "false" })` each skipped the field, and `@endec({ denyUnknownFields: "yes" })`
  denied unknown fields.
- A value that is no string was used as one or ignored: `@endec({ rename: 5 })` coded the field
  under `"5"`, `@endec({ tag: 5 })` tagged with `"5"`, `@debug({ rename: 5 })` kept the field's own
  name, and `@endec({ renameAll: "weird" })` renamed nothing.
- `@endec({ validate: [5] })` and `validate: [email]` checked nothing, and a `pattern` holding an
  unbalanced `[` broke the list it stood in.
- Options that exclude one another were resolved without a word: `tag` with `untagged`, `content`
  without `tag`, `tag` and `content` under one key, the same option given twice on one member.
- An option written where nothing reads it did nothing: `untagged` on an interface, `skip` on a
  type, `tag` on a field, any `@endec` option on an enum.
- `between(5, 1)` and `length(4, 2)` were accepted and refused every value.
- An error two derives each found was printed twice, word for word.

## Cause

A directive's arguments were found by counting brackets and its options by searching the text,
neither of which knew a string literal from the code around it: a flag was on when its name stood
anywhere in the arguments and was not followed by `false`, and a named value was whatever followed
the first occurrence of its name. Nothing checked what kind of value an option was given, or where
it was written.

## Introduced

c600200e (2025-11-24, "Improve decorator span handling and error reporting") wrote `has_flag` as a
search for the flag's name among the words of the arguments. dafd08ef (2025-12-07, "Add Serialize
and Deserialize derive macros for JSON support") read the endec options the same way. bb3be3c2
(2025-12-08, "Support enums and type aliases in all built-in macros") and 0a881898 (2025-12-13,
"Improve deserialization to support field validations on interfaces and type aliases") split a
validator list by brackets and commas wherever they stood. 65c22961 (2026-04-07, "Syncing monorepo")
counted a directive's brackets line by line to find where it ends.

## Fix

One scanner, `Nesting`, tells what is inside a bracket or a string literal, escapes included. The
JSDoc reader uses it to find where a directive ends, and `decorator_options` to split a decorator's
arguments into its options: a name, and what it is given by `:`, `=` or a call. Every reader looks
an option up in that list, so only a top-level option is one.

A type with a built-in derive has each option checked against what it takes. A flag stands alone or
takes `true` or `false`; `rename`, `tag` and `content` take a string literal that is not empty;
`renameAll` takes a convention it knows. Anything else is an expansion error, as are an option given
twice on one member, a field's option on a type, a type's option on a field, any option on an enum,
two ways of tagging on one type, `content` without `tag` or under the tag's own key, and `untagged`,
`externallyTagged` or `content` on a type that is no union.

An item of a `validate` list that is neither a string nor an object is an error, and so is a
`between` or `length` whose least is above its most. A diagnostic identical to one already reported
for the same span is reported once.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (162 passed), whose `jsdoc` tests scan strings holding
brackets, `cargo test -p macroforge_ts --features test-macros --lib` (734 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line with the `decorator_option_values` and `endec_option_values` fixtures,
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed), whose `edge-directives` suite codes fields whose options hold `(`, `]`, `,`
and the words `skip` and `flatten`, and `pixi run test:rust` (workspace and
`tooling/testground/macro` passed).
