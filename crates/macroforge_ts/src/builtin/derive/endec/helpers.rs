//! Helper functions for parsing decorator arguments.

use crate::builtin::derive::common::{find_named_value, whole_string_literal};
use crate::ts_syn::scan::top_level_letters;

/// What the option `name` of a decorator's arguments is given: a string
/// literal's decoded value, or else the expression written there.
pub(crate) fn extract_named_value(args: &str, name: &str) -> Option<String> {
    let value = find_named_value(args, name).filter(|value| !value.is_empty())?;
    Some(whole_string_literal(value).unwrap_or_else(|| value.to_string()))
}

/// The offset of the first comma of `s` outside its brackets and strings,
/// as between a generic's arguments.
pub(crate) fn find_top_level_comma(s: &str) -> Option<usize> {
    top_level_letters(s)
        .find(|(_, letter)| *letter == ',')
        .map(|(at, _)| at)
}
