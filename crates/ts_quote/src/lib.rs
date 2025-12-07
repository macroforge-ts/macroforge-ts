//! Thin wrappers around SWC's `quote!` machinery tailored for macroforge.
//!
//! Macro authors can depend on this crate to access the familiar `quote!`
//! macro (re-exported from `swc_core`) plus a couple of helpers for creating
//! identifiers with predictable hygiene. The goal is to decouple codegen
//! utilities from the heavier parsing utilities that live in `ts_syn`.

mod template;
#[cfg(test)]
mod test;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Spacing, Span, TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
use std::sync::atomic::{AtomicUsize, Ordering};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Type, parse_macro_input, parse_str};

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
            if let TokenTree::Ident(ref ident) = tokens[len - 2]
                && *ident == "as"
            {
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

    // Use swc_core::quote!
    let mut output = TokenStream2::new();

    // Start with swc_core::quote!(
    output.extend(quote_ident("swc_core"));
    output.extend(quote_punct("::"));
    output.extend(quote_ident("quote"));
    output.extend(quote_punct("!"));

    // Build the arguments group
    let mut args = TokenStream2::new();

    // Add the string literal
    let clean_lit = syn::LitStr::new(&final_str, Span::call_site());
    args.extend(clean_lit.to_token_stream());

    // Add "as Type" if specified
    if let Some(ty) = input.output_type {
        args.extend(quote_ident("as"));
        args.extend(ty.to_token_stream());
    }

    // Add bindings
    if !bindings.is_empty() {
        args.extend(quote_punct(","));
        for (i, binding) in bindings.iter().enumerate() {
            if i > 0 {
                args.extend(quote_punct(","));
            }
            args.extend(binding.clone());
        }
    }

    // Wrap in parentheses
    output.extend(TokenStream2::from(TokenTree::Group(
        proc_macro2::Group::new(Delimiter::Parenthesis, args),
    )));

    TokenStream::from(output)
}

fn quote_ident(s: &str) -> TokenStream2 {
    let ident = Ident::new(s, Span::call_site());
    TokenStream2::from(TokenTree::Ident(ident))
}

