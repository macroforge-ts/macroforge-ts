//! Code generation from IR to Rust TokenStream.
//!
//! This module generates Rust code that builds SWC AST at compile time.
//! It uses `macroforge_ts_quote::ts_quote!` for TypeScript parsing with native type support.
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

/// Parse context - determines what AST node type to parse as.
///
/// When template code is split by control flow, each chunk needs to know
/// what context it's in to parse correctly. For example, code inside an
/// object literal should parse as `PropOrSpread`, not `Stmt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParseContext {
    /// Top level module - expects ModuleItem (export, import, statements)
    #[default]
    Module,
    /// Function/method body - expects Stmt
    FunctionBody,
    /// Object literal `{ key: value }` - expects PropOrSpread
    ObjectLiteral,
    /// Array literal `[ elem, elem ]` - expects Expr (array element)
    ArrayLiteral,
    /// Class body - expects ClassMember
    ClassBody,
    /// Type object literal `{ prop: Type }` - expects TsPropertySignature
    TypeObjectLiteral,
}

impl ParseContext {
    /// Returns the ts_quote! type name for this context.
    fn quote_type(&self) -> &'static str {
        match self {
            ParseContext::Module => "ModuleItem",
            ParseContext::FunctionBody => "Stmt",
            ParseContext::ObjectLiteral => "PropOrSpread",
            ParseContext::ArrayLiteral => "Expr",
            ParseContext::ClassBody => "ModuleItem", // ClassMember wrapped in Stmt
            ParseContext::TypeObjectLiteral => "PropOrSpread", // Simplified
        }
    }

    /// Returns the output variable suffix for this context.
    fn output_suffix(&self) -> &'static str {
        match self {
            ParseContext::Module => "",
            ParseContext::FunctionBody => "",
            ParseContext::ObjectLiteral => "_props",
            ParseContext::ArrayLiteral => "_elems",
            ParseContext::ClassBody => "_members",
            ParseContext::TypeObjectLiteral => "_type_props",
        }
    }

    /// Returns true if this context produces ModuleItem directly.
    fn is_module_item(&self) -> bool {
        matches!(self, ParseContext::Module | ParseContext::ClassBody)
    }

    /// Returns true if this context produces statements.
    fn is_statement(&self) -> bool {
        matches!(self, ParseContext::FunctionBody)
    }

    /// Returns true if this context is a non-statement context that needs a collector.
    fn is_non_statement_context(&self) -> bool {
        matches!(
            self,
            ParseContext::ObjectLiteral
                | ParseContext::ArrayLiteral
                | ParseContext::ClassBody
                | ParseContext::TypeObjectLiteral
        )
    }

    /// Returns the collector type ID for this context.
    /// 0=ObjectLiteral, 1=ArrayLiteral, 2=ClassBody, 3=TypeObjectLiteral, _=Stmts
    fn collector_type(&self) -> u8 {
        match self {
            ParseContext::ObjectLiteral => 0,
            ParseContext::ArrayLiteral => 1,
            ParseContext::ClassBody => 2,
            ParseContext::TypeObjectLiteral => 3,
            ParseContext::Module | ParseContext::FunctionBody => 255,
        }
    }
}

/// Tracks context transitions in a template string.
///
/// Analyzes patterns like `return {`, `= [`, `class Foo {` to determine
/// what context each unclosed brace introduces.
#[derive(Debug, Clone)]
struct ContextAnalysis {
    /// The context that should be used for content after this chunk.
    /// This is the innermost unclosed context.
    inner_context: ParseContext,
    /// Stack of contexts introduced by unclosed braces in this chunk.
    /// Used to properly close contexts when matching `}` or `]` are found.
    context_stack: Vec<ParseContext>,
}

impl Default for ContextAnalysis {
    fn default() -> Self {
        Self {
            inner_context: ParseContext::Module,
            context_stack: Vec::new(),
        }
    }
}

