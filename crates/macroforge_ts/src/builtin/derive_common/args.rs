//! Reading values out of a decorator's argument text, such as the
//! `skip, rename: "id"` in `@serde(skip, rename: "id")`, and writing string
//! values back out as JavaScript.

/// Check if a decorator argument string contains the given flag.
///
/// Splits the argument string on non-alphanumeric characters and checks if
/// any token matches `flag` (case-insensitive). Returns `false` if the flag
/// is explicitly set to `false` (e.g., `skip: false` or `skip=false`).
pub fn has_flag(args: &str, flag: &str) -> bool {
    if flag_explicit_false(args, flag) {
        return false;
    }

    args.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|token| token.eq_ignore_ascii_case(flag))
}

fn flag_explicit_false(args: &str, flag: &str) -> bool {
    let lower = args.to_ascii_lowercase();
    let condensed: String = lower.chars().filter(|c| !c.is_whitespace()).collect();
    condensed.contains(&format!("{flag}:false")) || condensed.contains(&format!("{flag}=false"))
}

/// The text following the option `name` in `args`: what comes after
/// `name:` or `name =`, or what is inside `name(...)`. The name matches
/// case-insensitively and only as a whole word, so `name` is not found inside
/// `rename`.
pub(crate) fn find_named_value<'a>(args: &'a str, name: &str) -> Option<&'a str> {
    let mut search_start = 0;
    while let Some(found) = args[search_start..]
        .char_indices()
        .map(|(offset, _)| search_start + offset)
        .find(|&idx| {
            args[idx..]
                .get(..name.len())
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
        })
    {
        let starts_word = args[..found]
            .chars()
            .next_back()
            .is_none_or(|previous| !is_identifier_char(previous));
        let after = &args[found + name.len()..];
        let ends_word = after
            .chars()
            .next()
            .is_none_or(|next| !is_identifier_char(next));
        if starts_word && ends_word {
            let after = after.trim_start();
            if let Some(value) = after.strip_prefix(':').or_else(|| after.strip_prefix('=')) {
                return Some(value.trim_start());
            }
            if let Some(inner) = after.strip_prefix('(') {
                return Some(inner.trim_start());
            }
        }
        search_start = found + name.len();
    }
    None
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

/// Extract a named string value from a decorator argument string.
///
/// Looks for patterns like `name: "value"`, `name = "value"`, or `name("value")`
/// and returns the string's value, with its escapes decoded. The name matches
/// case-insensitively and only as a whole word.
///
/// Returns `None` if the name is not found or the value is not a string literal.
pub fn extract_named_string(args: &str, name: &str) -> Option<String> {
    find_named_value(args, name).and_then(parse_string_literal)
}

/// The value of the JavaScript string literal that `input` starts with,
/// quoted with `"` or `'`, with its escape sequences decoded. `None` when
/// `input` does not start with a complete string literal or an escape is
/// malformed.
pub fn parse_string_literal(input: &str) -> Option<String> {
    let mut chars = input.trim_start().chars();
    let quote = chars.next().filter(|c| *c == '"' || *c == '\'')?;
    let mut value = String::new();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(decoded) = decode_escape(&mut chars)? {
                    value.push(decoded);
                }
            }
            '\n' | '\r' => return None,
            c if c == quote => return Some(value),
            c => value.push(c),
        }
    }
    None
}

/// Decodes the escape after a backslash. `Some(None)` is a line
/// continuation, which contributes nothing; `None` is a malformed escape.
fn decode_escape(chars: &mut std::str::Chars<'_>) -> Option<Option<char>> {
    let decoded = match chars.next()? {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        'b' => '\u{8}',
        'f' => '\u{c}',
        'v' => '\u{b}',
        '0' => '\0',
        'x' => hex_char(&chars.by_ref().take(2).collect::<String>(), 2)?,
        'u' => {
            let rest = chars.as_str();
            if let Some(braced) = rest.strip_prefix('{') {
                let close = braced.find('}')?;
                let digits = &braced[..close];
                let decoded = (1..=6)
                    .contains(&digits.len())
                    .then(|| u32::from_str_radix(digits, 16).ok())
                    .flatten()
                    .and_then(char::from_u32)?;
                *chars = braced[close + 1..].chars();
                decoded
            } else {
                hex_char(&chars.by_ref().take(4).collect::<String>(), 4)?
            }
        }
        '\r' => {
            if chars.as_str().starts_with('\n') {
                chars.next();
            }
            return Some(None);
        }
        '\n' | '\u{2028}' | '\u{2029}' => return Some(None),
        other => other,
    };
    Some(Some(decoded))
}

fn hex_char(digits: &str, len: usize) -> Option<char> {
    if digits.len() != len {
        return None;
    }
    u32::from_str_radix(digits, 16)
        .ok()
        .and_then(char::from_u32)
}

/// `text` as a double-quoted JavaScript string literal.
pub fn js_string(text: &str) -> String {
    let mut literal = String::with_capacity(text.len() + 2);
    literal.push('"');
    for c in text.chars() {
        match c {
            '"' => literal.push_str("\\\""),
            '\\' => literal.push_str("\\\\"),
            '\n' => literal.push_str("\\n"),
            '\r' => literal.push_str("\\r"),
            '\u{2028}' => literal.push_str("\\u2028"),
            '\u{2029}' => literal.push_str("\\u2029"),
            c if c.is_control() => literal.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => literal.push(c),
        }
    }
    literal.push('"');
    literal
}

#[cfg(test)]
mod tests {
    use super::{extract_named_string, find_named_value, js_string, parse_string_literal};

    #[test]
    fn escapes_are_decoded() {
        let cases = [
            (r#""say \"hi\"\nthen stop""#, "say \"hi\"\nthen stop"),
            (r"'it\'s'", "it's"),
            (r#""tab\there""#, "tab\there"),
            (r#""\x41B\u{43}""#, "ABC"),
            (r#""back\\slash""#, "back\\slash"),
            ("\"line\\\ncontinued\"", "linecontinued"),
            (r#""\q""#, "q"),
        ];
        for (input, expected) in cases {
            assert_eq!(
                parse_string_literal(input).as_deref(),
                Some(expected),
                "{input}"
            );
        }
    }

    #[test]
    fn malformed_literals_are_rejected() {
        for input in [
            r#""open"#,
            r#""\x4""#,
            r#""\u{110000}""#,
            "\"raw\nnewline\"",
            "bare",
        ] {
            assert_eq!(parse_string_literal(input), None, "{input}");
        }
    }

    #[test]
    fn names_match_whole_words_only() {
        assert_eq!(
            extract_named_string(r#"rename: "a", name: "b""#, "name").as_deref(),
            Some("b")
        );
        assert_eq!(extract_named_string(r#"renamed: "a""#, "rename"), None);
        assert_eq!(
            extract_named_string(r#"Rename("a")"#, "rename").as_deref(),
            Some("a")
        );
        assert_eq!(find_named_value("default = 5", "default"), Some("5"));
        assert_eq!(
            extract_named_string(r#"label: "ünï", name: "x""#, "name").as_deref(),
            Some("x")
        );
    }

    #[test]
    fn a_decoded_string_round_trips_through_js_string() {
        let text = "say \"hi\"\nthen \\ stop\u{2028}";
        assert_eq!(
            parse_string_literal(&js_string(text)).as_deref(),
            Some(text)
        );
    }
}
