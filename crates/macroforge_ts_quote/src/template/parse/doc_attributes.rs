use proc_macro2::{Group, TokenTree};

use super::super::utils::extract_string_literal;

/// Extracts the string literal from a `#[doc = "..."]` attribute group.
pub fn parse_doc_attribute(g: &Group) -> Option<String> {
    let tokens: Vec<TokenTree> = g.stream().into_iter().collect();
    if tokens.len() < 3 {
        return None;
    }
    if let (TokenTree::Ident(ident), TokenTree::Punct(eq), TokenTree::Literal(lit)) =
        (&tokens[0], &tokens[1], &tokens[2])
        && ident == "doc" && eq.as_char() == '=' {
            return Some(extract_string_literal(lit));
        }
    None
}
