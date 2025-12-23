/// Escapes a template segment while preserving `${...}` interpolation markers.
pub(crate) fn escape_tpl_segment(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '`' => out.push_str("\\`"),
            '\\' => out.push_str("\\\\"),
            '$' => {
                if matches!(chars.peek(), Some('{')) {
                    chars.next();
                    out.push_str("\\${");
                } else {
                    out.push('$');
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// Escapes a template segment without touching dollar signs.
pub(crate) fn escape_tpl_segment_allow_dollar(input: &str) -> String {
    let mut out = String::new();
    for c in input.chars() {
        match c {
            '`' => out.push_str("\\`"),
            '\\' => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // escape_tpl_segment tests
    // ========================================

    #[test]
    fn escape_tpl_segment_plain_text() {
        assert_eq!(escape_tpl_segment("hello"), "hello");
        assert_eq!(escape_tpl_segment("hello world"), "hello world");
        assert_eq!(escape_tpl_segment("abc123"), "abc123");
    }

    #[test]
    fn escape_tpl_segment_empty_string() {
        assert_eq!(escape_tpl_segment(""), "");
    }

    #[test]
    fn escape_tpl_segment_backticks() {
        assert_eq!(escape_tpl_segment("`"), "\\`");
        assert_eq!(escape_tpl_segment("`template`"), "\\`template\\`");
        assert_eq!(escape_tpl_segment("``"), "\\`\\`");
        assert_eq!(escape_tpl_segment("a`b`c"), "a\\`b\\`c");
    }

    #[test]
    fn escape_tpl_segment_backslashes() {
        assert_eq!(escape_tpl_segment("\\"), "\\\\");
        assert_eq!(escape_tpl_segment("path\\file"), "path\\\\file");
        assert_eq!(escape_tpl_segment("\\\\"), "\\\\\\\\");
        assert_eq!(escape_tpl_segment("a\\b\\c"), "a\\\\b\\\\c");
    }

    #[test]
    fn escape_tpl_segment_dollar_brace() {
        assert_eq!(escape_tpl_segment("${name}"), "\\${name}");
        assert_eq!(escape_tpl_segment("${a}${b}"), "\\${a}\\${b}");
        assert_eq!(escape_tpl_segment("value is ${x}"), "value is \\${x}");
        assert_eq!(escape_tpl_segment("${nested${inner}}"), "\\${nested\\${inner}}");
    }

    #[test]
    fn escape_tpl_segment_lone_dollar() {
        assert_eq!(escape_tpl_segment("$"), "$");
        assert_eq!(escape_tpl_segment("$100"), "$100");
        assert_eq!(escape_tpl_segment("email$domain"), "email$domain");
        assert_eq!(escape_tpl_segment("cost: $50"), "cost: $50");
        assert_eq!(escape_tpl_segment("$var"), "$var");
    }

    #[test]
    fn escape_tpl_segment_dollar_not_followed_by_brace() {
        assert_eq!(escape_tpl_segment("$a"), "$a");
        assert_eq!(escape_tpl_segment("$("), "$(");
        assert_eq!(escape_tpl_segment("$["), "$[");
        assert_eq!(escape_tpl_segment("$ "), "$ ");
    }

    #[test]
    fn escape_tpl_segment_combined() {
        assert_eq!(escape_tpl_segment("`${foo}`"), "\\`\\${foo}\\`");
        assert_eq!(escape_tpl_segment("\\`"), "\\\\\\`");
        assert_eq!(escape_tpl_segment("\\${x}"), "\\\\\\${x}");
        assert_eq!(escape_tpl_segment("`$100`"), "\\`$100\\`");
    }

    #[test]
    fn escape_tpl_segment_consecutive_special_chars() {
        assert_eq!(escape_tpl_segment("``"), "\\`\\`");
        assert_eq!(escape_tpl_segment("\\\\\\"), "\\\\\\\\\\\\");
        assert_eq!(escape_tpl_segment("${${"), "\\${\\${");
        assert_eq!(escape_tpl_segment("`\\`"), "\\`\\\\\\`");
    }

    #[test]
    fn escape_tpl_segment_trailing_dollar() {
        assert_eq!(escape_tpl_segment("end$"), "end$");
        assert_eq!(escape_tpl_segment("$"), "$");
    }

    #[test]
    fn escape_tpl_segment_unicode() {
        assert_eq!(escape_tpl_segment("hello 世界"), "hello 世界");
        assert_eq!(escape_tpl_segment("`emoji 😀`"), "\\`emoji 😀\\`");
        assert_eq!(escape_tpl_segment("${日本語}"), "\\${日本語}");
    }

    // ========================================
    // escape_tpl_segment_allow_dollar tests
    // ========================================

    #[test]
    fn escape_allow_dollar_plain_text() {
        assert_eq!(escape_tpl_segment_allow_dollar("hello"), "hello");
        assert_eq!(escape_tpl_segment_allow_dollar("hello world"), "hello world");
    }

    #[test]
    fn escape_allow_dollar_empty_string() {
        assert_eq!(escape_tpl_segment_allow_dollar(""), "");
    }

    #[test]
    fn escape_allow_dollar_backticks() {
        assert_eq!(escape_tpl_segment_allow_dollar("`"), "\\`");
        assert_eq!(escape_tpl_segment_allow_dollar("`template`"), "\\`template\\`");
        assert_eq!(escape_tpl_segment_allow_dollar("``"), "\\`\\`");
    }

    #[test]
    fn escape_allow_dollar_backslashes() {
        assert_eq!(escape_tpl_segment_allow_dollar("\\"), "\\\\");
        assert_eq!(escape_tpl_segment_allow_dollar("path\\file"), "path\\\\file");
        assert_eq!(escape_tpl_segment_allow_dollar("\\\\"), "\\\\\\\\");
    }

    #[test]
    fn escape_allow_dollar_preserves_dollar_brace() {
        assert_eq!(escape_tpl_segment_allow_dollar("${name}"), "${name}");
        assert_eq!(escape_tpl_segment_allow_dollar("${a}${b}"), "${a}${b}");
        assert_eq!(
            escape_tpl_segment_allow_dollar("value is ${x}"),
            "value is ${x}"
        );
    }

    #[test]
    fn escape_allow_dollar_preserves_lone_dollar() {
        assert_eq!(escape_tpl_segment_allow_dollar("$"), "$");
        assert_eq!(escape_tpl_segment_allow_dollar("$100"), "$100");
        assert_eq!(escape_tpl_segment_allow_dollar("$var"), "$var");
    }

    #[test]
    fn escape_allow_dollar_combined() {
        assert_eq!(escape_tpl_segment_allow_dollar("`${foo}`"), "\\`${foo}\\`");
        assert_eq!(escape_tpl_segment_allow_dollar("\\`"), "\\\\\\`");
        assert_eq!(escape_tpl_segment_allow_dollar("\\${x}"), "\\\\${x}");
    }

    #[test]
    fn escape_allow_dollar_unicode() {
        assert_eq!(escape_tpl_segment_allow_dollar("hello 世界"), "hello 世界");
        assert_eq!(
            escape_tpl_segment_allow_dollar("`emoji 😀`"),
            "\\`emoji 😀\\`"
        );
        assert_eq!(escape_tpl_segment_allow_dollar("${日本語}"), "${日本語}");
    }
}
