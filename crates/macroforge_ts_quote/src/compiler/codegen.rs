//! Code generation from IR to Rust TokenStream.
//!
//! This module generates Rust code that builds SWC AST at compile time.
//! It uses `swc_core::quote!` for static TypeScript and ToTs* traits for placeholders.
//!
//! ## Virtual Completion Strategy
//!
//! When control flow (`{#for}`, `{#if}`, etc.) appears inside a block (function body,
//! class body, etc.), the template gets split into incomplete chunks. For example:
//!
//! ```text
//! export function foo(): void {
//!     const x = 1;
//!     {#for item in items}
//!         console.log(item);
//!     {/for}
//!     return x;
//! }
//! ```
//!
//! This splits into:
//! 1. `export function foo(): void { const x = 1;` (incomplete - missing `}`)
//! 2. `console.log(item);` (the loop body)
//! 3. `return x; }` (incomplete - orphan `}`)
//!
//! To handle this, we use "virtual completion":
//! - Track brace depth to detect incomplete chunks
//! - Add virtual closing braces for SWC quote! validation
//! - Generate runtime code that builds proper statement lists
//! - Assemble the final AST by combining validated pieces

use super::ir::{Ir, IrNode};
use super::semantic::PlaceholderKind;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::cell::Cell;

/// Configuration for code generation.
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Variable name for the output accumulator (Vec<ModuleItem>).
    pub output_var: String,
    /// Whether to generate code for class body members (body! macro).
    /// When true, templates are wrapped in a dummy class for compile-time validation.
    pub body_mode: bool,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            output_var: "__mf_stmts".to_string(),
            body_mode: false,
        }
    }
}

/// Brace balance information for a template chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BraceBalance {
    /// Net change in brace depth (positive = more opens, negative = more closes)
    net_change: i32,
    /// Number of unmatched opening braces
    unclosed_opens: i32,
    /// Number of unmatched closing braces
    unmatched_closes: i32,
}

impl BraceBalance {
    /// Analyzes a template string to determine brace balance.
    fn analyze(template: &str) -> Self {
        let mut depth: i32 = 0;
        let mut min_depth: i32 = 0;
        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;

        for ch in template.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => {
                    depth += 1;
                }
                '}' => {
                    depth -= 1;
                    if depth < min_depth {
                        min_depth = depth;
                    }
                }
                _ => {}
            }
        }

        // unclosed_opens = how many `{` don't have matching `}`
        // unmatched_closes = how many `}` don't have matching `{`
        let unmatched_closes = (-min_depth).max(0);
        let unclosed_opens = (depth - min_depth).max(0);

        Self {
            net_change: depth,
            unclosed_opens,
            unmatched_closes,
        }
    }

    /// Returns true if the template has balanced braces.
    fn is_balanced(&self) -> bool {
        self.net_change == 0 && self.unclosed_opens == 0 && self.unmatched_closes == 0
    }
}

