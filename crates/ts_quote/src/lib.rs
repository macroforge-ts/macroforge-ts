//! Thin wrappers around SWC's `quote!` machinery tailored for ts-macros.
//!
//! Macro authors can depend on this crate to access the familiar `quote!`
//! macro (re-exported from `swc_core`) plus a couple of helpers for creating
//! identifiers with predictable hygiene. The goal is to decouple codegen
//! utilities from the heavier parsing utilities that live in `ts_syn`.
use convert_case::Case;
use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Spacing, Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use std::sync::atomic::{AtomicUsize, Ordering};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Token, Type, parse_macro_input, parse_str};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

struct TsQuoteInput {
    // We store the raw tokens to be stringified later as the template
    template_tokens: TokenStream2,
    output_type: Option<Type>,
}

impl Parse for TsQuoteInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tokens = Vec::new();
        // Collect all tokens first
        while !input.is_empty() {
            tokens.push(input.parse::<TokenTree>()?);
        }

        // Heuristic: Scan backwards for `as Type`
        // We look for the pattern: ... `as` `Type` EOF
        // This handles cases like `as Stmt`, `as Expr` at the end of the macro.
        let len = tokens.len();
        let mut type_node = None;
        let mut cut_index = len;

        if len >= 2 {
            // Check second to last token for "as"
            if let TokenTree::Ident(ref ident) = tokens[len - 2] {
                if ident.to_string() == "as" {
                    // Check if the last token (or tokens) form a valid Type
                    let last_token = &tokens[len - 1];
                    let type_candidate: TokenStream2 = last_token.clone().into();

                    // We attempt to parse the last token as a Type.
                    if let Ok(ty) = parse_str::<Type>(&type_candidate.to_string()) {
                        type_node = Some(ty);
                        cut_index = len - 2;
                    }
                }
            }
        }

        let template_tokens: TokenStream2 = tokens.into_iter().take(cut_index).collect();

        Ok(TsQuoteInput {
            template_tokens,
            output_type: type_node,
        })
    }
}

#[proc_macro]
pub fn ts_quote(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TsQuoteInput);

    // Process the tokens to build the template string and extract bindings
    let (template_str, bindings) = process_tokens(input.template_tokens);

    // Smart Object Literal Handling
    // If the target is explicitly `Expr` and the string looks like `{ ... }`, wrap it.
    let mut final_str = template_str;
    let is_expr_target = if let Some(syn::Type::Path(ref type_path)) = input.output_type {
        type_path.path.is_ident("Expr")
    } else {
        false
    };

    let trimmed = final_str.trim();
    if is_expr_target && trimmed.starts_with('{') && trimmed.ends_with('}') {
        final_str = format!("({})", final_str);
    }

    let clean_lit = syn::LitStr::new(&final_str, Span::call_site());

    // Construct the target type syntax for swc_core::quote
    let quote_target = if let Some(ty) = input.output_type {
        quote! { #clean_lit as #ty }
    } else {
        quote! { #clean_lit }
    };

    // Generate the final macro call to swc_core::quote!
    // We expand our captured bindings into the `var: Type = val` format it expects.
    let expanded = quote! {
        swc_core::quote!(
            #quote_target,
            #(#bindings),*
        )
    };

    TokenStream::from(expanded)
}

/// Walks the token stream to reconstruct the JS string while extracting $(...) patterns.
fn process_tokens(tokens: TokenStream2) -> (String, Vec<TokenStream2>) {
    let mut output = String::new();
    let mut bindings = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(tt) = iter.next() {
        match tt {
            // Detect $(...) pattern
            TokenTree::Punct(ref p) if p.as_char() == '$' => {
                if let Some(TokenTree::Group(g)) = iter.peek() {
                    if g.delimiter() == Delimiter::Parenthesis {
                        // Found $(...)
                        let inner = g.stream();
                        iter.next(); // consume the group

                        // Parse the content of $()
                        let (bind_name, binding_code) = parse_interpolation(inner);
                        bindings.push(binding_code);

                        // Output valid SWC interpolation syntax: $var_name
                        output.push_str(&format!("${}", bind_name));
                        continue;
                    }
                }
                // Just a literal '$'
                output.push('$');
            }
            TokenTree::Group(g) => {
                // Recursively process groups (like `{ ... }` blocks in JS)
                let (inner_str, inner_bindings) = process_tokens(g.stream());
                bindings.extend(inner_bindings);

                let (open, close) = match g.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::None => ("", ""),
                };

                output.push_str(open);
                output.push_str(&inner_str);
                output.push_str(close);
            }
            TokenTree::Ident(ident) => {
                output.push_str(&ident.to_string());
                output.push(' '); // Add space to prevent accidental merging (e.g. `var x`)
            }
            TokenTree::Punct(p) => {
                output.push(p.as_char());
                // Avoid spacing for dots to keep `obj.prop` compact,
                // but generally spacing is safe in JS except within tokens.
                if p.spacing() == Spacing::Alone {
                    output.push(' ');
                }
            }
            TokenTree::Literal(lit) => {
                output.push_str(&lit.to_string());
            }
        }
    }
    (output, bindings)
}

