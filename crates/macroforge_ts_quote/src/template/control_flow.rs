//! Control flow parsing for templates (if/else, for, match).

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::iter::Peekable;

use super::parser::{Terminator, parse_fragment};

/// Parse an if/else-if/else chain starting from the condition.
pub fn parse_if_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    initial_cond: TokenStream2,
    open_span: proc_macro2::Span,
) -> syn::Result<TokenStream2> {
    // Parse the true block, stopping at {:else}, {:else if}, or {/if}
    let (true_block, terminator) = parse_fragment(
        iter,
        Some(&[
            Terminator::Else,
            Terminator::ElseIf(TokenStream2::new()),
            Terminator::EndIf,
        ]),
    )?;

    match terminator {
        Some(Terminator::EndIf) => {
            // Simple if without else
            Ok(quote! {
                if #initial_cond {
                    #true_block
                }
            })
        }
        Some(Terminator::Else) => {
            // if with else - parse else block until {/if}
            let (else_block, terminator) = parse_fragment(iter, Some(&[Terminator::EndIf]))?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(syn::Error::new(
                    open_span,
                    "Unclosed {:else} block: Missing {/if}",
                ));
            }
            Ok(quote! {
                if #initial_cond {
                    #true_block
                } else {
                    #else_block
                }
            })
        }
        Some(Terminator::ElseIf(else_if_cond)) => {
            // if with else if - recursively parse the else-if chain
            let else_if_chain = parse_if_chain(iter, else_if_cond, open_span)?;
            Ok(quote! {
                if #initial_cond {
                    #true_block
                } else {
                    #else_if_chain
                }
            })
        }
        None => Err(syn::Error::new(
            open_span,
            "Unclosed {#if} block: Missing {/if}",
        )),
        _ => unreachable!(),
    }
}

/// Parse an if-let/else chain starting from the pattern and expression.
pub fn parse_if_let_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    open_span: proc_macro2::Span,
) -> syn::Result<TokenStream2> {
    // Parse the true block, stopping at {:else} or {/if}
    let (true_block, terminator) =
        parse_fragment(iter, Some(&[Terminator::Else, Terminator::EndIf]))?;

    match terminator {
        Some(Terminator::EndIf) => {
            // Simple if let without else
            Ok(quote! {
                if let #pattern = #expr {
                    #true_block
                }
            })
        }
        Some(Terminator::Else) => {
            // if let with else - parse else block until {/if}
            let (else_block, terminator) = parse_fragment(iter, Some(&[Terminator::EndIf]))?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(syn::Error::new(
                    open_span,
                    "Unclosed {:else} block: Missing {/if}",
                ));
            }
            Ok(quote! {
                if let #pattern = #expr {
                    #true_block
                } else {
                    #else_block
                }
            })
        }
        None => Err(syn::Error::new(
            open_span,
            "Unclosed {#if let} block: Missing {/if}",
        )),
        _ => unreachable!(),
    }
}

/// Parse a while loop starting from the condition.
pub fn parse_while_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    condition: TokenStream2,
    open_span: proc_macro2::Span,
) -> syn::Result<TokenStream2> {
    // Parse the body until {/while}
    let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndWhile]))?;

    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(syn::Error::new(
            open_span,
            "Unclosed {#while} block: Missing {/while}",
        ));
    }

    Ok(quote! {
        while #condition {
            #body
        }
    })
}

/// Parse a while-let loop starting from the pattern and expression.
pub fn parse_while_let_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    open_span: proc_macro2::Span,
) -> syn::Result<TokenStream2> {
    // Parse the body until {/while}
    let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndWhile]))?;

    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(syn::Error::new(
            open_span,
            "Unclosed {#while let} block: Missing {/while}",
        ));
    }

    Ok(quote! {
        while let #pattern = #expr {
            #body
        }
    })
}

/// Parse match arms starting after `{#match expr}`.
/// Format: `{#match expr}{:case pattern1}body1{:case pattern2}body2{/match}`
pub fn parse_match_arms(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    match_expr: TokenStream2,
    open_span: proc_macro2::Span,
) -> syn::Result<TokenStream2> {
    let mut arms = TokenStream2::new();
    let mut current_pattern: Option<TokenStream2> = None;

    loop {
        // Parse until we hit {:case} or {/match}
        let (body, terminator) = parse_fragment(
            iter,
            Some(&[Terminator::Case(TokenStream2::new()), Terminator::EndMatch]),
        )?;

        match terminator {
            Some(Terminator::Case(pattern)) => {
                // If we have a previous pattern, emit its arm with the body we just parsed
                if let Some(prev_pattern) = current_pattern.take() {
                    arms.extend(quote! {
                        #prev_pattern => {
                            #body
                        }
                    });
                }
                // Store this pattern for the next iteration
                current_pattern = Some(pattern);
            }
            Some(Terminator::EndMatch) => {
                // Emit the final arm if we have one
                if let Some(prev_pattern) = current_pattern.take() {
                    arms.extend(quote! {
                        #prev_pattern => {
                            #body
                        }
                    });
                }
                break;
            }
            None => {
                return Err(syn::Error::new(
                    open_span,
                    "Unclosed {#match} block: Missing {/match}",
                ));
            }
            _ => unreachable!(),
        }
    }

    Ok(quote! {
        match #match_expr {
            #arms
        }
    })
}
