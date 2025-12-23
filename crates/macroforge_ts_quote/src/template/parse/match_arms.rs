use crate::template::{ControlNode, IdGen, MatchCase, Terminator, template_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::iter::Peekable;

use super::parse_segments;

/// Parses the arms for a `{#match ...}` block.
pub fn parse_match_arms(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    expr: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let mut cases = Vec::new();
    let mut pending_pattern: Option<TokenStream2> = None;

    loop {
        let (body, terminator) = parse_segments(
            iter,
            Some(&[Terminator::Case(TokenStream2::new()), Terminator::EndMatch]),
            ids,
            false,
        )?;

        match terminator {
            Some(Terminator::Case(pattern)) => {
                if let Some(prev) = pending_pattern.take() {
                    cases.push(MatchCase {
                        pattern: prev,
                        body,
                    });
                } else if !body.is_empty() {
                    return Err(template_error(
                        span,
                        "Unexpected content before {:case}",
                        None,
                    ));
                }
                pending_pattern = Some(pattern);
            }
            Some(Terminator::EndMatch) => {
                if let Some(prev) = pending_pattern.take() {
                    cases.push(MatchCase {
                        pattern: prev,
                        body,
                    });
                } else if !body.is_empty() {
                    return Err(template_error(
                        span,
                        "Unexpected content before {/match}",
                        None,
                    ));
                }
                break;
            }
            None => {
                return Err(template_error(
                    span,
                    "Unclosed {#match} block: Missing {/match}",
                    Some("{#match expr}{:case pattern}...{/match}"),
                ));
            }
            Some(other) => {
                return Err(template_error(
                    span,
                    &format!("Unexpected terminator in {{#match}}: {other:?}"),
                    None,
                ));
            }
        }
    }

    Ok(ControlNode::Match { expr, cases })
}
