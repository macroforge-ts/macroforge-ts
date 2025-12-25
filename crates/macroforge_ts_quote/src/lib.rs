//! TypeScript code generation macros for macroforge.
//!
//! This crate provides procedural macros for generating TypeScript code from Rust.
//! It offers two primary approaches:
//!
//! - [`ts_quote!`] - A thin wrapper around SWC's `quote!` macro with enhanced
//!   interpolation syntax for compile-time validated TypeScript generation.
//!
//! - [`ts_template!`] - A Rust-style template syntax with control flow (`{#if}`,
//!   `{#for}`, `{#match}`) and expression interpolation (`@{expr}`).
//!
//! # Architecture
//!
//! The quote implementation uses TypeScript parsing (`Syntax::Typescript`) instead
//! of JavaScript, enabling native support for type annotations and TypeScript syntax.
//!
//! # Insert Positions
//!
//! `ts_template!` supports an optional position keyword to control where generated
//! code is inserted:
//!
//! ```ignore
//! // Insert inside the class body
//! ts_template!(Within { ... })
//!
//! // Insert at the top of the file (for imports)
//! ts_template!(Top { ... })
//!
//! // Default: insert after the target (Below)
//! ts_template! { ... }
//! ```
//!
//! Available positions: `Top`, `Above`, `Within`, `Below`, `Bottom`

use std::iter::once;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, ExprBlock};

use self::{
    ast::ToCode,
    ctxt::{prepare_vars, Ctx},
    input::QuoteInput,
    ret_type::parse_input_type,
    template::parse_template,
};

mod ast;
mod builder;
#[allow(dead_code)]
mod compiler;
mod ctxt;
mod input;
mod ret_type;
#[allow(dead_code)]
mod template;
#[cfg(test)]
mod test;

