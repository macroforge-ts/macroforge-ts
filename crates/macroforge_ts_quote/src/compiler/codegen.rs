//! Code generation from IR to Rust TokenStream.
//!
//! This module generates Rust code that produces TypeScript output at runtime.
//! It handles placeholder substitution, control flow, and directives.

use super::ir::{Ir, IrNode};
use super::semantic::PlaceholderKind;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

/// Configuration for code generation.
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Whether to generate module-level code (vs expression-level).
    pub module_level: bool,
    /// Variable name for the output accumulator.
    pub output_var: String,
    /// Variable name for the module (for typescript directives).
    pub module_var: String,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            module_level: true,
            output_var: "__mf_out".to_string(),
            module_var: "__mf_module".to_string(),
        }
    }
}

/// Code generator from IR.
pub struct Codegen {
    config: CodegenConfig,
}

impl Codegen {
    /// Creates a new code generator with default config.
    pub fn new() -> Self {
        Self {
            config: CodegenConfig::default(),
        }
    }

    /// Creates a new code generator with the given config.
    pub fn with_config(config: CodegenConfig) -> Self {
        Self { config }
    }

    /// Generates Rust TokenStream from IR.
    pub fn generate(&self, ir: &Ir) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let body = self.generate_nodes(&ir.nodes);

