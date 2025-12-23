use crate::template::utils::{extract_string_literal, tokens_to_spaced_string};
use crate::template::TagType;
use proc_macro2::TokenTree;

/// Tries to parse a doc comment tag: `{>>..<<}`
///
/// # Pattern
/// Matches tokens enclosed in double angle brackets: `{>> content <<}` -> `TagType::DocComment(content)`
/// The content can be a string literal or raw tokens that will be joined with spaces.
///
/// # Returns
/// `Some(TagType::DocComment(String))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_doc_comment(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() >= 5
        && let (Some(TokenTree::Punct(p1)), Some(TokenTree::Punct(p2))) =
            (tokens.first(), tokens.get(1))
        && p1.as_char() == '>'
        && p2.as_char() == '>'
        && let (Some(TokenTree::Punct(p3)), Some(TokenTree::Punct(p4))) =
            (tokens.get(tokens.len() - 2), tokens.last())
        && p3.as_char() == '<'
        && p4.as_char() == '<'
    {
        if let Some(TokenTree::Literal(lit)) = tokens.get(2) {
            let content = extract_string_literal(lit);
            return Some(TagType::DocComment(content));
        }
        let content = tokens_to_spaced_string(&tokens[2..tokens.len() - 2]);
        return Some(TagType::DocComment(content));
    }
    None
}

/// Tries to parse a block comment tag: `{>..<}`
///
/// # Pattern
/// Matches tokens enclosed in single angle brackets: `{> content <}` -> `TagType::BlockComment(content)`
/// The content can be a string literal or raw tokens that will be joined with spaces.
///
/// # Returns
/// `Some(TagType::BlockComment(String))` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_block_comment(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() >= 3
        && let (Some(TokenTree::Punct(first)), Some(TokenTree::Punct(last))) =
            (tokens.first(), tokens.last())
        && first.as_char() == '>'
        && last.as_char() == '<'
    {
        if let Some(TokenTree::Literal(lit)) = tokens.get(1) {
            let content = extract_string_literal(lit);
            return Some(TagType::BlockComment(content));
        }
        let content = tokens_to_spaced_string(&tokens[1..tokens.len() - 1]);
        return Some(TagType::BlockComment(content));
    }
    None
}
