use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use super::compile_expr_segments;
use crate::template::{template_error, ControlNode};

/// Compiles control nodes in expression context into Rust expressions.
pub fn compile_control_expr(node: &ControlNode, span: Span) -> syn::Result<TokenStream2> {
    match node {
        ControlNode::If {
            cond,
            then_branch,
            else_branch,
        } => {
            let then_expr = compile_expr_segments(then_branch)?;
            let else_expr = if let Some(branch) = else_branch {
                compile_expr_segments(branch)?
            } else {
                return Err(template_error(
                    span,
                    "Expression-level {#if} requires an {:else} branch",
                    None,
                ));
            };
            Ok(quote! {
                if #cond {
                    #then_expr
                } else {
                    #else_expr
                }
            })
        }
        ControlNode::IfLet {
            pattern,
            expr,
            then_branch,
            else_branch,
        } => {
            let then_expr = compile_expr_segments(then_branch)?;
            let else_expr = if let Some(branch) = else_branch {
                compile_expr_segments(branch)?
            } else {
                return Err(template_error(
                    span,
                    "Expression-level {#if let} requires an {:else} branch",
                    None,
                ));
            };
            Ok(quote! {
                if let #pattern = #expr {
                    #then_expr
                } else {
                    #else_expr
                }
            })
        }
        ControlNode::Match { expr, cases } => {
            let mut arms = TokenStream2::new();
            for case in cases {
                let pattern = &case.pattern;
                let body_expr = compile_expr_segments(&case.body)?;
                arms.extend(quote! {
                    #pattern => #body_expr,
                });
            }
            Ok(quote! {
                match #expr {
                    #arms
                }
            })
        }
        ControlNode::For { .. } | ControlNode::While { .. } | ControlNode::WhileLet { .. } => {
            Err(template_error(
                span,
                "Loop constructs are not allowed in expression context",
                None,
            ))
        }
    }
}
