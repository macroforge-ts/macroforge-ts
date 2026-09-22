//! JSDoc directive parsing utilities.
//!
//! Pure string-parsing functions for locating the JSDoc block that belongs to
//! a declaration and extracting `@name(args)` directives from its body. Shared
//! between the SWC and Oxc lowering backends and the expansion host.

use std::collections::HashSet;

/// Modifiers that may sit between a declaration's JSDoc and the start of the
/// declaration's own span.
const DECLARATION_MODIFIERS: &[&str] = &["export", "declare", "abstract", "default", "async"];

/// A `/** … */` block, as 0-based byte offsets into the source. `start` is at
/// the opening `/**` and `end` just past the closing `*/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsDocBlock {
    pub start: usize,
    pub end: usize,
}

impl JsDocBlock {
    /// The text between `/**` and `*/`.
    pub fn body<'source>(&self, source: &'source str) -> &'source str {
        &source[self.start + 3..self.end - 2]
    }
}

/// The JSDoc block that belongs to the declaration starting at `target_start`
/// (0-based): the last one before it, provided only whitespace and
/// declaration modifiers separate the two. A comment that documents an
/// earlier declaration is never returned.
pub fn adjacent_jsdoc(source: &str, target_start: usize) -> Option<JsDocBlock> {
    let search_area = source.get(..target_start)?;
    let close = search_area.rfind("*/")?;
    let start = comment_start(search_area, close)?;
    let end = close + 2;
    let only_modifiers_between = search_area[end..]
        .split_whitespace()
        .all(|word| DECLARATION_MODIFIERS.contains(&word));
    only_modifiers_between.then_some(JsDocBlock { start, end })
}

/// Where the block comment closed by the `*/` at `close` opens. A comment
/// cannot contain `*/`, so it is the first `/**` after the previous `*/`;
/// the last `/**` before `close` could be text inside the comment, such as an
/// example that itself shows a JSDoc.
fn comment_start(source: &str, close: usize) -> Option<usize> {
    let previous_close = source[..close].rfind("*/").map_or(0, |at| at + 2);
    source[previous_close..close]
        .find("/**")
        .map(|offset| previous_close + offset)
}

/// Whether a JSDoc body is a `/** import macro … */` directive: the body is
/// the import itself. Its module paths (`"@playground/macro"`) would
/// otherwise read as `@` directives, while prose or an example that merely
/// mentions one is ordinary documentation.
pub fn is_macro_import_comment(body: &str) -> bool {
    body.trim_start()
        .trim_start_matches('*')
        .trim_start()
        .to_ascii_lowercase()
        .starts_with("import macro")
}

/// The JSDoc block stacked directly above `block`, separated from it by
/// whitespace alone.
pub fn stacked_jsdoc_above(source: &str, block: JsDocBlock) -> Option<JsDocBlock> {
    let before = source[..block.start].trim_end();
    let close = before.strip_suffix("*/")?.len();
    let start = comment_start(before, close)?;
    Some(JsDocBlock {
        start,
        end: close + 2,
    })
}

/// Parse the macro directives of a JSDoc comment body as `(name, args)` pairs.
/// A directive opens a line, several may follow one another on it
/// (`@derive(X) @default(Y)`), parentheses are optional (`@default`) and
/// arguments may span lines. Lines inside a ``` fence are skipped.
pub fn parse_all_macro_directives(
    comment_body: &str,
    valid_annotations: Option<&HashSet<String>>,
) -> Vec<(String, String)> {
    let mut results = Vec::new();

    let lines: Vec<&str> = comment_body
        .lines()
        .map(|line| line.trim().trim_start_matches('*').trim())
        .filter(|line| !line.is_empty())
        .collect();

    let mut accumulated = String::new();
    let mut paren_depth: i32 = 0;
    let mut brace_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;

    let mut in_fence = false;
    for line in &lines {
        let in_continuation = paren_depth > 0 || brace_depth > 0 || bracket_depth > 0;

        // A fenced block is example code; its `@` lines are not directives.
        if !in_continuation && line.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }

        if in_continuation {
            accumulated.push(' ');
            accumulated.push_str(line);
            for c in line.chars() {
                match c {
                    '(' => paren_depth += 1,
                    ')' => paren_depth -= 1,
                    '{' => brace_depth += 1,
                    '}' => brace_depth -= 1,
                    '[' => bracket_depth += 1,
                    ']' => bracket_depth -= 1,
                    _ => {}
                }
            }
            if paren_depth <= 0 && brace_depth <= 0 && bracket_depth <= 0 {
                parse_directives_from_text(&accumulated, valid_annotations, &mut results);
                accumulated.clear();
                paren_depth = 0;
                brace_depth = 0;
                bracket_depth = 0;
            }
            continue;
        }

        if !line.starts_with('@') {
            continue;
        }

        for c in line.chars() {
            match c {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                '{' => brace_depth += 1,
                '}' => brace_depth -= 1,
                '[' => bracket_depth += 1,
                ']' => bracket_depth -= 1,
                _ => {}
            }
        }

        if paren_depth > 0 || brace_depth > 0 || bracket_depth > 0 {
            accumulated = line.to_string();
            continue;
        }

        paren_depth = 0;
        brace_depth = 0;
        bracket_depth = 0;
        parse_directives_from_text(line, valid_annotations, &mut results);
    }

    if !accumulated.is_empty() {
        parse_directives_from_text(&accumulated, valid_annotations, &mut results);
    }

    results
}

