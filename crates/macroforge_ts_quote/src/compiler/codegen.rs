//! Code generation from IR to Rust TokenStream.
//!
//! This module generates Rust code that builds SWC AST at compile time.
//! It performs a simple tree walk over the IR, generating code for each node.

use super::ir::{Ir, IrNode, PlaceholderKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Configuration for code generation.
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Variable name for the output Vec<ModuleItem>.
    pub output_var: String,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            output_var: "__stmts".to_string(),
        }
    }
}

/// Code generator for IR.
pub struct Codegen {
    config: CodegenConfig,
    /// Counter for generating unique placeholder names.
    placeholder_counter: std::cell::Cell<usize>,
}

impl Codegen {
    /// Creates a new code generator with default config.
    pub fn new() -> Self {
        Self::with_config(CodegenConfig::default())
    }

    /// Creates a new code generator with the given config.
    pub fn with_config(config: CodegenConfig) -> Self {
        Self {
            config,
            placeholder_counter: std::cell::Cell::new(0),
        }
    }

    /// Generates a unique placeholder name.
    fn next_placeholder(&self) -> syn::Ident {
        let id = self.placeholder_counter.get();
        self.placeholder_counter.set(id + 1);
        format_ident!("__mf_ph{}", id)
    }

    /// Generates Rust TokenStream from IR.
    pub fn generate(&self, ir: &Ir) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let body = self.generate_nodes(&ir.nodes);

