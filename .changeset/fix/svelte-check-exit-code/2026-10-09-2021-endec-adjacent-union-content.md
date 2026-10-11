# fix(endec): encode an adjacently tagged union's variant content

- **Date:** 2026-10-09 20:21
- **Commit:** 6eaf9134, 3bb74ef6 (fixup)
- **Area:** Built-in Encode derive

## Problem

Encoding an adjacently tagged union whose members are object literals, such as
`{ variant: 'CreateInstance'; value: WebInstance }`, left each variant's content unencoded. A record
id inside it reached JSON as `table:key` text, which a typed reader refuses.

## Cause

The union encoder dispatched per-variant encoders only for intersection variants, untagged type refs
and externally tagged single-key objects. An adjacently tagged object member carries no `__type`, so
no encoder matched and the value passed through as it was.

## Introduced

1cc2f3c4 (2026-10-08),
`feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign handlers`. It wrote the
union encoder's object-variant dispatch, which matched internally tagged unions only.

## Fix

Adjacently tagged object members are dispatched by their tag like internally tagged ones, so each
field, the content included, goes through its own encoder and the tag is copied.

## Verified

New test `test_adjacently_tagged_union_encodes_each_variant_content`; the endec tests pass (81). The
fixup records the golden of the adjacently tagged fixture `type_alias_union_default_object.ts`,
which the fix moved.