/// A chunk of template content that can be processed together.
#[derive(Debug)]
enum Chunk<'a> {
    /// A parseable chunk of static text + placeholders.
    /// Can be compiled with swc_core::quote!.
    Parseable {
        /// Template string with $placeholder markers.
        template: String,
        /// Placeholder info: (placeholder_name, kind, rust_expr)
        placeholders: Vec<(String, PlaceholderKind, String)>,
        /// Brace balance for virtual completion.
        brace_balance: BraceBalance,
    },
    /// Control flow node that needs special handling.
    ControlFlow(&'a IrNode),
    /// Directive that generates Rust code directly.
    Directive(&'a IrNode),
    /// String interpolation for template literals.
    StringInterp {
        quote_char: char,
        parts: &'a [IrNode],
    },
    /// A doc comment to add to pending comments.
    Comment { text: String },
}

/// Code generator from IR.
pub struct Codegen {
    config: CodegenConfig,
    /// Counter for generating unique placeholder names.
    placeholder_counter: Cell<usize>,
}

impl Codegen {
    /// Creates a new code generator with default config.
    pub fn new() -> Self {
        Self {
            config: CodegenConfig::default(),
            placeholder_counter: Cell::new(0),
        }
    }

    /// Creates a new code generator with the given config.
    pub fn with_config(config: CodegenConfig) -> Self {
        Self {
            config,
            placeholder_counter: Cell::new(0),
        }
    }

    /// Generates a unique placeholder name.
    fn next_placeholder_name(&self) -> String {
        let n = self.placeholder_counter.get();
        self.placeholder_counter.set(n + 1);
        format!("__ph_{}", n)
    }

    /// Generates Rust TokenStream from IR.
    ///
    /// The generated code builds `Vec<ModuleItem>` at compile time using
    /// `swc_core::quote!` for static TypeScript and ToTs* traits for placeholders.
    pub fn generate(&self, ir: &Ir) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let body = self.generate_nodes(&ir.nodes);

        quote! {
            {
                let mut #output_var: Vec<swc_core::ecma::ast::ModuleItem> = Vec::new();
                #body
                #output_var
            }
        }
    }

    /// Generates code for a sequence of IR nodes.
    fn generate_nodes(&self, nodes: &[IrNode]) -> TokenStream {
        let chunks = self.chunk_nodes(nodes);

        // Check if we have any unbalanced chunks that need opener/closer tracking
        let has_unbalanced = chunks.iter().any(|c| match c {
            Chunk::Parseable { brace_balance, .. } => !brace_balance.is_balanced(),
            _ => false,
        });

        let stmts: Vec<TokenStream> = chunks.iter().map(|c| self.generate_chunk(c)).collect();

        if has_unbalanced {
            // When we have unbalanced chunks, we need a stack to track opener indices
            quote! {
                let mut __mf_opener_stack: Vec<usize> = Vec::new();
                #(#stmts)*
            }
        } else {
            quote! { #(#stmts)* }
        }
    }

    /// Groups consecutive nodes into chunks that can be processed together.
    fn chunk_nodes<'a>(&self, nodes: &'a [IrNode]) -> Vec<Chunk<'a>> {
        let mut chunks = Vec::new();
        let mut current_template = String::new();
        let mut current_placeholders: Vec<(String, PlaceholderKind, String)> = Vec::new();

        // Helper to flush the current parseable chunk
        let flush_parseable =
            |chunks: &mut Vec<Chunk<'a>>,
             template: &mut String,
             placeholders: &mut Vec<(String, PlaceholderKind, String)>| {
                if !template.is_empty() || !placeholders.is_empty() {
                    let tmpl = std::mem::take(template);
                    let brace_balance = BraceBalance::analyze(&tmpl);
                    chunks.push(Chunk::Parseable {
                        template: tmpl,
                        placeholders: std::mem::take(placeholders),
                        brace_balance,
                    });
                }
            };

        for node in nodes {
            match node {
                IrNode::Text(text) => {
                    current_template.push_str(text);
                }

                IrNode::Placeholder { kind, rust_expr } => {
                    let ph_name = self.next_placeholder_name();
                    match kind {
                        // For Type placeholders, use a concrete marker (no $ prefix)
                        // because SWC quote! doesn't support $ in type positions
                        PlaceholderKind::Type => {
                            current_template.push_str(&ph_name);
                        }
                        // For Ident, Expr - use $ prefix for SWC quote! substitution
                        _ => {
                            current_template.push('$');
                            current_template.push_str(&ph_name);
                        }
                    }
                    current_placeholders.push((ph_name, *kind, rust_expr.clone()));
                }

                IrNode::If { .. }
                | IrNode::For { .. }
                | IrNode::While { .. }
                | IrNode::Match { .. } => {
                    flush_parseable(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                    );
                    chunks.push(Chunk::ControlFlow(node));
                }

                IrNode::Let { .. } | IrNode::Do { .. } | IrNode::TypeScript { .. } => {
                    flush_parseable(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                    );
                    chunks.push(Chunk::Directive(node));
                }

                IrNode::IdentBlock { parts } => {
                    // Treat ident block as an Ident placeholder embedded in the template
                    let ph_name = self.next_placeholder_name();
                    current_template.push('$');
                    current_template.push_str(&ph_name);

                    // Generate the Rust expression that builds the identifier
                    let ident_builder = self.generate_ident_builder_expr(parts);
                    current_placeholders.push((
                        ph_name,
                        PlaceholderKind::Ident,
                        ident_builder,
                    ));
                }

                IrNode::StringInterp { quote: q, parts } => {
                    // Check if this string actually contains interpolations
                    let has_interpolations = parts.iter().any(|p| matches!(p, IrNode::Placeholder { .. }));

                    if has_interpolations {
                        // This is a true template literal with expressions
                        flush_parseable(
                            &mut chunks,
                            &mut current_template,
                            &mut current_placeholders,
                        );
                        chunks.push(Chunk::StringInterp {
                            quote_char: *q,
                            parts,
                        });
                    } else {
                        // No interpolations - just append as static text with quotes
                        current_template.push(*q);
                        for part in parts {
                            if let IrNode::Text(text) = part {
                                current_template.push_str(text);
                            }
                        }
                        current_template.push(*q);
                    }
                }

                IrNode::Comment { text } => {
                    // Flush any pending template before the comment
                    flush_parseable(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                    );

                    // Add a comment chunk
                    chunks.push(Chunk::Comment {
                        text: text.clone(),
                    });
                }
            }
        }

        // Flush remaining
        flush_parseable(
            &mut chunks,
            &mut current_template,
            &mut current_placeholders,
        );

        chunks
    }

    /// Generates code for a single chunk.
    fn generate_chunk(&self, chunk: &Chunk) -> TokenStream {
        match chunk {
            Chunk::Parseable {
                template,
                placeholders,
                brace_balance,
            } => self.generate_parseable_chunk(template, placeholders, *brace_balance),

            Chunk::ControlFlow(node) => self.generate_control_flow(node),

            Chunk::Directive(node) => self.generate_directive(node),

            Chunk::StringInterp { quote_char, parts } => {
                self.generate_string_interp(*quote_char, parts)
            }

            Chunk::Comment { text } => {
                // Emit code to add the comment to __pending_comments
                let comment_text = format!("* {} ", text.trim());
                quote! {
                    __pending_comments.push(swc_core::common::comments::Comment {
                        kind: swc_core::common::comments::CommentKind::Block,
                        span: swc_core::common::DUMMY_SP,
                        text: #comment_text.into(),
                    });
                }
            }
        }
    }

    /// Generates code for a parseable chunk.
    ///
    /// Uses swc_core::quote! to build AST at compile time with ToTs* traits
    /// for type-safe placeholder substitution.
    ///
    /// ## Virtual Completion
    ///
    /// When a chunk has unbalanced braces (due to control flow splitting), we use
    /// virtual completion to make it parseable:
    /// - Unclosed opens: add virtual `}` for validation, extract body statements
    /// - Unmatched closes: wrap in virtual function, extract body statements
    ///
    /// ## Type Placeholder Substitution
    ///
    /// SWC quote! doesn't support placeholders in type positions, so we use marker
    /// names (like `__ph_0`) and replace them after parsing using AST post-processing.
    fn generate_parseable_chunk(
        &self,
        template: &str,
        placeholders: &[(String, PlaceholderKind, String)],
        brace_balance: BraceBalance,
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        // Skip empty templates
        if template.trim().is_empty() && placeholders.is_empty() {
            return quote! {};
        }

        // Generate placeholder bindings (convert Rust exprs to SWC AST nodes)
        let (binding_stmts, quote_bindings, type_replacements) =
            self.generate_placeholder_bindings(placeholders);
        let has_type_placeholders = !type_replacements.is_empty();

        // Debug: print the template being passed to swc_core::quote!
        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!(
                "[MF_DEBUG_CODEGEN] Template for SWC quote: {:?} (balance: {:?})",
                template, brace_balance
            );
            for (ph_name, kind, rust_expr) in placeholders {
                eprintln!(
                    "[MF_DEBUG_CODEGEN]   Placeholder {}: kind={:?}, expr={}",
                    ph_name, kind, rust_expr
                );
            }
        }

        // Check if this is a module-level declaration (export/import)
        let trimmed = template.trim_start();
        let is_module_decl = trimmed.starts_with("export ") || trimmed.starts_with("import ");

        // Handle body mode (class member syntax)
        if self.config.body_mode {
            return self.generate_body_mode_chunk(template, &binding_stmts, &quote_bindings);
        }

        // Handle unbalanced braces with virtual completion
        if !brace_balance.is_balanced() {
            return self.generate_virtually_completed_chunk(
                template,
                &binding_stmts,
                &quote_bindings,
                &type_replacements,
                brace_balance,
                is_module_decl,
            );
        }

        // Normal case: balanced braces
        let template_lit = syn::LitStr::new(template, proc_macro2::Span::call_site());

        if is_module_decl {
            // Parse as ModuleItem for export/import declarations
            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    swc_core::quote!(#template_lit as ModuleItem)
                }
            } else {
                quote! {
                    swc_core::quote!(#template_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            if has_type_placeholders {
                // With type placeholders: parse, then replace markers
                quote! {
                    {
                        #(#binding_stmts)*
                        let mut __mf_type_replacements: std::collections::HashMap<String, swc_core::ecma::ast::TsType> = std::collections::HashMap::new();
                        #(#type_replacements)*
                        let mut __mf_item: swc_core::ecma::ast::ModuleItem = #quote_call;
                        macroforge_ts_syn::__internal::replace_type_markers(&mut __mf_item, __mf_type_replacements);
                        #output_var.push(__mf_item);
                    }
                }
            } else {
                quote! {
                    {
                        #(#binding_stmts)*
                        #output_var.push(#quote_call);
                    }
                }
            }
        } else {
            // Parse as Stmt for regular statements
            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    swc_core::quote!(#template_lit as Stmt)
                }
            } else {
                quote! {
                    swc_core::quote!(#template_lit as Stmt, #(#quote_bindings),*)
                }
            };

            if has_type_placeholders {
                // With type placeholders: parse, then replace markers
                quote! {
                    {
                        #(#binding_stmts)*
                        let mut __mf_type_replacements: std::collections::HashMap<String, swc_core::ecma::ast::TsType> = std::collections::HashMap::new();
                        #(#type_replacements)*
                        let mut __mf_stmt: swc_core::ecma::ast::Stmt = #quote_call;
                        macroforge_ts_syn::__internal::replace_type_markers_stmt(&mut __mf_stmt, __mf_type_replacements);
                        #output_var.push(swc_core::ecma::ast::ModuleItem::Stmt(__mf_stmt));
                    }
                }
            } else {
                quote! {
                    {
                        #(#binding_stmts)*
                        #output_var.push(swc_core::ecma::ast::ModuleItem::Stmt(#quote_call));
                    }
                }
            }
        }
    }

    /// Generates placeholder binding statements and quote bindings.
    ///
    /// The quote bindings include type annotations because `swc_core::quote!` requires
    /// them to know what AST node type to expect in each position.
    ///
    /// Returns:
    /// - binding_stmts: Let statements that convert Rust values to SWC AST nodes
    /// - quote_bindings: Bindings for swc_core::quote! macro
    /// - type_replacements: Statements to build the type replacement HashMap
    fn generate_placeholder_bindings(
        &self,
        placeholders: &[(String, PlaceholderKind, String)],
    ) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
        let mut binding_stmts = Vec::new();
        let mut quote_bindings = Vec::new();
        let mut type_replacements = Vec::new();

        for (ph_name, kind, rust_expr) in placeholders {
            let ph_ident = format_ident!("{}", ph_name);
            let ph_name_str = ph_name.as_str();
            let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| {
                let ident = format_ident!("{}", rust_expr);
                quote! { #ident }
            });

            // SWC quote! only supports Ident, Expr, Pat as placeholder types.
            // Type placeholders use concrete marker names and are post-processed.
            match kind {
                PlaceholderKind::Expr => {
                    binding_stmts.push(quote! {
                        let #ph_ident: swc_core::ecma::ast::Expr = macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: Expr = #ph_ident });
                }
                PlaceholderKind::Type => {
                    // Type placeholders use concrete marker names (no $ prefix) in the template.
                    // SWC quote! parses them as regular identifiers, then we replace them.
                    // Create the type for later replacement via AST post-processing
                    binding_stmts.push(quote! {
                        let #ph_ident: swc_core::ecma::ast::TsType = macroforge_ts_syn::ToTsType::to_ts_type(#expr);
                    });
                    // Add to type replacement map
                    type_replacements.push(quote! {
                        __mf_type_replacements.insert(#ph_name_str.to_string(), #ph_ident);
                    });
                    // Type placeholders don't get quote bindings - they're markers in the template
                    // that get replaced with the actual type after parsing
                }
                PlaceholderKind::Ident => {
                    binding_stmts.push(quote! {
                        let #ph_ident: swc_core::ecma::ast::Ident = macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: Ident = #ph_ident });
                }
                PlaceholderKind::Stmt => {
                    // SWC quote! doesn't support Stmt - use Expr and context determines usage
                    binding_stmts.push(quote! {
                        let #ph_ident: swc_core::ecma::ast::Expr = macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: Expr = #ph_ident });
                }
            }
        }

        (binding_stmts, quote_bindings, type_replacements)
    }

    /// Generates code for body mode (class member syntax).
    fn generate_body_mode_chunk(
        &self,
        template: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        // For class body members, wrap in a dummy class for compile-time validation
        let wrapped_template = format!("class __MfTemp {{ {} }}", template);
        let wrapped_lit = syn::LitStr::new(&wrapped_template, proc_macro2::Span::call_site());

        let quote_call = if quote_bindings.is_empty() {
            quote! {
                swc_core::quote!(#wrapped_lit as ModuleItem)
            }
        } else {
            quote! {
                swc_core::quote!(#wrapped_lit as ModuleItem, #(#quote_bindings),*)
            }
        };

        quote! {
            {
                #(#binding_stmts)*
                #output_var.push(#quote_call);
            }
        }
    }

    /// Generates code for a chunk with unbalanced braces using virtual completion.
    ///
    /// This handles the case where control flow has split a template into incomplete
    /// chunks. We virtually complete the template for validation, then extract
    /// just the statements we need at runtime.
    ///
    /// The generated code uses a stack (`__mf_opener_stack`) to track opener indices.
    /// Openers push their index, closers pop and finalize, middles peek the current opener.
    fn generate_virtually_completed_chunk(
        &self,
        template: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        type_replacements: &[TokenStream],
        brace_balance: BraceBalance,
        _is_module_decl: bool,
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let has_type_placeholders = !type_replacements.is_empty();

        // Helper to generate type replacement code if needed
        let type_replacement_code = if has_type_placeholders {
            quote! {
                let mut __mf_type_replacements: std::collections::HashMap<String, swc_core::ecma::ast::TsType> = std::collections::HashMap::new();
                #(#type_replacements)*
                macroforge_ts_syn::__internal::replace_type_markers(&mut __mf_item, __mf_type_replacements);
            }
        } else {
            quote! {}
        };

        // Determine the virtual completion strategy
        if brace_balance.unclosed_opens > 0 && brace_balance.unmatched_closes == 0 {
            // Case: Template opens blocks but doesn't close them
            // e.g., "export function foo(): void { const x = 1;"
            // Strategy: Add virtual closing braces, parse as ModuleItem, push and track index

            let virtual_closes = "}".repeat(brace_balance.unclosed_opens as usize);
            let completed_template = format!("{}{}", template, virtual_closes);
            let completed_lit =
                syn::LitStr::new(&completed_template, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    swc_core::quote!(#completed_lit as ModuleItem)
                }
            } else {
                quote! {
                    swc_core::quote!(#completed_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            let unclosed = brace_balance.unclosed_opens as usize;

            // Push the opener and track its index on the stack
            quote! {
                {
                    #(#binding_stmts)*
                    // Parse the virtually completed template
                    let mut __mf_item: swc_core::ecma::ast::ModuleItem = #quote_call;
                    #type_replacement_code

                    // Push the opener and record its index for later finalization
                    let __mf_idx = macroforge_ts_syn::__internal::push_opener(
                        __mf_item,
                        &mut #output_var,
                        #unclosed,
                    );
                    __mf_opener_stack.push(__mf_idx);
                }
            }
        } else if brace_balance.unmatched_closes > 0 && brace_balance.unclosed_opens == 0 {
            // Case: Template closes blocks without opening them
            // e.g., "return x; }"
            // Strategy: Wrap in virtual function opening, parse, finalize the opener

            let extra_opens = "{".repeat((brace_balance.unmatched_closes - 1).max(0) as usize);
            let virtual_opens = format!("function __mf_virtual() {{ {}", extra_opens);
            let completed_template = format!("{}{}", virtual_opens, template);
            let completed_lit =
                syn::LitStr::new(&completed_template, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    swc_core::quote!(#completed_lit as ModuleItem)
                }
            } else {
                quote! {
                    swc_core::quote!(#completed_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            // Pop the opener index and finalize
            quote! {
                {
                    #(#binding_stmts)*
                    // Parse the virtually completed template
                    let mut __mf_item: swc_core::ecma::ast::ModuleItem = #quote_call;
                    #type_replacement_code

                    // Pop the opener index and finalize with accumulated statements
                    let __mf_opener_idx = __mf_opener_stack.pop()
                        .expect("Virtual completion: no matching opener for closer");
                    macroforge_ts_syn::__internal::finalize_closer(
                        __mf_item,
                        &mut #output_var,
                        __mf_opener_idx,
                    );
                }
            }
        } else {
            // Case: Template has both unclosed opens AND unmatched closes
            // This is a middle chunk inside a block
            // e.g., "} else { const y = 2;" (closes one, opens one)
            // Strategy: Wrap for validation, collect statements to current opener

            let virtual_opens = "{".repeat(brace_balance.unmatched_closes as usize);
            let virtual_closes = "}".repeat(brace_balance.unclosed_opens as usize);
            let completed_template = format!(
                "function __mf_virtual() {{ {}{}{} }}",
                virtual_opens, template, virtual_closes
            );
            let completed_lit =
                syn::LitStr::new(&completed_template, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    swc_core::quote!(#completed_lit as ModuleItem)
                }
            } else {
                quote! {
                    swc_core::quote!(#completed_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            // For middle chunks, peek the opener index (don't pop - we're still in the same block)
            quote! {
                {
                    #(#binding_stmts)*
                    // Parse the virtually completed template for validation
                    let mut __mf_item: swc_core::ecma::ast::ModuleItem = #quote_call;
                    #type_replacement_code

                    // Get the current opener index (peek, not pop)
                    let __mf_opener_idx = *__mf_opener_stack.last()
                        .expect("Virtual completion: no matching opener for middle chunk");
                    macroforge_ts_syn::__internal::push_middle(
                        __mf_item,
                        &mut #output_var,
                        __mf_opener_idx,
                    );
                }
            }
        }
    }

    /// Generates code for control flow nodes.
    fn generate_control_flow(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::If {
                condition,
                then_body,
                else_if_branches,
                else_body,
            } => self.generate_if(condition, then_body, else_if_branches, else_body),

            IrNode::For {
                pattern,
                iterator,
                body,
            } => self.generate_for(pattern, iterator, body),

            IrNode::While { condition, body } => self.generate_while(condition, body),

            IrNode::Match { expr, arms } => self.generate_match(expr, arms),

            _ => quote! {},
        }
    }

    /// Generates code for directive nodes.
    fn generate_directive(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::Let {
                name,
                mutable,
                type_hint,
                value,
            } => self.generate_let(name, *mutable, type_hint.as_deref(), value),
            IrNode::Do { code } => self.generate_do(code),
            IrNode::TypeScript { stream } => self.generate_typescript(stream),
            _ => quote! {},
        }
    }

    /// Generates code for an if statement.
    fn generate_if(
        &self,
        condition: &str,
        then_body: &[IrNode],
        else_if_branches: &[(String, Vec<IrNode>)],
        else_body: &Option<Vec<IrNode>>,
    ) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { true });
        let then_code = self.generate_nodes(then_body);

        let else_if_code: Vec<TokenStream> = else_if_branches
            .iter()
            .map(|(cond, body)| {
                let c: TokenStream = cond.parse().unwrap_or_else(|_| quote! { true });
                let b = self.generate_nodes(body);
                quote! { else if #c { #b } }
            })
            .collect();

        let else_code = else_body.as_ref().map(|body| {
            let b = self.generate_nodes(body);
            quote! { else { #b } }
        });

        quote! {
            if #cond {
                #then_code
            }
            #(#else_if_code)*
            #else_code
        }
    }

    /// Generates code for a for loop.
    fn generate_for(&self, pattern: &str, iterator: &str, body: &[IrNode]) -> TokenStream {
        let pat: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { _ });
        let iter: TokenStream =
            iterator.parse().unwrap_or_else(|_| quote! { std::iter::empty::<()>() });
        let body_code = self.generate_nodes(body);

        quote! {
            for #pat in #iter {
                #body_code
            }
        }
    }

    /// Generates code for a while loop.
    fn generate_while(&self, condition: &str, body: &[IrNode]) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { false });
        let body_code = self.generate_nodes(body);

        quote! {
            while #cond {
                #body_code
            }
        }
    }

    /// Generates code for a match expression.
    fn generate_match(&self, expr: &str, arms: &[(String, Option<String>, Vec<IrNode>)]) -> TokenStream {
        let e: TokenStream = expr.parse().unwrap_or_else(|_| quote! { () });

        let arm_code: Vec<TokenStream> = arms
            .iter()
            .map(|(pattern, guard, body)| {
                let pat: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { _ });
                let body_code = self.generate_nodes(body);

                if let Some(g) = guard {
                    let guard_expr: TokenStream = g.parse().unwrap_or_else(|_| quote! { true });
                    quote! { #pat if #guard_expr => { #body_code } }
                } else {
                    quote! { #pat => { #body_code } }
                }
            })
            .collect();

        quote! {
            match #e {
                #(#arm_code)*
            }
        }
    }

    /// Generates a Rust expression string that builds an identifier from parts.
    ///
    /// This is used to embed ident blocks as placeholders in swc_core::quote!.
    fn generate_ident_builder_expr(&self, parts: &[IrNode]) -> String {
        let mut expr_parts = Vec::new();

        for part in parts {
            match part {
                IrNode::Text(text) => {
                    // Static text part
                    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
                    expr_parts.push(format!("\"{}\".to_string()", escaped));
                }
                IrNode::Placeholder { rust_expr, .. } => {
                    // Dynamic part - convert to string
                    expr_parts.push(format!("({}).to_string()", rust_expr));
                }
                _ => {}
            }
        }

        if expr_parts.is_empty() {
            // Empty ident block - just return empty string builder
            return "{ let s = String::new(); swc_core::ecma::ast::Ident::new_no_ctxt(s.into(), swc_core::common::DUMMY_SP) }".to_string();
        }

        // Generate expression that concatenates all parts and creates an Ident
        format!(
            "{{ let mut __s = String::new(); {} swc_core::ecma::ast::Ident::new_no_ctxt(__s.into(), swc_core::common::DUMMY_SP) }}",
            expr_parts.iter().map(|p| format!("__s.push_str(&{});", p)).collect::<Vec<_>>().join(" ")
        )
    }

    /// Generates code for an identifier block.
    ///
    /// DEPRECATED: Ident blocks are now handled as placeholders in chunk_nodes.
    /// This function is kept for backwards compatibility but should not be called.
    #[allow(dead_code)]
    fn generate_ident_block(&self, parts: &[IrNode]) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        let part_stmts: Vec<TokenStream> = parts
            .iter()
            .filter_map(|p| match p {
                IrNode::Text(text) => Some(quote! { __ident_parts.push_str(#text); }),
                IrNode::Placeholder { rust_expr, .. } => {
                    let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| {
                        let ident = format_ident!("{}", rust_expr);
                        quote! { #ident }
                    });
                    Some(quote! { __ident_parts.push_str(&(#expr).to_string()); })
                }
                _ => None,
            })
            .collect();

        // Build the identifier and create an expression statement with it
        quote! {
            {
                let mut __ident_parts = String::new();
                #(#part_stmts)*
                let __ident = swc_core::ecma::ast::Ident::new_no_ctxt(
                    __ident_parts.into(),
                    swc_core::common::DUMMY_SP
                );
                #output_var.push(swc_core::ecma::ast::ModuleItem::Stmt(
                    swc_core::ecma::ast::Stmt::Expr(swc_core::ecma::ast::ExprStmt {
                        span: swc_core::common::DUMMY_SP,
                        expr: Box::new(swc_core::ecma::ast::Expr::Ident(__ident)),
                    })
                ));
            }
        }
    }

    /// Generates code for string interpolation (template literals).
    fn generate_string_interp(&self, quote_char: char, parts: &[IrNode]) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        // Build quasis (static parts) and expressions
        let mut quasis = Vec::new();
        let mut exprs = Vec::new();
        let mut current_quasi = String::new();

        for part in parts {
            match part {
                IrNode::Text(text) => {
                    current_quasi.push_str(text);
                }
                IrNode::Placeholder { rust_expr, .. } => {
                    // Flush current quasi
                    let quasi_text = std::mem::take(&mut current_quasi);
                    quasis.push(quote! {
                        swc_core::ecma::ast::TplElement {
                            span: swc_core::common::DUMMY_SP,
                            tail: false,
                            cooked: Some(#quasi_text.into()),
                            raw: #quasi_text.into(),
                        }
                    });

                    let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| {
                        let ident = format_ident!("{}", rust_expr);
                        quote! { #ident }
                    });
                    exprs.push(quote! {
                        Box::new(macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr))
                    });
                }
                _ => {}
            }
        }

        // Final quasi (tail)
        let final_quasi = current_quasi;
        quasis.push(quote! {
            swc_core::ecma::ast::TplElement {
                span: swc_core::common::DUMMY_SP,
                tail: true,
                cooked: Some(#final_quasi.into()),
                raw: #final_quasi.into(),
            }
        });

        let _ = quote_char; // We build a template literal regardless of quote char

        quote! {
            #output_var.push(swc_core::ecma::ast::ModuleItem::Stmt(
                swc_core::ecma::ast::Stmt::Expr(swc_core::ecma::ast::ExprStmt {
                    span: swc_core::common::DUMMY_SP,
                    expr: Box::new(swc_core::ecma::ast::Expr::Tpl(swc_core::ecma::ast::Tpl {
                        span: swc_core::common::DUMMY_SP,
                        exprs: vec![#(#exprs),*],
                        quasis: vec![#(#quasis),*],
                    })),
                })
            ));
        }
    }

    /// Generates code for a let directive.
    fn generate_let(
        &self,
        name: &str,
        mutable: bool,
        type_hint: Option<&str>,
        value: &str,
    ) -> TokenStream {
        let ident = format_ident!("{}", name);
        let val: TokenStream = value.parse().unwrap_or_else(|_| quote! { () });

        let mut_token = if mutable { quote! { mut } } else { quote! {} };

        let type_annotation = if let Some(ty) = type_hint {
            let ty_tokens: TokenStream = ty.parse().unwrap_or_else(|_| quote! { _ });
            quote! { : #ty_tokens }
        } else {
            quote! {}
        };

        quote! {
            let #mut_token #ident #type_annotation = #val;
        }
    }

    /// Generates code for a do directive.
    fn generate_do(&self, code: &str) -> TokenStream {
        let c: TokenStream = code.parse().unwrap_or_else(|_| quote! { () });
        quote! {
            #c;
        }
    }

    /// Generates code for a typescript directive.
    fn generate_typescript(&self, stream: &str) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let s: TokenStream = stream.parse().unwrap_or_else(|_| quote! { () });

        quote! {
            // {$typescript} injects a TsStream - parse it and extend with its module items
            {
                let mut __ts_stream = #s;
                __patches.extend(__ts_stream.runtime_patches.drain(..));
                let __ts_source = __ts_stream.source().to_string();

                // Parse the TsStream source into a Module and extract items
                let mut __ts_parser = macroforge_ts::ts_syn::TsStream::from_string(__ts_source);
                let __ts_module: swc_core::ecma::ast::Module = __ts_parser
                    .parse()
                    .unwrap_or_else(|e| panic!(
                        "Failed to parse injected TsStream from {{$typescript}}: {:?}\n\nSource:\n{}",
                        e,
                        __ts_source
                    ));

                // Extend output with the module's items
                #output_var.extend(__ts_module.body);
            }
        }
    }
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::ir::lower;
    use crate::compiler::parser::Parser;
    use crate::compiler::semantic::analyze;
    use crate::compiler::syntax::SyntaxNode;

    fn compile_template(input: &str) -> TokenStream {
        let parser = Parser::new(input);
        let green = parser.parse();
        let root = SyntaxNode::new_root(green);
        let analysis = analyze(&root);
        let ir = lower(&root, analysis);
        Codegen::new().generate(&ir)
    }

    #[test]
    fn test_codegen_simple_text() {
        let code = compile_template("const x = 1;");
        let code_str = code.to_string();
        // Should use swc_core::quote! for static text
        assert!(
            code_str.contains("swc_core :: quote !"),
            "Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_interpolation() {
        let code = compile_template("const x = @{value};");
        let code_str = code.to_string();
        // Should contain ToTsExpr trait call
        assert!(
            code_str.contains("ToTsExpr") || code_str.contains("to_ts_expr"),
            "Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_type_placeholder() {
        let code = compile_template("const x: @{MyType} = 1;");
        let code_str = code.to_string();
        // Should contain ToTsType trait call for creating the type
        assert!(
            code_str.contains("ToTsType") || code_str.contains("to_ts_type"),
            "Expected ToTsType for type placeholder. Generated code: {}",
            code_str
        );
        // Should contain type replacement mechanism
        assert!(
            code_str.contains("replace_type_markers"),
            "Expected replace_type_markers for type substitution. Generated code: {}",
            code_str
        );
        // Should build the replacement HashMap
        assert!(
            code_str.contains("__mf_type_replacements"),
            "Expected type replacements map. Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_for_loop() {
        let code = compile_template("{#for item in items}@{item}{/for}");
        let code_str = code.to_string();
        // Should contain a for loop
        assert!(code_str.contains("for"), "Generated code: {}", code_str);
        assert!(code_str.contains("in"), "Generated code: {}", code_str);
    }

    #[test]
    fn test_codegen_if_block() {
        let code = compile_template("{#if cond}yes{/if}");
        let code_str = code.to_string();
        // Should contain an if statement
        assert!(code_str.contains("if"), "Generated code: {}", code_str);
    }

    #[test]
    fn test_codegen_generates_vec_module_item() {
        let code = compile_template("const x = 1;");
        let code_str = code.to_string();
        // Should generate Vec<ModuleItem>
        assert!(
            code_str.contains("Vec < swc_core :: ecma :: ast :: ModuleItem >"),
            "Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_function_with_doc_attribute_tokenstream_format() {
        // This is how doc comments appear after going through Rust's TokenStream:
        // /** Doc */ becomes # [doc = "Doc"]
        let code = compile_template(
            r#"# [doc = "Doc comment"] export function @{fn_name}(value: @{type_param}): string { return @{body_expr}; }"#
        );
        let code_str = code.to_string();
        eprintln!("Generated code:\n{}", code_str);

        // fn_name should be treated as Ident (ToTsIdent)
        assert!(
            code_str.contains("to_ts_ident"),
            "fn_name should use ToTsIdent. Generated code:\n{}", code_str
        );

        // type_param should be treated as Type (ToTsType)
        assert!(
            code_str.contains("to_ts_type"),
            "type_param should use ToTsType. Generated code:\n{}", code_str
        );

        // body_expr should be treated as Expr (ToTsExpr)
        assert!(
            code_str.contains("to_ts_expr"),
            "body_expr should use ToTsExpr. Generated code:\n{}", code_str
        );
    }
}
