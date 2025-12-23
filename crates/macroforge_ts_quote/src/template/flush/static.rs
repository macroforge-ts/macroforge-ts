use crate::template::{Segment, tokens_to_ts_string};
use proc_macro2::TokenStream as TokenStream2;

/// Flushes buffered static tokens into a static segment.
pub fn flush_static(segments: &mut Vec<Segment>, static_tokens: &mut TokenStream2) {
    if static_tokens.is_empty() {
        return;
    }
    let tokens = std::mem::take(static_tokens);
    let s = tokens_to_ts_string(tokens);
    if !s.trim().is_empty() {
        segments.push(Segment::Static(s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_flush_static_empty_tokens() {
        let mut segments = Vec::new();
        let mut static_tokens = TokenStream2::new();

        flush_static(&mut segments, &mut static_tokens);

        assert_eq!(segments.len(), 0, "Empty tokens should not add segments");
        assert!(static_tokens.is_empty(), "Token stream should remain empty");
    }

    #[test]
    fn test_flush_static_whitespace_only() {
        let mut segments = Vec::new();
        // Create a token stream that only contains whitespace when converted
        let mut static_tokens: TokenStream2 = quote! {};

        flush_static(&mut segments, &mut static_tokens);

        assert_eq!(segments.len(), 0, "Whitespace-only tokens should not add segments");
        assert!(static_tokens.is_empty(), "Token stream should be cleared");
    }

    #[test]
    fn test_flush_static_normal_tokens() {
        let mut segments = Vec::new();
        let mut static_tokens: TokenStream2 = quote! { const x = 42; };

        flush_static(&mut segments, &mut static_tokens);

        assert_eq!(segments.len(), 1, "Should add one segment");
        assert!(static_tokens.is_empty(), "Token stream should be cleared");

        match &segments[0] {
            Segment::Static(s) => {
                assert!(s.contains("const"), "Should contain const keyword");
                assert!(s.contains("42"), "Should contain literal value");
            }
            _ => panic!("Expected Static segment"),
        }
    }

    #[test]
    fn test_flush_static_multiple_statements() {
        let mut segments = Vec::new();
        let mut static_tokens: TokenStream2 = quote! {
            const x = 1;
            const y = 2;
        };

        flush_static(&mut segments, &mut static_tokens);

        assert_eq!(segments.len(), 1, "Should combine into one static segment");
        assert!(static_tokens.is_empty(), "Token stream should be cleared");

        match &segments[0] {
            Segment::Static(s) => {
                assert!(s.contains("const x"), "Should contain first statement");
                assert!(s.contains("const y"), "Should contain second statement");
            }
            _ => panic!("Expected Static segment"),
        }
    }

    #[test]
    fn test_flush_static_clears_tokens() {
        let mut segments = Vec::new();
        let mut static_tokens: TokenStream2 = quote! { let foo = "bar"; };

        flush_static(&mut segments, &mut static_tokens);

        assert!(static_tokens.is_empty(), "First flush should clear tokens");

        // Try flushing again with empty tokens
        flush_static(&mut segments, &mut static_tokens);

        assert_eq!(segments.len(), 1, "Should still have only one segment");
    }

    #[test]
    fn test_flush_static_complex_tokens() {
        let mut segments = Vec::new();
        let mut static_tokens: TokenStream2 = quote! {
            interface User {
                name: string;
                age: number;
            }
        };

        flush_static(&mut segments, &mut static_tokens);

        assert_eq!(segments.len(), 1);
        match &segments[0] {
            Segment::Static(s) => {
                assert!(s.contains("interface"), "Should contain interface keyword");
                assert!(s.contains("User"), "Should contain type name");
            }
            _ => panic!("Expected Static segment"),
        }
    }

    #[test]
    fn test_flush_static_preserves_existing_segments() {
        let mut segments = vec![
            Segment::Static("existing".to_string()),
        ];
        let mut static_tokens: TokenStream2 = quote! { new_content };

        flush_static(&mut segments, &mut static_tokens);

        assert_eq!(segments.len(), 2, "Should preserve existing segments");

        match &segments[0] {
            Segment::Static(s) => assert_eq!(s, "existing"),
            _ => panic!("First segment should remain unchanged"),
        }

        match &segments[1] {
            Segment::Static(s) => assert!(s.contains("new_content")),
            _ => panic!("Expected Static segment"),
        }
    }
}
