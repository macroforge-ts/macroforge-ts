use crate::template::{StringPart, template_error};
use quote::ToTokens;

/// Parses a backtick template literal encoded as a Rust string literal.
pub fn parse_backtick_template(lit: &proc_macro2::Literal) -> syn::Result<Option<Vec<StringPart>>> {
    let raw = lit.to_string();
    let span = lit.span();

    let content = if raw.starts_with("\"'^") && raw.ends_with("^'\"") && raw.len() >= 6 {
        Some(raw[3..raw.len() - 3].to_string())
    } else if raw.starts_with("r\"'^") && raw.ends_with("^'\"") {
        Some(raw[4..raw.len() - 3].to_string())
    } else if raw.starts_with("r#\"'^") && raw.ends_with("^'\"#") {
        Some(raw[5..raw.len() - 4].to_string())
    } else {
        None
    };

    let Some(content) = content else {
        return Ok(None);
    };

    if content.contains("{#") || content.contains("{/") || content.contains("{:") {
        return Err(template_error(
            span,
            "Template control flow tags cannot be used inside backtick template literals",
            Some(&format!(
                "\"'^{}...^'\"",
                content.chars().take(40).collect::<String>()
            )),
        ));
    }

    let mut parts = Vec::new();
    let mut chars = content.chars().peekable();
    let mut current = String::new();
    let mut char_pos = 0usize;

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
                        Ok(expr) => parts.push(StringPart::Expr(expr.to_token_stream())),
                        Err(parse_err) => {
                            return Err(template_error(
                                span,
                                &format!(
                                    "Invalid Rust expression in template literal interpolation: {}",
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

    Ok(Some(parts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Literal;

    #[test]
    fn test_not_backtick_template() {
        let lit = Literal::string("hello world");
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_basic_backtick_template() {
        let lit: Literal = syn::parse_str(r#""'^hello world^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "hello world"));
    }

    #[test]
    fn test_backtick_with_interpolation() {
        let lit: Literal = syn::parse_str(r#""'^hello @{name}^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "hello "));
        assert!(matches!(&parts[1], StringPart::Expr(_)));
    }

    #[test]
    fn test_backtick_with_multiple_interpolations() {
        let lit: Literal = syn::parse_str(r#""'^@{greeting} @{name}!^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 4);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
        assert!(matches!(&parts[1], StringPart::Text(s) if s == " "));
        assert!(matches!(&parts[2], StringPart::Expr(_)));
        assert!(matches!(&parts[3], StringPart::Text(s) if s == "!"));
    }

    #[test]
    fn test_backtick_escape_at_sign() {
        let lit: Literal = syn::parse_str(r#""'^email@@example.com^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "email@example.com"));
    }

    #[test]
    fn test_backtick_at_without_brace() {
        let lit: Literal = syn::parse_str(r#""'^email@domain.com^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "email@domain.com"));
    }

    #[test]
    fn test_backtick_nested_braces() {
        let lit: Literal = syn::parse_str(r#""'^@{obj.get(key)}^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
    }

    #[test]
    fn test_backtick_unclosed_interpolation() {
        let lit: Literal = syn::parse_str(r#""'^@{name^'""#).unwrap();
        let result = parse_backtick_template(&lit);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unclosed"));
    }

    #[test]
    fn test_backtick_invalid_expression() {
        let lit: Literal = syn::parse_str(r#""'^@{+++}^'""#).unwrap();
        let result = parse_backtick_template(&lit);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid Rust expression"));
    }

    #[test]
    fn test_backtick_control_flow_rejected() {
        let lit: Literal = syn::parse_str(r#""'^{#if x}yes{/if}^'""#).unwrap();
        let result = parse_backtick_template(&lit);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("control flow tags cannot be used"));
    }

    #[test]
    fn test_backtick_raw_string() {
        let lit: Literal = syn::parse_str(r#"r"'^hello @{name}^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_backtick_raw_hash_string() {
        let lit: Literal = syn::parse_str(r##"r#"'^hello @{name}^'"#"##).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_backtick_preserves_dollar_signs() {
        // Dollar signs should be preserved (they're not special in backtick templates)
        let lit: Literal = syn::parse_str(r#""'^price: $@{amount}^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "price: $"));
        assert!(matches!(&parts[1], StringPart::Expr(_)));
    }

    #[test]
    fn test_backtick_empty_template() {
        let lit: Literal = syn::parse_str(r#""'^'^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        // Empty template results in empty text part
        assert!(parts.len() <= 1);
    }

    #[test]
    fn test_backtick_only_interpolation() {
        let lit: Literal = syn::parse_str(r#""'^@{value}^'""#).unwrap();
        let result = parse_backtick_template(&lit).unwrap();
        assert!(result.is_some());
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Expr(_)));
    }
}
