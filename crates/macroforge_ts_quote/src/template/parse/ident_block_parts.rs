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

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::{Delimiter, Group, TokenStream};
    use quote::quote;

    fn make_ident_block(tokens: TokenStream) -> Group {
        Group::new(Delimiter::Brace, quote! { | #tokens | })
    }

    #[test]
    fn test_static_only() {
        let g = make_ident_block(quote! { getUserName });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], IdentPart::Static(s) if s == "getUserName"));
    }

    #[test]
    fn test_single_interpolation() {
        let g = make_ident_block(quote! { get@{field} });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], IdentPart::Static(s) if s == "get"));
        assert!(matches!(&parts[1], IdentPart::Interpolation { .. }));
    }

    #[test]
    fn test_multiple_interpolations() {
        let g = make_ident_block(quote! { @{prefix}@{name}@{suffix} });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 3);
        assert!(matches!(&parts[0], IdentPart::Interpolation { .. }));
        assert!(matches!(&parts[1], IdentPart::Interpolation { .. }));
        assert!(matches!(&parts[2], IdentPart::Interpolation { .. }));
    }

    #[test]
    fn test_mixed_static_and_interpolation() {
        let g = make_ident_block(quote! { get@{name}Value });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 3);
        assert!(matches!(&parts[0], IdentPart::Static(s) if s == "get"));
        assert!(matches!(&parts[1], IdentPart::Interpolation { .. }));
        assert!(matches!(&parts[2], IdentPart::Static(s) if s == "Value"));
    }

    #[test]
    fn test_at_without_brace() {
        let g = make_ident_block(quote! { user@domain });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], IdentPart::Static(s) if s == "user@domain"));
    }

    #[test]
    fn test_empty_block() {
        let g = make_ident_block(quote! {});
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 0);
    }

    #[test]
    fn test_only_interpolation() {
        let g = make_ident_block(quote! { @{ident} });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], IdentPart::Interpolation { .. }));
    }

    #[test]
    fn test_with_punctuation() {
        let g = make_ident_block(quote! { _get@{name}_value });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 3);
        assert!(matches!(&parts[0], IdentPart::Static(s) if s == "_get"));
        assert!(matches!(&parts[1], IdentPart::Interpolation { .. }));
        assert!(matches!(&parts[2], IdentPart::Static(s) if s == "_value"));
    }

    #[test]
    fn test_with_numbers() {
        let g = make_ident_block(quote! { item@{index}v2 });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 3);
        assert!(matches!(&parts[0], IdentPart::Static(s) if s == "item"));
        assert!(matches!(&parts[1], IdentPart::Interpolation { .. }));
        assert!(matches!(&parts[2], IdentPart::Static(s) if s == "v2"));
    }

    #[test]
    fn test_interpolation_at_start() {
        let g = make_ident_block(quote! { @{prefix}Name });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], IdentPart::Interpolation { .. }));
        assert!(matches!(&parts[1], IdentPart::Static(s) if s == "Name"));
    }

    #[test]
    fn test_interpolation_at_end() {
        let g = make_ident_block(quote! { get@{suffix} });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 2);
        assert!(matches!(&parts[0], IdentPart::Static(s) if s == "get"));
        assert!(matches!(&parts[1], IdentPart::Interpolation { .. }));
    }

    #[test]
    fn test_complex_static_content() {
        let g = make_ident_block(quote! { __private_get_user_name__ });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], IdentPart::Static(s) if s == "__private_get_user_name__"));
    }

    #[test]
    fn test_alternating_static_and_interpolation() {
        let g = make_ident_block(quote! { a@{b}c@{d}e });
        let parts = parse_ident_block_parts(&g).unwrap();
        assert_eq!(parts.len(), 5);
        assert!(matches!(&parts[0], IdentPart::Static(s) if s == "a"));
        assert!(matches!(&parts[1], IdentPart::Interpolation { .. }));
        assert!(matches!(&parts[2], IdentPart::Static(s) if s == "c"));
        assert!(matches!(&parts[3], IdentPart::Interpolation { .. }));
        assert!(matches!(&parts[4], IdentPart::Static(s) if s == "e"));
    }
}
