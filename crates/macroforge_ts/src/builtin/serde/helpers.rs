//! Helper functions for parsing decorator arguments.

use crate::builtin::derive_common::{find_named_value, parse_string_literal};

/// The value of the option `name` in a decorator's argument text: a string
/// literal's decoded value, or else the expression written there, up to the
/// next top-level `,` or closing bracket.
pub(crate) fn extract_named_value(args: &str, name: &str) -> Option<String> {
    let value = find_named_value(args, name)?;
    parse_string_literal(value).or_else(|| extract_expression_value(value))
}

/// Extract an expression value up to the next `,` or `}` at the same nesting level.
/// Handles arrow functions, function calls, and other expressions.
fn extract_expression_value(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut paren_depth: i32 = 0;
    let mut brace_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut end = trimmed.len();

    for (i, c) in trimmed.char_indices() {
        match c {
            '(' => paren_depth += 1,
            ')' => {
                paren_depth -= 1;
                if paren_depth < 0 {
                    end = i;
                    break;
                }
            }
            '{' => brace_depth += 1,
            '}' => {
                brace_depth -= 1;
                if brace_depth < 0 {
                    end = i;
                    break;
                }
            }
            '[' => bracket_depth += 1,
            ']' => {
                bracket_depth -= 1;
                if bracket_depth < 0 {
                    end = i;
                    break;
                }
            }
            ',' if paren_depth == 0 && brace_depth == 0 && bracket_depth == 0 => {
                end = i;
                break;
            }
            _ => {}
        }
    }

    let result = trimmed[..end].trim();
    if result.is_empty() {
        None
    } else {
        Some(result.to_string())
    }
}

/// Find the position of a comma at the top level (not inside <> brackets)
pub(crate) fn find_top_level_comma(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

/// Split a type string on `|` at the top level only.
///
/// Respects `<>`, `()`, `[]` nesting and string literals (`'...'`, `"..."`),
/// so `Pick<User, 'name' | 'email'>` is **not** split on the `|` inside the
/// angle brackets.  Returns `None` when no top-level `|` exists.
pub(crate) fn split_top_level_union(s: &str) -> Option<Vec<&str>> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut in_string = false;
    let mut string_char = '"';
    let mut start = 0;
    let mut found_pipe = false;

    for (i, c) in s.char_indices() {
        if in_string {
            if c == string_char {
                in_string = false;
            }
            continue;
        }
        match c {
            '\'' | '"' => {
                in_string = true;
                string_char = c;
            }
            '<' | '(' | '[' | '{' => depth += 1,
            '>' | ')' | ']' | '}' => depth = depth.saturating_sub(1),
            '|' if depth == 0 => {
                parts.push(s[start..i].trim());
                start = i + 1;
                found_pipe = true;
            }
            _ => {}
        }
    }

    if !found_pipe {
        return None;
    }
    parts.push(s[start..].trim());
    Some(parts)
}