impl ContextAnalysis {
    /// Analyzes a template string to determine context transitions.
    ///
    /// Detects patterns like:
    /// - `return {` or `= {` → ObjectLiteral
    /// - `[` → ArrayLiteral
    /// - `function foo() {` or `=> {` → FunctionBody
    /// - `class Foo {` → ClassBody
    fn analyze(template: &str, starting_context: ParseContext) -> Self {
        let mut context_stack: Vec<ParseContext> = vec![starting_context];
        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;

        let chars: Vec<char> = template.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            let ch = chars[i];

            if escape_next {
                escape_next = false;
                i += 1;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                i += 1;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                i += 1;
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => {
                    // Determine what kind of context this brace opens
                    let context = Self::classify_open_brace(&chars, i);
                    context_stack.push(context);
                }
                '}' => {
                    // Pop the context (but keep at least one)
                    if context_stack.len() > 1 {
                        context_stack.pop();
                    }
                }
                '[' => {
                    // Array literal context
                    context_stack.push(ParseContext::ArrayLiteral);
                }
                ']' => {
                    // Pop array context if that's what we're in
                    if context_stack.last() == Some(&ParseContext::ArrayLiteral) {
                        context_stack.pop();
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let inner_context = *context_stack.last().unwrap_or(&starting_context);

        Self {
            inner_context,
            context_stack,
        }
    }

    /// Classifies what context an opening brace `{` introduces.
    ///
    /// Looks at preceding tokens to determine:
    /// - ObjectLiteral: after `return`, `=`, `(`, `:`, `,`
    /// - FunctionBody: after `)` with function signature, or `=>`
    /// - ClassBody: after `class Name` or `extends Name`
    fn classify_open_brace(chars: &[char], brace_pos: usize) -> ParseContext {
        // Look backwards to find what precedes the brace
        let before: String = chars[..brace_pos].iter().collect();
        let trimmed = before.trim_end();

        // Check for arrow function: `=> {`
        if trimmed.ends_with("=>") {
            return ParseContext::FunctionBody;
        }

        // Check for function/method: `) {` after a parameter list
        if trimmed.ends_with(')') {
            // Could be function, method, if, for, while, etc.
            // Look for function-like keywords
            let lower = trimmed.to_lowercase();
            if lower.contains("function ")
                || lower.contains("function(")
                || Self::looks_like_method_signature(trimmed)
            {
                return ParseContext::FunctionBody;
            }
            // Control flow statements also use FunctionBody context
            if lower.ends_with("if") || lower.ends_with("for") || lower.ends_with("while") {
                return ParseContext::FunctionBody;
            }
            // Default: assume it's a block statement
            return ParseContext::FunctionBody;
        }

        // Check for class: `class Name {` or `extends Name {`
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        if words.len() >= 2 {
            let second_last = words.get(words.len().saturating_sub(2));
            if second_last == Some(&"class") || second_last == Some(&"extends") || second_last == Some(&"implements") {
                return ParseContext::ClassBody;
            }
        }

        // Check for object literal patterns
        let last_non_ws = trimmed.chars().last();
        match last_non_ws {
            // After these, `{` is an object literal
            Some('=' | ':' | ',' | '(' | '[' | '?') => ParseContext::ObjectLiteral,
            // `return {` is object literal
            _ if trimmed.ends_with("return") => ParseContext::ObjectLiteral,
            // `yield {` is object literal
            _ if trimmed.ends_with("yield") => ParseContext::ObjectLiteral,
            // Default: assume function body (block statement)
            _ => ParseContext::FunctionBody,
        }
    }

    /// Checks if the string looks like a method signature.
    fn looks_like_method_signature(s: &str) -> bool {
        // Simple heuristic: contains a method name followed by (
        // e.g., `foo()`, `getValue()`, `async process()`
        let trimmed = s.trim();
        if let Some(paren_pos) = trimmed.rfind('(') {
            let before_paren = &trimmed[..paren_pos];
            let last_word = before_paren.split_whitespace().last().unwrap_or("");
            // If last word before ( is an identifier, it's likely a method
            !last_word.is_empty() && last_word.chars().all(|c| c.is_alphanumeric() || c == '_')
        } else {
            false
        }
    }
}

/// Configuration for code generation.
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Variable name for the output accumulator (Vec<ModuleItem>).
    pub output_var: String,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            output_var: "__mf_stmts".to_string(),
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
    /// Can be compiled with macroforge_ts_quote::ts_quote!.
    Parseable {
        /// Template string with $placeholder markers.
        template: String,
        /// Placeholder info: (placeholder_name, kind, rust_expr)
        placeholders: Vec<(String, PlaceholderKind, String)>,
        /// Brace balance for virtual completion.
        brace_balance: BraceBalance,
        /// The parse context for this chunk (what AST type to parse as).
        parse_context: ParseContext,
        /// Context for content that follows (after unclosed braces).
        inner_context: ParseContext,
    },
    /// Control flow node that needs special handling.
    /// Includes the context that should be used for the loop/if body.
    ControlFlow {
        node: &'a IrNode,
        /// The context inherited from the preceding chunk.
        body_context: ParseContext,
    },
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
        format!("MfPh{}", n)
    }

    /// Generates Rust TokenStream from IR.
    ///
    /// The generated code builds `Vec<ModuleItem>` at compile time using
    /// `macroforge_ts_quote::ts_quote!` for static TypeScript and ToTs* traits for placeholders.
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

        // Check if any opener has non-statement inner context (needs context collector)
        let needs_context_collector = chunks.iter().any(|c| match c {
            Chunk::Parseable { brace_balance, inner_context, .. } => {
                brace_balance.unclosed_opens > 0 && matches!(
                    inner_context,
                    ParseContext::ObjectLiteral | ParseContext::ArrayLiteral |
                    ParseContext::ClassBody | ParseContext::TypeObjectLiteral
                )
            }
            _ => false,
        });

        let stmts: Vec<TokenStream> = chunks.iter().map(|c| self.generate_chunk(c)).collect();

        match (has_unbalanced, needs_context_collector) {
            (true, true) => {
                // Need both opener stack and context collector
                quote! {
                    let mut __mf_opener_stack: Vec<usize> = Vec::new();
                    let mut __mf_context_collector = macroforge_ts_syn::__internal::ContextCollector::for_object();
                    #(#stmts)*
                }
            }
            (true, false) => {
                // Only need opener stack (statement-level virtual completion)
                quote! {
                    let mut __mf_opener_stack: Vec<usize> = Vec::new();
                    #(#stmts)*
                }
            }
            _ => {
                quote! { #(#stmts)* }
            }
        }
    }

    /// Groups consecutive nodes into chunks that can be processed together.
    ///
    /// Tracks parse context through the chunks so that content inside object literals,
    /// arrays, etc. is parsed with the correct AST node type.
    fn chunk_nodes<'a>(&self, nodes: &'a [IrNode]) -> Vec<Chunk<'a>> {
        self.chunk_nodes_with_context(nodes, ParseContext::Module)
    }

    /// Groups nodes into chunks with explicit starting context.
    fn chunk_nodes_with_context<'a>(
        &self,
        nodes: &'a [IrNode],
        starting_context: ParseContext,
    ) -> Vec<Chunk<'a>> {
        let mut chunks = Vec::new();
        let mut current_template = String::new();
        let mut current_placeholders: Vec<(String, PlaceholderKind, String)> = Vec::new();
        let mut current_context = starting_context;

        for node in nodes {
            match node {
                IrNode::Text(text) => {
                    current_template.push_str(text);
                }

                IrNode::Placeholder { kind, rust_expr } => {
                    let ph_name = self.next_placeholder_name();
                    // Use $ prefix for ALL placeholders - SWC quote! will substitute them
                    // For Type placeholders, we provide an Ident binding with the marker name,
                    // then replace the Ident in type positions with actual TsType after parsing
                    current_template.push('$');
                    current_template.push_str(&ph_name);
                    current_placeholders.push((ph_name, *kind, rust_expr.clone()));
                }

                IrNode::If { .. }
                | IrNode::For { .. }
                | IrNode::While { .. }
                | IrNode::Match { .. } => {
                    // Flush pending template and get the inner context
                    let inner_ctx = self.flush_parseable_with_context(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                        current_context,
                    );
                    // Control flow body uses the inner context from the preceding chunk
                    chunks.push(Chunk::ControlFlow {
                        node,
                        body_context: inner_ctx,
                    });
                    // After control flow, we're back to the same context level
                    current_context = inner_ctx;
                }

                IrNode::Let { .. } | IrNode::Do { .. } | IrNode::TypeScript { .. } => {
                    let inner_ctx = self.flush_parseable_with_context(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                        current_context,
                    );
                    chunks.push(Chunk::Directive(node));
                    current_context = inner_ctx;
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
                        // Convert to inline template literal with placeholders
                        // This keeps the string inline in the template rather than splitting
                        // the expression into separate chunks
                        current_template.push('`');
                        for part in parts {
                            match part {
                                IrNode::Text(text) => {
                                    // Escape backticks and ${} in static parts
                                    let escaped = text.replace('`', "\\`").replace("${", "\\${");
                                    current_template.push_str(&escaped);
                                }
                                IrNode::Placeholder { kind, rust_expr } => {
                                    // Add placeholder inline using template literal syntax
                                    let ph_name = self.next_placeholder_name();
                                    current_template.push_str("${");
                                    current_template.push_str(&ph_name);
                                    current_template.push('}');
                                    current_placeholders.push((ph_name, *kind, rust_expr.clone()));
                                }
                                _ => {}
                            }
                        }
                        current_template.push('`');
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
                    let inner_ctx = self.flush_parseable_with_context(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                        current_context,
                    );
                    current_context = inner_ctx;

                    // Add a comment chunk
                    chunks.push(Chunk::Comment {
                        text: text.clone(),
                    });
                }
            }
        }

        // Flush remaining
        self.flush_parseable_with_context(
            &mut chunks,
            &mut current_template,
            &mut current_placeholders,
            current_context,
        );

        chunks
    }

    /// Flushes pending template content into a Parseable chunk with context tracking.
    ///
    /// Analyzes the template to determine context transitions and returns the inner
    /// context that should be used for subsequent chunks.
    fn flush_parseable_with_context(
        &self,
        chunks: &mut Vec<Chunk>,
        current_template: &mut String,
        current_placeholders: &mut Vec<(String, PlaceholderKind, String)>,
        current_context: ParseContext,
    ) -> ParseContext {
        let template = std::mem::take(current_template);
        let placeholders = std::mem::take(current_placeholders);

        if template.trim().is_empty() && placeholders.is_empty() {
            return current_context;
        }

        let brace_balance = BraceBalance::analyze(&template);
        let context_analysis = ContextAnalysis::analyze(&template, current_context);

        chunks.push(Chunk::Parseable {
            template,
            placeholders,
            brace_balance,
            parse_context: current_context,
            inner_context: context_analysis.inner_context,
        });

        context_analysis.inner_context
    }

    /// Generates code for a single chunk.
    fn generate_chunk(&self, chunk: &Chunk) -> TokenStream {
        match chunk {
            Chunk::Parseable {
                template,
                placeholders,
                brace_balance,
                parse_context,
                inner_context: _,
            } => self.generate_parseable_chunk(template, placeholders, *brace_balance, *parse_context),

            Chunk::ControlFlow { node, body_context } => {
                self.generate_control_flow_with_context(node, *body_context)
            }

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
    /// Uses macroforge_ts_quote::ts_quote! to build AST at compile time with ToTs* traits
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
    /// ts_quote! natively supports TsType placeholders via `$name: TsType = expr` syntax.
    ///
    /// ## Context-Aware Parsing
    ///
    /// The `parse_context` determines what AST node type to parse as:
    /// - Module: ModuleItem (exports, imports, top-level)
    /// - FunctionBody: Stmt
    /// - ObjectLiteral: PropOrSpread
    /// - ArrayLiteral: Expr (element)
    /// - ClassBody: ClassMember
    fn generate_parseable_chunk(
        &self,
        template: &str,
        placeholders: &[(String, PlaceholderKind, String)],
        brace_balance: BraceBalance,
        parse_context: ParseContext,
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        // Skip empty templates
        if template.trim().is_empty() && placeholders.is_empty() {
            return quote! {};
        }

        // Generate placeholder bindings (convert Rust exprs to SWC AST nodes)
        let (binding_stmts, quote_bindings) = self.generate_placeholder_bindings(placeholders);

        // Debug: print the template being passed to ts_quote!
        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!(
                "[MF_DEBUG_CODEGEN] Template for ts_quote!: {:?} (balance: {:?}, context: {:?})",
                template, brace_balance, parse_context
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

        // Handle unbalanced braces with virtual completion
        if !brace_balance.is_balanced() {
            return self.generate_virtually_completed_chunk(
                template,
                &binding_stmts,
                &quote_bindings,
                brace_balance,
                is_module_decl,
                parse_context,
            );
        }

        // Normal case: balanced braces - use context-aware parsing
        let template_lit = syn::LitStr::new(template, proc_macro2::Span::call_site());

        match parse_context {
            ParseContext::ObjectLiteral => {
                // For balanced chunks in ObjectLiteral context (e.g., properties inside a for-loop),
                // wrap in a virtual object, parse, extract the property, and push to collector.
                let wrapped_template = format!("({{ {} }})", template);
                let wrapped_lit = syn::LitStr::new(&wrapped_template, proc_macro2::Span::call_site());

                #[cfg(debug_assertions)]
                if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
                    eprintln!(
                        "[MF_DEBUG_CODEGEN] ObjectLiteral balanced chunk - collecting property: {:?}",
                        wrapped_template
                    );
                }

                let quote_call = if quote_bindings.is_empty() {
                    quote! {
                        macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr)
                    }
                } else {
                    quote! {
                        macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr, #(#quote_bindings),*)
                    }
                };

                // Parse the property and push to the context collector
                quote! {
                    {
                        #(#binding_stmts)*
                        let __mf_wrapped_expr: swc_core::ecma::ast::Expr = #quote_call;
                        if let Some(__mf_prop) = macroforge_ts_syn::__internal::extract_prop_from_wrapped_expr(&__mf_wrapped_expr) {
                            macroforge_ts_syn::__internal::push_object_prop(&mut __mf_context_collector, __mf_prop);
                        }
                    }
                }
            }

            ParseContext::ArrayLiteral => {
                // For balanced chunks in ArrayLiteral context (e.g., elements inside a for-loop),
                // wrap in array, parse, extract the element, and push to collector.
                let wrapped_template = format!("[{}]", template);
                let wrapped_lit = syn::LitStr::new(&wrapped_template, proc_macro2::Span::call_site());

                #[cfg(debug_assertions)]
                if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
                    eprintln!(
                        "[MF_DEBUG_CODEGEN] ArrayLiteral balanced chunk - collecting element: {:?}",
                        wrapped_template
                    );
                }

                let quote_call = if quote_bindings.is_empty() {
                    quote! {
                        macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr)
                    }
                } else {
                    quote! {
                        macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr, #(#quote_bindings),*)
                    }
                };

                // Parse the element and push to the context collector
                quote! {
                    {
                        #(#binding_stmts)*
                        let __mf_wrapped_expr: swc_core::ecma::ast::Expr = #quote_call;
                        if let Some(__mf_elem) = macroforge_ts_syn::__internal::extract_elem_from_wrapped_expr(&__mf_wrapped_expr) {
                            macroforge_ts_syn::__internal::push_array_elem(&mut __mf_context_collector, __mf_elem);
                        }
                    }
                }
            }

            ParseContext::Module | ParseContext::ClassBody => {
                if is_module_decl {
                    // Parse as ModuleItem for export/import declarations
                    let quote_call = if quote_bindings.is_empty() {
                        quote! {
                            macroforge_ts_quote::ts_quote!(#template_lit as ModuleItem)
                        }
                    } else {
                        quote! {
                            macroforge_ts_quote::ts_quote!(#template_lit as ModuleItem, #(#quote_bindings),*)
                        }
                    };

                    quote! {
                        {
                            #(#binding_stmts)*
                            #output_var.push(#quote_call);
                        }
                    }
                } else {
                    // Parse as Stmt for regular statements
                    let quote_call = if quote_bindings.is_empty() {
                        quote! {
                            macroforge_ts_quote::ts_quote!(#template_lit as Stmt)
                        }
                    } else {
                        quote! {
                            macroforge_ts_quote::ts_quote!(#template_lit as Stmt, #(#quote_bindings),*)
                        }
                    };

                    quote! {
                        {
                            #(#binding_stmts)*
                            #output_var.push(swc_core::ecma::ast::ModuleItem::Stmt(#quote_call));
                        }
                    }
                }
            }

            ParseContext::FunctionBody => {
                // Parse as Stmt for function body
                let quote_call = if quote_bindings.is_empty() {
                    quote! {
                        macroforge_ts_quote::ts_quote!(#template_lit as Stmt)
                    }
                } else {
                    quote! {
                        macroforge_ts_quote::ts_quote!(#template_lit as Stmt, #(#quote_bindings),*)
                    }
                };

                quote! {
                    {
                        #(#binding_stmts)*
                        #output_var.push(swc_core::ecma::ast::ModuleItem::Stmt(#quote_call));
                    }
                }
            }

            ParseContext::TypeObjectLiteral => {
                // TypeScript type literals - treat like ObjectLiteral for now
                // TODO: Add proper TsPropertySignature support
                let wrapped_template = format!("({{ {} }})", template);
                let wrapped_lit = syn::LitStr::new(&wrapped_template, proc_macro2::Span::call_site());

                let quote_call = if quote_bindings.is_empty() {
                    quote! {
                        macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr)
                    }
                } else {
                    quote! {
                        macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr, #(#quote_bindings),*)
                    }
                };

                quote! {
                    {
                        #(#binding_stmts)*
                        let _ = #quote_call;
                        // TODO: Collect and merge type properties with opener's type literal
                    }
                }
            }
        }
    }

    /// Generates placeholder binding statements and quote bindings.
    ///
    /// The quote bindings include type annotations because `ts_quote!` requires
    /// them to know what AST node type to expect in each position.
    ///
    /// Returns:
    /// - binding_stmts: Let statements that convert Rust values to SWC AST nodes
    /// - quote_bindings: Bindings for ts_quote! macro
    fn generate_placeholder_bindings(
        &self,
        placeholders: &[(String, PlaceholderKind, String)],
    ) -> (Vec<TokenStream>, Vec<TokenStream>) {
        let mut binding_stmts = Vec::new();
        let mut quote_bindings = Vec::new();

        for (ph_name, kind, rust_expr) in placeholders {
            let ph_ident = format_ident!("{}", ph_name);
            let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| {
                let ident = format_ident!("{}", rust_expr);
                quote! { #ident }
            });

            // ts_quote! supports Ident, Expr, Pat, and TsType placeholder types natively.
            match kind {
                PlaceholderKind::Expr => {
                    binding_stmts.push(quote! {
                        let #ph_ident: swc_core::ecma::ast::Expr = macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: Expr = #ph_ident });
                }
                PlaceholderKind::Type => {
                    // ts_quote! natively supports TsType placeholders
                    binding_stmts.push(quote! {
                        let #ph_ident: swc_core::ecma::ast::TsType = macroforge_ts_syn::ToTsType::to_ts_type(&#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: TsType = #ph_ident });
                }
                PlaceholderKind::Ident => {
                    binding_stmts.push(quote! {
                        let #ph_ident: swc_core::ecma::ast::Ident = macroforge_ts_syn::ToTsIdent::to_ts_ident(&#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: Ident = #ph_ident });
                }
                PlaceholderKind::Stmt => {
                    // ts_quote! supports Stmt directly
                    binding_stmts.push(quote! {
                        let #ph_ident: swc_core::ecma::ast::Stmt = macroforge_ts_syn::ToTsStmt::to_ts_stmt(#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: Stmt = #ph_ident });
                }
            }
        }

        (binding_stmts, quote_bindings)
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
        brace_balance: BraceBalance,
        _is_module_decl: bool,
        _parse_context: ParseContext,
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        // Determine the virtual completion strategy
        if brace_balance.unclosed_opens > 0 && brace_balance.unmatched_closes == 0 {
            // Case: Template opens blocks but doesn't close them
            // e.g., "export function foo(): void { const x = 1;"
            // Strategy: Add virtual closing braces, parse as ModuleItem, push and track index

            let virtual_closes = "}".repeat(brace_balance.unclosed_opens as usize);
            let completed_template = format!("{}{}", template, virtual_closes);

            #[cfg(debug_assertions)]
            if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
                eprintln!(
                    "[MF_DEBUG_CODEGEN] Virtually completed opener template: {:?}",
                    completed_template
                );
            }

            let completed_lit =
                syn::LitStr::new(&completed_template, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem)
                }
            } else {
                quote! {
                    macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            let unclosed = brace_balance.unclosed_opens as usize;

            // Push the opener and track its index on the stack
            quote! {
                {
                    #(#binding_stmts)*
                    // Parse the virtually completed template
                    let __mf_item: swc_core::ecma::ast::ModuleItem = #quote_call;

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
            // e.g., "return x; }" or "} as Type; }"
            // Strategy: Wrap in virtual function opening, parse, finalize the opener
            //
            // Context-aware: If we're in ObjectLiteral context, the first `}` closes
            // an object literal, so we need to open with `return { __dummy: 0` not just `{`

            let completed_template = match _parse_context {
                ParseContext::ObjectLiteral => {
                    // First close is an object literal, subsequent closes are blocks/function
                    // For 2 closes: `} as X; }` → object close + function close
                    // Virtual: `function __mf_virtual() { return { __mf_dummy: 0 } as X; }`
                    //
                    // For n>2 closes, add extra block opens before the return
                    let extra_opens = if brace_balance.unmatched_closes > 2 {
                        "{".repeat((brace_balance.unmatched_closes - 2) as usize)
                    } else {
                        String::new()
                    };
                    format!("function __mf_virtual() {{ {}return {{ __mf_dummy: 0 {}", extra_opens, template)
                }
                ParseContext::ArrayLiteral => {
                    // First close is an array literal
                    let extra_opens = if brace_balance.unmatched_closes > 2 {
                        "{".repeat((brace_balance.unmatched_closes - 2) as usize)
                    } else {
                        String::new()
                    };
                    format!("function __mf_virtual() {{ {}return [ 0 {}", extra_opens, template)
                }
                _ => {
                    // Standard block context
                    let extra_opens = "{".repeat((brace_balance.unmatched_closes - 1).max(0) as usize);
                    format!("function __mf_virtual() {{ {}{}", extra_opens, template)
                }
            };

            #[cfg(debug_assertions)]
            if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
                eprintln!(
                    "[MF_DEBUG_CODEGEN] Virtually completed closer template (context={:?}): {:?}",
                    _parse_context, completed_template
                );
            }

            let completed_lit =
                syn::LitStr::new(&completed_template, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem)
                }
            } else {
                quote! {
                    macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            // Pop the opener index and finalize, merging any collected items
            let merge_code = match _parse_context {
                ParseContext::ObjectLiteral => {
                    quote! {
                        // Merge collected object properties into the opener
                        if let macroforge_ts_syn::__internal::ContextCollector::ObjectProps(props) =
                            std::mem::replace(&mut __mf_context_collector, macroforge_ts_syn::__internal::ContextCollector::for_object())
                        {
                            if let Some(opener) = #output_var.get_mut(__mf_opener_idx) {
                                macroforge_ts_syn::__internal::merge_object_props(opener, props);
                            }
                        }
                    }
                }
                ParseContext::ArrayLiteral => {
                    quote! {
                        // Merge collected array elements into the opener
                        if let macroforge_ts_syn::__internal::ContextCollector::ArrayElems(elems) =
                            std::mem::replace(&mut __mf_context_collector, macroforge_ts_syn::__internal::ContextCollector::for_array())
                        {
                            if let Some(opener) = #output_var.get_mut(__mf_opener_idx) {
                                macroforge_ts_syn::__internal::merge_array_elems(opener, elems);
                            }
                        }
                    }
                }
                ParseContext::ClassBody => {
                    quote! {
                        // Merge collected class members into the opener
                        if let macroforge_ts_syn::__internal::ContextCollector::ClassMembers(members) =
                            std::mem::replace(&mut __mf_context_collector, macroforge_ts_syn::__internal::ContextCollector::for_class())
                        {
                            if let Some(opener) = #output_var.get_mut(__mf_opener_idx) {
                                macroforge_ts_syn::__internal::merge_class_members(opener, members);
                            }
                        }
                    }
                }
                ParseContext::TypeObjectLiteral => {
                    quote! {
                        // Merge collected type properties into the opener
                        if let macroforge_ts_syn::__internal::ContextCollector::TypeProps(props) =
                            std::mem::replace(&mut __mf_context_collector, macroforge_ts_syn::__internal::ContextCollector::for_type_object())
                        {
                            if let Some(opener) = #output_var.get_mut(__mf_opener_idx) {
                                macroforge_ts_syn::__internal::merge_type_props(opener, props);
                            }
                        }
                    }
                }
                _ => quote! {},
            };

            quote! {
                {
                    #(#binding_stmts)*
                    // Parse the virtually completed template
                    let __mf_item: swc_core::ecma::ast::ModuleItem = #quote_call;

                    // Pop the opener index and finalize with accumulated statements
                    let __mf_opener_idx = __mf_opener_stack.pop()
                        .expect("Virtual completion: no matching opener for closer");

                    // Merge collected items (if any) into the opener
                    #merge_code

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
            //
            // Context-aware for ObjectLiteral: opening close is object literal

            let (virtual_opens, virtual_closes) = match _parse_context {
                ParseContext::ObjectLiteral => {
                    // First close is object literal
                    let extra_opens = if brace_balance.unmatched_closes > 1 {
                        "{".repeat((brace_balance.unmatched_closes - 1) as usize)
                    } else {
                        String::new()
                    };
                    let opens = format!("return {{ __mf_dummy: 0 {}", extra_opens);
                    let closes = "}".repeat(brace_balance.unclosed_opens as usize);
                    (opens, closes)
                }
                ParseContext::ArrayLiteral => {
                    let extra_opens = if brace_balance.unmatched_closes > 1 {
                        "{".repeat((brace_balance.unmatched_closes - 1) as usize)
                    } else {
                        String::new()
                    };
                    let opens = format!("return [ 0 {}", extra_opens);
                    let closes = "}".repeat(brace_balance.unclosed_opens as usize);
                    (opens, closes)
                }
                _ => {
                    let opens = "{".repeat(brace_balance.unmatched_closes as usize);
                    let closes = "}".repeat(brace_balance.unclosed_opens as usize);
                    (opens, closes)
                }
            };

            let completed_template = format!(
                "function __mf_virtual() {{ {}{}{} }}",
                virtual_opens, template, virtual_closes
            );

            #[cfg(debug_assertions)]
            if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
                eprintln!(
                    "[MF_DEBUG_CODEGEN] Virtually completed middle template (context={:?}): {:?}",
                    _parse_context, completed_template
                );
            }

            let completed_lit =
                syn::LitStr::new(&completed_template, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem)
                }
            } else {
                quote! {
                    macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            // For middle chunks, peek the opener index (don't pop - we're still in the same block)
            quote! {
                {
                    #(#binding_stmts)*
                    // Parse the virtually completed template for validation
                    let __mf_item: swc_core::ecma::ast::ModuleItem = #quote_call;

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
        self.generate_control_flow_with_context(node, ParseContext::Module)
    }

    /// Generates code for control flow nodes with a specific body context.
    ///
    /// The body_context determines how the body content should be parsed:
    /// - ObjectLiteral: body generates PropOrSpread items
    /// - ArrayLiteral: body generates Expr items
    /// - FunctionBody/Module: body generates Stmt/ModuleItem
    fn generate_control_flow_with_context(
        &self,
        node: &IrNode,
        body_context: ParseContext,
    ) -> TokenStream {
        match node {
            IrNode::If {
                condition,
                then_body,
                else_if_branches,
                else_body,
            } => self.generate_if_with_context(
                condition,
                then_body,
                else_if_branches,
                else_body,
                body_context,
            ),

            IrNode::For {
                pattern,
                iterator,
                body,
            } => self.generate_for_with_context(pattern, iterator, body, body_context),

            IrNode::While { condition, body } => {
                self.generate_while_with_context(condition, body, body_context)
            }

            IrNode::Match { expr, arms } => self.generate_match_with_context(expr, arms, body_context),

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
        self.generate_match_with_context(expr, arms, ParseContext::Module)
    }

    /// Generates code for an if statement with a specific body context.
    fn generate_if_with_context(
        &self,
        condition: &str,
        then_body: &[IrNode],
        else_if_branches: &[(String, Vec<IrNode>)],
        else_body: &Option<Vec<IrNode>>,
        body_context: ParseContext,
    ) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { true });
        let then_code = self.generate_nodes_with_context(then_body, body_context);

        let else_if_code: Vec<TokenStream> = else_if_branches
            .iter()
            .map(|(cond, body)| {
                let c: TokenStream = cond.parse().unwrap_or_else(|_| quote! { true });
                let b = self.generate_nodes_with_context(body, body_context);
                quote! { else if #c { #b } }
            })
            .collect();

        let else_code = else_body.as_ref().map(|body| {
            let b = self.generate_nodes_with_context(body, body_context);
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

    /// Generates code for a for loop with a specific body context.
    fn generate_for_with_context(
        &self,
        pattern: &str,
        iterator: &str,
        body: &[IrNode],
        body_context: ParseContext,
    ) -> TokenStream {
        let pat: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { _ });
        let iter: TokenStream =
            iterator.parse().unwrap_or_else(|_| quote! { std::iter::empty::<()>() });
        let body_code = self.generate_nodes_with_context(body, body_context);

        quote! {
            for #pat in #iter {
                #body_code
            }
        }
    }

    /// Generates code for a while loop with a specific body context.
    fn generate_while_with_context(
        &self,
        condition: &str,
        body: &[IrNode],
        body_context: ParseContext,
    ) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { false });
        let body_code = self.generate_nodes_with_context(body, body_context);

        quote! {
            while #cond {
                #body_code
            }
        }
    }

    /// Generates code for a match expression with a specific body context.
    fn generate_match_with_context(
        &self,
        expr: &str,
        arms: &[(String, Option<String>, Vec<IrNode>)],
        body_context: ParseContext,
    ) -> TokenStream {
        let e: TokenStream = expr.parse().unwrap_or_else(|_| quote! { () });

        let arm_code: Vec<TokenStream> = arms
            .iter()
            .map(|(pattern, guard, body)| {
                let pat: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { _ });
                let body_code = self.generate_nodes_with_context(body, body_context);

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

    /// Generates code for a sequence of IR nodes with a specific starting context.
    fn generate_nodes_with_context(&self, nodes: &[IrNode], context: ParseContext) -> TokenStream {
        let chunks = self.chunk_nodes_with_context(nodes, context);

        // Check if we have any unbalanced chunks that need opener/closer tracking
        let has_unbalanced = chunks.iter().any(|c| match c {
            Chunk::Parseable { brace_balance, .. } => !brace_balance.is_balanced(),
            _ => false,
        });

        // Check if any opener has non-statement inner context (needs context collector)
        let needs_context_collector = chunks.iter().any(|c| match c {
            Chunk::Parseable { brace_balance, inner_context, .. } => {
                brace_balance.unclosed_opens > 0 && matches!(
                    inner_context,
                    ParseContext::ObjectLiteral | ParseContext::ArrayLiteral |
                    ParseContext::ClassBody | ParseContext::TypeObjectLiteral
                )
            }
            _ => false,
        });

        let stmts: Vec<TokenStream> = chunks.iter().map(|c| self.generate_chunk(c)).collect();

        match (has_unbalanced, needs_context_collector) {
            (true, true) => {
                // Need both opener stack and context collector
                quote! {
                    let mut __mf_opener_stack: Vec<usize> = Vec::new();
                    let mut __mf_context_collector = macroforge_ts_syn::__internal::ContextCollector::for_object();
                    #(#stmts)*
                }
            }
            (true, false) => {
                // Only need opener stack (statement-level virtual completion)
                quote! {
                    let mut __mf_opener_stack: Vec<usize> = Vec::new();
                    #(#stmts)*
                }
            }
            _ => {
                // No unbalanced chunks - just generate the statements
                // But we might still need the collector for balanced non-statement chunks
                if context.is_non_statement_context() {
                    let collector_type = context.collector_type();
                    quote! {
                        let mut __mf_context_collector = macroforge_ts_syn::__internal::ContextCollector::new(#collector_type);
                        #(#stmts)*
                    }
                } else {
                    quote! { #(#stmts)* }
                }
            }
        }
    }

    /// Generates a Rust expression string that builds an identifier from parts.
    ///
    /// This is used to embed ident blocks as placeholders in macroforge_ts_quote::ts_quote!.
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
        // Should use macroforge_ts_quote::ts_quote! for static text
        assert!(
            code_str.contains("macroforge_ts_quote :: ts_quote !"),
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
        // ts_quote! handles TsType placeholders natively via $name: TsType = expr
        assert!(
            code_str.contains("TsType ="),
            "Expected native TsType placeholder binding. Generated code: {}",
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
