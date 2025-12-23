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
