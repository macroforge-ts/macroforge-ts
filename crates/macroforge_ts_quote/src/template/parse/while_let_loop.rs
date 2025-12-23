use crate::template::{ControlNode, IdGen, Terminator, template_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::iter::Peekable;

use super::parse_segments;

/// Parses a `{#while let ...}` block.
pub fn parse_while_let_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (body, terminator) = parse_segments(iter, Some(&[Terminator::EndWhile]), ids, false)?;
    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            span,
            "Unclosed {#while let} block: Missing {/while}",
            Some("{#while let pattern = expr}...{/while}"),
        ));
    }
    Ok(ControlNode::WhileLet {
        pattern,
        expr,
        body,
    })
}
