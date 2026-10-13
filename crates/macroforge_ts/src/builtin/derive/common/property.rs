//! A property by its name, rendered so that a name which is not an
//! identifier, such as `first-name` or `0`, still names the property.

use crate::ts_syn::ToTsString;

use super::js_string;

/// Whether `name` can follow a `.` or stand unquoted as an object key.
/// Anything outside ASCII is quoted rather than judged.
pub fn is_identifier_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || matches!(first, '_' | '$'))
        && chars.all(|letter| letter.is_ascii_alphanumeric() || matches!(letter, '_' | '$'))
}

/// A property reached off an object: `.name`, or `["first-name"]`. It follows
/// the object in a template with nothing between them: `value@{member}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Member(String);

impl Member {
    pub fn of(name: &str) -> Self {
        Self(if is_identifier_name(name) {
            format!(".{name}")
        } else {
            format!("[{}]", js_string(name))
        })
    }
}

impl Member {
    /// A member of a dictionary by its key: `["name"]`. A type with a string
    /// index signature reads `.constructor` and `.toString` as the members
    /// every object has, and a key in brackets as one of its own.
    pub fn keyed(name: &str) -> Self {
        Self(format!("[{}]", js_string(name)))
    }
}

impl ToTsString for Member {
    fn to_ts_string(&self) -> String {
        self.0.clone()
    }
}

/// A property's key in an object literal or an object type: `name`, or
/// `"first-name"`.
pub fn property_key(name: &str) -> String {
    if is_identifier_name(name) {
        name.to_string()
    } else {
        js_string(name)
    }
}

/// `name` as part of a generated local's name: itself when it is an
/// identifier, and otherwise with each other character spelled as `$` and its
/// code point, so two names never share a local.
pub fn local_name_part(name: &str) -> String {
    if is_identifier_name(name) {
        return name.to_string();
    }
    name.chars()
        .map(|letter| {
            if letter.is_ascii_alphanumeric() || letter == '_' {
                letter.to_string()
            } else {
                format!("${:x}", u32::from(letter))
            }
        })
        .collect()
}
