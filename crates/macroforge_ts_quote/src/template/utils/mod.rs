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

    let needs_space = !last_char.is_whitespace() && !no_space_before && !no_space_after;

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
