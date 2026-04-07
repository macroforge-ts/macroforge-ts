//! String and template literal interpolation.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Check if a literal is a string (starts with " or ').
pub fn is_string_literal(lit: &proc_macro2::Literal) -> bool {
    let s = lit.to_string();
    s.starts_with('"') || s.starts_with('\'') || s.starts_with("r\"") || s.starts_with("r#")
}

/// Check if a literal is a backtick template literal marker: `"'^...^'"`.
/// This syntax outputs JS template literals with backticks: `` `...` ``.
pub fn is_backtick_template(lit: &proc_macro2::Literal) -> bool {
    let s = lit.to_string();
    // Check for "'^...^'" pattern (the outer quotes are part of the Rust string)
    if s.starts_with("\"'^") && s.ends_with("^'\"") && s.len() >= 6 {
        return true;
    }
    // Also support raw strings: r"'^...^'" or r#"'^...^'"#
    if s.starts_with("r\"'^") && s.ends_with("^'\"") {
        return true;
    }
    if s.starts_with("r#\"'^") && s.ends_with("^'\"#") {
        return true;
    }
    false
}

/// Process a backtick template literal `"'^...^'"` -> `` `...` ``.
/// Supports `@{expr}` interpolation for Rust expressions within the template.
pub fn process_backtick_template(lit: &proc_macro2::Literal) -> TokenStream2 {
    let raw = lit.to_string();

    // Extract content between '^...^' markers
    let content = if raw.starts_with("\"'^") && raw.ends_with("^'\"") {
        &raw[3..raw.len() - 3]
    } else if raw.starts_with("r\"'^") && raw.ends_with("^'\"") {
        &raw[4..raw.len() - 3]
    } else if raw.starts_with("r#\"'^") && raw.ends_with("^'\"#") {
        &raw[5..raw.len() - 4]
    } else {
        return quote! { __out.push_str(#raw); };
    };

    // Check if there are any @{} interpolations or @@ escapes
    if !content.contains('@') {
        // No @ at all, output the backtick string as-is
        // The content may contain ${} for JS interpolation, which passes through
        let mut output = TokenStream2::new();
        output.extend(quote! { __out.push_str("`"); });
        output.extend(quote! { __out.push_str(#content); });
        output.extend(quote! { __out.push_str("`"); });
        return output;
    }

    // Handle @{} Rust interpolations and @@ escapes within the backtick template
    let mut output = TokenStream2::new();
    output.extend(quote! { __out.push_str("`"); });

    let mut chars = content.chars().peekable();
    let mut current_literal = String::new();

    while let Some(c) = chars.next() {
        if c == '@' {
            match chars.peek() {
                Some(&'@') => {
                    // @@ -> literal @
                    chars.next(); // Consume second @
                    current_literal.push('@');
                }
                Some(&'{') => {
                    // @{ -> interpolation
                    // Flush current literal
                    if !current_literal.is_empty() {
                        output.extend(quote! { __out.push_str(#current_literal); });
                        current_literal.clear();
                    }

                    chars.next(); // Consume '{'

                    // Collect expression until matching '}'
                    let mut expr_str = String::new();
                    let mut brace_depth = 1;

                    for ec in chars.by_ref() {
                        if ec == '{' {
                            brace_depth += 1;
                            expr_str.push(ec);
                        } else if ec == '}' {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                break;
                            }
                            expr_str.push(ec);
                        } else {
                            expr_str.push(ec);
                        }
                    }

                    // Parse the expression and generate interpolation code
                    if let Ok(expr) = syn::parse_str::<syn::Expr>(&expr_str) {
                        output.extend(quote! {
                            __out.push_str(&macroforge_ts::ts_syn::ToTsString::to_ts_string(&#expr));
                        });
                    } else {
                        // Failed to parse, output as literal
                        let fallback = format!("@{{{}}}", expr_str);
                        output.extend(quote! { __out.push_str(#fallback); });
                    }
                }
                _ => {
                    // Just a literal @
                    current_literal.push('@');
                }
            }
        } else {
            current_literal.push(c);
        }
    }

    // Flush remaining literal
    if !current_literal.is_empty() {
        output.extend(quote! { __out.push_str(#current_literal); });
    }

    output.extend(quote! { __out.push_str("`"); });
    output
}

/// Process a string literal and handle `@{expr}` interpolations inside it.
pub fn interpolate_string_literal(lit: &proc_macro2::Literal) -> TokenStream2 {
    let raw = lit.to_string();

    // Determine quote character and extract content
    let (quote_char, content) = if raw.starts_with('"') {
        ('"', &raw[1..raw.len() - 1])
    } else if raw.starts_with('\'') {
        ('\'', &raw[1..raw.len() - 1])
    } else if raw.starts_with("r\"") {
        // Raw string r"..."
        ('"', &raw[2..raw.len() - 1])
    } else if raw.starts_with("r#") {
        // Raw string r#"..."# - find the actual content
        let hash_count = raw[1..].chars().take_while(|&c| c == '#').count();
        let start = 2 + hash_count; // r + # + "
        let end = raw.len() - 1 - hash_count; // " + #
        ('"', &raw[start..end])
    } else {
        // Not a string we recognize, just output as-is
        return quote! { __out.push_str(#raw); };
    };

    // Check if there are any interpolations or escapes
    if !content.contains('@') {
        // No @ at all, output the string as-is
        return quote! { __out.push_str(#raw); };
    }

    // Parse and interpolate
    let mut output = TokenStream2::new();
    let quote_str = quote_char.to_string();
    output.extend(quote! { __out.push_str(#quote_str); });

    let mut chars = content.chars().peekable();
    let mut current_literal = String::new();

    while let Some(c) = chars.next() {
        if c == '@' {
            match chars.peek() {
                Some(&'@') => {
                    // @@ -> literal @
                    chars.next(); // Consume second @
                    current_literal.push('@');
                }
                Some(&'{') => {
                    // @{ -> interpolation
                    // Flush current literal
                    if !current_literal.is_empty() {
                        output.extend(quote! { __out.push_str(#current_literal); });
                        current_literal.clear();
                    }

                    chars.next(); // Consume '{'

                    // Collect expression until matching '}'
                    let mut expr_str = String::new();
                    let mut brace_depth = 1;

                    for ec in chars.by_ref() {
                        if ec == '{' {
                            brace_depth += 1;
                            expr_str.push(ec);
                        } else if ec == '}' {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                break;
                            }
                            expr_str.push(ec);
                        } else {
                            expr_str.push(ec);
                        }
                    }

                    // Parse the expression and generate interpolation code
                    if let Ok(expr) = syn::parse_str::<syn::Expr>(&expr_str) {
                        output.extend(quote! {
                            __out.push_str(&macroforge_ts::ts_syn::ToTsString::to_ts_string(&#expr));
                        });
                    } else {
                        // Failed to parse, output as literal
                        let fallback = format!("@{{{}}}", expr_str);
                        output.extend(quote! { __out.push_str(#fallback); });
                    }
                }
                _ => {
                    // Just a literal @
                    current_literal.push('@');
                }
            }
        } else if c == '\\' {
            // Handle escape sequences - pass through as-is
            current_literal.push(c);
            if chars.peek().is_some() {
                current_literal.push(chars.next().unwrap());
            }
        } else {
            current_literal.push(c);
        }
    }

    // Flush remaining literal
    if !current_literal.is_empty() {
        output.extend(quote! { __out.push_str(#current_literal); });
    }

    output.extend(quote! { __out.push_str(#quote_str); });

    output
}
