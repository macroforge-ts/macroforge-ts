mod object_prop_loop;
mod tag_parsers;

use crate::template::{IdGen, PlaceholderUse, Segment, TagType};
use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2, TokenTree};

use object_prop_loop::{find_for_loop_in_segments, validate_key_value_body};
use tag_parsers::{
    try_parse_block_comment, try_parse_control_branch, try_parse_control_end,
    try_parse_control_start, try_parse_doc_comment, try_parse_ident_block, try_parse_runtime,
};

/// Assigns a precedence rank for placeholder usage kinds.
pub(crate) fn use_rank(use_kind: &PlaceholderUse) -> usize {
    match use_kind {
        PlaceholderUse::Stmt => 5,
        PlaceholderUse::Type => 4,
        PlaceholderUse::Ident => 3,
        PlaceholderUse::IdentName => 2,
        PlaceholderUse::Expr => 1,
    }
}

/// Appends a template part, inserting spaces between tokens when needed.
pub(crate) fn append_part(out: &mut String, part: &str) {
    if out.is_empty() {
        out.push_str(part);
        return;
    }

    let last_char = out.chars().last().unwrap_or(' ');
    let first_char = part.chars().next().unwrap_or(' ');

    // Don't add space if:
    // - Last char is whitespace
    // - First char is opening bracket (no space before)
    // - Last char is opening bracket (no space after)
    // - First char is closing bracket, colon, semicolon, comma, or dot (no space before)
    // - Last char is a dot (no space after dots for member access)
    let no_space_before = matches!(
        first_char,
        '(' | '[' | ')' | ']' | '}' | ':' | ';' | ',' | '.'
    );
    let no_space_after = matches!(last_char, '(' | '[' | '.');

    let needs_space = !last_char.is_whitespace()
        && !first_char.is_whitespace()
        && !no_space_before
        && !no_space_after;

    if needs_space {
        out.push(' ');
    }
    out.push_str(part);
}

/// Converts a token stream into a TypeScript-like string.
pub(crate) fn tokens_to_ts_string(tokens: TokenStream2) -> String {
    let mut output = String::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Group(g) => {
                let inner = tokens_to_ts_string(g.stream());
                let (open, close) = match g.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::None => ("", ""),
                };
                output.push_str(open);
                output.push_str(&inner);
                output.push_str(close);
            }
            TokenTree::Ident(ident) => {
                output.push_str(&ident.to_string());
                // Only add space after ident if the next token needs it
                // Don't add space before: (, [, ., :, ;, ,
                let next_needs_space = match iter.peek() {
                    Some(TokenTree::Punct(p)) => {
                        !matches!(p.as_char(), '(' | '[' | '.' | ':' | ';' | ',' | ')' | ']')
                    }
                    Some(TokenTree::Group(g)) => {
                        // No space before groups
                        g.delimiter() == Delimiter::Brace
                    }
                    None => false,
                    _ => true,
                };
                if next_needs_space {
                    output.push(' ');
                }
            }
            TokenTree::Punct(p) => {
                output.push(p.as_char());
                // Add space after standalone punct, but not after certain punctuation
                if p.spacing() == proc_macro2::Spacing::Alone {
                    // @ is for interpolations - never add space after it
                    let no_space_after =
                        matches!(p.as_char(), '.' | '(' | '[' | ':' | ';' | ',' | '@');
                    if !no_space_after {
                        output.push(' ');
                    }
                }
            }
            TokenTree::Literal(lit) => output.push_str(&lit.to_string()),
        }
    }

    output
}

/// Classifies a brace-delimited group as a template tag or plain block.
pub(crate) fn analyze_tag(g: &Group) -> TagType {
    let tokens: Vec<TokenTree> = g.stream().into_iter().collect();

    try_parse_ident_block(&tokens)
        .or_else(|| try_parse_doc_comment(&tokens))
        .or_else(|| try_parse_block_comment(&tokens))
        .or_else(|| try_parse_control_start(&tokens))
        .or_else(|| try_parse_control_branch(&tokens))
        .or_else(|| try_parse_control_end(&tokens))
        .or_else(|| try_parse_runtime(&tokens))
        .unwrap_or(TagType::Block)
}

