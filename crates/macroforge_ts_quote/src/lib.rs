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
//! Additionally, scoped template macros are provided for code injection:
//! - [`above!`] - Inject code above a definition
//! - [`below!`] - Inject code below a definition
//! - [`body!`] - Inject code into method/function bodies
//! - [`signature!`] - Inject code into function signatures
//!
//! # Architecture
//!
//! The crate is designed to decouple code generation utilities from the heavier
//! parsing utilities in `ts_syn`. Templates compile to string-building Rust code
//! at macro expansion time, then produce [`TsStream`] objects at runtime that can
//! be parsed by SWC into typed AST nodes.
//!
//! # Examples
//!
//! Using `ts_quote!` for simple interpolation:
//!
//! ```ignore
//! let name = quote_ident("MyClass");
//! let stmt = ts_quote!(class $name {} as Stmt, name = name);
//! ```
//!
//! Using `ts_template!` for complex code generation:
//!
//! ```ignore
//! let fields = vec!["name", "age"];
//! let stream = ts_template! {
//!     {#for field in &fields}
//!         this.@{field} = @{field};
//!     {/for}
//! };
//! ```
//!
//! [`TsStream`]: macroforge_ts::ts_syn::TsStream

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

/// Global counter for generating unique binding names in macro expansions.
///
/// Each call to [`parse_interpolation`] increments this counter to ensure
/// that generated variable names (e.g., `__ts_bind_0_0`, `__ts_bind_1_0`)
/// don't collide within a single compilation unit.
static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Parsed input for the [`ts_quote!`] macro.
///
/// This struct captures the token stream that will be converted to a TypeScript
/// template string, along with an optional output type annotation.
///
/// # Input Format
///
/// The macro accepts tokens followed by an optional `as Type` suffix:
/// - `ts_quote!(class Foo {})` - No type annotation (default)
/// - `ts_quote!(class Foo {} as Stmt)` - Explicit `Stmt` type
/// - `ts_quote!({ x: 1 } as Expr)` - Object literal as `Expr` (auto-wrapped)
///
/// # Type Detection Heuristic
///
/// The parser scans backwards from the end looking for an `as Type` pattern.
/// When the last two tokens are an `as` identifier followed by a valid type,
/// they are extracted as the output type annotation.
struct TsQuoteInput {
    /// The raw tokens to be converted to a TypeScript template string.
    /// These tokens are processed by [`process_tokens`] to extract
    /// interpolation bindings and build the template.
    template_tokens: TokenStream2,

    /// Optional output type for the generated AST node.
    /// When specified (e.g., `as Stmt`), this type is passed to SWC's `quote!`
    /// macro to produce a typed result. When `Some(Expr)` and the template
    /// looks like an object literal `{ ... }`, it's automatically wrapped
    /// in parentheses to avoid parsing ambiguity.
    output_type: Option<Type>,
}

/// Implements [`syn::Parse`] to extract the template tokens and optional type annotation.
///
/// # Parsing Strategy
///
/// 1. Collect all tokens from the input stream
/// 2. Scan backwards looking for `as Type` pattern at the end
/// 3. If found, extract the type and truncate the template tokens
///
/// # Example Parsing
///
/// ```text
/// Input: `class $name {} as Stmt`
/// Result: template_tokens = `class $name {}`, output_type = Some(Stmt)
///
/// Input: `{ x: $(value) }`
/// Result: template_tokens = `{ x: $(value) }`, output_type = None
/// ```
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

/// Generates TypeScript AST nodes using SWC's `quote!` macro with enhanced interpolation.
///
/// This macro provides a thin wrapper around `swc_core::quote!` with additional
/// conveniences for TypeScript code generation:
///
/// - **Automatic binding extraction**: `$(expr)` patterns are converted to SWC bindings
/// - **Type-aware interpolation**: `ident!()`, `stmt!()`, `stmt_vec!()` for typed codegen
/// - **Smart object literal handling**: `{ ... } as Expr` is auto-wrapped in parentheses
///
/// # Syntax
///
/// ```text
/// ts_quote!(template_tokens as Type, bindings...)
/// ts_quote!(template_tokens)
/// ```
///
/// ## Interpolation Patterns
///
/// | Pattern | Description |
/// |---------|-------------|
/// | `$(expr)` | Interpolate expression (inferred type) |
/// | `$(expr: Type)` | Interpolate with explicit type annotation |
/// | `$(ident!("name"))` | Create an `Ident` with formatted name |
/// | `$(stmt!(expr))` | Interpolate as `Stmt` |
/// | `$(stmt_vec!(vec))` | Inline a `Vec<Stmt>` |
///
/// # Examples
///
/// Simple class generation:
///
/// ```ignore
/// let name = quote_ident("MyClass");
/// let stmt = ts_quote!(class $name {} as Stmt, name = name);
/// ```
///
/// With formatted identifier:
///
/// ```ignore
/// let field = "userName";
/// let stmt = ts_quote!(
///     this.$(ident!("get{}", field.to_pascal_case()))()
/// as Expr);
/// ```
///
/// Object literal (auto-wrapped):
///
/// ```ignore
/// let key = "status";
/// let value = quote_ident("active");
/// // Generates: ({ status: active })
/// let expr = ts_quote!({ $(ident!(key)): $value } as Expr, value = value);
/// ```
///
/// # Generated Code
///
/// The macro expands to a call to `swc_core::quote!` with:
/// 1. The template string (with `$var` placeholders)
/// 2. Optional `as Type` annotation
/// 3. Extracted bindings (e.g., `var_name = expression`)
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

