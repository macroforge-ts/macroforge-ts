/// Normalizes template spacing that Rust's tokenizer introduces.
///
/// Rust's proc_macro tokenizer adds spaces around punctuation, so:
/// - `@{expr}` becomes `@ { expr }`
/// - `{#if cond}` becomes `{ # if cond }`
/// - `{/if}` becomes `{ / if }`
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

        // `{ #` → `{#`, `{ :` → `{:`, `{ $` → `{$`, `{ |` → `{|`
        // For opening constructs, skip whitespace after the control char
        if c == '{' && i + 1 < len {
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && chars[j] == ':' {
                // Handle `{:else}` and `{:else if ...}` and `{:case ...}`
                let mut k = j + 1;
                while k < len && chars[k].is_whitespace() {
                    k += 1;
                }
                let remaining: String = chars[k..].iter().collect();

                // Check for `{:else}` (complete token)
                if remaining.starts_with("else") {
                    let kw_end = k + 4;
                    // Check if it's just `else` followed by whitespace and `}`
                    let mut m = kw_end;
                    while m < len && chars[m].is_whitespace() {
                        m += 1;
                    }
                    if m < len && chars[m] == '}' {
                        // It's `{:else}` - normalize completely
                        result.push_str("{:else}");
                        i = m + 1;
                        continue;
                    }
                    // Otherwise it's `{:else if ...}` - normalize the opening
                }

                // For other `{:` patterns (like `{:else if`, `{:case`), just normalize the opening
                result.push('{');
                result.push(':');
                i = j + 1;
                // Skip whitespace after the control char
                while i < len && chars[i].is_whitespace() {
                    i += 1;
                }
                continue;
            }
            if j < len && matches!(chars[j], '#' | '$' | '|') {
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

        // `{ / keyword }` → `{/keyword}` - closing control flow tags
        // Handle `{/if}`, `{/for}`, `{/while}`, `{/match}`
        if c == '{' && i + 1 < len {
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && chars[j] == '/' {
                // Skip whitespace after /
                let mut k = j + 1;
                while k < len && chars[k].is_whitespace() {
                    k += 1;
                }
                // Check for closing keywords: if, for, while, match
                let remaining: String = chars[k..].iter().collect();
                let closing_keywords = ["if", "for", "while", "match"];
                let mut matched = false;
                for kw in closing_keywords {
                    if remaining.starts_with(kw) {
                        // Check word boundary after keyword
                        let kw_end = k + kw.len();
                        let has_boundary = kw_end >= len
                            || (!chars[kw_end].is_alphanumeric() && chars[kw_end] != '_');
                        if has_boundary {
                            // Find closing brace
                            let mut m = kw_end;
                            while m < len && chars[m].is_whitespace() {
                                m += 1;
                            }
                            if m < len && chars[m] == '}' {
                                // Found complete closing tag: {/keyword}
                                result.push('{');
                                result.push('/');
                                result.push_str(kw);
                                result.push('}');
                                i = m + 1;
                                matched = true;
                                break;
                            }
                        }
                    }
                }
                if matched {
                    continue;
                }
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
