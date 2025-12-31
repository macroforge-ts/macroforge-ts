//! Code generation from IR to Rust TokenStream.
//!
//! Generates Rust code that builds SWC AST nodes directly.
//! Each IR node type maps to corresponding SWC AST construction code.

mod class;
mod expr;
mod ident;
mod interface;
mod module_item;
mod node;
mod op;
mod param;
mod pat;
mod prop;
mod signature;
mod stmt;
mod target;
mod r#type;

pub use super::ir::{
    Accessibility, AssignOp, BinaryOp, Ir, IrNode, MatchArm, MethodKind, PlaceholderKind,
    TsKeyword, UnaryOp, UpdateOp, VarDeclarator, VarKind,
};
pub use proc_macro2::TokenStream;
pub use quote::{format_ident, quote};

/// Configuration for code generation.
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    pub output_var: String,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            output_var: "__stmts".to_string(),
        }
    }
}

pub struct Codegen {
    config: CodegenConfig,
}

impl Codegen {
    pub fn new() -> Self {
        Self::with_config(CodegenConfig::default())
    }

    pub fn with_config(config: CodegenConfig) -> Self {
        Self { config }
    }

    /// Generate the complete output for an IR tree.
    pub fn generate(&self, ir: &Ir) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let body = self.generate_module_items(&ir.nodes);

        let result = quote! {
            {
                let mut #output_var: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = Vec::new();
                #body
                #output_var
            }
        };

        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!("[MF_DEBUG_CODEGEN] Generated code:\n{}", result);
        }

        result
    }

    fn generate_var_declarators(&self, decls: &[VarDeclarator]) -> TokenStream {
        let decls_code: Vec<TokenStream> = decls
            .iter()
            .map(|d| {
                let name_code = self.generate_pat(&d.name);
                let init_code = d
                    .init
                    .as_ref()
                    .map(|i| {
                        let ic = self.generate_expr(i);
                        quote! { Some(Box::new(#ic)) }
                    })
                    .unwrap_or(quote! { None });

                quote! {
                    macroforge_ts::swc_core::ecma::ast::VarDeclarator {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        name: #name_code,
                        init: #init_code,
                        definite: false,
                    }
                }
            })
            .collect();

        quote! { vec![#(#decls_code),*] }
    }

    fn generate_entity_name(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::Ident(name) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                        macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        )
                    )
                }
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
            } => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                        macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone())
                    )
                }
            }
            _ => quote! {
                macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                    macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                        "".into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                )
            },
        }
    }

    fn generate_implements(&self, _implements: &[IrNode]) -> TokenStream {
        quote! { vec![] }
    }
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

fn _unused_variants() {
    let _ = UnaryOp::Minus;
    let _ = UpdateOp::Increment;
    let _ = Accessibility::Public;
    let _ = MethodKind::Method;
}
