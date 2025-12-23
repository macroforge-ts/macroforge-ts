use crate::template::TagType;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;

/// Tries to parse an if/if-let tag: `{#if condition}` or `{#if let pattern = expr}`
///
/// # Pattern
/// - `{#if cond}` -> `TagType::If(cond)`
/// - `{#if let pat = expr}` -> `TagType::IfLet(pat, expr)`
///
/// # Returns
/// `Some(TagType::If(_))` or `Some(TagType::IfLet(_, _))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_if(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '#'
        && i == "if"
    {
        if let Some(TokenTree::Ident(let_kw)) = tokens.get(2)
            && let_kw == "let"
        {
            let mut pattern = TokenStream2::new();
            let mut expr = TokenStream2::new();
            let mut seen_eq = false;

            for t in tokens.iter().skip(3) {
                if let TokenTree::Punct(eq) = t
                    && eq.as_char() == '='
                    && !seen_eq
                {
                    seen_eq = true;
                    continue;
                }
                if !seen_eq {
                    t.to_tokens(&mut pattern);
                } else {
                    t.to_tokens(&mut expr);
                }
            }
            return Some(TagType::IfLet(pattern, expr));
        }

        let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
        return Some(TagType::If(cond));
    }
    None
}

/// Tries to parse a match tag: `{#match expr}`
///
/// # Pattern
/// `{#match expr}` -> `TagType::Match(expr)`
///
/// # Returns
/// `Some(TagType::Match(_))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_match(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '#'
        && i == "match"
    {
        let expr: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
        return Some(TagType::Match(expr));
    }
    None
}

/// Tries to parse a while/while-let tag: `{#while condition}` or `{#while let pattern = expr}`
///
/// # Pattern
/// - `{#while cond}` -> `TagType::While(cond)`
/// - `{#while let pat = expr}` -> `TagType::WhileLet(pat, expr)`
///
/// # Returns
/// `Some(TagType::While(_))` or `Some(TagType::WhileLet(_, _))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_while(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '#'
        && i == "while"
    {
        if let Some(TokenTree::Ident(let_kw)) = tokens.get(2)
            && let_kw == "let"
        {
            let mut pattern = TokenStream2::new();
            let mut expr = TokenStream2::new();
            let mut seen_eq = false;

            for t in tokens.iter().skip(3) {
                if let TokenTree::Punct(eq) = t
                    && eq.as_char() == '='
                    && !seen_eq
                {
                    seen_eq = true;
                    continue;
                }
                if !seen_eq {
                    t.to_tokens(&mut pattern);
                } else {
                    t.to_tokens(&mut expr);
                }
            }
            return Some(TagType::WhileLet(pattern, expr));
        }

        let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
        return Some(TagType::While(cond));
    }
    None
}

/// Tries to parse a for tag: `{#for item in list}`
///
/// # Pattern
/// `{#for item in list}` -> `TagType::For(item, list)`
///
/// # Returns
/// `Some(TagType::For(_, _))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_for(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '#'
        && i == "for"
    {
        let mut item = TokenStream2::new();
        let mut list = TokenStream2::new();
        let mut seen_in = false;

        for t in tokens.iter().skip(2) {
            if let TokenTree::Ident(ident) = t
                && ident == "in"
                && !seen_in
            {
                seen_in = true;
                continue;
            }
            if !seen_in {
                t.to_tokens(&mut item);
            } else {
                t.to_tokens(&mut list);
            }
        }

        return Some(TagType::For(item, list));
    }
    None
}

/// Tries to parse any control flow start tag: if, match, while, or for.
///
/// # Returns
/// The result of trying each parser in sequence, or `None` if none match.
pub(crate) fn try_parse_control_start(tokens: &[TokenTree]) -> Option<TagType> {
    try_parse_if(tokens)
        .or_else(|| try_parse_match(tokens))
        .or_else(|| try_parse_while(tokens))
        .or_else(|| try_parse_for(tokens))
}