        if self.config.module_level {
            let module_var = format_ident!("{}", self.config.module_var);
            quote! {
                {
                    let mut #output_var = String::new();
                    let #module_var = &mut __mf_module;
                    #body
                    #output_var
                }
            }
        } else {
            quote! {
                {
                    let mut #output_var = String::new();
                    #body
                    #output_var
                }
            }
        }
    }

    /// Generates code for a sequence of IR nodes.
    fn generate_nodes(&self, nodes: &[IrNode]) -> TokenStream {
        let stmts: Vec<TokenStream> = nodes.iter().map(|n| self.generate_node(n)).collect();
        quote! { #(#stmts)* }
    }

    /// Generates code for a single IR node.
    fn generate_node(&self, node: &IrNode) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        match node {
            IrNode::Text(text) => {
                // Emit literal text
                quote! {
                    #output_var.push_str(#text);
                }
            }

            IrNode::Placeholder { kind, rust_expr, .. } => {
                self.generate_placeholder(*kind, rust_expr)
            }

            IrNode::IdentBlock { parts } => {
                self.generate_ident_block(parts)
            }

            IrNode::StringInterp { quote: q, parts } => {
                self.generate_string_interp(*q, parts)
            }

            IrNode::If { condition, then_body, else_if_branches, else_body } => {
                self.generate_if(condition, then_body, else_if_branches, else_body)
            }

            IrNode::For { pattern, iterator, body } => {
                self.generate_for(pattern, iterator, body)
            }

            IrNode::While { condition, body } => {
                self.generate_while(condition, body)
            }

            IrNode::Match { expr, arms } => {
                self.generate_match(expr, arms)
            }

            IrNode::Let { name, mutable, value } => {
                self.generate_let(name, *mutable, value)
            }

            IrNode::Do { code } => {
                self.generate_do(code)
            }

            IrNode::TypeScript { stream } => {
                self.generate_typescript(stream)
            }

            IrNode::Comment { text, is_doc } => {
                self.generate_comment(text, *is_doc)
            }
        }
    }

    /// Generates code for a placeholder based on its kind.
    fn generate_placeholder(&self, kind: PlaceholderKind, rust_expr: &str) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| {
            let ident = format_ident!("{}", rust_expr);
            quote! { #ident }
        });

        match kind {
            PlaceholderKind::Expr => {
                // Expression placeholder - use to_ts_expr trait
                quote! {
                    #output_var.push_str(&macroforge_ts_quote::ToTsExpr::to_ts_expr(&#expr));
                }
            }
            PlaceholderKind::Type => {
                // Type placeholder - use to_ts_type trait
                quote! {
                    #output_var.push_str(&macroforge_ts_quote::ToTsType::to_ts_type(&#expr));
                }
            }
            PlaceholderKind::Ident => {
                // Identifier placeholder - use to_ts_ident trait
                quote! {
                    #output_var.push_str(&macroforge_ts_quote::ToTsIdent::to_ts_ident(&#expr));
                }
            }
            PlaceholderKind::Stmt => {
                // Statement placeholder - use to_ts_stmt trait
                quote! {
                    #output_var.push_str(&macroforge_ts_quote::ToTsStmt::to_ts_stmt(&#expr));
                }
            }
        }
    }

    /// Generates code for an identifier block.
    fn generate_ident_block(&self, parts: &[IrNode]) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let part_stmts: Vec<TokenStream> = parts.iter().map(|p| {
            match p {
                IrNode::Text(text) => {
                    quote! { __ident_parts.push_str(#text); }
                }
                IrNode::Placeholder { rust_expr, .. } => {
                    let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| {
                        let ident = format_ident!("{}", rust_expr);
                        quote! { #ident }
                    });
                    quote! { __ident_parts.push_str(&(#expr).to_string()); }
                }
                _ => quote! {}
            }
        }).collect();

        quote! {
            {
                let mut __ident_parts = String::new();
                #(#part_stmts)*
                #output_var.push_str(&__ident_parts);
            }
        }
    }

    /// Generates code for string interpolation.
    fn generate_string_interp(&self, q: char, parts: &[IrNode]) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let quote_char = q.to_string();
        let part_stmts = self.generate_nodes(parts);

        quote! {
            #output_var.push_str(#quote_char);
            #part_stmts
            #output_var.push_str(#quote_char);
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

        let else_if_code: Vec<TokenStream> = else_if_branches.iter().map(|(cond, body)| {
            let c: TokenStream = cond.parse().unwrap_or_else(|_| quote! { true });
            let b = self.generate_nodes(body);
            quote! { else if #c { #b } }
        }).collect();

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
        let iter: TokenStream = iterator.parse().unwrap_or_else(|_| quote! { std::iter::empty::<()>() });
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
    fn generate_match(
        &self,
        expr: &str,
        arms: &[(String, Option<String>, Vec<IrNode>)],
    ) -> TokenStream {
        let e: TokenStream = expr.parse().unwrap_or_else(|_| quote! { () });

        let arm_code: Vec<TokenStream> = arms.iter().map(|(pattern, guard, body)| {
            let pat: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { _ });
            let body_code = self.generate_nodes(body);

            if let Some(g) = guard {
                let guard_expr: TokenStream = g.parse().unwrap_or_else(|_| quote! { true });
                quote! { #pat if #guard_expr => { #body_code } }
            } else {
                quote! { #pat => { #body_code } }
            }
        }).collect();

        quote! {
            match #e {
                #(#arm_code)*
            }
        }
    }

    /// Generates code for a let directive.
    fn generate_let(&self, name: &str, mutable: bool, value: &str) -> TokenStream {
        let ident = format_ident!("{}", name);
        let val: TokenStream = value.parse().unwrap_or_else(|_| quote! { () });

        let mut_token = if mutable {
            quote! { mut }
        } else {
            quote! {}
        };

        quote! {
            let #mut_token #ident = #val;
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
        let module_var = format_ident!("{}", self.config.module_var);
        let s: TokenStream = stream.parse().unwrap_or_else(|_| quote! { () });

        quote! {
            #module_var.extend(#s);
        }
    }

    /// Generates code for a comment (usually a no-op, but can emit debug info).
    fn generate_comment(&self, _text: &str, _is_doc: bool) -> TokenStream {
        // Comments are stripped from output by default
        quote! {}
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
    use crate::compiler::parser::Parser;
    use crate::compiler::semantic::analyze;
    use crate::compiler::ir::lower;
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
        let code = compile_template("hello world");
        let code_str = code.to_string();
        // Should contain push_str with the text
        assert!(code_str.contains("push_str"));
    }

    #[test]
    fn test_codegen_interpolation() {
        let code = compile_template("value = @{value}");
        let code_str = code.to_string();
        // Should contain ToTsExpr/ToTsStmt call (interpolations at statement level become Stmt)
        assert!(
            code_str.contains("to_ts_expr")
            || code_str.contains("ToTsExpr")
            || code_str.contains("to_ts_stmt")
            || code_str.contains("ToTsStmt"),
            "Generated code: {}", code_str
        );
    }

    #[test]
    fn test_codegen_type_placeholder() {
        let code = compile_template("const x: @{MyType} = 1");
        let code_str = code.to_string();
        // Should contain ToTsType call for the type placeholder
        assert!(code_str.contains("to_ts_type") || code_str.contains("ToTsType"));
    }

    #[test]
    fn test_codegen_for_loop() {
        let code = compile_template("{#for item in items}@{item}{/for}");
        let code_str = code.to_string();
        // Should contain a for loop
        assert!(code_str.contains("for"));
        assert!(code_str.contains("in"));
    }

    #[test]
    fn test_codegen_if_block() {
        let code = compile_template("{#if cond}yes{/if}");
        let code_str = code.to_string();
        // Should contain an if statement
        assert!(code_str.contains("if"));
    }

    #[test]
    fn test_codegen_generates_valid_rust() {
        // This test just verifies the output is parseable Rust
        let code = compile_template("const @{name}: @{T} = @{value}");
        // If this compiles without panicking, the output is syntactically valid
        let _parsed: syn::File = syn::parse2(quote! {
            fn test() {
                #code
            }
        }).expect("Generated code should be valid Rust");
    }
}
