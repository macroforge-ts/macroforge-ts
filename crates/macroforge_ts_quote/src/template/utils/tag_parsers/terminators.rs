use crate::template::TagType;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;

/// Tries to parse an else tag: `{:else}` or `{:else if cond}`
///
/// # Pattern
/// - `{:else}` -> `TagType::Else`
/// - `{:else if cond}` -> `TagType::ElseIf(cond)`
///
/// # Returns
/// `Some(TagType::Else)` or `Some(TagType::ElseIf(_))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_else(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == ':'
        && i == "else"
    {
        if tokens.len() >= 4
            && let Some(TokenTree::Ident(if_kw)) = tokens.get(2)
            && if_kw == "if"
        {
            let cond: TokenStream2 =
                tokens.iter().skip(3).map(|t| t.to_token_stream()).collect();
            return Some(TagType::ElseIf(cond));
        }
        return Some(TagType::Else);
    }
    None
}

/// Tries to parse a case tag: `{:case pattern}`
///
/// # Pattern
/// `{:case pattern}` -> `TagType::Case(pattern)`
///
/// # Returns
/// `Some(TagType::Case(_))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_case(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == ':'
        && i == "case"
    {
        let pattern: TokenStream2 =
            tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
        return Some(TagType::Case(pattern));
    }
    None
}

/// Tries to parse an end-if tag: `{/if}`
///
/// # Pattern
/// `{/if}` -> `TagType::EndIf`
///
/// # Returns
/// `Some(TagType::EndIf)` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_end_if(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '/'
        && i == "if"
    {
        return Some(TagType::EndIf);
    }
    None
}

/// Tries to parse an end-for tag: `{/for}`
///
/// # Pattern
/// `{/for}` -> `TagType::EndFor`
///
/// # Returns
/// `Some(TagType::EndFor)` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_end_for(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '/'
        && i == "for"
    {
        return Some(TagType::EndFor);
    }
    None
}

/// Tries to parse an end-while tag: `{/while}`
///
/// # Pattern
/// `{/while}` -> `TagType::EndWhile`
///
/// # Returns
/// `Some(TagType::EndWhile)` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_end_while(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '/'
        && i == "while"
    {
        return Some(TagType::EndWhile);
    }
    None
}

/// Tries to parse an end-match tag: `{/match}`
///
/// # Pattern
/// `{/match}` -> `TagType::EndMatch`
///
/// # Returns
/// `Some(TagType::EndMatch)` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_end_match(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() < 2 {
        return None;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '/'
        && i == "match"
    {
        return Some(TagType::EndMatch);
    }
    None
}

/// Tries to parse any control flow branch tag: else or case.
///
/// # Returns
/// The result of trying each parser in sequence, or `None` if none match.
pub(crate) fn try_parse_control_branch(tokens: &[TokenTree]) -> Option<TagType> {
    try_parse_else(tokens).or_else(|| try_parse_case(tokens))
}

/// Tries to parse any control flow end tag: end-if, end-for, end-while, or end-match.
///
/// # Returns
/// The result of trying each parser in sequence, or `None` if none match.
pub(crate) fn try_parse_control_end(tokens: &[TokenTree]) -> Option<TagType> {
    try_parse_end_if(tokens)
        .or_else(|| try_parse_end_for(tokens))
        .or_else(|| try_parse_end_while(tokens))
        .or_else(|| try_parse_end_match(tokens))
}
