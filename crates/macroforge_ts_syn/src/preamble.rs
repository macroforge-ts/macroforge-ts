//! The part of a file that has to stay first in it.

/// Where the preamble of `source` ends, in bytes: its byte order mark, a
/// `#!` line, the directives that open the module (`"use client"`), and the
/// comments TypeScript reads only at the top of a file (`/// <reference>`,
/// `// @ts-nocheck`, a `@jsx` pragma), with whatever comments stand before
/// any of these. Code a macro adds at the top of a file goes after it, as
/// each of them loses its meaning with code above it.
pub fn preamble_end(source: &str) -> usize {
    let offset = |rest: &str| source.len() - rest.len();
    let mut rest = source.strip_prefix('\u{feff}').unwrap_or(source);
    if rest.starts_with("#!") {
        rest = after_line(rest);
    }
    let mut end = offset(rest);
    loop {
        rest = rest.trim_start();
        if let Some(comment) = rest.strip_prefix("//") {
            let text = comment.lines().next().unwrap_or_default();
            rest = after_line(rest);
            if text.starts_with('/') || is_check_pragma(text) {
                end = offset(rest);
            }
        } else if let Some(comment) = rest.strip_prefix("/*") {
            let Some((text, after)) = comment.split_once("*/") else {
                break;
            };
            rest = after;
            if text.contains("@jsx") {
                rest = after_blank_line_end(rest);
                end = offset(rest);
            }
        } else if let Some(after) = after_directive(rest) {
            rest = after_blank_line_end(after);
            end = offset(rest);
        } else {
            break;
        }
    }
    end
}

/// `// @ts-check` and `// @ts-nocheck`, which hold for the file they open.
fn is_check_pragma(comment: &str) -> bool {
    matches!(
        comment.split_whitespace().next(),
        Some("@ts-check" | "@ts-nocheck")
    )
}

/// What follows the line `text` starts in.
fn after_line(text: &str) -> &str {
    text.split_once('\n').map_or("", |(_, after)| after)
}

/// `text` without the end of its line, when nothing but blanks is left on it.
fn after_blank_line_end(text: &str) -> &str {
    match text.split_once('\n') {
        Some((line, after)) if line.trim().is_empty() => after,
        Some(_) => text,
        None if text.trim().is_empty() => "",
        None => text,
    }
}

/// What follows the directive `text` starts with: a string literal that is
/// a statement by itself. `None` when it starts with anything else, a string
/// that goes on as an expression among them.
fn after_directive(text: &str) -> Option<&str> {
    let quote = text
        .chars()
        .next()
        .filter(|first| matches!(first, '"' | '\''))?;
    let mut escaped = false;
    let closing = text.char_indices().skip(1).find_map(|(index, letter)| {
        if letter == '\n' {
            return Some(None);
        }
        let closes = letter == quote && !escaped;
        escaped = letter == '\\' && !escaped;
        closes.then_some(Some(index))
    })??;
    let after = text
        .get(closing + quote.len_utf8()..)?
        .trim_start_matches([' ', '\t']);
    match after.strip_prefix(';') {
        Some(ended) => Some(ended),
        None if after.is_empty() || after.starts_with(['\n', '\r']) => Some(after),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::preamble_end;

    /// What `source` holds after its preamble.
    fn body(source: &str) -> &str {
        source
            .get(preamble_end(source)..)
            .expect("the preamble ends on a character")
    }

    #[test]
    fn a_file_without_a_preamble_has_none() {
        assert_eq!(preamble_end(""), 0);
        assert_eq!(body("export const one = 1;\n"), "export const one = 1;\n");
        assert_eq!(
            body("/** Docs. */\nexport class Held {}\n"),
            "/** Docs. */\nexport class Held {}\n"
        );
        assert_eq!(
            body("// A note.\n\nimport { a } from 'a';\n"),
            "// A note.\n\nimport { a } from 'a';\n"
        );
    }

    #[test]
    fn a_byte_order_mark_and_a_hashbang_stay_first() {
        assert_eq!(body("\u{feff}export {};\n"), "export {};\n");
        assert_eq!(body("#!/usr/bin/env node\nexport {};\n"), "export {};\n");
        assert_eq!(
            body("\u{feff}#!/usr/bin/env node\r\nexport {};\r\n"),
            "export {};\r\n"
        );
        assert_eq!(body("#!/usr/bin/env node"), "");
    }

    #[test]
    fn the_directives_that_open_a_module_stay_first() {
        assert_eq!(body("'use client';\n\nexport {};\n"), "\nexport {};\n");
        assert_eq!(
            body("\"use strict\"\n'use client';\nexport {};\n"),
            "export {};\n"
        );
        assert_eq!(
            body("// Ours.\n'use client';\nexport {};\n"),
            "export {};\n"
        );
        assert_eq!(body("'use client'; export {};\n"), " export {};\n");
        // A string that goes on is an expression, and one that never closes
        // is no string.
        assert_eq!(body("'a' + b;\n"), "'a' + b;\n");
        assert_eq!(body("'open\n"), "'open\n");
        assert_eq!(body("'it\\'s';\nexport {};\n"), "export {};\n");
    }

    #[test]
    fn the_comments_read_only_at_the_top_stay_first() {
        assert_eq!(
            body("/// <reference types=\"node\" />\n// @ts-nocheck\n/** Docs. */\nexport {};\n"),
            "/** Docs. */\nexport {};\n"
        );
        assert_eq!(
            body("/*! license */\n// @ts-check\nexport {};\n"),
            "export {};\n"
        );
        assert_eq!(
            body("/** @jsxImportSource preact */\nexport {};\n"),
            "export {};\n"
        );
        // A comment for the line below it is no pragma of the file.
        assert_eq!(
            body("// @ts-ignore\nexport {};\n"),
            "// @ts-ignore\nexport {};\n"
        );
        assert_eq!(body("/* open\nexport {};\n"), "/* open\nexport {};\n");
    }
}
