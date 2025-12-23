use crate::template::{ControlNode, IdGen, Segment, Terminator, template_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::iter::Peekable;

use super::parse_segments;

/// Parses an `{#if ...}` chain into a control node.
pub fn parse_if_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    cond: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (then_branch, terminator) = parse_segments(
        iter,
        Some(&[
            Terminator::Else,
            Terminator::ElseIf(TokenStream2::new()),
            Terminator::EndIf,
        ]),
        ids,
        false,
    )?;

    match terminator {
        Some(Terminator::Else) => {
            let (else_branch, terminator) =
                parse_segments(iter, Some(&[Terminator::EndIf]), ids, false)?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(template_error(
                    span,
                    "Unclosed {#if} block: Missing {/if}",
                    Some("{#if condition}...{:else}...{/if}"),
                ));
            }
            Ok(ControlNode::If {
                cond,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::ElseIf(next_cond)) => {
            let else_branch = vec![Segment::Control {
                id: ids.next(),
                node: parse_if_chain(iter, next_cond, span, ids)?,
            }];
            Ok(ControlNode::If {
                cond,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::EndIf) => Ok(ControlNode::If {
            cond,
            then_branch,
            else_branch: None,
        }),
        _ => Err(template_error(
            span,
            "Unclosed {#if} block: Missing {/if}",
            Some("{#if condition}...{/if}"),
        )),
    }
}
