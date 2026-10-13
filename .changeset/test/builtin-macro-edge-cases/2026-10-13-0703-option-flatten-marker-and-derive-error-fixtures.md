# test(core): pin the errors for misused options, flattens, markers and derives

- **Date:** 2026-10-13 07:03
- **Commit:** b7139c56
- **Area:** Conformance fixtures and snapshots

## Problem

No fixture wrote an option with a value of the wrong kind, an option in the wrong place, a flatten
of a type with no fields, two flattened records, two `@default` markers, a declaration the module
merges or an `@derive` attached to nothing.

## Fix

Seven conformance error fixtures: `decorator_option_values`, `endec_option_values`,
`endec_flatten_misuse`, `endec_flattened_records`, `default_several_markers`, `merged_declarations`
and `unattached_derive`. Each holds the valid forms beside the refused ones, so its snapshot shows
which lines report and which do not.

## Verified

Ran `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
shape of each changed line.
