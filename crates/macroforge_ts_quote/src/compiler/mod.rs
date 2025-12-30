//! Compiler infrastructure for the template language.
//!
//! This module provides a clean compiler architecture:
//! - Lexer: Tokenizes input into a token stream (with normalization)
//! - Parser: Parses tokens directly into IR with inline placeholder classification
//! - Codegen: Generates Rust TokenStream output from IR

mod codegen;
mod ir;
mod lexer;
mod parser;
mod syntax;
#[cfg(test)]
mod tests;

use codegen::{Codegen, CodegenConfig};
use parser::Parser;
use proc_macro2::TokenStream;
use quote::quote;

/// Compiles a template from a TokenStream (the macro input).
/// Handles position parsing (Top, Above, Within, Below, Bottom).
pub fn compile_template_tokens(input: TokenStream) -> syn::Result<TokenStream> {
    let (position, body) = parse_position(input)?;

    let template_str = body.to_string();

    #[cfg(debug_assertions)]
    if std::env::var("MF_DEBUG_TEMPLATE").is_ok() {
        eprintln!(
            "[MF_DEBUG] Template string ({} chars): {:?}",
            template_str.len(),
            template_str
        );
    }

    // For Within position, wrap in dummy class
    let template_str = if position == Some("Within") {
        format!("class __MF_DUMMY__ {{ {} }}", template_str)
    } else {
        template_str
    };

    compile_template(&template_str, position)
}

/// Parse position keyword from input if present.
fn parse_position(input: TokenStream) -> syn::Result<(Option<&'static str>, TokenStream)> {
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
            iter.next(); // Consume the position ident

            let remaining: TokenStream = iter.collect();
            let mut remaining_iter = remaining.into_iter();

            if let Some(proc_macro2::TokenTree::Group(group)) = remaining_iter.next()
                && group.delimiter() == proc_macro2::Delimiter::Brace
            {
                return Ok((pos, group.stream()));
            }

            return Err(syn::Error::new_spanned(
                input,
                "expected `{` after position keyword (e.g., `ts_template!(Within { ... })`)",
            ));
        }
    }

    Ok((None, input))
}

/// Compiles a template string into Rust code that produces a TsStream.
pub fn compile_template(template: &str, position: Option<&str>) -> syn::Result<TokenStream> {
    if template.trim().is_empty() {
        let insert_pos = position_to_tokens(position);
        return Ok(quote! {
            macroforge_ts::ts_syn::TsStream::with_insert_pos(String::new(), #insert_pos)
        });
    }

    // Parse directly to IR with inline placeholder classification
    let ir = Parser::new(template).parse();

    // Generate code that builds Vec<ModuleItem>
    let config = CodegenConfig::default();
    let stmts_code = Codegen::with_config(config).generate(&ir);

    // Wrap in TsStream construction
    let insert_pos = position_to_tokens(position);
    let is_within = position == Some("Within");

    if is_within {
        Ok(quote! {
            {
                let __stmts: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = #stmts_code;
                let __comments = macroforge_ts::swc_core::common::comments::SingleThreadedComments::default();
                let __full_source = macroforge_ts::ts_syn::emit_module_items(&__stmts, &__comments);

                // Extract body from __MF_DUMMY__ wrapper
                let __body_source = {
                    let marker = "class __MF_DUMMY__";
                    if let Some(pos) = __full_source.find(marker) {
                        let after = &__full_source[pos + marker.len()..];
                        let after_trimmed = after.trim_start();
                        if after_trimmed.starts_with('{') {
                            let brace_offset = after.len() - after_trimmed.len();
                            let after_brace = &after[brace_offset + 1..];
                            if let Some(end) = after_brace.rfind('}') {
                                after_brace[..end].trim().to_string()
                            } else {
                                after_brace.trim().to_string()
                            }
                        } else {
                            after.trim().to_string()
                        }
                    } else {
                        __full_source.clone()
                    }
                };

                let __source = format!("/* @macroforge:body */{}", __body_source);
                macroforge_ts::ts_syn::TsStream::with_insert_pos(__source, #insert_pos)
            }
        })
    } else {
        Ok(quote! {
            {
                let __stmts: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = #stmts_code;
                let __comments = macroforge_ts::swc_core::common::comments::SingleThreadedComments::default();
                let __source = macroforge_ts::ts_syn::emit_module_items(&__stmts, &__comments);
                macroforge_ts::ts_syn::TsStream::with_insert_pos(__source, #insert_pos)
            }
        })
    }
}

fn position_to_tokens(position: Option<&str>) -> TokenStream {
    match position {
        Some("Top") => quote! { macroforge_ts::ts_syn::InsertPos::Top },
        Some("Above") => quote! { macroforge_ts::ts_syn::InsertPos::Above },
        Some("Within") => quote! { macroforge_ts::ts_syn::InsertPos::Within },
        Some("Bottom") => quote! { macroforge_ts::ts_syn::InsertPos::Bottom },
        _ => quote! { macroforge_ts::ts_syn::InsertPos::Below },
    }
}
