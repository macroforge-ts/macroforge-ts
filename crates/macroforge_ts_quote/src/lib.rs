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

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Block, ExprBlock};

use self::{
    ctxt::{Ctx, prepare_vars},
    input::QuoteInput,
};

#[cfg(feature = "oxc")]
use self::oxc_ret_type::parse_input_type;
#[cfg(all(feature = "swc", not(feature = "oxc")))]
use self::swc_ret_type::parse_input_type;

#[cfg(all(feature = "swc", not(feature = "oxc")))]
mod swc_ast;
#[cfg(all(feature = "swc", not(feature = "oxc")))]
mod swc_builder;
#[cfg(all(feature = "swc", not(feature = "oxc")))]
mod swc_ret_type;

#[cfg(feature = "oxc")]
mod oxc_ret_type;

#[cfg(feature = "compiler")]
mod compiler;
mod ctxt;
mod input;
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
    #[cfg(feature = "oxc")]
    {
        match ts_quote_impl(input.into()) {
            Ok(tokens) => tokens.into(),
            Err(err) => err.to_compile_error().into(),
        }
    }
    #[cfg(all(not(feature = "oxc"), feature = "swc"))]
    {
        match ts_quote_impl(input.into()) {
            Ok(tokens) => tokens.into(),
            Err(err) => err.to_compile_error().into(),
        }
    }
    #[cfg(all(not(feature = "swc"), not(feature = "oxc")))]
    {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "Either 'swc' or 'oxc' feature must be enabled for macroforge_ts_quote",
        )
        .to_compile_error()
        .into()
    }
}

pub(crate) trait ToCode {
    fn to_code(&self, cx: &Ctx) -> syn::Expr;
}

#[cfg(any(feature = "swc", feature = "oxc"))]
fn ts_quote_impl(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    use std::iter::once;

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
    #[cfg(feature = "compiler")]
    {
        match compiler::compile_template_tokens(input.into()) {
            Ok(tokens) => tokens.into(),
            Err(err) => err.to_compile_error().into(),
        }
    }

    #[cfg(not(feature = "compiler"))]
    {
        match template::compile_template(input.into()) {
            Ok(tokens) => tokens.into(),
            Err(err) => err.to_compile_error().into(),
        }
    }
}
