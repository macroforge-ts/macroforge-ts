use crate::template::IdGen;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use super::parse_segments;
use super::super::compile::compile_stmt_segments;

/// Parses a template token stream into Rust that builds TypeScript AST output.
pub fn parse_template(input: TokenStream2) -> syn::Result<TokenStream2> {
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut input.into_iter().peekable(), None, &mut ids, false)?;
    let stmts_ident = proc_macro2::Ident::new("__stmts", Span::call_site());
    let comments_ident = proc_macro2::Ident::new("__comments", Span::call_site());
    let pending_ident = proc_macro2::Ident::new("__pending_comments", Span::call_site());
    let pos_ident = proc_macro2::Ident::new("__mf_pos_counter", Span::call_site());

    let stmts_builder = compile_stmt_segments(
        &segments,
        &stmts_ident,
        &comments_ident,
        &pending_ident,
        &pos_ident,
    )?;

    Ok(quote! {
        {
            let mut __stmts: Vec<swc_core::ecma::ast::ModuleItem> = Vec::new();
            let mut __patches: Vec<macroforge_ts::ts_syn::abi::Patch> = Vec::new();
            let __comments = swc_core::common::comments::SingleThreadedComments::default();
            let mut __pending_comments: Vec<swc_core::common::comments::Comment> = Vec::new();
            let mut __mf_pos_counter: u32 = 1;
            #stmts_builder
            (__stmts, __patches, __comments)
        }
    })
}
