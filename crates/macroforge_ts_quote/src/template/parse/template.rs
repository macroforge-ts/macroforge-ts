use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::compiler::compile_template_with_mode;

/// Converts Rust doc attributes back to JSDoc comments.
///
/// When `/** Doc */` goes through Rust's TokenStream, it becomes `# [doc = r" Doc "]`.
/// This function converts them back to `/** Doc */` for valid TypeScript.
fn convert_doc_attributes_to_jsdoc(input: &str) -> String {
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
                eprintln!("[MF_DEBUG_DOC] Found # at pos {}, context: {:?}", i, context);
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
                if j + 3 <= len
                    && chars[j] == 'd'
                    && chars[j + 1] == 'o'
                    && chars[j + 2] == 'c'
                {
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
                                    let doc_text = doc_text.trim();

                                    // Output JSDoc comment
                                    result.push_str("/** ");
                                    result.push_str(doc_text);
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
/// - `{|ident|}` becomes `{ | ident | }`
///
/// This function normalizes these back to the expected format.
fn normalize_template_spacing(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // Handle @ followed by optional whitespace then { or @
        if c == '@' {
            result.push('@');
            i += 1;
            // Skip whitespace after @
            while i < len && chars[i].is_whitespace() {
                i += 1;
            }
            // Check what follows
            if i < len && (chars[i] == '{' || chars[i] == '@') {
                // Don't add space, continue to next iteration which will handle the char
                continue;
            }
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
            if i < len && (chars[i] == '#' || chars[i] == '/' || chars[i] == ':' || chars[i] == '$' || chars[i] == '|') {
                result.push('{');
                // The next char will be handled in the next iteration
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
        }
        else {
            result.push(c);
            i += 1;
        }
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
    parse_template_with_mode(input, false)
}

/// Parses a template for class body content (body! macro).
///
/// This wraps the template in a dummy class for compile-time validation
/// of class member syntax like `static methodName()...`.
pub fn parse_template_body(input: TokenStream2) -> syn::Result<TokenStream2> {
    parse_template_with_mode(input, true)
}

fn parse_template_with_mode(input: TokenStream2, body_mode: bool) -> syn::Result<TokenStream2> {
    // Convert the token stream to a template string
    // The Rowan parser will handle @{expr} interpolations and control flow
    let template_str = input.to_string();

    // Debug: print the raw tokenized string (only in debug builds during tests)
    #[cfg(debug_assertions)]
    if std::env::var("MF_DEBUG_TEMPLATE").is_ok() {
        eprintln!("[MF_DEBUG] Raw tokenized ({} chars): {:?}", template_str.len(), template_str);
    }

    // Convert doc attributes back to JSDoc comments
    // When `/** Doc */` goes through TokenStream, it becomes `# [doc = r" Doc "]`
    let template_str = convert_doc_attributes_to_jsdoc(&template_str);

    #[cfg(debug_assertions)]
    if std::env::var("MF_DEBUG_TEMPLATE").is_ok() {
        eprintln!("[MF_DEBUG] After doc conversion ({} chars): {:?}", template_str.len(), template_str);
    }

    // Normalize spacing: Rust tokenizer adds spaces around punctuation,
    // so "@ { expr }" needs to become "@{expr}" for the lexer to recognize it.
    // Also normalize control flow tags: "{ # if" -> "{#if", "{ / if" -> "{/if", etc.
    let template_str = normalize_template_spacing(&template_str);

    #[cfg(debug_assertions)]
    if std::env::var("MF_DEBUG_TEMPLATE").is_ok() {
        eprintln!("[MF_DEBUG] After normalization ({} chars): {:?}", template_str.len(), template_str);
    }

    // Compile using the new Rowan-based compiler
    let stmts_builder = compile_template_with_mode(&template_str, "__stmts", body_mode)?;

    // Wrap in the expected output structure
    Ok(quote! {
        {
            let mut __stmts: Vec<swc_core::ecma::ast::ModuleItem> = Vec::new();
            let mut __patches: Vec<macroforge_ts::ts_syn::abi::Patch> = Vec::new();
            let __comments = swc_core::common::comments::SingleThreadedComments::default();
            let mut __pending_comments: Vec<swc_core::common::comments::Comment> = Vec::new();
            let __mf_items: Vec<swc_core::ecma::ast::ModuleItem> = #stmts_builder;
            __stmts.extend(__mf_items);
            (__stmts, __patches, __comments)
        }
    })
}
