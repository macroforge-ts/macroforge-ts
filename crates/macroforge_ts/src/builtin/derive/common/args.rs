//! Reading the options out of a decorator's argument text, such as the
//! `skip, rename: "id"` in `@endec(skip, rename: "id")`, and writing string
//! values back out as JavaScript.

use crate::ts_syn::scan::Nesting;

/// The pieces of `text` between its top-level commas, trimmed, the empty
/// ones left out. A comma inside a bracket or a string literal separates
/// nothing.
pub(crate) fn split_top_level_items(text: &str) -> Vec<&str> {
    let mut items = Vec::new();
    let mut nesting = Nesting::default();
    let mut start = 0;
    for (at, letter) in text.char_indices() {
        if letter == ',' && nesting.is_top_level() {
            items.push(text.get(start..at).unwrap_or_default());
            start = at + 1;
        }
        nesting.feed(letter);
    }
    items.push(text.get(start..).unwrap_or_default());
    items
        .into_iter()
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .collect()
}

/// What stands inside the brackets `open` and `close`, when they are around
/// all of `text`: `[a], [b]` opens and ends with a bracket and is not this.
pub(crate) fn unwrapped(text: &str, open: char, close: char) -> Option<&str> {
    let inner = text.trim().strip_prefix(open)?.strip_suffix(close)?;
    let mut nesting = Nesting::default();
    inner
        .chars()
        .all(|letter| {
            let closes_early = letter == close && nesting.is_top_level();
            nesting.feed(letter);
            !closes_early
        })
        .then_some(inner)
}

/// What an option of a decorator is given.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionValue<'a> {
    /// Nothing: the name stands alone, as `skip` does.
    Bare,
    /// What follows the name's `:` or `=`.
    Assigned(&'a str),
    /// What stands in the parentheses after the name, as in `minLength(2)`.
    Called(&'a str),
}

/// One option of a decorator's arguments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecoratorOption<'a> {
    /// The option as written.
    pub text: &'a str,
    /// The word it opens with, which a key may write quoted. Empty when the
    /// option opens with none, as a number or a lone string does.
    pub name: &'a str,
    pub value: OptionValue<'a>,
}

impl<'a> DecoratorOption<'a> {
    fn read(text: &'a str) -> Self {
        let unnamed = Self {
            text,
            name: "",
            value: OptionValue::Bare,
        };
        let quoted = text
            .chars()
            .next()
            .filter(|first| matches!(first, '"' | '\''))
            .and_then(|quote| text.get(1..)?.split_once(quote));
        let (name, after) = quoted.unwrap_or_else(|| {
            text.split_at(
                text.find(|letter: char| !is_identifier_char(letter))
                    .unwrap_or(text.len()),
            )
        });
        let after = after.trim_start();
        let assigned = after.strip_prefix(':').or_else(|| {
            after
                .strip_prefix('=')
                .filter(|rest| !rest.starts_with('>'))
        });
        let value = match assigned {
            Some(value) => OptionValue::Assigned(value.trim()),
            // A string on its own is a value, not the name of an option.
            None if quoted.is_some() => return unnamed,
            None if after.is_empty() => OptionValue::Bare,
            None => match unwrapped(after, '(', ')') {
                Some(inner) => OptionValue::Called(inner.trim()),
                None => return unnamed,
            },
        };
        if name.is_empty() || name.starts_with(|first: char| first.is_ascii_digit()) {
            return unnamed;
        }
        Self { text, name, value }
    }
}

/// The options a decorator's arguments list, in order: its top-level items,
/// with the braces of an object literal around them set aside.
pub fn decorator_options(args: &str) -> Vec<DecoratorOption<'_>> {
    let args = args.trim();
    split_top_level_items(unwrapped(args, '{', '}').unwrap_or(args))
        .into_iter()
        .map(DecoratorOption::read)
        .collect()
}

