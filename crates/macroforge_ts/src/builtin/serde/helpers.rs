//! Helper functions for parsing decorator arguments.

/// Check if a flag is present in decorator arguments (case-insensitive).
/// Returns false if the flag is explicitly set to `false`.
pub fn has_flag(args: &str, flag: &str) -> bool {
    if flag_explicit_false(args, flag) {
        return false;
    }

    args.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|token| token.eq_ignore_ascii_case(flag))
}

pub(crate) fn flag_explicit_false(args: &str, flag: &str) -> bool {
    let lower = args.to_ascii_lowercase();
    let condensed: String = lower.chars().filter(|c| !c.is_whitespace()).collect();
    condensed.contains(&format!("{flag}:false")) || condensed.contains(&format!("{flag}=false"))
}

pub fn extract_named_string(args: &str, name: &str) -> Option<String> {
    let lower = args.to_ascii_lowercase();
    let name_lower = name.to_ascii_lowercase();

    // Find all occurrences and check for whole-word match
    let mut search_start = 0;
    while let Some(relative_idx) = lower[search_start..].find(&name_lower) {
        let idx = search_start + relative_idx;

        // Check that we're at a word boundary (not part of a larger identifier)
        // The character before must be non-alphanumeric (or we're at the start)
        let at_word_start = idx == 0 || {
            let prev_char = lower.chars().nth(idx - 1).unwrap_or(' ');
            !prev_char.is_alphanumeric() && prev_char != '_'
        };

        if at_word_start {
            let remainder = &args[idx + name.len()..];
            let remainder = remainder.trim_start();

            if remainder.starts_with(':') || remainder.starts_with('=') {
                let value = remainder[1..].trim_start();
                // Try string literal first, then expression value
                if let Some(s) = parse_string_literal(value) {
                    return Some(s);
                }
                return extract_expression_value(value);
            }

            if remainder.starts_with('(')
                && let Some(close) = remainder.rfind(')')
            {
                let inner = remainder[1..close].trim();
                if let Some(s) = parse_string_literal(inner) {
                    return Some(s);
                }
                return extract_expression_value(inner);
            }
        }

        // Continue searching from after this match
        search_start = idx + 1;
    }

    None
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

pub(crate) fn parse_string_literal(input: &str) -> Option<String> {
    let trimmed = input.trim();
    let mut chars = trimmed.chars();
    let quote = chars.next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let mut escaped = false;
    let mut buf = String::new();
    for c in chars {
        if escaped {
            buf.push(c);
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == quote {
            return Some(buf);
        }
        buf.push(c);
    }
    None
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
