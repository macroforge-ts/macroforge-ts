use crate::template::TagType;
use proc_macro2::TokenTree;

/// Tries to parse an ident block tag: `{|...|}`
///
/// # Pattern
/// Matches tokens enclosed in pipe characters: `{| identifier |}` -> `TagType::IdentBlock`
///
/// # Returns
/// `Some(TagType::IdentBlock)` if the pattern matches, `None` otherwise.
pub(crate) fn try_parse_ident_block(tokens: &[TokenTree]) -> Option<TagType> {
    if tokens.len() >= 2
        && let (Some(TokenTree::Punct(first)), Some(TokenTree::Punct(last))) =
            (tokens.first(), tokens.last())
        && first.as_char() == '|'
        && last.as_char() == '|'
    {
        return Some(TagType::IdentBlock);
    }
    None
}