/// The option `name` of `args`, its name matched in any case. A word inside
/// another option's value is not one: `rename: "skip"` holds no `skip`.
pub(crate) fn find_option<'a>(args: &'a str, name: &str) -> Option<DecoratorOption<'a>> {
    decorator_options(args)
        .into_iter()
        .find(|option| option.name.eq_ignore_ascii_case(name))
}

/// How the flag `flag` is written in `args`: on when it stands alone or is
/// given `true`, off when given `false`. `Err` holds anything else it is
/// given, which turns nothing on.
fn flag_value<'a>(args: &'a str, flag: &str) -> Option<Result<bool, &'a str>> {
    Some(match find_option(args, flag)?.value {
        OptionValue::Bare | OptionValue::Assigned("true") => Ok(true),
        OptionValue::Assigned("false") => Ok(false),
        OptionValue::Assigned(other) | OptionValue::Called(other) => Err(other),
    })
}

/// Whether a decorator's arguments turn `flag` on: `skip`, `skip: true` or
/// `skip = true`.
pub fn has_flag(args: &str, flag: &str) -> bool {
    matches!(flag_value(args, flag), Some(Ok(true)))
}

/// What the option `name` of `args` is given: what comes after `name:` or
/// `name =`, or what is inside `name(...)`.
pub(crate) fn find_named_value<'a>(args: &'a str, name: &str) -> Option<&'a str> {
    match find_option(args, name)?.value {
        OptionValue::Assigned(value) | OptionValue::Called(value) => Some(value),
        OptionValue::Bare => None,
    }
}

