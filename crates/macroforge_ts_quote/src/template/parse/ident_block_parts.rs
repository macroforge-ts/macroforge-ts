use crate::template::IdentPart;
use proc_macro2::{Delimiter, Group, TokenTree};

use super::super::utils::group_to_string;

/// Parses the contents of an ident block into static and interpolated parts.
pub fn parse_ident_block_parts(g: &Group) -> syn::Result<Vec<IdentPart>> {
    let mut tokens: Vec<TokenTree> = g.stream().into_iter().collect();
    if tokens.len() >= 2 {
        tokens.remove(0);
        tokens.pop();
    }

    let mut parts = Vec::new();
    let mut current = String::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Punct(p) if p.as_char() == '@' => {
                if let Some(TokenTree::Group(g)) = iter.peek()
                    && g.delimiter() == Delimiter::Brace
                {
                    if !current.is_empty() {
                        parts.push(IdentPart::Static(std::mem::take(&mut current)));
                    }
                    let g = match iter.next() {
                        Some(TokenTree::Group(group)) => group,
                        _ => continue,
                    };
                    parts.push(IdentPart::Interpolation {
                        expr: g.stream(),
                    });
                } else {
                    current.push('@');
                }
            }
            TokenTree::Group(g) => {
                current.push_str(&group_to_string(&g));
            }
            TokenTree::Ident(ident) => current.push_str(&ident.to_string()),
            TokenTree::Punct(p) => current.push(p.as_char()),
            TokenTree::Literal(lit) => current.push_str(&lit.to_string()),
        }
    }

    if !current.is_empty() {
        parts.push(IdentPart::Static(current));
    }

    Ok(parts)
}