/// Renders a group token into a string, preserving delimiters.
pub(crate) fn group_to_string(g: &Group) -> String {
    let inner = tokens_to_ts_string(g.stream());
    let (open, close) = match g.delimiter() {
        Delimiter::Parenthesis => ("(", ")"),
        Delimiter::Brace => ("{", "}"),
        Delimiter::Bracket => ("[", "]"),
        Delimiter::None => ("", ""),
    };
    format!("{}{}{}", open, inner, close)
}

/// Joins tokens into a space-separated string.
pub(super) fn tokens_to_spaced_string(tokens: &[TokenTree]) -> String {
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(&token.to_string());
    }
    result
}

/// Extracts and unescapes the contents of a Rust string literal token.
pub(super) fn extract_string_literal(lit: &proc_macro2::Literal) -> String {
    let s = lit.to_string();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        let inner = &s[1..s.len() - 1];
        return unescape_string(inner);
    }
    if s.starts_with("r\"") && s.ends_with("\"") {
        return s[2..s.len() - 1].to_string();
    }
    if s.starts_with("r#\"")
        && let Some(idx) = s.rfind("\"") {
            return s[3..idx].to_string();
        }
    s
}

/// Performs a minimal unescape pass for Rust string literal escapes.
pub(crate) fn unescape_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            result.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => result.push('\n'),
            Some('t') => result.push('\t'),
            Some('r') => result.push('\r'),
            Some('0') => result.push('\0'),
            Some('"') => result.push('"'),
            Some('\\') => result.push('\\'),
            Some(other) => {
                result.push('\\');
                result.push(other);
            }
            None => result.push('\\'),
        }
    }
    result
}
/// Tries to extract an object property loop pattern from brace block inner segments.
///
/// Matches the pattern: `{ {#for (key, val) in items} @{key}: @{val}, {/for} }`
/// where the for loop contains key-value property assignments.
///
/// Returns `Some(ObjectPropLoop)` if the pattern is detected, `None` otherwise.
pub(crate) fn try_extract_object_prop_loop(
    inner_segments: &[Segment],
    ids: &mut IdGen,
) -> Option<Segment> {
    // Find the for loop control node
    let (pat, iter, body) = find_for_loop_in_segments(inner_segments)?;

    // Validate the body matches the key-value property pattern
    let (key_expr, value_expr) = validate_key_value_body(&body)?;

    let id = ids.next();
    Some(Segment::ObjectPropLoop {
        id,
        pat,
        iter,
        key_expr,
        value_expr,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;

    #[test]
    fn test_unescape_string_newline() {
        assert_eq!(unescape_string("hello\\nworld"), "hello\nworld");
    }

    #[test]
    fn test_unescape_string_tab() {
        assert_eq!(unescape_string("hello\\tworld"), "hello\tworld");
    }

    #[test]
    fn test_unescape_string_carriage_return() {
        assert_eq!(unescape_string("hello\\rworld"), "hello\rworld");
    }

    #[test]
    fn test_unescape_string_null() {
        assert_eq!(unescape_string("hello\\0world"), "hello\0world");
    }

    #[test]
    fn test_unescape_string_backslash() {
        assert_eq!(unescape_string("hello\\\\world"), "hello\\world");
    }

    #[test]
    fn test_unescape_string_quote() {
        assert_eq!(unescape_string("hello\\\"world"), "hello\"world");
    }

    #[test]
    fn test_unescape_string_unknown_escape() {
        // Unknown escapes preserve the backslash and the character
        assert_eq!(unescape_string("hello\\xworld"), "hello\\xworld");
    }

    #[test]
    fn test_unescape_string_trailing_backslash() {
        assert_eq!(unescape_string("hello\\"), "hello\\");
    }

    #[test]
    fn test_unescape_string_empty() {
        assert_eq!(unescape_string(""), "");
    }

    #[test]
    fn test_unescape_string_multiple_escapes() {
        assert_eq!(
            unescape_string("line1\\nline2\\ttab\\r\\0null\\\\backslash\\\"quote"),
            "line1\nline2\ttab\r\0null\\backslash\"quote"
        );
    }

    #[test]
    fn test_append_part_empty_buffer() {
        let mut out = String::new();
        append_part(&mut out, "hello");
        assert_eq!(out, "hello");
    }

    #[test]
    fn test_append_part_space_between_words() {
        let mut out = String::from("hello");
        append_part(&mut out, "world");
        assert_eq!(out, "hello world");
    }

    #[test]
    fn test_append_part_no_space_before_open_paren() {
        let mut out = String::from("func");
        append_part(&mut out, "(");
        assert_eq!(out, "func(");
    }

    #[test]
    fn test_append_part_no_space_before_open_bracket() {
        let mut out = String::from("arr");
        append_part(&mut out, "[");
        assert_eq!(out, "arr[");
    }

    #[test]
    fn test_append_part_no_space_after_open_paren() {
        let mut out = String::from("(");
        append_part(&mut out, "arg");
        assert_eq!(out, "(arg");
    }

    #[test]
    fn test_append_part_no_space_after_open_bracket() {
        let mut out = String::from("[");
        append_part(&mut out, "1");
        assert_eq!(out, "[1");
    }

    #[test]
    fn test_append_part_no_space_before_close_paren() {
        let mut out = String::from("arg");
        append_part(&mut out, ")");
        assert_eq!(out, "arg)");
    }

    #[test]
    fn test_append_part_no_space_before_close_bracket() {
        let mut out = String::from("1");
        append_part(&mut out, "]");
        assert_eq!(out, "1]");
    }

    #[test]
    fn test_append_part_no_space_before_close_brace() {
        let mut out = String::from("x");
        append_part(&mut out, "}");
        assert_eq!(out, "x}");
    }

    #[test]
    fn test_append_part_no_space_before_colon() {
        let mut out = String::from("key");
        append_part(&mut out, ":");
        assert_eq!(out, "key:");
    }

    #[test]
    fn test_append_part_no_space_before_semicolon() {
        let mut out = String::from("x");
        append_part(&mut out, ";");
        assert_eq!(out, "x;");
    }

    #[test]
    fn test_append_part_no_space_before_comma() {
        let mut out = String::from("a");
        append_part(&mut out, ",");
        assert_eq!(out, "a,");
    }

    #[test]
    fn test_append_part_no_space_before_dot() {
        let mut out = String::from("obj");
        append_part(&mut out, ".");
        assert_eq!(out, "obj.");
    }

    #[test]
    fn test_append_part_no_space_after_dot() {
        let mut out = String::from(".");
        append_part(&mut out, "prop");
        assert_eq!(out, ".prop");
    }

    #[test]
    fn test_append_part_whitespace_handling() {
        let mut out = String::from("hello ");
        append_part(&mut out, "world");
        assert_eq!(out, "hello world");
    }

    #[test]
    fn test_tokens_to_ts_string_single_ident() {
        let tokens: TokenStream2 = quote! { hello };
        assert_eq!(tokens_to_ts_string(tokens), "hello");
    }

    #[test]
    fn test_tokens_to_ts_string_function_call() {
        let tokens: TokenStream2 = quote! { func() };
        assert_eq!(tokens_to_ts_string(tokens), "func()");
    }

    #[test]
    fn test_tokens_to_ts_string_member_access() {
        let tokens: TokenStream2 = quote! { obj.prop };
        assert_eq!(tokens_to_ts_string(tokens), "obj.prop");
    }

    #[test]
    fn test_tokens_to_ts_string_object_literal() {
        let tokens: TokenStream2 = quote! { { key: value } };
        assert_eq!(tokens_to_ts_string(tokens), "{key:value}");
    }

    #[test]
    fn test_tokens_to_ts_string_array() {
        let tokens: TokenStream2 = quote! { [1, 2, 3] };
        assert_eq!(tokens_to_ts_string(tokens), "[1,2,3]");
    }

    #[test]
    fn test_tokens_to_ts_string_type_annotation() {
        let tokens: TokenStream2 = quote! { x: string };
        assert_eq!(tokens_to_ts_string(tokens), "x:string");
    }

    #[test]
    fn test_tokens_to_ts_string_binary_expr() {
        let tokens: TokenStream2 = quote! { a + b };
        assert_eq!(tokens_to_ts_string(tokens), "a + b");
    }

    #[test]
    fn test_tokens_to_ts_string_at_handling() {
        let tokens: TokenStream2 = quote! { @interpolation };
        assert_eq!(tokens_to_ts_string(tokens), "@interpolation");
    }

    #[test]
    fn test_tokens_to_ts_string_nested_groups() {
        let tokens: TokenStream2 = quote! { func(nested(arg)) };
        assert_eq!(tokens_to_ts_string(tokens), "func(nested(arg))");
    }

    #[test]
    fn test_tokens_to_ts_string_string_literal() {
        let tokens: TokenStream2 = quote! { "hello" };
        assert_eq!(tokens_to_ts_string(tokens), "\"hello\"");
    }

    #[test]
    fn test_tokens_to_ts_string_number_literal() {
        let tokens: TokenStream2 = quote! { 42 };
        assert_eq!(tokens_to_ts_string(tokens), "42");
    }

    #[test]
    fn test_tokens_to_ts_string_empty() {
        let tokens: TokenStream2 = TokenStream2::new();
        assert_eq!(tokens_to_ts_string(tokens), "");
    }

    #[test]
    fn test_extract_string_literal_regular_string() {
        let lit: proc_macro2::Literal = proc_macro2::Literal::string("hello world");
        assert_eq!(extract_string_literal(&lit), "hello world");
    }

    #[test]
    fn test_extract_string_literal_escaped_chars() {
        let lit: proc_macro2::Literal = proc_macro2::Literal::string("hello\\nworld");
        let result = extract_string_literal(&lit);
        // The Literal::string constructor doesn't escape the input, so we get the raw string
        assert_eq!(result, "hello\\nworld");
    }

    #[test]
    fn test_extract_string_literal_empty_string() {
        let lit: proc_macro2::Literal = proc_macro2::Literal::string("");
        assert_eq!(extract_string_literal(&lit), "");
    }

    #[test]
    fn test_extract_string_literal_raw_string() {
        // Create a raw string literal by parsing
        let tokens: TokenStream2 = "r\"hello world\"".parse().unwrap();
        let lit = tokens.into_iter().next().unwrap();
        if let TokenTree::Literal(l) = lit {
            assert_eq!(extract_string_literal(&l), "hello world");
        } else {
            panic!("Expected literal");
        }
    }

    #[test]
    fn test_extract_string_literal_raw_string_with_hash() {
        // Create a raw string literal with # delimiter by parsing
        let tokens: TokenStream2 = "r#\"hello \"world\"\"#".parse().unwrap();
        let lit = tokens.into_iter().next().unwrap();
        if let TokenTree::Literal(l) = lit {
            assert_eq!(extract_string_literal(&l), "hello \"world\"");
        } else {
            panic!("Expected literal");
        }
    }

    #[test]
    fn test_use_rank_expr() {
        assert_eq!(use_rank(&PlaceholderUse::Expr), 1);
    }

    #[test]
    fn test_use_rank_ident_name() {
        assert_eq!(use_rank(&PlaceholderUse::IdentName), 2);
    }

    #[test]
    fn test_use_rank_ident() {
        assert_eq!(use_rank(&PlaceholderUse::Ident), 3);
    }

    #[test]
    fn test_use_rank_type() {
        assert_eq!(use_rank(&PlaceholderUse::Type), 4);
    }

    #[test]
    fn test_use_rank_stmt() {
        assert_eq!(use_rank(&PlaceholderUse::Stmt), 5);
    }
}