/// Creates a [`TokenStream2`] containing a single identifier token.
///
/// This is a convenience function for building token streams programmatically
/// when generating macro output. The identifier is created with [`Span::call_site()`]
/// for proper hygiene.
///
/// # Arguments
///
/// * `s` - The identifier string (must be a valid Rust identifier)
///
/// # Example
///
/// ```ignore
/// // Creates tokens for: swc_core
/// let ts = quote_ident("swc_core");
/// ```
fn quote_ident(s: &str) -> TokenStream2 {
    let ident = Ident::new(s, Span::call_site());
    TokenStream2::from(TokenTree::Ident(ident))
}

/// Creates a [`TokenStream2`] containing punctuation tokens.
///
/// Each character in the input string becomes a separate [`Punct`] token
/// with [`Spacing::Joint`], allowing multi-character operators like `::` or `->`.
///
/// # Arguments
///
/// * `s` - The punctuation characters to convert
///
/// # Example
///
/// ```ignore
/// // Creates tokens for: ::
/// let ts = quote_punct("::");
/// ```
fn quote_punct(s: &str) -> TokenStream2 {
    use proc_macro2::Punct;
    s.chars()
        .map(|c| TokenTree::Punct(Punct::new(c, Spacing::Joint)))
        .collect()
}

/// Walks the token stream to build a TypeScript template string and extract bindings.
///
/// This function processes the raw tokens from [`ts_quote!`] input, converting them
/// into a string template suitable for SWC's `quote!` macro while extracting
/// `$(...)` interpolation patterns as separate bindings.
///
/// # Arguments
///
/// * `tokens` - The token stream to process
///
/// # Returns
///
/// A tuple of:
/// 1. The template string with `$var_name` placeholders for SWC
/// 2. A vector of binding token streams (e.g., `var_name = expression`)
///
/// # Processing Rules
///
/// - `$(...)` patterns are extracted and replaced with `$__ts_bind_N_0`
/// - Groups (`{}`, `()`, `[]`) are recursively processed
/// - Identifiers get trailing spaces to prevent accidental merging
/// - Punctuation spacing follows the original token's [`Spacing`]
///
/// # Example
///
/// ```text
/// Input tokens: class $(name) { $(stmt_vec!(methods)) }
/// Output: ("class $__ts_bind_0_0 { *$__ts_bind_1_0 }",
///          [name = name, __ts_bind_1_0 = methods])
/// ```
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

