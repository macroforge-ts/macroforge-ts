//! TypeScript-enabled quote implementation.
//!
//! This is a fork of swc_ecma_quote_macros with TypeScript parsing support.
//! The key difference is using `Syntax::Typescript` instead of `Syntax::Es`.

use std::iter::once;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Block, ExprBlock};

use self::{
    ast::ToCode,
    ctxt::{prepare_vars, Ctx},
    input::QuoteInput,
    ret_type::parse_input_type,
};

mod ast;
mod builder;
mod ctxt;
mod input;
mod ret_type;

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
    let input = proc_macro2::TokenStream::from(input);
    let QuoteInput {
        src,
        as_token: _,
        output_type,
        vars,
    } = syn::parse2::<QuoteInput>(input).expect("failed to parse input to quote_ts!()");

    let ret_type =
        parse_input_type(&src.value(), &output_type).expect("failed to parse TypeScript input");

    let vars = vars.map(|v| v.1);

    let (stmts, vars) = if let Some(vars) = vars {
        prepare_vars(&ret_type, vars)
    } else {
        Default::default()
    };

    let cx = Ctx { vars };

    let expr_for_ast_creation = ret_type.to_code(&cx);

    syn::Expr::Block(ExprBlock {
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
    .to_token_stream()
    .into()
}
