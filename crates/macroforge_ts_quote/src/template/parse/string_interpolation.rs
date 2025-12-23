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

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Literal;

    #[test]
    fn test_not_string_literal() {
        let lit = Literal::i32_unsuffixed(42);
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_string_without_at_sign() {
        let lit = Literal::string("hello world");
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_basic_interpolation() {
        let lit: Literal = syn::parse_str(r#""hello @{name}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "hello "));
        assert!(matches!(&parts[1], StringPart::Expr(_)));
    }

    #[test]
    fn test_multiple_interpolations() {
        let lit: Literal = syn::parse_str(r#""@{greeting} @{name}!""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 4);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
        assert!(matches!(&parts[1], StringPart::Text(s) if s == " "));
        assert!(matches!(&parts[2], StringPart::Expr(_)));
        assert!(matches!(&parts[3], StringPart::Text(s) if s == "!"));
    }

    #[test]
    fn test_escape_double_at() {
        let lit: Literal = syn::parse_str(r#""email@@example.com""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "email@example.com"));
    }

    #[test]
    fn test_at_without_brace() {
        let lit: Literal = syn::parse_str(r#""email@domain.com""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        // Should return None because @ is not followed by {
        assert!(result.is_none());
    }

    #[test]
    fn test_nested_braces() {
        let lit: Literal = syn::parse_str(r#""@{obj.get(key)}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
    }

    #[test]
    fn test_deeply_nested_braces() {
        let lit: Literal = syn::parse_str(r#""@{map.get(vec)}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
    }

    #[test]
    fn test_unclosed_interpolation() {
        let lit: Literal = syn::parse_str(r#""@{name""#).unwrap();
        let result = parse_string_interpolation(&lit);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unclosed"));
    }

    #[test]
    fn test_invalid_expression() {
        let lit: Literal = syn::parse_str(r#""@{+++}""#).unwrap();
        let result = parse_string_interpolation(&lit);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid Rust expression"));
    }

    #[test]
    fn test_control_flow_rejected() {
        // Control flow tags are only checked if @ interpolation is present
        let lit = Literal::string("{#if x}yes{/if}");
        let result = parse_string_interpolation(&lit);
        // This should return None because there's no @ sign
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_raw_string() {
        let lit: Literal = syn::parse_str(r#"r"hello @{name}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_empty_string_with_at() {
        let lit: Literal = syn::parse_str(r#""@{x}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
    }

    #[test]
    fn test_mixed_escape_and_interpolation() {
        let lit: Literal = syn::parse_str(r#""@@{literal} @{expr}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "@{literal} "));
        assert!(matches!(&parts[1], StringPart::Expr(_)));
    }

    #[test]
    fn test_interpolation_at_start() {
        let lit: Literal = syn::parse_str(r#""@{x} world""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
        assert!(matches!(&parts[1], StringPart::Text(s) if s == " world"));
    }

    #[test]
    fn test_interpolation_at_end() {
        let lit: Literal = syn::parse_str(r#""hello @{x}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "hello "));
        assert!(matches!(&parts[1], StringPart::Expr(_)));
    }

    #[test]
    fn test_only_interpolation() {
        let lit: Literal = syn::parse_str(r#""@{value}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
    }

    #[test]
    fn test_consecutive_interpolations() {
        let lit: Literal = syn::parse_str(r#""@{a}@{b}@{c}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 3);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
        assert!(matches!(&parts[1], StringPart::Expr(_)));
        assert!(matches!(&parts[2], StringPart::Expr(_)));
    }

    #[test]
    fn test_complex_expression() {
        let lit: Literal = syn::parse_str(r#""Result: @{x.iter().map(|i| i * 2).sum::<i32>()}""#).unwrap();
        let result = parse_string_interpolation(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "Result: "));
        assert!(matches!(&parts[1], StringPart::Expr(_)));
    }
}
