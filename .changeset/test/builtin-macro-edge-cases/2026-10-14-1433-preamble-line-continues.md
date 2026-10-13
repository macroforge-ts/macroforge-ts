# fix(syn): end the preamble before a string the next line goes on with

- **Date:** 2026-10-14 14:33
- **Commit:** 61ce060d
- **Area:** `macroforge_ts_syn`

## Problem

A string at the top of a file with no `;`, followed by a line that goes on with it, such as `'a'`
then `(f)();`, was taken for a directive, and code a macro adds was written between the string and
the line that calls it.

## Cause

`after_directive` took a line break after a string as the end of the statement, which it is only
where the next line cannot continue the expression.

## Introduced

`a01aac91` (2026-10-13, "fix(core): keep a file's preamble above the code a macro adds") read a line
break as the end of a directive.

## Fix

A line break ends a directive only where the code after it, past comments, does not continue an
expression.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (195 passed), whose preamble property generates opening
lines and code and checks each directive, the hashbang and each top-only comment oxc reads end
before the preamble does and no statement starts in it, and whose test pins `'a'` followed by
`(f)();`, by a comment and `.length;`, and by `++b;`.