/// Parse and generate code for a TypeScript quote.
///
/// # Example
///
/// ```ignore
/// use macroforge_ts_quote::ts_quote;
///
/// let ast = ts_quote!("function foo(x: number): string { return x.toString(); }" as ModuleItem);
/// ```
#[proc_macro]
pub fn ts_quote(input: TokenStream) -> TokenStream {
    match ts_quote_impl(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn ts_quote_impl(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let QuoteInput {
        src,
        as_token: _,
        output_type,
        vars,
    } = syn::parse2::<QuoteInput>(input)?;

    let ret_type = parse_input_type(&src.value(), &output_type).map_err(|err| {
        syn::Error::new_spanned(&src, format!("failed to parse TypeScript: {err}"))
    })?;

    let vars = vars.map(|v| v.1);

    let (stmts, vars) = if let Some(vars) = vars {
        prepare_vars(&ret_type, vars)?
    } else {
        Default::default()
    };

    let cx = Ctx { vars };

    let expr_for_ast_creation = ret_type.to_code(&cx);

    Ok(syn::Expr::Block(ExprBlock {
        attrs: Default::default(),
        label: Default::default(),
        block: Block {
            brace_token: Default::default(),
            stmts: stmts
                .into_iter()
                .chain(once(syn::Stmt::Expr(expr_for_ast_creation, None)))
                .collect(),
        },
    })
    .to_token_stream())
}

/// Generate TypeScript code with Rust-style control flow and interpolation.
///
/// Returns a `TsStream` that can be used as macro output.
///
/// # Syntax
///
/// ```ignore
/// // Default position (Below - after the target)
/// ts_template! {
///     const x = @{expr};
///     {#for item in items}
///         console.log(@{item});
///     {/for}
/// }
///
/// // With explicit position
/// ts_template!(Within {
///     // Generated methods go inside the class body
///     debug() { return "..."; }
/// })
///
/// ts_template!(Top {
///     // Imports go at the top of the file
///     import { foo } from "./runtime";
/// })
/// ```
///
/// # Positions
///
/// - `Top` - Insert at the top of the file (for imports)
/// - `Above` - Insert before the target declaration
/// - `Within` - Insert inside the target's body (class members, etc.)
/// - `Below` - Insert after the target declaration (default)
/// - `Bottom` - Insert at the bottom of the file
#[proc_macro]
pub fn ts_template(input: TokenStream) -> TokenStream {
    match ts_template_impl(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Parse position keyword from input if present.
/// Returns (position_variant, remaining_tokens).
fn parse_position(
    input: proc_macro2::TokenStream,
) -> syn::Result<(Option<&'static str>, proc_macro2::TokenStream)> {
    let mut iter = input.clone().into_iter().peekable();

    // Check if first token is a position keyword
    if let Some(proc_macro2::TokenTree::Ident(ident)) = iter.peek() {
        let pos = match ident.to_string().as_str() {
            "Top" => Some("Top"),
            "Above" => Some("Above"),
            "Within" => Some("Within"),
            "Below" => Some("Below"),
            "Bottom" => Some("Bottom"),
            _ => None,
        };

        if pos.is_some() {
            // Consume the position ident
            iter.next();

            // The rest should be the body in braces
            let remaining: proc_macro2::TokenStream = iter.collect();

            // Check if remaining starts with a brace group
            let mut remaining_iter = remaining.into_iter();
            if let Some(proc_macro2::TokenTree::Group(group)) = remaining_iter.next()
                && group.delimiter() == proc_macro2::Delimiter::Brace
            {
                // Return the position and the contents of the brace group
                return Ok((pos, group.stream()));
            }

            // Position keyword without braces - error
            return Err(syn::Error::new_spanned(
                input,
                "expected `{` after position keyword (e.g., `ts_template!(Within { ... })`)",
            ));
        }
    }

    // No position keyword, use entire input
    Ok((None, input))
}

fn ts_template_impl(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let (position, body) = parse_position(input)?;

    // Generate the InsertPos variant
    let insert_pos = match position {
        Some("Top") => quote! { macroforge_ts::ts_syn::InsertPos::Top },
        Some("Above") => quote! { macroforge_ts::ts_syn::InsertPos::Above },
        Some("Within") => quote! { macroforge_ts::ts_syn::InsertPos::Within },
        Some("Below") => quote! { macroforge_ts::ts_syn::InsertPos::Below },
        Some("Bottom") => quote! { macroforge_ts::ts_syn::InsertPos::Bottom },
        _ => quote! { macroforge_ts::ts_syn::InsertPos::Below },
    };

    // For Within position, we need to parse as class members, not statements
    if position == Some("Within") {
        // Wrap body in a dummy class so it parses as valid TypeScript
        // The template content contains class member syntax (static methods, constructors, etc.)
        let wrapped_body = quote! { class __MF_DUMMY__ { #body } };

        // Parse the wrapped template to handle placeholders and control flow
        let template_code = parse_template(wrapped_body)?;

        // Extract just the class body from the output and add the body marker
        Ok(quote! {
            {
                let (__stmts, mut __patches, __comments, __injected_streams) = #template_code;

                // Build source from AST - this will be "class __MF_DUMMY__ { ... }"
                let __full_source = macroforge_ts::ts_syn::emit_module_items(&__stmts, &__comments);

                // Extract just the class body (everything between first { and last })
                let __body_source = if let Some(start) = __full_source.find('{') {
                    if let Some(end) = __full_source.rfind('}') {
                        &__full_source[start + 1..end]
                    } else {
                        &__full_source[start + 1..]
                    }
                } else {
                    __full_source.as_str()
                };

                let __source = format!("/* @macroforge:body */{}", __body_source.trim());

                // Create TsStream with position
                let mut __stream = macroforge_ts::ts_syn::TsStream::with_insert_pos(__source, #insert_pos);
                __stream.runtime_patches = __patches;

                // Merge any injected TsStreams (from {$typescript} directives)
                for __injected in __injected_streams {
                    __stream = __stream.merge(__injected);
                }

                __stream
            }
        })
    } else {
        // Parse the template body normally for non-class-body content
        let template_code = parse_template(body)?;

        // The template_code generates: (__stmts, __patches, __comments, __injected_streams)
        // We need to wrap this in TsStream construction and merge any injected streams
        Ok(quote! {
            {
                let (__stmts, mut __patches, __comments, __injected_streams) = #template_code;

                // Build source from AST
                let __source = macroforge_ts::ts_syn::emit_module_items(&__stmts, &__comments);

                // Create TsStream with position
                let mut __stream = macroforge_ts::ts_syn::TsStream::with_insert_pos(__source, #insert_pos);
                __stream.runtime_patches = __patches;

                // Merge any injected TsStreams (from {$typescript} directives)
                for __injected in __injected_streams {
                    __stream = __stream.merge(__injected);
                }

                __stream
            }
        })
    }
}
