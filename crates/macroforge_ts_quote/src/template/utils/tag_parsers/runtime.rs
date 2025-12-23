use crate::template::TagType;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;

/// Tries to parse a let tag: `{$let statement}`
///
/// # Pattern
/// `{$let statement}` -> `TagType::Let(statement)`
///
/// # Returns
/// `Some(TagType::Let(_))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_let(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '$'
        && i == "let"
    {
        let rest: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
        return Some(TagType::Let(rest));
    }
    None
}

/// Tries to parse a do tag: `{$do statement}`
///
/// # Pattern
/// `{$do statement}` -> `TagType::Do(statement)`
///
/// # Returns
/// `Some(TagType::Do(_))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_do(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '$'
        && i == "do"
    {
        let rest: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
        return Some(TagType::Do(rest));
    }
    None
}

/// Tries to parse a typescript tag: `{$typescript code}`
///
/// # Pattern
/// `{$typescript code}` -> `TagType::Typescript(code)`
///
/// # Returns
/// `Some(TagType::Typescript(_))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_typescript(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '$'
        && i == "typescript"
    {
        let rest: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
        return Some(TagType::Typescript(rest));
    }
    None
}

/// Tries to parse any runtime tag: let, do, or typescript.
///
/// # Returns
/// The result of trying each parser in sequence, or `None` if none match.
pub(crate) fn try_parse_runtime(tokens: &[TokenTree]) -> Option<TagType> {
    try_parse_let(tokens)
        .or_else(|| try_parse_do(tokens))
        .or_else(|| try_parse_typescript(tokens))
}
