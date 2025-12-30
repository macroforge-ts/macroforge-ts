//! Rust-style templating for TypeScript code generation (AST-based)
//!
//! Provides a template syntax with interpolation and control flow:
//! - `@{expr}` - Interpolate expressions (calls `.to_string()`)
//! - ` content ` - Ident block: concatenates content without spaces (e.g., `get@{name}` → `getUser`)
//! - `{> "comment" <}` - Line comment: outputs `// comment` (string preserves whitespace)
//! - `{>> "comment" <<}` - Block comment: outputs `/* comment */` (string preserves whitespace)
//! - `///` or `/** */` - Rust doc comments in the template emit JSDoc blocks (`/** ... */`)
//! - `@@{` - Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)
//! - `"string @{expr}"` - String interpolation (auto-detected)
//! - `"'^template ${expr}^'"` - JS backtick template literal (outputs `` `template ${expr}` ``)
//! - `{#if cond}...{/if}` - Conditional blocks
//! - `{#if let pattern = expr}...{/if}` - Pattern matching if-let blocks
//! - `{:else}` - Else clause
//! - `{:else if cond}` - Else-if clause
//! - `{#match expr}{:case pattern}...{/match}` - Match blocks with case arms
//! - `{#for item in list}...{/for}` - Iteration
//! - `{#while cond}...{/while}` - While loop
//! - `{#while let pattern = expr}...{/while}` - While-let pattern matching loop
//! - `{$let name = expr}` - Local constants
//! - `{$let mut name = expr}` - Mutable local binding
//! - `{$do expr}` - Execute side-effectful expression (discard result)
//! - `{$typescript stream}` - Inject a TsStream, preserving its source and runtime_patches (imports)
//!
//! Note: A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com`).

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::compiler::compile_template;

