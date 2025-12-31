/// Normalizes template spacing that Rust's tokenizer introduces.
///
/// Rust's proc_macro tokenizer adds spaces around punctuation, so:
/// - `@{expr}` becomes `@ { expr }`
/// - `{#if cond}` becomes `{ # if cond }`
///
/// This function collapses these back to the expected format.
pub fn normalize_template(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // `@ {` → `@{` (with optional whitespace)
        if c == '@' && i + 1 < len {
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && chars[j] == '{' {
                result.push('@');
                result.push('{');
                i = j + 1;
                continue;
            }
        }

        // `{ #` → `{#`, `{ /` → `{/`, `{ :` → `{:`, `{ $` → `{$`, `{ |` → `{|`
        if c == '{' && i + 1 < len {
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && matches!(chars[j], '#' | '/' | ':' | '$' | '|') {
                result.push('{');
                result.push(chars[j]);
                i = j + 1;
                // Also skip whitespace after the control char
                while i < len && chars[i].is_whitespace() {
                    i += 1;
                }
                continue;
            }
        }

        // `| }` → `|}`
        if c == '|' && i + 1 < len {
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && chars[j] == '}' {
                result.push('|');
                result.push('}');
                i = j + 1;
                continue;
            }
        }

        result.push(c);
        i += 1;
    }

    result
}