/// Parses the content inside a `$(...)` interpolation pattern.
///
/// This function analyzes the tokens inside an interpolation and generates
/// the appropriate SWC binding code. It supports several syntax forms for
/// different use cases.
///
/// # Arguments
///
/// * `tokens` - The token stream inside `$(...)` to parse
///
/// # Returns
///
/// A tuple of:
/// 1. `bind_name` - The generated unique binding name (e.g., `__ts_bind_0_0`)
/// 2. `binding_code` - Token stream for the binding (e.g., `name: Expr = expr`)
/// 3. `is_vec_expansion` - Whether this should use `*$var` syntax for vec inlining
///
/// # Supported Syntax Forms
///
/// | Input | Generated Binding | Vec Expansion |
/// |-------|-------------------|---------------|
/// | `$(ident!("get{}", name))` | `__bind = Ident::new_no_ctxt(format!(...).into(), DUMMY_SP)` | No |
/// | `$(stmt_vec!(methods))` | `__bind = methods` | Yes (`*$__bind`) |
/// | `$(stmt!(s))` | `__bind: Stmt = s` | No |
/// | `$(expr: Type)` | `__bind: Type = expr` | No |
/// | `$(expr: Vec<Stmt>)` | `__bind: Vec<Stmt> = expr` | Yes |
/// | `$(expr)` | `__bind = expr` | No |
/// | `$(StmtVec(vec))` | `__bind = vec` | Yes |
///
/// # Panics
///
/// Panics if:
/// - Expression inside wrapper macro is invalid
/// - Type annotation cannot be parsed
/// - Expression cannot be parsed
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
/// - `{> "comment" <}` - Line comment: outputs `// comment` (string preserves whitespace)
/// - `{>> "comment" <<}` - Block comment: outputs `/* comment */` (string preserves whitespace)
/// - `///` or `/** */` - Rust doc comments in the template emit JSDoc blocks (`/** ... */`)
/// - `@@{` - Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)
/// - `"'^template ${js}^'"` - JS backtick template literal (outputs `` `template ${js}` ``)
/// - `{#if cond}...{/if}` - Conditional blocks
/// - `{:else}` - Else clause
/// - `{:else if cond}` - Else-if clause
/// - `{#for item in list}...{/for}` - Iteration
/// - `{$let name = expr}` - Local constants
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

    // Parse the template to generate AST-building code
    // parse_template returns a tuple: (Vec<Stmt>, Vec<Patch>)
    let template_builder = match template::parse_template(input) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };

    // Wrap in code that builds AST, emits TypeScript, and collects patches
    let output = quote::quote! {
        {
            let (__stmts, __collected_patches, __comments) = #template_builder;
            let __ts_code = {
                let __module = swc_core::ecma::ast::Module {
                    span: swc_core::common::DUMMY_SP,
                    body: __stmts.into_iter().map(swc_core::ecma::ast::ModuleItem::Stmt).collect(),
                    shebang: None,
                };
                let __cm = swc_core::common::sync::Lrc::new(swc_core::common::SourceMap::default());
                let mut __buf = Vec::new();
                {
                    use swc_core::ecma::codegen::{text_writer::JsWriter, Emitter};
                    let mut __writer = JsWriter::new(__cm.clone(), "\n", &mut __buf, None);
                    let mut __emitter = Emitter {
                        cfg: swc_core::ecma::codegen::Config::default(),
                        cm: __cm.clone(),
                        comments: Some(&__comments),
                        wr: __writer,
                    };
                    __emitter.emit_module(&__module).expect("Failed to emit TypeScript");
                }
                String::from_utf8(__buf).expect("Failed to convert TypeScript output to UTF-8")
            };
            let mut __stream = macroforge_ts::ts_syn::TsStream::from_string(__ts_code);
            __stream.runtime_patches = __collected_patches;
            __stream
        }
    };

    TokenStream::from(output)
}

/// Generates code to be inserted **above** a class or function definition.
///
/// This macro creates a [`TsStream`] prefixed with a special marker comment
/// (`/* @macroforge:above */`) that instructs the macroforge runtime to place
/// the generated code above the decorated definition.
///
/// # Use Cases
///
/// - Adding imports or type declarations before a class
/// - Generating helper functions that should precede the main definition
/// - Adding JSDoc comments or decorators
///
/// # Example
///
/// ```ignore
/// let import_name = "lodash";
/// let stream = above! {
///     import * as _ from "@{import_name}";
/// };
/// // Result: /* @macroforge:above */import * as _ from "lodash";
/// ```
///
/// # Syntax
///
/// Supports the same template syntax as [`ts_template!`]:
/// - `@{expr}` for interpolation
/// - `{#if}`, `{#for}`, `{#match}` for control flow
/// - `{$let}`, `{$do}` for local bindings
///
/// [`TsStream`]: macroforge_ts::ts_syn::TsStream
#[proc_macro]
pub fn above(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    generate_scoped_template(input, "/* @macroforge:above */")
}

/// Generates code to be inserted **below** a class or function definition.
///
/// This macro creates a [`TsStream`] prefixed with a special marker comment
/// (`/* @macroforge:below */`) that instructs the macroforge runtime to place
/// the generated code after the decorated definition.
///
/// # Use Cases
///
/// - Adding prototype extensions after a class definition
/// - Generating registration or initialization code
/// - Adding exports or module augmentations
///
/// # Example
///
/// ```ignore
/// let class_name = "User";
/// let stream = below! {
///     @{class_name}.prototype.toJSON = function() {
///         return { ...this };
///     };
/// };
/// // Result: /* @macroforge:below */User.prototype.toJSON = function() { ... };
/// ```
///
/// # Syntax
///
/// Supports the same template syntax as [`ts_template!`].
///
/// [`TsStream`]: macroforge_ts::ts_syn::TsStream
#[proc_macro]
pub fn below(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    generate_scoped_template(input, "/* @macroforge:below */")
}