/// Parses the content inside $(...).
/// Handles:
/// 1. ident!("fmt", args...) -> Ident coercion with formatting
/// 2. stmt!(expr) -> Stmt coercion
/// 3. expr : Type -> Explicit typing
/// 4. expr -> Default (untyped, usually Expr)
fn parse_interpolation(tokens: TokenStream2) -> (String, TokenStream2) {
    // Generate a unique binding name for SWC
    let bind_name = format!(
        "__ts_bind_{}_{}",
        COUNTER.fetch_add(1, Ordering::Relaxed),
        0
    );
    let bind_ident = Ident::new(&bind_name, Span::call_site());

    let token_vec: Vec<TokenTree> = tokens.clone().into_iter().collect();

    // Check for wrapper syntax: type!(...)
    // Pattern: Ident `name` + Punct `!` + Group `(...)`
    if token_vec.len() >= 3 {
        if let TokenTree::Ident(ref wrapper) = token_vec[0] {
            if let TokenTree::Punct(ref p) = token_vec[1] {
                if p.as_char() == '!' {
                    if let TokenTree::Group(ref g) = token_vec[2] {
                        let wrapper_name = wrapper.to_string();
                        let args = g.stream();

                        // Special case: ident!(...) supports format! syntax directly
                        if wrapper_name == "ident" {
                            return (
                                bind_name,
                                quote! {
                                    #bind_ident: swc_core::ecma::ast::Ident = swc_core::ecma::ast::Ident::new(
                                        format!(#args).into(),
                                        swc_core::common::DUMMY_SP
                                    )
                                },
                            );
                        }

                        // General case: stmt!(expr), expr!(expr), module_item!(expr)
                        // We convert wrapper name to PascalCase type:
                        // stmt -> Stmt, module_item -> ModuleItem
                        let type_name = to_pascal_case(&wrapper_name);
                        let type_ident = Ident::new(&type_name, Span::call_site());

                        return (
                            bind_name,
                            quote! {
                                #bind_ident: #type_ident = #args
                            },
                        );
                    }
                }
            }
        }
    }

    // Check for explicit type syntax: `expr : Type`
    let mut split_idx = None;
    for (i, tt) in token_vec.iter().enumerate() {
        if let TokenTree::Punct(ref p) = tt {
            if p.as_char() == ':' {
                split_idx = Some(i);
                break;
            }
        }
    }

    if let Some(idx) = split_idx {
        let expr_tokens: TokenStream2 = token_vec[0..idx].iter().cloned().collect();
        let type_tokens: TokenStream2 = token_vec[idx + 1..].iter().cloned().collect();

        let ty: Type = parse_str(&type_tokens.to_string()).expect("Invalid type in $(expr: Type)");
        let expr: Expr = syn::parse2(expr_tokens).expect("Invalid expression in $(expr: Type)");

        return (
            bind_name,
            quote! {
                #bind_ident: #ty = #expr
            },
        );
    }

    // Default: untyped expression
    let expr: Expr = syn::parse2(tokens).expect("Invalid expression in $()");
    (
        bind_name,
        quote! {
            #bind_ident = #expr
        },
    )
}

#[cfg(feature = "swc")]
pub use swc_core::{common, ecma::ast};

/// Create an identifier without syntax context, mirroring `syn::Ident::new`.
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! ident {
    ($name:expr) => {
        swc_core::ecma::ast::Ident::new_no_ctxt($name.into(), swc_core::common::DUMMY_SP)
    };
}

/// Create a unique private identifier using a fresh mark.
#[cfg(feature = "swc")]
#[macro_export]
macro_rules! private_ident {
    ($name:expr) => {{
        let mark = swc_core::common::Mark::fresh(swc_core::common::Mark::root());
        swc_core::ecma::ast::Ident::new(
            $name.into(),
            swc_core::common::DUMMY_SP,
            swc_core::common::SyntaxContext::empty().apply_mark(mark),
        )
    }};
}
