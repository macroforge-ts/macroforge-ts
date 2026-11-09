# fix(core): read a decorator's string literals as strict JavaScript does

- **Date:** 2026-10-14 14:34
- **Commit:** 13ff715e
- **Area:** `macroforge_ts` decorator arguments

## Problem

A string in a decorator's arguments was read otherwise than JavaScript reads it: `"\01"` and `"\8"`,
which strict mode refuses, were taken; `"\u{0000041}"` and the surrogate pair `"\uD83D\uDE00"` were
refused; `"\x+4"` and `"\u{+41}"` were taken as escapes. A quoted option name holding an escaped
quote, `"a\"": 1`, was cut at that quote.

## Cause

`decode_escape` read digits with `u32::from_str_radix`, which takes a sign, capped a braced escape
at six digits, read each `\u` alone, and passed a digit escape through. `DecoratorOption::read` took
the first quote after the opening one as the closing one.

## Introduced

`48fc5def` (2026-10-02, "perf: make macro expansion 3.7x faster on real projects") wrote
`decode_escape`; `c7415b9b` (2026-10-13, "fix(core)!: read a decorator's arguments as written") read
a quoted option name.

## Fix

Escapes are read as strict mode reads them, a surrogate pair as its one character and a lone
surrogate refused, and a quoted name ends where `scan::string_literal_len` says its literal does.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose property reads
generated literals of plain letters and escapes as oxc reads them, held to strict mode's refusal of
octal escapes, `\8` and `\9` as Deno and tsc refuse them, and whose tests pin each escape above and
a quoted name ending past an escaped quote.
