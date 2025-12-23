use crate::template::{ControlNode, IdGen, Terminator, template_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::iter::Peekable;

use super::parse_segments;

/// Parses a `{#while ...}` block.
pub fn parse_while_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    cond: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (body, terminator) = parse_segments(iter, Some(&[Terminator::EndWhile]), ids, false)?;
    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            span,
            "Unclosed {#while} block: Missing {/while}",
            Some("{#while condition}...{/while}"),
        ));
    }
    Ok(ControlNode::While { cond, body })
}
