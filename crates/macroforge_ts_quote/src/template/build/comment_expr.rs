use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use swc_core::common::comments::CommentKind;

use crate::template::CommentStyle;

/// Builds a SWC comment literal for the pending comment buffer.
pub fn build_comment_expr(style: &CommentStyle, text: &str) -> TokenStream2 {
    let mut content = text.trim().to_string();
    let kind = match style {
        CommentStyle::DocBlock => {
            if !content.starts_with('*') {
                content = format!("* {}", content.trim_start());
            }
            CommentKind::Block
        }
        CommentStyle::Block => {
            if !content.starts_with(' ') {
                content = format!(" {}", content.trim_start());
            }
            CommentKind::Block
        }
        CommentStyle::Line => {
            if !content.starts_with(' ') {
                content = format!(" {}", content.trim_start());
            }
            CommentKind::Line
        }
    };

    content = content.trim_end().to_string();
    if !matches!(kind, CommentKind::Line) && !content.ends_with(' ') {
        content.push(' ');
    }

    let kind_tokens = match kind {
        CommentKind::Line => quote!(swc_core::common::comments::CommentKind::Line),
        CommentKind::Block => quote!(swc_core::common::comments::CommentKind::Block),
    };
    let text_lit = syn::LitStr::new(&content, Span::call_site());
    quote! {
        swc_core::common::comments::Comment {
            kind: #kind_tokens,
            span: swc_core::common::DUMMY_SP,
            text: #text_lit.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docblock_comment_adds_star_prefix() {
        let result = build_comment_expr(&CommentStyle::DocBlock, "This is a doc comment");
        let result_str = result.to_string();

        // Should contain "* This is a doc comment " (with trailing space)
        assert!(result_str.contains("* This is a doc comment "));
        assert!(result_str.contains("CommentKind :: Block"));
    }

    #[test]
    fn test_docblock_comment_preserves_existing_star() {
        let result = build_comment_expr(&CommentStyle::DocBlock, "* Already has star");
        let result_str = result.to_string();

        // Should not add duplicate star
        assert!(result_str.contains("* Already has star "));
        assert!(!result_str.contains("* * Already"));
    }

    #[test]
    fn test_block_comment_adds_leading_space() {
        let result = build_comment_expr(&CommentStyle::Block, "Block comment");
        let result_str = result.to_string();

        // Should contain " Block comment " (leading and trailing space)
        assert!(result_str.contains(" Block comment "));
        assert!(result_str.contains("CommentKind :: Block"));
    }

    #[test]
    fn test_block_comment_preserves_existing_space() {
        let result = build_comment_expr(&CommentStyle::Block, " Already has space");
        let result_str = result.to_string();

        // Should not add duplicate space
        assert!(result_str.contains(" Already has space "));
    }

    #[test]
    fn test_line_comment_adds_leading_space() {
        let result = build_comment_expr(&CommentStyle::Line, "Line comment");
        let result_str = result.to_string();

        // Should contain " Line comment" (leading space, no trailing for Line)
        assert!(result_str.contains(" Line comment"));
        assert!(result_str.contains("CommentKind :: Line"));
    }

    #[test]
    fn test_line_comment_no_trailing_space() {
        let result = build_comment_expr(&CommentStyle::Line, "Line comment");
        let result_str = result.to_string();

        // Line comments should NOT have trailing space added
        // Should end with just the text, not " Line comment "
        assert!(result_str.contains(" Line comment\""));
    }

    #[test]
    fn test_line_comment_preserves_existing_space() {
        let result = build_comment_expr(&CommentStyle::Line, " Already has space");
        let result_str = result.to_string();

        // Should not add duplicate space
        assert!(result_str.contains(" Already has space"));
    }

    #[test]
    fn test_comment_trims_whitespace() {
        let result = build_comment_expr(&CommentStyle::Block, "  \n  Text with whitespace  \n  ");
        let result_str = result.to_string();

        // Should trim both leading and trailing whitespace
        assert!(result_str.contains(" Text with whitespace "));
    }

    #[test]
    fn test_docblock_with_whitespace() {
        let result = build_comment_expr(&CommentStyle::DocBlock, "\n  Multiline\n  docblock\n  ");
        let result_str = result.to_string();

        // Should trim and add star prefix
        assert!(result_str.contains("* Multiline"));
    }

    #[test]
    fn test_empty_comment() {
        let result = build_comment_expr(&CommentStyle::Block, "");
        let result_str = result.to_string();

        // Should handle empty strings gracefully
        assert!(result_str.contains("CommentKind :: Block"));
    }

    #[test]
    fn test_comment_with_special_characters() {
        let result = build_comment_expr(&CommentStyle::Line, "Comment with \"quotes\" and 'apostrophes'");
        let result_str = result.to_string();

        // Should preserve special characters (they'll be escaped in the string literal)
        assert!(result_str.contains("Comment with"));
    }

    #[test]
    fn test_block_comment_trailing_space_added() {
        let result = build_comment_expr(&CommentStyle::Block, "Text");
        let result_str = result.to_string();

        // Block comments should have trailing space
        assert!(result_str.contains(" Text "));
    }

    #[test]
    fn test_docblock_comment_trailing_space_added() {
        let result = build_comment_expr(&CommentStyle::DocBlock, "Text");
        let result_str = result.to_string();

        // DocBlock comments should have trailing space
        assert!(result_str.contains("* Text "));
    }
}
