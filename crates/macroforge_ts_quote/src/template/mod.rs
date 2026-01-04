//! String-based template system for TypeScript code generation.
//!
//! Provides a template syntax with interpolation and control flow:
//! - `@{expr}` - Interpolate expressions
//! - `@@{` - Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)
//! - `"string @{expr}"` - String interpolation (auto-detected)
//! - `"'^template ${expr}^'"` - JS backtick template literal (outputs `` `template ${expr}` ``)
//! - `{#if cond}...{/if}` - Conditional blocks
//! - `{#if let pattern = expr}...{/if}` - Pattern matching if-let blocks
//! - `{:else}` - Else clause
//! - `{:else if cond}` - Else-if clause
//! - `{#match expr}{:case pattern}...{/match}` - Match blocks with case arms
//! - `{#for item in list}...{/for}` - Iteration
//! - `{%let name = expr}` - Local constants
//!
//! Note: A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com`).

mod control_flow;
mod interpolation;
mod parser;
mod spacing;
mod tag;

use proc_macro2::{Delimiter, TokenStream as TokenStream2, TokenTree};
use quote::quote;

use parser::parse_fragment;

/// Compile a template from a TokenStream (the macro input).
/// Handles position parsing (Top, Above, Within, Below, Bottom).
pub fn compile_template(input: TokenStream2) -> syn::Result<TokenStream2> {
    let parsed = parse_position(input)?;
    let position = parsed.position;

    // Parse the template body
    let (body, _) = parse_fragment(&mut parsed.body.into_iter().peekable(), None)?;

    // Generate the output code
    let insert_pos = position_to_tokens(position);

    // For Within position, wrap the body marker
    let output = if position == Some("Within") {
        quote! {
            {
                let mut __out = String::new();
                let mut __patches: Vec<macroforge_ts::ts_syn::abi::Patch> = Vec::new();
                __out.push_str("/* @macroforge:body */");
                #body
                macroforge_ts::ts_syn::TsStream::with_insert_pos_and_patches(__out, #insert_pos, __patches)
            }
        }
    } else {
        quote! {
            {
                let mut __out = String::new();
                let mut __patches: Vec<macroforge_ts::ts_syn::abi::Patch> = Vec::new();
                #body
                macroforge_ts::ts_syn::TsStream::with_insert_pos_and_patches(__out, #insert_pos, __patches)
            }
        }
    };

    Ok(output)
}

/// Result of parsing position keyword.
struct ParsedInput {
    position: Option<&'static str>,
    body: TokenStream2,
}

/// Parse position keyword from input if present.
fn parse_position(input: TokenStream2) -> syn::Result<ParsedInput> {
    let mut iter = input.clone().into_iter().peekable();

    // Check if first token is a position keyword
    if let Some(TokenTree::Ident(ident)) = iter.peek() {
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

            let remaining: TokenStream2 = iter.collect();
            let mut remaining_iter = remaining.into_iter();

            if let Some(TokenTree::Group(group)) = remaining_iter.next()
                && group.delimiter() == Delimiter::Brace
            {
                return Ok(ParsedInput {
                    position: pos,
                    body: group.stream(),
                });
            }

            return Err(syn::Error::new_spanned(
                input,
                "expected `{` after position keyword (e.g., `ts_template!(Within { ... })`)",
            ));
        }
    }

    Ok(ParsedInput {
        position: None,
        body: input,
    })
}

/// Convert position keyword to InsertPos tokens.
fn position_to_tokens(position: Option<&str>) -> TokenStream2 {
    match position {
        Some("Top") => quote! { macroforge_ts::ts_syn::InsertPos::Top },
        Some("Above") => quote! { macroforge_ts::ts_syn::InsertPos::Above },
        Some("Within") => quote! { macroforge_ts::ts_syn::InsertPos::Within },
        Some("Bottom") => quote! { macroforge_ts::ts_syn::InsertPos::Bottom },
        _ => quote! { macroforge_ts::ts_syn::InsertPos::Below },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_interpolation() {
        // Using TokenStream directly to avoid quote! parsing issues with our syntax
        let input: TokenStream2 = "const x = @{value};".parse().unwrap();
        let result = compile_template(input);
        assert!(result.is_ok(), "Failed to compile: {:?}", result.err());
    }

    #[test]
    fn test_with_position() {
        // This test uses standard Rust syntax which works with quote!
        use quote::quote;
        let input = quote! {
            Within {
                debug() { return "test"; }
            }
        };
        let result = compile_template(input);
        assert!(result.is_ok(), "Failed to compile: {:?}", result.err());
    }
}
