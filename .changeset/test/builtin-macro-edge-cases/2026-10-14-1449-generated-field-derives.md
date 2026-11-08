# test(core): expand each built-in derive over fields of generated types

- **Date:** 2026-10-14 14:49
- **Commit:** 26007ef1
- **Area:** `macroforge_ts` expansion host

## Fix

A property test expands each built-in derive, and Encode with Decode, over a field of each of 96
generated types, and holds that none panics and that an expansion reporting no error parses.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed).