/// Converts Rust doc attributes back to JSDoc comments.
///
/// When `/** Doc */` goes through Rust's TokenStream, it becomes `# [doc = r" Doc "]`.
/// This function converts them back to `/** Doc */` for valid TypeScript.
pub(crate) fn convert_doc_attributes_to_jsdoc(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    #[cfg(debug_assertions)]
    let debug = std::env::var("MF_DEBUG_TEMPLATE").is_ok();

    while i < len {
        // Look for pattern: # [doc = ...]
        if i + 7 < len && chars[i] == '#' {
            #[cfg(debug_assertions)]
            if debug {
                let context: String = chars[i..std::cmp::min(i + 20, len)].iter().collect();
                eprintln!(
                    "[MF_DEBUG_DOC] Found # at pos {}, context: {:?}",
                    i, context
                );
            }
            // Skip whitespace after #
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }

            // Check for [
            if j < len && chars[j] == '[' {
                j += 1;
                // Skip whitespace
                while j < len && chars[j].is_whitespace() {
                    j += 1;
                }

                // Check for "doc"
                if j + 3 <= len && chars[j] == 'd' && chars[j + 1] == 'o' && chars[j + 2] == 'c' {
                    j += 3;
                    // Skip whitespace
                    while j < len && chars[j].is_whitespace() {
                        j += 1;
                    }

                    // Check for =
                    if j < len && chars[j] == '=' {
                        j += 1;
                        // Skip whitespace
                        while j < len && chars[j].is_whitespace() {
                            j += 1;
                        }

                        // Check for optional r prefix
                        if j < len && chars[j] == 'r' {
                            j += 1;
                        }

                        // Check for opening "
                        if j < len && chars[j] == '"' {
                            j += 1;
                            let doc_start = j;

                            // Find closing "
                            while j < len && chars[j] != '"' {
                                j += 1;
                            }
                            let doc_end = j;

                            if j < len && chars[j] == '"' {
                                j += 1;
                                // Skip whitespace
                                while j < len && chars[j].is_whitespace() {
                                    j += 1;
                                }

                                // Check for ]
                                if j < len && chars[j] == ']' {
                                    j += 1;

                                    // Extract and trim doc text
                                    let doc_text: String =
                                        chars[doc_start..doc_end].iter().collect();
                                    let mut doc_text = doc_text.trim().to_string();
                                    if let Some(stripped) = doc_text.strip_prefix('*') {
                                        doc_text = stripped.trim_start().to_string();
                                    }
                                    if let Some(stripped) = doc_text.strip_suffix("*/") {
                                        doc_text = stripped.trim_end().to_string();
                                    }

                                    // Output JSDoc comment
                                    result.push_str("/** ");
                                    result.push_str(&doc_text);
                                    result.push_str(" */");

                                    i = j;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Normalizes template spacing that Rust's tokenizer introduces.
///
/// Rust's proc_macro tokenizer adds spaces around punctuation, so:
/// - `@{expr}` becomes `@ { expr }`
/// - `{#if cond}` becomes `{ # if cond }`
/// - `{/if}` becomes `{ / if }`
/// - `{:else}` becomes `{ : else }`
/// - `{$let x = 1}` becomes `{ $ let x = 1 }`
/// - `ident` becomes `{ | ident | }`
/// - `===` becomes `= = =` (JavaScript strict equality)
/// - `!==` becomes `! = =` (JavaScript strict inequality)
///
/// This function normalizes these back to the expected format.
pub(crate) fn normalize_template_spacing(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // Handle JavaScript operators that get space-separated by Rust tokenizer
        // Check for `= = =` → `===` (strict equality)
        if c == '='
            && i + 4 < len
            && chars[i + 1] == ' '
            && chars[i + 2] == '='
            && chars[i + 3] == ' '
            && chars[i + 4] == '='
        {
            result.push_str("===");
            i += 5;
            continue;
        }

        // Check for `! = =` → `!==` (strict inequality)
        // Note: Rust tokenizes `!==` as `!=` (Ne) followed by `=`, so it becomes `! = =` with spaces
        if c == '!'
            && i + 4 < len
            && chars[i + 1] == ' '
            && chars[i + 2] == '='
            && chars[i + 3] == ' '
            && chars[i + 4] == '='
        {
            result.push_str("!==");
            i += 5;
            continue;
        }

        // Check for `! =` → `!=` (Rust Ne token becomes space-separated)
        if c == '!' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '=' {
            // But first check it's not part of `! = =` (already handled above)
            if !(i + 4 < len && chars[i + 3] == ' ' && chars[i + 4] == '=') {
                result.push_str("!=");
                i += 3;
                continue;
            }
        }

        // Check for `= =` → `==` (equality, but not if part of `= = =`)
        if c == '=' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '=' {
            // Check if this is part of `= = =`
            if i + 4 < len && chars[i + 3] == ' ' && chars[i + 4] == '=' {
                // Part of ===, will be handled by the === check
            } else {
                result.push_str("==");
                i += 3;
                continue;
            }
        }

        // Check for `& &` → `&&` (logical AND)
        if c == '&' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '&' {
            result.push_str("&&");
            i += 3;
            continue;
        }

        // Check for `| |` → `||` (logical OR)
        if c == '|' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '|' {
            result.push_str("||");
            i += 3;
            continue;
        }

        // Check for `: :` → `::` (TypeScript namespace/module access)
        if c == ':' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == ':' {
            result.push_str("::");
            i += 3;
            continue;
        }

        // Check for `= >` → `=>` (arrow function)
        if c == '=' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '>' {
            result.push_str("=>");
            i += 3;
            continue;
        }

        // Check for `< =` → `<=` (less than or equal)
        if c == '<' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '=' {
            result.push_str("<=");
            i += 3;
            continue;
        }

        // Check for `> =` → `>=` (greater than or equal)
        if c == '>' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '=' {
            result.push_str(">=");
            i += 3;
            continue;
        }

        // Check for `+ =` → `+=`
        if c == '+' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '=' {
            result.push_str("+=");
            i += 3;
            continue;
        }

        // Check for `- =` → `-=`
        if c == '-' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '=' {
            result.push_str("-=");
            i += 3;
            continue;
        }

        // Check for `- >` → `->` (not common in TS but used in comments/generics)
        if c == '-' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '>' {
            result.push_str("->");
            i += 3;
            continue;
        }

        // Check for `. . .` → `...` (spread operator)
        if c == '.'
            && i + 4 < len
            && chars[i + 1] == ' '
            && chars[i + 2] == '.'
            && chars[i + 3] == ' '
            && chars[i + 4] == '.'
        {
            result.push_str("...");
            i += 5;
            continue;
        }

        // Restore missing '/' for doc comments that were tokenized without it.
        if c == '*' && i + 1 < len && chars[i + 1] == '*' {
            let mut j = i;
            while j > 0 {
                let prev = chars[j - 1];
                if prev == '\n' || prev == '\r' {
                    break;
                }
                if prev.is_whitespace() {
                    j -= 1;
                    continue;
                }
                break;
            }
            if j == 0 || chars[j.saturating_sub(1)] == '\n' || chars[j.saturating_sub(1)] == '\r' {
                result.push_str("/**");
                i += 2;
                continue;
            }
        }

        // Handle @ followed by optional whitespace then { or @
        if c == '@' {
            // Skip whitespace after @
            let mut peek = i + 1;
            while peek < len && chars[peek].is_whitespace() {
                peek += 1;
            }

            // Check if this is @{ or @@ (interpolation or escape)
            if peek < len && (chars[peek] == '{' || chars[peek] == '@') {
                result.push('@');
                i = peek; // Skip to the { or @
                continue;
            }

            result.push('@');
            i += 1;
        }
        // Handle { followed by optional whitespace then #, /, :, $, or |
        else if c == '{' {
            let start = i;
            i += 1;
            // Skip whitespace after {
            while i < len && chars[i].is_whitespace() {
                i += 1;
            }
            // Check if this is a control flow or ident block
            if i < len
                && (chars[i] == '#'
                    || chars[i] == '/'
                    || chars[i] == ':'
                    || chars[i] == '$'
                    || chars[i] == '|')
            {
                result.push('{');
                // Push the control character (#, /, :, $, |)
                result.push(chars[i]);
                i += 1;
                // Also skip whitespace after the control character (e.g., "{ # for" -> "{#for")
                while i < len && chars[i].is_whitespace() {
                    i += 1;
                }
                continue;
            } else {
                // Not a special construct, output as-is
                result.push('{');
                // Re-add the whitespace we skipped
                for &ch in chars.iter().take(i).skip(start + 1) {
                    result.push(ch);
                }
                continue;
            }
        }
        // Handle | followed by optional whitespace then }
        else if c == '|' {
            result.push('|');
            i += 1;
            // Check if followed by whitespace then }
            let ws_start = i;
            while i < len && chars[i].is_whitespace() {
                i += 1;
            }
            if i < len && chars[i] == '}' {
                // Don't add the whitespace, just continue
                continue;
            } else {
                // Re-add the whitespace we skipped
                for &ch in chars.iter().take(i).skip(ws_start) {
                    result.push(ch);
                }
                continue;
            }
        } else {
            result.push(c);
            i += 1;
        }
    }

    result
}

pub(crate) fn collapse_template_newlines(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    let mut in_string = false;
    let mut string_char = '\0';
    let mut escape_next = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while i < len {
        let ch = chars[i];

        if in_line_comment {
            result.push(ch);
            if ch == '\n' {
                in_line_comment = false;
            }
            i += 1;
            continue;
        }

        if in_block_comment {
            result.push(ch);
            if ch == '*' && i + 1 < len && chars[i + 1] == '/' {
                result.push('/');
                i += 2;
                in_block_comment = false;
                continue;
            }
            i += 1;
            continue;
        }

        if in_string {
            result.push(ch);
            if escape_next {
                escape_next = false;
            } else if ch == '\\' {
                escape_next = true;
            } else if ch == string_char {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if ch == '/' && i + 1 < len {
            if chars[i + 1] == '/' {
                in_line_comment = true;
                result.push(ch);
                result.push('/');
                i += 2;
                continue;
            }
            if chars[i + 1] == '*' {
                in_block_comment = true;
                result.push(ch);
                result.push('*');
                i += 2;
                continue;
            }
        }

        if ch == '\n' || ch == '\r' {
            if !result.ends_with(' ') {
                result.push(' ');
            }
            i += 1;
            continue;
        }

        if ch == '"' || ch == '\'' || ch == '`' {
            in_string = true;
            string_char = ch;
        }

        result.push(ch);
        i += 1;
    }

    result
}

/// Parses a template token stream into Rust that builds TypeScript AST output.
///
/// This function takes the raw token stream from the macro invocation,
/// converts it to a template string, and compiles it using the Rowan-based
/// compiler.
///
/// # Example
///
/// ```ignore
/// ts_template! {
///     const x = @{expr};
///     {#for item in items}
///         console.log(@{item});
///     {/for}
/// }
/// ```
pub fn parse_template(input: TokenStream2) -> syn::Result<TokenStream2> {
    // Convert the token stream to a template string
    // The Rowan parser will handle @{expr} interpolations and control flow
    let template_str = input.to_string();

    // Debug: print the raw tokenized string (only in debug builds during tests)
    #[cfg(debug_assertions)]
    if std::env::var("MF_DEBUG_TEMPLATE").is_ok() {
        eprintln!(
            "[MF_DEBUG] Raw tokenized ({} chars): {:?}",
            template_str.len(),
            template_str
        );
    }

    // Convert doc attributes back to JSDoc comments
    // When `/** Doc */` goes through TokenStream, it becomes `# [doc = r" Doc "]`
    let template_str = convert_doc_attributes_to_jsdoc(&template_str);

    #[cfg(debug_assertions)]
    if std::env::var("MF_DEBUG_TEMPLATE").is_ok() {
        eprintln!(
            "[MF_DEBUG] After doc conversion ({} chars): {:?}",
            template_str.len(),
            template_str
        );
    }

    // Normalize spacing: Rust tokenizer adds spaces around punctuation,
    // so "@ { expr }" needs to become "@{expr}" for the lexer to recognize it.
    // Also normalize control flow tags: "{ # if" -> "{#if", "{ / if" -> "{/if", etc.
    let template_str = normalize_template_spacing(&template_str);
    let template_str = collapse_template_newlines(&template_str);

    #[cfg(debug_assertions)]
    if std::env::var("MF_DEBUG_TEMPLATE").is_ok() {
        eprintln!(
            "[MF_DEBUG] After normalization ({} chars): {:?}",
            template_str.len(),
            template_str
        );
    }

    // Compile using the Rowan-based compiler
    let stmts_builder = compile_template(&template_str, "__stmts")?;

    // Wrap in the expected output structure
    Ok(quote! {
        {
            let mut __stmts: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = Vec::new();
            let mut __patches: Vec<macroforge_ts::ts_syn::abi::Patch> = Vec::new();
            let __comments = macroforge_ts::swc_core::common::comments::SingleThreadedComments::default();
            let mut __pending_comments: Vec<macroforge_ts::swc_core::common::comments::Comment> = Vec::new();
            // Collect injected TsStreams for merging at output
            let mut __injected_streams: Vec<macroforge_ts::ts_syn::TsStream> = Vec::new();
            let __mf_items: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = #stmts_builder;
            __stmts.extend(__mf_items);
            (__stmts, __patches, __comments, __injected_streams)
        }
    })
}

pub(crate) fn strip_doc_comments(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut result = String::with_capacity(input.len());
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i..].starts_with(b"/**")
            && let Some(end) = input[i + 3..].find("*/")
        {
            i += 3 + end + 2;
            continue;
        }

        if bytes[i..].starts_with(b"///") {
            if let Some(end) = input[i + 3..].find('\n') {
                i += 3 + end + 1;
                continue;
            } else {
                break;
            }
        }

        result.push(bytes[i] as char);
        i += 1;
    }

    result
}

pub(crate) fn parse_template_str(template_str: &str) -> syn::Result<TokenStream2> {
    let stmts_builder = compile_template(template_str, "__stmts")?;

    Ok(quote! {
        {
            let mut __stmts: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = Vec::new();
            let mut __patches: Vec<macroforge_ts::ts_syn::abi::Patch> = Vec::new();
            let __comments = macroforge_ts::swc_core::common::comments::SingleThreadedComments::default();
            let mut __pending_comments: Vec<macroforge_ts::swc_core::common::comments::Comment> = Vec::new();
            // Collect injected TsStreams for merging at output
            let mut __injected_streams: Vec<macroforge_ts::ts_syn::TsStream> = Vec::new();
            let __mf_items: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = #stmts_builder;
            __stmts.extend(__mf_items);
            (__stmts, __patches, __comments, __injected_streams)
        }
    })
}
