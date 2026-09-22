//! TypeScript code generation macros for macroforge.
//!
//! This crate provides procedural macros for generating TypeScript code from Rust.
//! It offers two primary approaches:
//!
//! - [`ts_quote!`] - Compile-time validated TypeScript generation with `$var`
//!   interpolation, e.g. `ts_quote!("$name = $rhs" as Expr, name = "count", rhs: Expr = rhs)`,
//!   parsed into the caller's `arena`.
//!
//! - [`ts_template!`] - A Rust-style template syntax with control flow (`{#if}`,
//!   `{#for}`, `{#match}`, ...) and expression interpolation (`@{expr}`).
//!
//! # Architecture
//!
//! The template source string is parsed as TypeScript at macro-expansion time,
//! enabling native support for type annotations and TypeScript syntax. Parsing
//! is backed by OXC with the default `oxc` feature; the SWC backend is
//! available behind the opt-in `swc` feature.
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

mod ctxt;
mod input;
mod template;

/// Parse and generate code for a TypeScript quote.
///
/// The node is parsed into an oxc arena and lives as long as it does. The arena
/// is the `arena` binding in scope at the call, an owned `Allocator` or an
/// `&Allocator` parameter; pass another one as the first argument instead.
///
/// # Example
///
/// ```ignore
/// use macroforge_ts_quote::ts_quote;
///
/// let arena = Allocator::default();
/// let ast = ts_quote!("function foo(x: number): string { return x.toString(); }" as ModuleItem);
/// let other = ts_quote!(&other_arena, "x + 1" as Expr);
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
        #[cfg(feature = "oxc")]
        allocator,
        src,
        output_type,
        vars,
    } = syn::parse2::<QuoteInput>(input)?;

    let ret_type = parse_input_type(&src.value(), &output_type).map_err(|err| {
        syn::Error::new_spanned(&src, format!("failed to parse TypeScript: {err}"))
    })?;

    let vars = vars.map(|v| v.1);

    let (var_stmts, vars) = if let Some(vars) = vars {
        prepare_vars(&ret_type, vars)?
    } else {
        Default::default()
    };
    // The generated parse calls read the caller's arena through this binding.
    #[cfg(feature = "oxc")]
    let allocator_binding: Vec<syn::Stmt> = vec![syn::parse_quote! {
        let __mf_quote_allocator = macroforge_ts::ts_syn::QuoteArena::quote_arena(&#allocator);
    }];
    #[cfg(not(feature = "oxc"))]
    let allocator_binding: Vec<syn::Stmt> = Vec::new();
    let stmts: Vec<syn::Stmt> = allocator_binding.into_iter().chain(var_stmts).collect();

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
///
/// # Template Tags
///
/// - `@{expr}` - Interpolate an expression (`@@{` escapes a literal `@{`)
/// - `{#if cond}...{:else if cond}...{:else}...{/if}` - Conditionals
/// - `{#if let pattern = expr}...{/if}` - Pattern-matching if-let
/// - `{#match expr}{:case pattern}...{/match}` - Match with case arms
/// - `{#for item in list}...{/for}` - Iteration
/// - `{#while cond}...{/while}` / `{#while let pattern = expr}...{/while}` - Loops
/// - `{%let name = expr}` or `{$let name = expr}` - Local constants
///   (`{$let mut ...}` for mutable bindings)
/// - `{$do expr}` - Execute a Rust expression for its side effects
/// - `{$typescript expr}` - Inject a `TsStream` into the output
/// - `{> comment <}` / `{>> comment <<}` - Line / block comments in the output
#[proc_macro]
pub fn ts_template(input: TokenStream) -> TokenStream {
    match template::compile_template(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