/// Generates code to be inserted into a method or function **body**.
///
/// This macro creates a [`TsStream`] prefixed with a special marker comment
/// (`/* @macroforge:body */`) that instructs the macroforge runtime to inject
/// the generated code into the body of a function or method.
///
/// # Use Cases
///
/// - Adding validation logic at the start of methods
/// - Injecting logging or instrumentation
/// - Generating field initialization in constructors
///
/// # Example
///
/// ```ignore
/// let fields = vec!["name", "age"];
/// let stream = body! {
///     {#for field in &fields}
///         this.@{field} = @{field};
///     {/for}
/// };
/// // Result: /* @macroforge:body */this.name = name; this.age = age;
/// ```
///
/// # Syntax
///
/// Supports the same template syntax as [`ts_template!`].
///
/// [`TsStream`]: macroforge_ts::ts_syn::TsStream
#[proc_macro]
pub fn body(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    generate_scoped_template(input, "/* @macroforge:body */")
}

/// Generates code to be inserted into a function **signature**.
///
/// This macro creates a [`TsStream`] prefixed with a special marker comment
/// (`/* @macroforge:signature */`) that instructs the macroforge runtime to
/// modify the function signature (parameters, return type, etc.).
///
/// # Use Cases
///
/// - Adding generated parameters to function signatures
/// - Injecting type annotations or modifiers
/// - Adding decorators to parameters
///
/// # Example
///
/// ```ignore
/// let param_name = "context";
/// let param_type = "RequestContext";
/// let stream = signature! {
///     @{param_name}: @{param_type}
/// };
/// // Result: /* @macroforge:signature */context: RequestContext
/// ```
///
/// # Syntax
///
/// Supports the same template syntax as [`ts_template!`].
///
/// [`TsStream`]: macroforge_ts::ts_syn::TsStream
#[proc_macro]
pub fn signature(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    generate_scoped_template(input, "/* @macroforge:signature */")
}

/// Internal helper that generates a scoped template with a position marker.
///
/// This function wraps [`template::parse_template`] to create a [`TsStream`]
/// that is prefixed with a marker comment indicating where the generated code
/// should be placed relative to the decorated definition.
///
/// # Arguments
///
/// * `input` - The template token stream to process
/// * `marker` - The position marker comment (e.g., `/* @macroforge:above */`)
///
/// # Returns
///
/// A proc macro token stream that, when executed, produces a [`TsStream`] with:
/// - The marker comment prepended to the source
/// - Any runtime patches (imports, etc.) collected from `{$typescript}` injections
///
/// # Errors
///
/// Returns a compile error if template parsing fails.
fn generate_scoped_template(input: TokenStream2, marker: &str) -> TokenStream {
    let template_builder = match template::parse_template(input) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };

    // parse_template now returns a tuple: (Vec<Stmt>, Vec<Patch>)
    // We destructure it and set runtime_patches on the resulting TsStream

    let output = quote::quote! {
        {
            let (__stmts, __collected_patches, __comments) = #template_builder;
            let __content = {
                let __module = swc_core::ecma::ast::Module {
                    span: swc_core::common::DUMMY_SP,
                    body: __stmts.into_iter().map(swc_core::ecma::ast::ModuleItem::Stmt).collect(),
                    shebang: None,
                };
                let __cm = swc_core::common::sync::Lrc::new(swc_core::common::SourceMap::default());
                let mut __buf = Vec::new();
                {
                    use swc_core::ecma::codegen::{text_writer::JsWriter, Emitter};
                    let mut __writer = JsWriter::new(__cm.clone(), "\n", &mut __buf, None);
                    let mut __emitter = Emitter {
                        cfg: swc_core::ecma::codegen::Config::default(),
                        cm: __cm.clone(),
                        comments: Some(&__comments),
                        wr: __writer,
                    };
                    __emitter.emit_module(&__module).expect("Failed to emit TypeScript");
                }
                String::from_utf8(__buf).expect("Failed to convert TypeScript output to UTF-8")
            };
            let mut __ts_code = String::from(#marker);
            __ts_code.push_str(&__content);
            let mut __stream = macroforge_ts::ts_syn::TsStream::from_string(__ts_code);
            __stream.runtime_patches = __collected_patches;
            __stream
        }
    };

    TokenStream::from(output)
}
