use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::compile_stmt_segments;
use crate::template::ControlNode;

/// Compiles statement-level control nodes into Rust control flow.
pub fn compile_stmt_control(
    node: &ControlNode,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    match node {
        ControlNode::If {
            cond,
            then_branch,
            else_branch,
        } => {
            let then_code = compile_stmt_segments(
                then_branch,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            )?;
            let else_code = if let Some(branch) = else_branch {
                compile_stmt_segments(branch, out_ident, comments_ident, pending_ident, pos_ident)?
            } else {
                TokenStream2::new()
            };
            Ok(quote! {
                if #cond {
                    #then_code
                } else {
                    #else_code
                }
            })
        }
        ControlNode::IfLet {
            pattern,
            expr,
            then_branch,
            else_branch,
        } => {
            let then_code = compile_stmt_segments(
                then_branch,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            )?;
            let else_code = if let Some(branch) = else_branch {
                compile_stmt_segments(branch, out_ident, comments_ident, pending_ident, pos_ident)?
            } else {
                TokenStream2::new()
            };
            Ok(quote! {
                if let #pattern = #expr {
                    #then_code
                } else {
                    #else_code
                }
            })
        }
        ControlNode::For { pat, iter, body } => {
            let body_code =
                compile_stmt_segments(body, out_ident, comments_ident, pending_ident, pos_ident)?;
            Ok(quote! {
                for #pat in #iter {
                    #body_code
                }
            })
        }
        ControlNode::While { cond, body } => {
            let body_code =
                compile_stmt_segments(body, out_ident, comments_ident, pending_ident, pos_ident)?;
            Ok(quote! {
                while #cond {
                    #body_code
                }
            })
        }
        ControlNode::WhileLet {
            pattern,
            expr,
            body,
        } => {
            let body_code =
                compile_stmt_segments(body, out_ident, comments_ident, pending_ident, pos_ident)?;
            Ok(quote! {
                while let #pattern = #expr {
                    #body_code
                }
            })
        }
        ControlNode::Match { expr, cases } => {
            let mut arms = TokenStream2::new();
            for case in cases {
                let body_code = compile_stmt_segments(
                    &case.body,
                    out_ident,
                    comments_ident,
                    pending_ident,
                    pos_ident,
                )?;
                let pattern = &case.pattern;
                arms.extend(quote! {
                    #pattern => { #body_code }
                });
            }
            Ok(quote! {
                match #expr {
                    #arms
                }
            })
        }
    }
}
