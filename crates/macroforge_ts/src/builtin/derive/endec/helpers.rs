//! Helper functions for parsing decorator arguments.

use crate::builtin::derive::common::{find_named_value, whole_string_literal};

/// What the option `name` of a decorator's arguments is given: a string
/// literal's decoded value, or else the expression written there.
pub(crate) fn extract_named_value(args: &str, name: &str) -> Option<String> {
    let value = find_named_value(args, name).filter(|value| !value.is_empty())?;
    Some(whole_string_literal(value).unwrap_or_else(|| value.to_string()))
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