fn quote_punct(s: &str) -> TokenStream2 {
    use proc_macro2::Punct;
    s.chars()
        .map(|c| TokenTree::Punct(Punct::new(c, Spacing::Joint)))
        .collect()
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
                if let Some(TokenTree::Group(g)) = iter.peek()
                    && g.delimiter() == Delimiter::Parenthesis
                {
                    // Found $(...)
                    let inner = g.stream();
                    iter.next(); // consume the group

                    // Parse the content of $()
                    let (bind_name, binding_code, is_vec_expansion) = parse_interpolation(inner);

                    // Only push binding if there's actual code
                    if !binding_code.is_empty() {
                        bindings.push(binding_code);
                    }

                    // Output valid SWC interpolation syntax
                    // Use *$var_name for vec expansion, $var_name otherwise
                    if is_vec_expansion {
                        output.push_str(&format!("*${}", bind_name));
                    } else {
                        output.push_str(&format!("${}", bind_name));
                    }
                    continue;
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
/// 2. stmt_vec!(expr) -> Vec<Stmt> expansion (returns is_vec=true)
/// 3. stmt!(expr) -> Stmt coercion
/// 4. expr : Type -> Explicit typing
/// 5. expr -> Default (untyped, usually Expr)
///
/// Returns: (bind_name, binding_code, is_vec_expansion)
fn parse_interpolation(tokens: TokenStream2) -> (String, TokenStream2, bool) {
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
    if token_vec.len() >= 3
        && let TokenTree::Ident(ref wrapper) = token_vec[0]
        && let TokenTree::Punct(ref p) = token_vec[1]
        && p.as_char() == '!'
        && let TokenTree::Group(ref g) = token_vec[2]
    {
        let wrapper_name = wrapper.to_string();
        let args = g.stream();

        // Special case: stmt_vec!(...) for Vec<Stmt> - inline the statements
        if wrapper_name == "stmt_vec" {
            // The args should be a simple expression (the vec variable)
            let expr: Expr = syn::parse2(args).expect("Invalid expression in stmt_vec!()");

            // Generate: bind_ident = expr (without type annotation, inferred from usage)
            let mut binding = TokenStream2::new();
            binding.extend(bind_ident.to_token_stream());
            binding.extend(quote_punct("="));
            binding.extend(expr.to_token_stream());

            // Return with is_vec_expansion = true
            return (bind_name, binding, true);
        }

        // Special case: ident!(...) supports format! syntax directly
        if wrapper_name == "ident" {
            // Build the function arguments: (format!(args).into(), swc_core::common::DUMMY_SP)
            let mut fn_args = TokenStream2::new();

            // format!(args).into()
            fn_args.extend(quote_ident("format"));
            fn_args.extend(quote_punct("!"));
            fn_args.extend(TokenStream2::from(TokenTree::Group(
                proc_macro2::Group::new(Delimiter::Parenthesis, args.clone()),
            )));
            fn_args.extend(quote_punct("."));
            fn_args.extend(quote_ident("into"));
            fn_args.extend(TokenStream2::from(TokenTree::Group(
                proc_macro2::Group::new(Delimiter::Parenthesis, TokenStream2::new()),
            )));
            fn_args.extend(quote_punct(","));

            // swc_core::common::DUMMY_SP
            fn_args.extend(quote_ident("swc_core"));
            fn_args.extend(quote_punct("::"));
            fn_args.extend(quote_ident("common"));
            fn_args.extend(quote_punct("::"));
            fn_args.extend(quote_ident("DUMMY_SP"));

            // Generate: bind_ident = swc_core::ecma::ast::Ident::new(...)
            let mut binding = TokenStream2::new();
            binding.extend(bind_ident.to_token_stream());
            binding.extend(quote_punct("="));
            binding.extend(quote_ident("swc_core"));
            binding.extend(quote_punct("::"));
            binding.extend(quote_ident("ecma"));
            binding.extend(quote_punct("::"));
            binding.extend(quote_ident("ast"));
            binding.extend(quote_punct("::"));
            binding.extend(quote_ident("Ident"));
            binding.extend(quote_punct("::"));
            binding.extend(quote_ident("new_no_ctxt"));
            binding.extend(TokenStream2::from(TokenTree::Group(
                proc_macro2::Group::new(Delimiter::Parenthesis, fn_args),
            )));

            return (bind_name, binding, false);
        }

        // General case: stmt!(expr), expr!(expr), module_item!(expr)
        // Add type annotation to help swc_core::quote! understand the type
        let type_name = wrapper_name.to_case(Case::Pascal);
        let type_ident = Ident::new(&type_name, Span::call_site());

        // Generate: bind_ident: Type = args
        let mut binding = TokenStream2::new();
        binding.extend(bind_ident.to_token_stream());
        binding.extend(quote_punct(":"));
        binding.extend(type_ident.to_token_stream());
        binding.extend(quote_punct("="));
        binding.extend(args);

        return (bind_name, binding, false);
    }

    // Check for explicit type syntax: `expr : Type`
    let mut split_idx = None;
    for (i, tt) in token_vec.iter().enumerate() {
        if let TokenTree::Punct(p) = tt
            && p.as_char() == ':'
        {
            split_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = split_idx {
        let expr_tokens: TokenStream2 = token_vec[0..idx].iter().cloned().collect();
        let type_tokens: TokenStream2 = token_vec[idx + 1..].iter().cloned().collect();
        let type_str = type_tokens.to_string();

        // Check if it's Vec<Stmt> - special case for inlining
        if type_str.contains("Vec") && type_str.contains("Stmt") {
            // Parse the expression
            let expr: Expr =
                syn::parse2(expr_tokens).expect("Invalid expression in $(expr: Vec<Stmt>)");

            // Generate: bind_ident: Vec<Stmt> = expr
            let mut binding = TokenStream2::new();
            binding.extend(bind_ident.to_token_stream());
            binding.extend(quote_punct(":"));
            binding.extend(quote_ident("Vec"));
            binding.extend(quote_punct("<"));
            binding.extend(quote_ident("Stmt"));
            binding.extend(quote_punct(">"));
            binding.extend(quote_punct("="));
            binding.extend(expr.to_token_stream());

            // Return with is_vec_expansion = true
            return (bind_name, binding, true);
        }

        // Parse both expr and type (swc_core::quote! supports typed bindings)
        let ty: Type = parse_str(&type_str).expect("Invalid type in $(expr: Type)");
        let expr: Expr = syn::parse2(expr_tokens).expect("Invalid expression in $(expr: Type)");

        // Generate: bind_ident: Type = expr
        let mut binding = TokenStream2::new();
        binding.extend(bind_ident.to_token_stream());
        binding.extend(quote_punct(":"));
        binding.extend(ty.to_token_stream());
        binding.extend(quote_punct("="));
        binding.extend(expr.to_token_stream());

        return (bind_name, binding, false);
    }

    // Default: try to parse as expression
    let expr: Expr = syn::parse2(tokens.clone()).expect("Invalid expression in $()");

    // Check if this is a StmtVec constructor call: ts_syn::StmtVec(vec_expr) or StmtVec(vec_expr)
    if let Expr::Call(call) = &expr
        && let Expr::Path(path) = &*call.func
    {
        let path_str = path
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");

        if path_str.contains("StmtVec") && call.args.len() == 1 {
            // This is a StmtVec wrapper - extract the inner vec expression
            let inner_arg = &call.args[0];

            // Generate: bind_ident = inner_arg
            let mut binding = TokenStream2::new();
            binding.extend(bind_ident.to_token_stream());
            binding.extend(quote_punct("="));
            binding.extend(inner_arg.to_token_stream());

            return (bind_name, binding, true);
        }
    }

    // Generate: bind_ident = expr
    let mut binding = TokenStream2::new();
    binding.extend(bind_ident.to_token_stream());
    binding.extend(quote_punct("="));
    binding.extend(expr.to_token_stream());

    (bind_name, binding, false)
}

/// Template-style macro for TypeScript code generation.
///
/// # Syntax
///
/// - `@{expr}` - Interpolate expressions (converts to string)
/// - `@@{` - Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)
/// - `"'^template ${js}^'"` - JS backtick template literal (outputs `` `template ${js}` ``)
/// - `{#if cond}...{/if}` - Conditional blocks
/// - `{:else}` - Else clause
/// - `{:else if cond}` - Else-if clause
/// - `{#for item in list}...{/for}` - Iteration
/// - `{%let name = expr}` - Local constants
///
/// Note: A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com`).
///
/// # Examples
///
/// ```ignore
/// let fields = vec!["name", "age"];
/// let class_name = "User";
///
/// let stmt = ts_template! {
///     @{class_name}.prototype.toJSON = function() {
///         const result = {};
///         {#for field in fields}
///             result.@{field} = this.@{field};
///         {/for}
///         return result;
///     };
/// };
/// ```
///
/// For JS template literals with backticks, use `"'^...^'"`:
///
/// ```ignore
/// let tag = "div";
/// let stmt = ts_template! {
///     const html = "'^<@{tag}>${content}</@{tag}>^'";
/// };
/// // Generates: const html = `<div>${content}</div>`;
/// ```
///
/// The template is compiled to a string at runtime, then parsed with SWC
/// to produce a typed AST node.
#[proc_macro]
pub fn ts_template(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    // Parse the template to generate string-building code
    let string_builder = match template::parse_template(input) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };

    // Wrap in code that builds string and parses it
    let output = quote::quote! {
        {
            let __ts_code = #string_builder;
            macroforge_ts::ts_syn::TsStream::from_string(__ts_code)
        }
    };

    TokenStream::from(output)
}

#[proc_macro]
pub fn above(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    generate_scoped_template(input, "/* @macroforge:above */")
}

#[proc_macro]
pub fn below(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    generate_scoped_template(input, "/* @macroforge:below */")
}

#[proc_macro]
pub fn body(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    generate_scoped_template(input, "/* @macroforge:body */")
}

#[proc_macro]
pub fn signature(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    generate_scoped_template(input, "/* @macroforge:signature */")
}

fn generate_scoped_template(input: TokenStream2, marker: &str) -> TokenStream {
    let string_builder = match template::parse_template(input) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };

    // Inject the marker at the start of the string building block
    // parse_template returns: { let mut __out = String::new(); ... __out }
    // We want to inject __out.push_str(marker); right after String::new();

    // Since parse_template returns a block, we can't easily inject inside it without parsing it.
    // However, we can just prepend the marker to the result string.

    let output = quote::quote! {
        {
            let mut __ts_code = String::from(#marker);
            __ts_code.push_str(&#string_builder);
            macroforge_ts::ts_syn::TsStream::from_string(__ts_code)
        }
    };

    TokenStream::from(output)
}
