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