        quote! {
            {
                let mut #output_var: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = Vec::new();
                #body
                #output_var
            }
        }
    }

    /// Generates code for a sequence of IR nodes.
    fn generate_nodes(&self, nodes: &[IrNode]) -> TokenStream {
        let stmts: Vec<TokenStream> = nodes.iter().filter_map(|n| self.generate_node(n)).collect();
        quote! { #(#stmts)* }
    }

    /// Generates code for a single IR node.
    fn generate_node(&self, node: &IrNode) -> Option<TokenStream> {
        let output_var = format_ident!("{}", self.config.output_var);

        match node {
            IrNode::Text(text) => {
                if text.trim().is_empty() {
                    return None;
                }
                Some(self.generate_text(text))
            }

            IrNode::Placeholder { kind, rust_expr } => {
                Some(self.generate_placeholder(*kind, rust_expr))
            }

            IrNode::IdentBlock { parts } => Some(self.generate_ident_block(parts)),

            IrNode::StringInterp { quote: q, parts } => {
                Some(self.generate_string_interp(*q, parts))
            }

            IrNode::If {
                condition,
                then_body,
                else_if_branches,
                else_body,
            } => Some(self.generate_if(condition, then_body, else_if_branches, else_body)),

            IrNode::For {
                pattern,
                iterator,
                body,
            } => Some(self.generate_for(pattern, iterator, body)),

            IrNode::While { condition, body } => Some(self.generate_while(condition, body)),

            IrNode::Match { expr, arms } => Some(self.generate_match(expr, arms)),

            IrNode::Let {
                name,
                mutable,
                type_hint,
                value,
            } => Some(self.generate_let(name, *mutable, type_hint.as_deref(), value)),

            IrNode::Do { code } => Some(self.generate_do(code)),

            IrNode::TypeScript { stream } => Some(self.generate_typescript(stream, &output_var)),

            IrNode::Comment { .. } => None, // Skip comments
        }
    }

    /// Generates code for static text using ts_quote!
    fn generate_text(&self, text: &str) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        quote! {
            #output_var.extend(
                macroforge_ts::macroforge_ts_quote::ts_quote!(#text)
            );
        }
    }

    /// Generates code for a placeholder.
    fn generate_placeholder(&self, kind: PlaceholderKind, rust_expr: &str) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let ph_ident = self.next_placeholder();
        let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| quote! { #rust_expr });

        match kind {
            PlaceholderKind::Expr => {
                quote! {
                    let #ph_ident: macroforge_ts::swc_core::ecma::ast::Expr =
                        macroforge_ts::macroforge_ts_syn::ToTsExpr::to_ts_expr((#expr).clone());
                    #output_var.extend(
                        macroforge_ts::macroforge_ts_quote::ts_quote!($ph: Expr = #ph_ident; $ph)
                    );
                }
            }
            PlaceholderKind::Type => {
                quote! {
                    let #ph_ident: macroforge_ts::swc_core::ecma::ast::TsType =
                        macroforge_ts::macroforge_ts_syn::ToTsType::to_ts_type(&#expr);
                    #output_var.extend(
                        macroforge_ts::macroforge_ts_quote::ts_quote!($ph: TsType = #ph_ident; const _: $ph = undefined)
                    );
                }
            }
            PlaceholderKind::Ident => {
                quote! {
                    let #ph_ident: macroforge_ts::swc_core::ecma::ast::Ident =
                        macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(&#expr);
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(#ph_ident)),
                            }
                        )
                    ));
                }
            }
            PlaceholderKind::Stmt => {
                quote! {
                    let #ph_ident: macroforge_ts::swc_core::ecma::ast::Stmt =
                        macroforge_ts::macroforge_ts_syn::ToTsStmt::to_ts_stmt(#expr);
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(#ph_ident));
                }
            }
        }
    }

    /// Generates code for an identifier block.
    fn generate_ident_block(&self, parts: &[IrNode]) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        let part_stmts: Vec<TokenStream> = parts
            .iter()
            .map(|p| match p {
                IrNode::Text(text) => quote! { __ident_parts.push_str(#text); },
                IrNode::Placeholder { rust_expr, .. } => {
                    let expr: TokenStream =
                        rust_expr.parse().unwrap_or_else(|_| quote! { #rust_expr });
                    quote! {
                        __ident_parts.push_str(&macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(&#expr).sym.to_string());
                    }
                }
                _ => quote! {},
            })
            .collect();

        quote! {
            {
                let mut __ident_parts = String::new();
                #(#part_stmts)*
                let __ident = macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                    __ident_parts.into(),
                    macroforge_ts::swc_core::common::DUMMY_SP,
                );
                #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                    macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                        macroforge_ts::swc_core::ecma::ast::ExprStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(__ident)),
                        }
                    )
                ));
            }
        }
    }

    /// Generates code for string interpolation.
    fn generate_string_interp(&self, _quote_char: char, parts: &[IrNode]) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        // For template literals with interpolations, generate a Tpl node
        let mut quasis = Vec::new();
        let mut exprs = Vec::new();
        let mut current_text = String::new();

        for part in parts {
            match part {
                IrNode::Text(text) => {
                    current_text.push_str(text);
                }
                IrNode::Placeholder { rust_expr, .. } => {
                    // Push current text as quasi
                    let text = std::mem::take(&mut current_text);
                    quasis.push(quote! {
                        macroforge_ts::swc_core::ecma::ast::TplElement {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            tail: false,
                            cooked: Some(#text.into()),
                            raw: #text.into(),
                        }
                    });
                    // Add expression
                    let expr: TokenStream =
                        rust_expr.parse().unwrap_or_else(|_| quote! { #rust_expr });
                    exprs.push(quote! {
                        Box::new(macroforge_ts::macroforge_ts_syn::ToTsExpr::to_ts_expr((#expr).clone()))
                    });
                }
                _ => {}
            }
        }

        // Push final quasi (always needed, marked as tail)
        let final_text = current_text;
        quasis.push(quote! {
            macroforge_ts::swc_core::ecma::ast::TplElement {
                span: macroforge_ts::swc_core::common::DUMMY_SP,
                tail: true,
                cooked: Some(#final_text.into()),
                raw: #final_text.into(),
            }
        });

        quote! {
            #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                    macroforge_ts::swc_core::ecma::ast::ExprStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Tpl(
                            macroforge_ts::swc_core::ecma::ast::Tpl {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                quasis: vec![#(#quasis),*],
                                exprs: vec![#(#exprs),*],
                            }
                        )),
                    }
                )
            ));
        }
    }

    /// Generates code for an if block.
    fn generate_if(
        &self,
        condition: &str,
        then_body: &[IrNode],
        else_if_branches: &[(String, Vec<IrNode>)],
        else_body: &Option<Vec<IrNode>>,
    ) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { #condition });
        let then_stmts = self.generate_nodes(then_body);

        let else_if_parts: Vec<TokenStream> = else_if_branches
            .iter()
            .map(|(cond_str, body)| {
                let c: TokenStream = cond_str.parse().unwrap_or_else(|_| quote! { #cond_str });
                let b = self.generate_nodes(body);
                quote! { else if #c { #b } }
            })
            .collect();

        let else_part = else_body.as_ref().map(|body| {
            let b = self.generate_nodes(body);
            quote! { else { #b } }
        });

        quote! {
            if #cond {
                #then_stmts
            }
            #(#else_if_parts)*
            #else_part
        }
    }

    /// Generates code for a for loop.
    fn generate_for(&self, pattern: &str, iterator: &str, body: &[IrNode]) -> TokenStream {
        let pat: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { #pattern });
        let iter: TokenStream = iterator.parse().unwrap_or_else(|_| quote! { #iterator });
        let body_stmts = self.generate_nodes(body);

        quote! {
            for #pat in #iter {
                #body_stmts
            }
        }
    }

    /// Generates code for a while loop.
    fn generate_while(&self, condition: &str, body: &[IrNode]) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { #condition });
        let body_stmts = self.generate_nodes(body);

        quote! {
            while #cond {
                #body_stmts
            }
        }
    }

    /// Generates code for a match expression.
    fn generate_match(
        &self,
        expr: &str,
        arms: &[(String, Option<String>, Vec<IrNode>)],
    ) -> TokenStream {
        let e: TokenStream = expr.parse().unwrap_or_else(|_| quote! { #expr });

        let arm_tokens: Vec<TokenStream> = arms
            .iter()
            .map(|(pattern, guard, body)| {
                let p: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { #pattern });
                let b = self.generate_nodes(body);
                if let Some(g) = guard {
                    let guard_expr: TokenStream = g.parse().unwrap_or_else(|_| quote! { #g });
                    quote! { #p if #guard_expr => { #b } }
                } else {
                    quote! { #p => { #b } }
                }
            })
            .collect();

        quote! {
            match #e {
                #(#arm_tokens)*
            }
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
        let name_ident = format_ident!("{}", name);
        let val: TokenStream = value.parse().unwrap_or_else(|_| quote! { #value });

        let mutability = if mutable {
            quote! { mut }
        } else {
            quote! {}
        };

        if let Some(ty) = type_hint {
            let type_tokens: TokenStream = ty.parse().unwrap_or_else(|_| quote! { #ty });
            quote! { let #mutability #name_ident: #type_tokens = #val; }
        } else {
            quote! { let #mutability #name_ident = #val; }
        }
    }

    /// Generates code for a do directive.
    fn generate_do(&self, code: &str) -> TokenStream {
        let stmts: TokenStream = code.parse().unwrap_or_else(|_| quote! {});
        quote! { #stmts }
    }

    /// Generates code for a typescript directive.
    fn generate_typescript(&self, stream: &str, output_var: &syn::Ident) -> TokenStream {
        let s: TokenStream = stream.parse().unwrap_or_else(|_| quote! { #stream });
        quote! {
            #output_var.extend(#s);
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
    use crate::compiler::parser::Parser;

    fn compile_template(input: &str) -> TokenStream {
        let ir = Parser::new(input).parse();
        Codegen::new().generate(&ir)
    }

    #[test]
    fn test_codegen_simple_text() {
        let code = compile_template("const x = 1;");
        let code_str = code.to_string();
        assert!(
            code_str.contains("ts_quote"),
            "Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_interpolation() {
        let code = compile_template("const x = @{value};");
        let code_str = code.to_string();
        assert!(
            code_str.contains("ToTsExpr"),
            "Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_type_placeholder() {
        let code = compile_template("const x: @{MyType} = 1;");
        let code_str = code.to_string();
        assert!(
            code_str.contains("ToTsType"),
            "Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_for_loop() {
        let code = compile_template("{#for item in items}@{item}{/for}");
        let code_str = code.to_string();
        assert!(code_str.contains("for"), "Generated code: {}", code_str);
        assert!(code_str.contains("in"), "Generated code: {}", code_str);
    }

    #[test]
    fn test_codegen_if_block() {
        let code = compile_template("{#if cond}yes{/if}");
        let code_str = code.to_string();
        assert!(code_str.contains("if"), "Generated code: {}", code_str);
    }

    #[test]
    fn test_codegen_generates_vec_module_item() {
        let code = compile_template("const x = 1;");
        let code_str = code.to_string();
        assert!(
            code_str.contains("ModuleItem"),
            "Generated code: {}",
            code_str
        );
    }
}
