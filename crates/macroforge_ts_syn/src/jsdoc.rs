//! JSDoc directive parsing utilities.
//!
//! Pure string-parsing functions for extracting `@name(args)` directives from
//! JSDoc comment bodies. Shared between the SWC and Oxc lowering backends.

use std::collections::HashSet;

/// Parse ALL macro directives from a JSDoc comment body.
/// Returns a Vec of (name, args) tuples for each @directive or @directive(...) found.
/// Supports multiple directives on the same line: `@derive(X) @default(Y)`
/// Also supports directives without parens: `@default` (treated as empty args)
/// Supports multiline decorator arguments: `@overview({ ... \n ... })`
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

    for line in &lines {
        let in_continuation = paren_depth > 0 || brace_depth > 0 || bracket_depth > 0;

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

/// Parse directives from a single text fragment (may contain multiple `@name(args)` directives).
pub fn parse_directives_from_text(
    text: &str,
    valid_annotations: Option<&HashSet<String>>,
    results: &mut Vec<(String, String)>,
) {
    let mut remaining = text;
    while let Some(at_idx) = remaining.find('@') {
        let after_at = &remaining[at_idx + 1..];

        let name_end = after_at
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(after_at.len());

        if name_end == 0 {
            remaining = &remaining[at_idx + 1..];
            continue;
        }

        let name = &after_at[..name_end];

        if let Some(valid) = valid_annotations
            && !valid.contains(&name.to_ascii_lowercase())
        {
            remaining = &remaining[at_idx + 1 + name_end..];
            continue;
        }

        let after_name = &after_at[name_end..];

        let trimmed_after_name = after_name.trim_start();
        if trimmed_after_name.starts_with('(') {
            let paren_start = after_name.len() - trimmed_after_name.len();
            let args_start = paren_start + 1;

            let mut depth: i32 = 1;
            let mut brace_depth: i32 = 0;
            let mut bracket_depth: i32 = 0;
            let mut close_idx = None;

            for (i, c) in after_name[args_start..].char_indices() {
                match c {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 && brace_depth == 0 && bracket_depth == 0 {
                            close_idx = Some(args_start + i);
                            break;
                        }
                    }
                    '{' => brace_depth += 1,
                    '}' => brace_depth = brace_depth.saturating_sub(1),
                    '[' => bracket_depth += 1,
                    ']' => bracket_depth = bracket_depth.saturating_sub(1),
                    _ => {}
                }
            }

            if let Some(close) = close_idx {
                let args = after_name[args_start..close].trim();

                let normalized_name = if name.eq_ignore_ascii_case("derive") {
                    "Derive".to_string()
                } else {
                    name.to_string()
                };

                results.push((normalized_name, args.to_string()));

                let end_of_directive = at_idx + 1 + name_end + close + 1;
                if end_of_directive < remaining.len() {
                    remaining = &remaining[end_of_directive..];
                } else {
                    break;
                }
            } else {
                remaining = &remaining[at_idx + 1..];
            }
        } else {
            let normalized_name = if name.eq_ignore_ascii_case("derive") {
                "Derive".to_string()
            } else {
                name.to_string()
            };

            results.push((normalized_name, String::new()));

            let end_of_directive = at_idx + 1 + name_end;
            if end_of_directive < remaining.len() {
                remaining = &remaining[end_of_directive..];
            } else {
                break;
            }
        }
    }
}