/// What an option of a built-in decorator is given.
#[derive(Clone, Copy)]
pub(crate) enum Takes {
    /// `true` or `false`, or nothing at all.
    Flag,
    /// A string literal that is not empty.
    Text,
    /// A string literal naming one of a known set: whether a text names
    /// one, and the set as a message lists it.
    Named(fn(&str) -> bool, &'static str),
    /// Anything: an expression, a list of validators.
    Anything,
}

impl Takes {
    /// Why `value` is not what the option takes, when it is not.
    pub(crate) fn refuses(self, value: OptionValue<'_>) -> Option<String> {
        let written = match value {
            OptionValue::Bare => None,
            OptionValue::Assigned(written) | OptionValue::Called(written) => Some(written),
        };
        let text = written.and_then(whole_string_literal);
        match (self, written, text.as_deref()) {
            (Self::Anything, _, _) | (Self::Flag, None | Some("true" | "false"), _) => None,
            (Self::Flag, Some(written), _) => {
                Some(format!("takes `true` or `false`, not `{written}`"))
            }
            (Self::Text | Self::Named(_, _), None, _) => Some("takes a string".to_string()),
            (Self::Text | Self::Named(_, _), Some(written), None) => {
                Some(format!("takes a string literal, not `{written}`"))
            }
            (Self::Text, _, Some("")) => Some("takes a string that is not empty".to_string()),
            (Self::Text, _, Some(_)) => None,
            (Self::Named(names, known), Some(written), Some(text)) => {
                (!names(text)).then(|| format!("takes {known}, not `{written}`"))
            }
        }
    }
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
    find_named_value(args, name).and_then(whole_string_literal)
}

/// The value of the JavaScript string literal that `input` starts with,
/// quoted with `"` or `'`, with its escape sequences decoded. `None` when
/// `input` does not start with a complete string literal or an escape is
/// malformed.
pub fn parse_string_literal(input: &str) -> Option<String> {
    leading_string_literal(input).map(|(value, _)| value)
}

/// The value of the string literal `input` is, when it is one and nothing
/// more: `"a" + "b"` is an expression, not the text `a`.
pub fn whole_string_literal(input: &str) -> Option<String> {
    leading_string_literal(input)
        .filter(|(_, rest)| rest.trim().is_empty())
        .map(|(value, _)| value)
}

/// The string literal `input` starts with, and what follows it.
fn leading_string_literal(input: &str) -> Option<(String, &str)> {
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
            c if c == quote => return Some((value, chars.as_str())),
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
    format!("\"{}\"", js_string_text(text))
}

/// `text` as it reads between the quotes of a double-quoted JavaScript
/// string literal.
pub fn js_string_text(text: &str) -> String {
    let mut literal = String::with_capacity(text.len());
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
    literal
}

/// `digits` as a JavaScript BigInt literal.
pub fn bigint_literal(digits: &str) -> String {
    let mut literal = String::with_capacity(digits.len() + 1);
    literal.push_str(digits);
    literal.push('n');
    literal
}

/// `pattern` as the text between the slashes of a JavaScript regex literal:
/// each unescaped `/` is escaped, a line break is written as its escape, and
/// everything already escaped is kept as written.
fn regex_body(pattern: &str) -> String {
    let mut body = String::with_capacity(pattern.len());
    let mut escaped = false;
    for letter in pattern.chars() {
        let line_break = match letter {
            '\n' => Some("n"),
            '\r' => Some("r"),
            '\u{2028}' => Some("u2028"),
            '\u{2029}' => Some("u2029"),
            _ => None,
        };
        if let Some(escape) = line_break {
            // A backslash before the break already escapes it.
            if !escaped {
                body.push('\\');
            }
            body.push_str(escape);
            escaped = false;
            continue;
        }
        if letter == '/' && !escaped {
            body.push('\\');
        }
        escaped = letter == '\\' && !escaped;
        body.push(letter);
    }
    body
}

/// `pattern` as a JavaScript regex literal with `flags`.
pub fn regex_literal(pattern: &str, flags: &str) -> String {
    format!("/{}/{flags}", regex_body(pattern))
}

/// Why `pattern` is no regular expression, when it is none.
pub fn regex_error(pattern: &str) -> Option<String> {
    let allocator = oxc::allocator::Allocator::default();
    let body = regex_body(pattern);
    oxc::regular_expression::LiteralParser::new(
        &allocator,
        &body,
        None,
        oxc::regular_expression::Options::default(),
    )
    .parse()
    .err()
    .map(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::{
        DecoratorOption, OptionValue, Takes, decorator_options, extract_named_string,
        find_named_value, flag_value, has_flag, js_string, parse_string_literal, regex_error,
        regex_literal, split_top_level_items, unwrapped, whole_string_literal,
    };

    #[test]
    fn a_pattern_is_written_as_a_literal_that_parses() {
        assert_eq!(regex_literal("a/b", "i"), r"/a\/b/i");
        assert_eq!(regex_literal(r"a\/b", ""), r"/a\/b/");
        assert_eq!(regex_literal("a\nb\r", ""), r"/a\nb\r/");
        assert_eq!(regex_literal("a\\\nb", ""), r"/a\nb/");
        assert_eq!(regex_literal("\u{2028}", ""), ["/", "u2028/"].join("\\"));
        assert_eq!(regex_error("^[A-Z]{3}$"), None);
        assert_eq!(regex_error("a/b\nc"), None);
        assert!(regex_error("(").is_some());
        assert!(regex_error("[a-").is_some());
    }

    #[test]
    fn options_are_the_top_level_items_of_the_arguments() {
        let option = |text, name, value| DecoratorOption { text, name, value };
        assert_eq!(
            decorator_options(r#"{ skip, rename: "a, b", minLength(2), "tag" : 'x', 5, "lone" }"#),
            vec![
                option("skip", "skip", OptionValue::Bare),
                option(
                    r#"rename: "a, b""#,
                    "rename",
                    OptionValue::Assigned(r#""a, b""#)
                ),
                option("minLength(2)", "minLength", OptionValue::Called("2")),
                option(r#""tag" : 'x'"#, "tag", OptionValue::Assigned("'x'")),
                option("5", "", OptionValue::Bare),
                option(r#""lone""#, "", OptionValue::Bare),
            ]
        );
        assert_eq!(
            decorator_options("encodeWith = (value) => [value, 1], skip = false"),
            vec![
                option(
                    "encodeWith = (value) => [value, 1]",
                    "encodeWith",
                    OptionValue::Assigned("(value) => [value, 1]")
                ),
                option("skip = false", "skip", OptionValue::Assigned("false")),
            ]
        );
        assert_eq!(
            split_top_level_items(r#"pattern("^[a,b]\"$"), 'it\'s, one', [1, 2]"#),
            vec![r#"pattern("^[a,b]\"$")"#, r"'it\'s, one'", "[1, 2]"]
        );
        assert_eq!(unwrapped("{ a }, { b }", '{', '}'), None);
        assert_eq!(unwrapped(r#"[ "]", 1 ]"#, '[', ']'), Some(r#" "]", 1 "#));
    }

    #[test]
    fn a_flag_is_on_alone_or_given_true() {
        assert!(has_flag("skip", "skip"));
        assert!(has_flag("{ skip: true }", "skip"));
        assert!(has_flag("Skip = true", "skip"));
        assert!(!has_flag("{ skip: false }", "skip"));
        // A word in another option's value turns nothing on.
        assert!(!has_flag(r#"{ rename: "skip" }"#, "skip"));
        assert!(!has_flag(r#"{ default: "flatten, skip" }"#, "skip"));
        assert!(!has_flag("{ skipDecoding: true }", "skip"));
        // Neither does a value that is no `true`.
        assert_eq!(
            flag_value(r#"{ skip: "yes" }"#, "skip"),
            Some(Err(r#""yes""#))
        );
        assert_eq!(flag_value("{ skip: 1 }", "skip"), Some(Err("1")));
        assert!(!has_flag("{ skip: null }", "skip"));
        assert_eq!(flag_value("{ rename: 1 }", "skip"), None);
    }

    #[test]
    fn an_option_refuses_a_value_of_the_wrong_kind() {
        let refused = |takes: Takes, value| takes.refuses(value);
        for accepted in [
            OptionValue::Bare,
            OptionValue::Assigned("true"),
            OptionValue::Assigned("false"),
        ] {
            assert_eq!(refused(Takes::Flag, accepted), None);
        }
        for wrong in [r#""yes""#, "1", "null", r#""false""#] {
            assert!(
                refused(Takes::Flag, OptionValue::Assigned(wrong)).is_some(),
                "{wrong}"
            );
        }
        assert_eq!(refused(Takes::Text, OptionValue::Assigned(r#""id""#)), None);
        assert_eq!(refused(Takes::Text, OptionValue::Called("'id'")), None);
        for wrong in [
            OptionValue::Bare,
            OptionValue::Assigned("5"),
            OptionValue::Assigned("\"\""),
        ] {
            assert!(refused(Takes::Text, wrong).is_some(), "{wrong:?}");
        }
        let named = Takes::Named(|text| text == "decimal", "\"decimal\"");
        assert_eq!(refused(named, OptionValue::Assigned(r#""decimal""#)), None);
        assert_eq!(
            refused(named, OptionValue::Assigned(r#""hex""#)).as_deref(),
            Some(r#"takes "decimal", not `"hex"`"#)
        );
        assert_eq!(refused(Takes::Anything, OptionValue::Assigned("(")), None);
    }

    #[test]
    fn a_string_literal_is_whole_when_nothing_follows_it() {
        assert_eq!(whole_string_literal(r#" "a" "#).as_deref(), Some("a"));
        assert_eq!(whole_string_literal(r#""a" + "b""#), None);
        assert_eq!(parse_string_literal(r#""a" + "b""#).as_deref(), Some("a"));
        assert_eq!(extract_named_string("rename: 5", "rename"), None);
    }

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
            find_named_value(r#"default: [1, 2], rename: "a""#, "default"),
            Some("[1, 2]")
        );
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