/// Parse the directives a text fragment opens with: `@name` or `@name(args)`,
/// one after another. The run ends at the first text that is not a directive,
/// so a tag mentioned in prose (`@returns the enclosing @derive`) is not one.
pub fn parse_directives_from_text(
    text: &str,
    valid_annotations: Option<&HashSet<String>>,
    results: &mut Vec<(String, String)>,
) {
    let mut remaining = text.trim_start();
    while let Some(after_at) = remaining.strip_prefix('@') {
        let name_end = after_at
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(after_at.len());
        if name_end == 0 {
            return;
        }
        let name = &after_at[..name_end];
        let after_name = &after_at[name_end..];

        let trimmed_after_name = after_name.trim_start();
        let (args, rest) = if let Some(args_and_rest) = trimmed_after_name.strip_prefix('(') {
            let Some(close) = closing_paren(args_and_rest) else {
                return;
            };
            (args_and_rest[..close].trim(), &args_and_rest[close + 1..])
        } else {
            ("", after_name)
        };

        let is_valid =
            valid_annotations.is_none_or(|valid| valid.contains(&name.to_ascii_lowercase()));
        if is_valid {
            let normalized_name = if name.eq_ignore_ascii_case("derive") {
                "Derive".to_string()
            } else {
                name.to_string()
            };
            results.push((normalized_name, args.to_string()));
        }
        remaining = rest.trim_start();
    }
}

/// The offset of the `)` that closes a directive's argument list, given the
/// text after its `(`. Parentheses, braces and brackets nest.
fn closing_paren(args: &str) -> Option<usize> {
    let mut depth: i32 = 1;
    let mut brace_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    for (offset, c) in args.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 && brace_depth == 0 && bracket_depth == 0 {
                    return Some(offset);
                }
            }
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod adjacency_tests {
    use super::*;

    #[test]
    fn finds_the_comment_directly_above_a_declaration() {
        let source = "/** @derive(Debug) */\nexport interface Box {}";
        let block = adjacent_jsdoc(source, source.find("interface").unwrap_or_default());
        assert_eq!(
            block.map(|found| found.body(source)),
            Some(" @derive(Debug) ")
        );
    }

    #[test]
    fn ignores_a_comment_that_belongs_to_an_earlier_declaration() {
        let source = "/** @derive(Debug) */\ninterface Box {}\n\nexport type Other = string;";
        let target = source.find("type Other").unwrap_or_default();
        assert_eq!(adjacent_jsdoc(source, target), None);
    }

    #[test]
    fn walks_stacked_comments() {
        let source = "/** first */\n/** second */\ninterface Box {}";
        let target = source.find("interface").unwrap_or_default();
        let nearest = adjacent_jsdoc(source, target);
        assert_eq!(nearest.map(|block| block.body(source)), Some(" second "));
        let above = nearest.and_then(|block| stacked_jsdoc_above(source, block));
        assert_eq!(above.map(|block| block.body(source)), Some(" first "));
    }

    #[test]
    fn an_example_showing_a_jsdoc_stays_inside_its_comment() {
        // The example's inner comments end in `*\u{200b}/`, so they don't close
        // the outer one; its `/**` must not be taken as the comment's start.
        let source = "/**\n * Collects modules.\n *\n * ```ts\n * /** @derive(Form) *\u{200b}/\n * class F {}\n * ```\n */\nexport function collect() {}";
        let target = source.find("export").unwrap_or_default();
        let block = adjacent_jsdoc(source, target);
        assert_eq!(block.map(|found| found.start), Some(0));
        let directives =
            parse_all_macro_directives(block.map_or("", |found| found.body(source)), None);
        assert!(
            directives.is_empty(),
            "the example is not a directive: {directives:?}"
        );
    }

    #[test]
    fn a_tag_mentioned_in_prose_is_not_a_directive() {
        let body = "\n * @returns names from the enclosing @derive, or null\n * @see {@link find} - For `@derive` decorators\n ";
        let valid: HashSet<String> = ["derive".to_string()].into_iter().collect();
        assert_eq!(parse_all_macro_directives(body, Some(&valid)), Vec::new());
    }

    #[test]
    fn a_fenced_example_is_not_a_directive() {
        let body = "\n * @example\n * ```ts\n * @serde({ skip: true })\n * ```\n * @debug\n ";
        let names: Vec<String> = parse_all_macro_directives(body, None)
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert_eq!(names, ["example", "debug"]);
    }

    #[test]
    fn directives_may_follow_one_another_on_a_line() {
        assert_eq!(
            parse_all_macro_directives(" @derive(Debug) @default(1) ", None),
            [
                ("Derive".to_string(), "Debug".to_string()),
                ("default".to_string(), "1".to_string())
            ]
        );
    }

    #[test]
    fn only_an_import_macro_directive_is_an_import_comment() {
        assert!(is_macro_import_comment(
            " import macro { $vec } from \"./vec\"; "
        ));
        assert!(!is_macro_import_comment(
            "\n * Parses import macro comments like `import macro { A } from \"pkg\"`.\n "
        ));
    }
}
