use crate::template::{StringPart, template_error};
use quote::ToTokens;

use super::super::utils::extract_string_literal;

/// Parses a normal string literal that may contain `@{}` interpolations.
pub fn parse_string_interpolation(lit: &proc_macro2::Literal) -> syn::Result<Option<Vec<StringPart>>> {
    let raw = lit.to_string();
    let span = lit.span();

    let content = if (raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2)
        || raw.starts_with("r\"")
        || raw.starts_with("r#")
    {
        extract_string_literal(lit)
    } else {
        return Ok(None);
    };

    if !content.contains('@') {
        return Ok(None);
    }

    if content.contains("{#") || content.contains("{/") || content.contains("{:") {
        return Err(template_error(
            span,
            "Template control flow tags cannot be used inside string literals",
            Some(&format!(
                "\"{}...\"",
                content.chars().take(40).collect::<String>()
            )),
        ));
    }

    let mut parts = Vec::new();
    let mut chars = content.chars().peekable();
    let mut current = String::new();
    let mut char_pos = 0usize;
    let mut has_expr = false;

    while let Some(c) = chars.next() {
        char_pos += 1;
        if c == '@' {
            match chars.peek() {
                Some('@') => {
                    chars.next();
                    char_pos += 1;
                    current.push('@');
                }
                Some('{') => {
                    chars.next();
                    char_pos += 1;
                    if !current.is_empty() {
                        parts.push(StringPart::Text(std::mem::take(&mut current)));
                    }

                    let mut expr_str = String::new();
                    let mut brace_depth = 1;
                    let expr_start_pos = char_pos;

                    for ec in chars.by_ref() {
                        char_pos += 1;
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

                    if brace_depth != 0 {
                        return Err(template_error(
                            span,
                            &format!(
                                "Unclosed @{{}} interpolation at position {}",
                                expr_start_pos
                            ),
                            Some(&format!("@{{{}", expr_str)),
                        ));
                    }

                    match syn::parse_str::<syn::Expr>(&expr_str) {
                        Ok(expr) => {
                            has_expr = true;
                            parts.push(StringPart::Expr(expr.to_token_stream()));
                        }
                        Err(parse_err) => {
                            return Err(template_error(
                                span,
                                &format!(
                                    "Invalid Rust expression in string interpolation: {}",
                                    parse_err
                                ),
                                Some(&format!("@{{{}}}", expr_str)),
                            ));
                        }
                    }
                }
                _ => current.push('@'),
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        parts.push(StringPart::Text(current));
    }

    if !has_expr
        && let Some(StringPart::Text(text)) = parts.first()
            && parts.len() == 1 && text == &content {
                return Ok(None);
            }

    Ok(Some(parts))
}
