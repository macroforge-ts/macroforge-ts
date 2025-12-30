//! Code generation from IR to Rust TokenStream.
//!
//! Generates Rust code that builds SWC AST nodes directly.
//! Each IR node type maps to corresponding SWC AST construction code.

use super::ir::{
    Accessibility, AssignOp, BinaryOp, Ir, IrNode, MatchArm, MethodKind, PlaceholderKind,
    TsKeyword, UnaryOp, UpdateOp, VarDeclarator, VarKind,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

        quote! {
            {
                let mut #output_var: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = Vec::new();
                #body
                #output_var
            }
        }
    }

    /// Generate code for a list of module-level items.
    fn generate_module_items(&self, nodes: &[IrNode]) -> TokenStream {
        let stmts: Vec<TokenStream> = nodes
            .iter()
            .filter_map(|n| self.generate_module_item(n))
            .collect();
        quote! { #(#stmts)* }
    }

    /// Generate code for a single module-level item.
    fn generate_module_item(&self, node: &IrNode) -> Option<TokenStream> {
        let output_var = format_ident!("{}", self.config.output_var);

        match node {
            // =================================================================
            // Declarations
            // =================================================================
            IrNode::FnDecl {
                exported,
                declare,
                async_,
                generator,
                name,
                type_params,
                params,
                return_type,
                body,
            } => {
                let name_code = self.generate_ident(name);
                let params_code = self.generate_params(params);
                let body_code = body
                    .as_ref()
                    .map(|b| self.generate_block_stmt_opt(b))
                    .unwrap_or(quote! { None });
                let return_type_code = return_type
                    .as_ref()
                    .map(|t| {
                        let tc = self.generate_type_ann(t);
                        quote! { Some(Box::new(#tc)) }
                    })
                    .unwrap_or(quote! { None });
                let type_params_code = type_params
                    .as_ref()
                    .map(|tp| {
                        let tpc = self.generate_type_params(tp);
                        quote! { Some(Box::new(#tpc)) }
                    })
                    .unwrap_or(quote! { None });

                let fn_decl = quote! {
                    macroforge_ts::swc_core::ecma::ast::FnDecl {
                        ident: #name_code,
                        declare: #declare,
                        function: Box::new(macroforge_ts::swc_core::ecma::ast::Function {
                            params: #params_code,
                            decorators: vec![],
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            body: #body_code,
                            is_generator: #generator,
                            is_async: #async_,
                            type_params: #type_params_code,
                            return_type: #return_type_code,
                        }),
                    }
                };

                if *exported {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::Fn(#fn_decl),
                                }
                            )
                        ));
                    })
                } else {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::Fn(#fn_decl)
                            )
                        ));
                    })
                }
            }

            IrNode::ClassDecl {
                exported,
                declare,
                abstract_,
                name,
                type_params,
                extends,
                implements,
                body,
            } => {
                let name_code = self.generate_ident(name);
                let body_code = self.generate_class_members(body);
                let extends_code = extends
                    .as_ref()
                    .map(|e| {
                        let ec = self.generate_expr(e);
                        quote! {
                            Some(Box::new(macroforge_ts::swc_core::ecma::ast::ExtendsClause {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                super_class: Box::new(#ec),
                                type_args: None,
                            }))
                        }
                    })
                    .unwrap_or(quote! { None });
                let type_params_code = type_params
                    .as_ref()
                    .map(|tp| {
                        let tpc = self.generate_type_params(tp);
                        quote! { Some(Box::new(#tpc)) }
                    })
                    .unwrap_or(quote! { None });
                let implements_code = self.generate_implements(implements);

                let class_decl = quote! {
                    macroforge_ts::swc_core::ecma::ast::ClassDecl {
                        ident: #name_code,
                        declare: #declare,
                        class: Box::new(macroforge_ts::swc_core::ecma::ast::Class {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            decorators: vec![],
                            body: #body_code,
                            super_class: #extends_code,
                            is_abstract: #abstract_,
                            type_params: #type_params_code,
                            super_type_params: None,
                            implements: #implements_code,
                        }),
                    }
                };

                if *exported {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::Class(#class_decl),
                                }
                            )
                        ));
                    })
                } else {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::Class(#class_decl)
                            )
                        ));
                    })
                }
            }

            IrNode::InterfaceDecl {
                exported,
                declare,
                name,
                type_params,
                extends,
                body,
            } => {
                let name_code = self.generate_ident(name);
                let body_code = self.generate_ts_interface_body(body);
                let type_params_code = type_params
                    .as_ref()
                    .map(|tp| {
                        let tpc = self.generate_type_params(tp);
                        quote! { Some(Box::new(#tpc)) }
                    })
                    .unwrap_or(quote! { None });
                let extends_code = self.generate_ts_expr_with_type_args(extends);

                let interface_decl = quote! {
                    macroforge_ts::swc_core::ecma::ast::TsInterfaceDecl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        id: #name_code,
                        declare: #declare,
                        type_params: #type_params_code,
                        extends: #extends_code,
                        body: #body_code,
                    }
                };

                if *exported {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::TsInterface(Box::new(#interface_decl)),
                                }
                            )
                        ));
                    })
                } else {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::TsInterface(Box::new(#interface_decl))
                            )
                        ));
                    })
                }
            }

            IrNode::TypeAliasDecl {
                exported,
                declare,
                name,
                type_params,
                type_ann,
            } => {
                let name_code = self.generate_ident(name);
                let type_ann_code = self.generate_type(type_ann);
                let type_params_code = type_params
                    .as_ref()
                    .map(|tp| {
                        let tpc = self.generate_type_params(tp);
                        quote! { Some(Box::new(#tpc)) }
                    })
                    .unwrap_or(quote! { None });

                let type_alias = quote! {
                    macroforge_ts::swc_core::ecma::ast::TsTypeAliasDecl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        id: #name_code,
                        declare: #declare,
                        type_params: #type_params_code,
                        type_ann: Box::new(#type_ann_code),
                    }
                };

                if *exported {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::TsTypeAlias(Box::new(#type_alias)),
                                }
                            )
                        ));
                    })
                } else {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::TsTypeAlias(Box::new(#type_alias))
                            )
                        ));
                    })
                }
            }

            IrNode::VarDecl {
                exported,
                declare,
                kind,
                decls,
            } => {
                let kind_code = match kind {
                    VarKind::Const => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Const },
                    VarKind::Let => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Let },
                    VarKind::Var => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Var },
                };
                let decls_code = self.generate_var_declarators(decls);

                let var_decl = quote! {
                    macroforge_ts::swc_core::ecma::ast::VarDecl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        kind: #kind_code,
                        declare: #declare,
                        decls: #decls_code,
                    }
                };

                if *exported {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::Var(Box::new(#var_decl)),
                                }
                            )
                        ));
                    })
                } else {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::Var(Box::new(#var_decl))
                            )
                        ));
                    })
                }
            }

            // =================================================================
            // Statements as module items
            // =================================================================
            IrNode::ExprStmt { expr } => {
                let expr_code = self.generate_expr(expr);
                Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(#expr_code),
                            }
                        )
                    ));
                })
            }

            IrNode::ReturnStmt { arg } => {
                let arg_code = arg
                    .as_ref()
                    .map(|a| {
                        let ac = self.generate_expr(a);
                        quote! { Some(Box::new(#ac)) }
                    })
                    .unwrap_or(quote! { None });
                Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Return(
                            macroforge_ts::swc_core::ecma::ast::ReturnStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                arg: #arg_code,
                            }
                        )
                    ));
                })
            }

            IrNode::BlockStmt { stmts } => {
                let stmts_code = self.generate_stmts_vec(stmts);
                Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Block(
                            macroforge_ts::swc_core::ecma::ast::BlockStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                                stmts: #stmts_code,
                            }
                        )
                    ));
                })
            }

            // =================================================================
            // Template Constructs - Control Flow (Rust)
            // =================================================================
            IrNode::If {
                condition,
                then_body,
                else_if_branches,
                else_body,
            } => {
                let cond = condition;
                let then_stmts = self.generate_module_items(then_body);

                let else_ifs: Vec<TokenStream> = else_if_branches
                    .iter()
                    .map(|(c, body)| {
                        let b = self.generate_module_items(body);
                        quote! { else if #c { #b } }
                    })
                    .collect();

                let else_part = else_body.as_ref().map(|body| {
                    let b = self.generate_module_items(body);
                    quote! { else { #b } }
                });

                Some(quote! {
                    if #cond { #then_stmts }
                    #(#else_ifs)*
                    #else_part
                })
            }

            IrNode::For {
                pattern,
                iterator,
                body,
            } => {
                let body_stmts = self.generate_module_items(body);
                Some(quote! {
                    for #pattern in #iterator { #body_stmts }
                })
            }

            IrNode::While { condition, body } => {
                let body_stmts = self.generate_module_items(body);
                Some(quote! {
                    while #condition { #body_stmts }
                })
            }

            IrNode::Match { expr, arms } => {
                let arm_tokens: Vec<TokenStream> = arms
                    .iter()
                    .map(|MatchArm { pattern, guard, body }| {
                        let b = self.generate_module_items(body);
                        if let Some(g) = guard {
                            quote! { #pattern if #g => { #b } }
                        } else {
                            quote! { #pattern => { #b } }
                        }
                    })
                    .collect();

                Some(quote! {
                    match #expr { #(#arm_tokens)* }
                })
            }

            IrNode::Let {
                pattern,
                mutable,
                type_hint,
                value,
            } => {
                let mutability = if *mutable {
                    quote! { mut }
                } else {
                    quote! {}
                };

                if let Some(ty) = type_hint {
                    Some(quote! { let #mutability #pattern: #ty = #value; })
                } else {
                    Some(quote! { let #mutability #pattern = #value; })
                }
            }

            IrNode::Do { code } => Some(quote! { #code }),

            IrNode::TypeScript { stream } => {
                Some(quote! {
                    {
                        let __ts: macroforge_ts::ts_syn::TsStream = #stream;
                        let __parsed = macroforge_ts::ts_syn::parse_ts_module(__ts.source())
                            .expect("Failed to parse TypeScript stream");
                        #output_var.extend(__parsed.body);
                    }
                })
            }

            // =================================================================
            // Placeholders
            // =================================================================
            IrNode::Placeholder { kind, expr } => match kind {
                PlaceholderKind::Expr => Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(macroforge_ts::macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr)),
                            }
                        )
                    ));
                }),
                PlaceholderKind::Stmt => Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::macroforge_ts_syn::ToTsStmt::to_ts_stmt(#expr)
                    ));
                }),
                PlaceholderKind::Ident => Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                                    macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr)
                                )),
                            }
                        )
                    ));
                }),
                PlaceholderKind::Type => {
                    // Type placeholders in flat IR need to be emitted as expressions
                    // We use a type assertion (undefined as T) to include the type
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                                macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::TsAs(
                                        macroforge_ts::swc_core::ecma::ast::TsAsExpr {
                                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                                            expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                                                macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                                    "undefined".into(),
                                                    macroforge_ts::swc_core::common::DUMMY_SP,
                                                )
                                            )),
                                            type_ann: Box::new(macroforge_ts::macroforge_ts_syn::ToTsType::to_ts_type(#expr)),
                                        }
                                    )),
                                }
                            )
                        ));
                    })
                }
            },

            // =================================================================
            // Comments
            // =================================================================
            IrNode::LineComment { text: _ } | IrNode::BlockComment { text: _ } => {
                // Comments are emitted as empty statements for now
                // TODO: Use SWC comments API
                Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Empty(
                            macroforge_ts::swc_core::ecma::ast::EmptyStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                            }
                        )
                    ));
                })
            }

            IrNode::DocComment { text: _ } => None,

            IrNode::Documented { doc: _, inner } => self.generate_module_item(inner),

            // =================================================================
            // Special constructs
            // =================================================================
            IrNode::IdentBlock { parts } => {
                let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .filter_map(|p| match p {
                        IrNode::Raw(text) => Some(quote! { __ident.push_str(#text); }),
                        IrNode::StrLit(text) => Some(quote! { __ident.push_str(#text); }),
                        IrNode::Ident(text) => Some(quote! { __ident.push_str(#text); }),
                        IrNode::Placeholder { expr, .. } => {
                            Some(quote! {
                                __ident.push_str(&macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr).sym.to_string());
                            })
                        }
                        _ => None,
                    })
                    .collect();

                Some(quote! {
                    {
                        let mut __ident = String::new();
                        #(#part_exprs)*
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                                macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                                        macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                            __ident.into(),
                                            macroforge_ts::swc_core::common::DUMMY_SP,
                                        )
                                    )),
                                }
                            )
                        ));
                    }
                })
            }

            IrNode::StringInterp { quote: _, parts } => {
                let mut quasi_parts = Vec::new();
                let mut expr_parts = Vec::new();
                let mut current_text = String::new();

                for part in parts {
                    match part {
                        IrNode::Raw(text) | IrNode::StrLit(text) => current_text.push_str(text),
                        IrNode::Placeholder { expr, .. } => {
                            let text = std::mem::take(&mut current_text);
                            quasi_parts.push(quote! {
                                macroforge_ts::swc_core::ecma::ast::TplElement {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    tail: false,
                                    cooked: Some(#text.into()),
                                    raw: #text.into(),
                                }
                            });
                            expr_parts.push(quote! {
                                Box::new(macroforge_ts::macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr))
                            });
                        }
                        _ => {}
                    }
                }

                let final_text = current_text;
                quasi_parts.push(quote! {
                    macroforge_ts::swc_core::ecma::ast::TplElement {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        tail: true,
                        cooked: Some(#final_text.into()),
                        raw: #final_text.into(),
                    }
                });

                Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Tpl(
                                    macroforge_ts::swc_core::ecma::ast::Tpl {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        quasis: vec![#(#quasi_parts),*],
                                        exprs: vec![#(#expr_parts),*],
                                    }
                                )),
                            }
                        )
                    ));
                })
            }

            // Raw text - use ts_quote! as fallback
            IrNode::Raw(text) => {
                if text.trim().is_empty() {
                    return None;
                }
                Some(quote! {
                    #output_var.extend(
                        macroforge_ts::macroforge_ts_quote::ts_quote!(#text as Module).body
                    );
                })
            }

            // All other node types that don't make sense at module level
            _ => {
                // For nodes that should be expressions or statements,
                // wrap them appropriately
                if let Some(expr_code) = self.try_generate_as_expr(node) {
                    Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                                macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    expr: Box::new(#expr_code),
                                }
                            )
                        ));
                    })
                } else {
                    None
                }
            }
        }
    }

    /// Try to generate a node as an expression.
    fn try_generate_as_expr(&self, node: &IrNode) -> Option<TokenStream> {
        match node {
            IrNode::Ident(_)
            | IrNode::StrLit(_)
            | IrNode::NumLit(_)
            | IrNode::BoolLit(_)
            | IrNode::NullLit
            | IrNode::ThisExpr
            | IrNode::CallExpr { .. }
            | IrNode::MemberExpr { .. }
            | IrNode::ObjectLit { .. }
            | IrNode::ArrayLit { .. }
            | IrNode::BinExpr { .. }
            | IrNode::AssignExpr { .. }
            | IrNode::CondExpr { .. }
            | IrNode::ArrowExpr { .. }
            | IrNode::NewExpr { .. }
            | IrNode::TplLit { .. } => Some(self.generate_expr(node)),
            _ => None,
        }
    }

    // =========================================================================
    // Expression Generation
    // =========================================================================

    fn generate_expr(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::Ident(name) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                        macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        )
                    )
                }
            }

            IrNode::StrLit(value) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                        macroforge_ts::swc_core::ecma::ast::Lit::Str(
                            macroforge_ts::swc_core::ecma::ast::Str {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                value: #value.into(),
                                raw: None,
                            }
                        )
                    )
                }
            }

            IrNode::NumLit(value) => {
                let num: f64 = value.parse().unwrap_or(0.0);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                        macroforge_ts::swc_core::ecma::ast::Lit::Num(
                            macroforge_ts::swc_core::ecma::ast::Number {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                value: #num,
                                raw: Some(#value.into()),
                            }
                        )
                    )
                }
            }

            IrNode::BoolLit(value) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                        macroforge_ts::swc_core::ecma::ast::Lit::Bool(
                            macroforge_ts::swc_core::ecma::ast::Bool {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                value: #value,
                            }
                        )
                    )
                }
            }

            IrNode::NullLit => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                        macroforge_ts::swc_core::ecma::ast::Lit::Null(
                            macroforge_ts::swc_core::ecma::ast::Null {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                            }
                        )
                    )
                }
            }

            IrNode::ThisExpr => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::This(
                        macroforge_ts::swc_core::ecma::ast::ThisExpr {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                        }
                    )
                }
            }

            IrNode::CallExpr {
                callee,
                type_args: _,
                args,
            } => {
                let callee_code = self.generate_expr(callee);
                let args_code: Vec<TokenStream> = args
                    .iter()
                    .map(|a| {
                        let expr = self.generate_expr(a);
                        quote! {
                            macroforge_ts::swc_core::ecma::ast::ExprOrSpread {
                                spread: None,
                                expr: Box::new(#expr),
                            }
                        }
                    })
                    .collect();

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Call(
                        macroforge_ts::swc_core::ecma::ast::CallExpr {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            callee: macroforge_ts::swc_core::ecma::ast::Callee::Expr(Box::new(#callee_code)),
                            args: vec![#(#args_code),*],
                            type_args: None,
                        }
                    )
                }
            }

            IrNode::MemberExpr { obj, prop, computed } => {
                let obj_code = self.generate_expr(obj);
                let prop_code = if *computed {
                    let p = self.generate_expr(prop);
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::MemberProp::Computed(
                            macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(#p),
                            }
                        )
                    }
                } else {
                    let ident_code = self.generate_ident_name(prop);
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::MemberProp::Ident(#ident_code)
                    }
                };

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Member(
                        macroforge_ts::swc_core::ecma::ast::MemberExpr {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            obj: Box::new(#obj_code),
                            prop: #prop_code,
                        }
                    )
                }
            }

            IrNode::ObjectLit { props } => {
                let props_code = self.generate_props(props);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Object(
                        macroforge_ts::swc_core::ecma::ast::ObjectLit {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            props: #props_code,
                        }
                    )
                }
            }

            IrNode::ArrayLit { elems } => {
                let elems_code: Vec<TokenStream> = elems
                    .iter()
                    .map(|e| {
                        let expr = self.generate_expr(e);
                        quote! {
                            Some(macroforge_ts::swc_core::ecma::ast::ExprOrSpread {
                                spread: None,
                                expr: Box::new(#expr),
                            })
                        }
                    })
                    .collect();

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Array(
                        macroforge_ts::swc_core::ecma::ast::ArrayLit {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            elems: vec![#(#elems_code),*],
                        }
                    )
                }
            }

            IrNode::BinExpr { left, op, right } => {
                let left_code = self.generate_expr(left);
                let right_code = self.generate_expr(right);
                let op_code = self.generate_binary_op(op);

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Bin(
                        macroforge_ts::swc_core::ecma::ast::BinExpr {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            op: #op_code,
                            left: Box::new(#left_code),
                            right: Box::new(#right_code),
                        }
                    )
                }
            }

            IrNode::AssignExpr { left, op, right } => {
                let left_code = self.generate_assign_target(left);
                let right_code = self.generate_expr(right);
                let op_code = self.generate_assign_op(op);

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Assign(
                        macroforge_ts::swc_core::ecma::ast::AssignExpr {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            op: #op_code,
                            left: #left_code,
                            right: Box::new(#right_code),
                        }
                    )
                }
            }

            IrNode::CondExpr { test, cons, alt } => {
                let test_code = self.generate_expr(test);
                let cons_code = self.generate_expr(cons);
                let alt_code = self.generate_expr(alt);

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Cond(
                        macroforge_ts::swc_core::ecma::ast::CondExpr {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            test: Box::new(#test_code),
                            cons: Box::new(#cons_code),
                            alt: Box::new(#alt_code),
                        }
                    )
                }
            }

            IrNode::NewExpr {
                callee,
                type_args: _,
                args,
            } => {
                let callee_code = self.generate_expr(callee);
                let args_code: Vec<TokenStream> = args
                    .iter()
                    .map(|a| {
                        let expr = self.generate_expr(a);
                        quote! {
                            macroforge_ts::swc_core::ecma::ast::ExprOrSpread {
                                spread: None,
                                expr: Box::new(#expr),
                            }
                        }
                    })
                    .collect();

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::New(
                        macroforge_ts::swc_core::ecma::ast::NewExpr {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            callee: Box::new(#callee_code),
                            args: Some(vec![#(#args_code),*]),
                            type_args: None,
                        }
                    )
                }
            }

            IrNode::ArrowExpr {
                async_,
                type_params: _,
                params,
                return_type: _,
                body,
            } => {
                let params_code = self.generate_pats(params);
                let body_code = self.generate_block_stmt_or_expr(body);

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Arrow(
                        macroforge_ts::swc_core::ecma::ast::ArrowExpr {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            params: #params_code,
                            body: Box::new(#body_code),
                            is_async: #async_,
                            is_generator: false,
                            type_params: None,
                            return_type: None,
                        }
                    )
                }
            }

            IrNode::TplLit { quasis, exprs } => {
                let quasis_code: Vec<TokenStream> = quasis
                    .iter()
                    .enumerate()
                    .map(|(i, text)| {
                        let tail = i == quasis.len() - 1;
                        quote! {
                            macroforge_ts::swc_core::ecma::ast::TplElement {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                tail: #tail,
                                cooked: Some(#text.into()),
                                raw: #text.into(),
                            }
                        }
                    })
                    .collect();

                let exprs_code: Vec<TokenStream> = exprs
                    .iter()
                    .map(|e| {
                        let expr = self.generate_expr(e);
                        quote! { Box::new(#expr) }
                    })
                    .collect();

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Tpl(
                        macroforge_ts::swc_core::ecma::ast::Tpl {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            quasis: vec![#(#quasis_code),*],
                            exprs: vec![#(#exprs_code),*],
                        }
                    )
                }
            }

            // StringInterp from template literals - convert to Tpl expression
            IrNode::StringInterp { quote: _, parts } => {
                // Convert parts into quasis and exprs
                let mut quasis = Vec::new();
                let mut exprs = Vec::new();
                let mut current_text = String::new();

                for part in parts {
                    match part {
                        IrNode::Raw(text) | IrNode::StrLit(text) => current_text.push_str(text),
                        IrNode::Placeholder { expr, .. } => {
                            quasis.push(std::mem::take(&mut current_text));
                            exprs.push(quote! {
                                Box::new(macroforge_ts::macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr))
                            });
                        }
                        _ => {}
                    }
                }
                // Add final quasi
                quasis.push(current_text);

                let quasis_code: Vec<TokenStream> = quasis
                    .iter()
                    .enumerate()
                    .map(|(i, text)| {
                        let tail = i == quasis.len() - 1;
                        quote! {
                            macroforge_ts::swc_core::ecma::ast::TplElement {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                tail: #tail,
                                cooked: Some(#text.into()),
                                raw: #text.into(),
                            }
                        }
                    })
                    .collect();

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Tpl(
                        macroforge_ts::swc_core::ecma::ast::Tpl {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            quasis: vec![#(#quasis_code),*],
                            exprs: vec![#(#exprs),*],
                        }
                    )
                }
            }

            IrNode::Placeholder { kind, expr } => match kind {
                PlaceholderKind::Expr => {
                    quote! { macroforge_ts::macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr) }
                }
                PlaceholderKind::Ident => {
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                            macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr)
                        )
                    }
                }
                _ => quote! { /* placeholder in wrong context */ },
            },

            // Raw text - parse as expression at runtime
            IrNode::Raw(text) => {
                quote! {
                    {
                        let __source = #text;
                        macroforge_ts::ts_syn::parse_ts_expr(__source)
                            .unwrap_or_else(|_| macroforge_ts::swc_core::ecma::ast::Expr::Invalid(
                                macroforge_ts::swc_core::ecma::ast::Invalid {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                }
                            ))
                    }
                }
            }

            // IdentBlock with multiple parts - build string and parse
            IrNode::IdentBlock { parts } => {
                let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .map(|p| match p {
                        IrNode::Raw(text) => quote! { __expr_str.push_str(#text); },
                        IrNode::StrLit(text) => quote! { __expr_str.push_str(#text); },
                        IrNode::Ident(text) => quote! { __expr_str.push_str(#text); },
                        IrNode::Placeholder { kind, expr } => {
                            match kind {
                                PlaceholderKind::Expr => {
                                    // For expressions, emit as TypeScript literal
                                    quote! {
                                        let __expr = macroforge_ts::macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr);
                                        __expr_str.push_str(&macroforge_ts::ts_syn::emit_expr(&__expr));
                                    }
                                }
                                PlaceholderKind::Ident => {
                                    quote! {
                                        __expr_str.push_str(&macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr).sym.to_string());
                                    }
                                }
                                _ => quote! { /* placeholder kind not supported in expr context */ },
                            }
                        }
                        _ => quote! {},
                    })
                    .collect();

                quote! {
                    {
                        let mut __expr_str = String::new();
                        #(#part_exprs)*
                        macroforge_ts::ts_syn::parse_ts_expr(&__expr_str)
                            .unwrap_or_else(|_| macroforge_ts::swc_core::ecma::ast::Expr::Invalid(
                                macroforge_ts::swc_core::ecma::ast::Invalid {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                }
                            ))
                    }
                }
            }

            // Default: identity expression for unknown node types
            _ => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Invalid(
                        macroforge_ts::swc_core::ecma::ast::Invalid {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                        }
                    )
                }
            }
        }
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    fn generate_ident(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::Ident(name) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                        #name.into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                }
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
            } => {
                quote! { macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr) }
            }
            _ => quote! {
                macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                    "".into(),
                    macroforge_ts::swc_core::common::DUMMY_SP,
                )
            },
        }
    }

    fn generate_ident_name(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::Ident(name) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::IdentName::new(
                        #name.into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                }
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
            } => {
                quote! {
                    {
                        let __ident = macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr);
                        macroforge_ts::swc_core::ecma::ast::IdentName::new(
                            __ident.sym,
                            __ident.span,
                        )
                    }
                }
            }
            _ => quote! {
                macroforge_ts::swc_core::ecma::ast::IdentName::new(
                    "".into(),
                    macroforge_ts::swc_core::common::DUMMY_SP,
                )
            },
        }
    }

    fn generate_params(&self, params: &[IrNode]) -> TokenStream {
        let params_code: Vec<TokenStream> = params.iter().map(|p| self.generate_param(p)).collect();
        quote! { vec![#(#params_code),*] }
    }

    fn generate_param(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::Param { decorators: _, pat } => {
                let pat_code = self.generate_pat(pat);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Param {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        decorators: vec![],
                        pat: #pat_code,
                    }
                }
            }
            IrNode::BindingIdent {
                name,
                type_ann,
                optional,
            } => {
                let name_code = self.generate_ident(name);
                let type_ann_code = type_ann
                    .as_ref()
                    .map(|t| {
                        let tc = self.generate_type_ann(t);
                        quote! { Some(Box::new(#tc)) }
                    })
                    .unwrap_or(quote! { None });

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Param {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        decorators: vec![],
                        pat: macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                            macroforge_ts::swc_core::ecma::ast::BindingIdent {
                                id: #name_code,
                                type_ann: #type_ann_code,
                            }
                        ),
                    }
                }
            }
            // For other node types, assume they're identifiers
            _ => {
                let ident_code = self.generate_ident(node);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Param {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        decorators: vec![],
                        pat: macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                            macroforge_ts::swc_core::ecma::ast::BindingIdent {
                                id: #ident_code,
                                type_ann: None,
                            }
                        ),
                    }
                }
            }
        }
    }

    fn generate_pat(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::BindingIdent {
                name,
                type_ann,
                optional: _,
            } => {
                let name_code = self.generate_ident(name);
                let type_ann_code = type_ann
                    .as_ref()
                    .map(|t| {
                        let tc = self.generate_type_ann(t);
                        quote! { Some(Box::new(#tc)) }
                    })
                    .unwrap_or(quote! { None });

                quote! {
                    macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                        macroforge_ts::swc_core::ecma::ast::BindingIdent {
                            id: #name_code,
                            type_ann: #type_ann_code,
                        }
                    )
                }
            }
            IrNode::Ident(name) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                        macroforge_ts::swc_core::ecma::ast::BindingIdent {
                            id: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                #name.into(),
                                macroforge_ts::swc_core::common::DUMMY_SP,
                            ),
                            type_ann: None,
                        }
                    )
                }
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
            } => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                        macroforge_ts::swc_core::ecma::ast::BindingIdent {
                            id: macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr),
                            type_ann: None,
                        }
                    )
                }
            }
            _ => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Pat::Invalid(
                        macroforge_ts::swc_core::ecma::ast::Invalid {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                        }
                    )
                }
            }
        }
    }

    fn generate_pats(&self, nodes: &[IrNode]) -> TokenStream {
        let pats_code: Vec<TokenStream> = nodes.iter().map(|p| self.generate_pat(p)).collect();
        quote! { vec![#(#pats_code),*] }
    }

    fn generate_block_stmt_opt(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::BlockStmt { stmts } => {
                let stmts_code = self.generate_stmts_vec(stmts);
                quote! {
                    Some(macroforge_ts::swc_core::ecma::ast::BlockStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        stmts: #stmts_code,
                    })
                }
            }
            _ => quote! { None },
        }
    }

    fn generate_block_stmt_or_expr(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::BlockStmt { stmts } => {
                let stmts_code = self.generate_stmts_vec(stmts);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::BlockStmtOrExpr::BlockStmt(
                        macroforge_ts::swc_core::ecma::ast::BlockStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            stmts: #stmts_code,
                        }
                    )
                }
            }
            _ => {
                let expr_code = self.generate_expr(node);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::BlockStmtOrExpr::Expr(Box::new(#expr_code))
                }
            }
        }
    }

    fn generate_stmts_vec(&self, nodes: &[IrNode]) -> TokenStream {
        // Check if any node requires dynamic generation (control flow)
        let has_control_flow = nodes.iter().any(|n| matches!(n,
            IrNode::For { .. } | IrNode::If { .. } | IrNode::While { .. } |
            IrNode::Match { .. } | IrNode::Let { .. } | IrNode::Do { .. }
        ));

        if has_control_flow {
            // Generate code that dynamically builds the vec
            let stmt_pushes = self.generate_stmt_pushes(nodes);
            quote! {
                {
                    let mut __body_stmts: Vec<macroforge_ts::swc_core::ecma::ast::Stmt> = Vec::new();
                    #stmt_pushes
                    __body_stmts
                }
            }
        } else {
            // No control flow - generate literal vec
            let stmts: Vec<TokenStream> = nodes
                .iter()
                .filter_map(|n| self.generate_stmt(n))
                .collect();
            quote! { vec![#(#stmts),*] }
        }
    }

    fn generate_stmt_pushes(&self, nodes: &[IrNode]) -> TokenStream {
        let pushes: Vec<TokenStream> = nodes
            .iter()
            .filter_map(|n| self.generate_stmt_push(n))
            .collect();
        quote! { #(#pushes)* }
    }

    fn generate_stmt_push(&self, node: &IrNode) -> Option<TokenStream> {
        match node {
            IrNode::For { pattern, iterator, body } => {
                let body_pushes = self.generate_stmt_pushes(body);
                Some(quote! {
                    for #pattern in #iterator { #body_pushes }
                })
            }
            IrNode::If { condition, then_body, else_if_branches, else_body } => {
                let then_pushes = self.generate_stmt_pushes(then_body);

                let else_code = if else_if_branches.is_empty() && else_body.is_none() {
                    quote! {}
                } else {
                    let mut branches = TokenStream::new();
                    for (cond, body) in else_if_branches {
                        let b = self.generate_stmt_pushes(body);
                        branches.extend(quote! { else if #cond { #b } });
                    }
                    if let Some(eb) = else_body {
                        let eb_pushes = self.generate_stmt_pushes(eb);
                        branches.extend(quote! { else { #eb_pushes } });
                    }
                    branches
                };

                Some(quote! {
                    if #condition { #then_pushes } #else_code
                })
            }
            IrNode::While { condition, body } => {
                let body_pushes = self.generate_stmt_pushes(body);
                Some(quote! {
                    while #condition { #body_pushes }
                })
            }
            IrNode::Match { expr, arms } => {
                let arm_tokens: Vec<TokenStream> = arms
                    .iter()
                    .map(|MatchArm { pattern, guard, body }| {
                        let b = self.generate_stmt_pushes(body);
                        if let Some(g) = guard {
                            quote! { #pattern if #g => { #b } }
                        } else {
                            quote! { #pattern => { #b } }
                        }
                    })
                    .collect();
                Some(quote! {
                    match #expr { #(#arm_tokens)* }
                })
            }
            IrNode::Let { pattern, mutable, type_hint, value } => {
                let mutability = if *mutable { quote! { mut } } else { quote! {} };
                if let Some(ty) = type_hint {
                    Some(quote! { let #mutability #pattern: #ty = #value; })
                } else {
                    Some(quote! { let #mutability #pattern = #value; })
                }
            }
            IrNode::Do { code } => Some(quote! { #code }),
            _ => {
                // Regular statement - push to __body_stmts
                if let Some(stmt_code) = self.generate_stmt(node) {
                    Some(quote! { __body_stmts.push(#stmt_code); })
                } else {
                    None
                }
            }
        }
    }

    fn generate_stmt(&self, node: &IrNode) -> Option<TokenStream> {
        match node {
            IrNode::ExprStmt { expr } => {
                let expr_code = self.generate_expr(expr);
                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                        macroforge_ts::swc_core::ecma::ast::ExprStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            expr: Box::new(#expr_code),
                        }
                    )
                })
            }
            IrNode::ReturnStmt { arg } => {
                let arg_code = arg
                    .as_ref()
                    .map(|a| {
                        let ac = self.generate_expr(a);
                        quote! { Some(Box::new(#ac)) }
                    })
                    .unwrap_or(quote! { None });
                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::Stmt::Return(
                        macroforge_ts::swc_core::ecma::ast::ReturnStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            arg: #arg_code,
                        }
                    )
                })
            }
            IrNode::BlockStmt { stmts } => {
                let stmts_code = self.generate_stmts_vec(stmts);
                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::Stmt::Block(
                        macroforge_ts::swc_core::ecma::ast::BlockStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            stmts: #stmts_code,
                        }
                    )
                })
            }
            IrNode::VarDecl {
                exported: _,
                declare,
                kind,
                decls,
            } => {
                let kind_code = match kind {
                    VarKind::Const => {
                        quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Const }
                    }
                    VarKind::Let => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Let },
                    VarKind::Var => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Var },
                };
                let decls_code = self.generate_var_declarators(decls);

                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                        macroforge_ts::swc_core::ecma::ast::Decl::Var(Box::new(
                            macroforge_ts::swc_core::ecma::ast::VarDecl {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                                kind: #kind_code,
                                declare: #declare,
                                decls: #decls_code,
                            }
                        ))
                    )
                })
            }
            // Control flow that generates Rust code
            IrNode::If { .. } | IrNode::For { .. } | IrNode::While { .. } | IrNode::Match { .. } => {
                // These generate Rust control flow, not TS statements
                // Return None to skip them as direct statements
                None
            }
            _ => {
                // Try to generate as expression statement
                if let Some(expr_code) = self.try_generate_as_expr(node) {
                    Some(quote! {
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(#expr_code),
                            }
                        )
                    })
                } else {
                    None
                }
            }
        }
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

    fn generate_type_ann(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::TypeAnnotation { type_ann } => {
                let type_code = self.generate_type(type_ann);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsTypeAnn {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        type_ann: Box::new(#type_code),
                    }
                }
            }
            _ => {
                let type_code = self.generate_type(node);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsTypeAnn {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        type_ann: Box::new(#type_code),
                    }
                }
            }
        }
    }

    fn generate_type(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::TypeRef { name, type_params } => {
                let name_code = self.generate_entity_name(name);
                let type_params_code = type_params
                    .as_ref()
                    .map(|tp| {
                        let tpc = self.generate_type_param_instantiation(tp);
                        quote! { Some(Box::new(#tpc)) }
                    })
                    .unwrap_or(quote! { None });

                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsType::TsTypeRef(
                        macroforge_ts::swc_core::ecma::ast::TsTypeRef {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            type_name: #name_code,
                            type_params: #type_params_code,
                        }
                    )
                }
            }

            IrNode::KeywordType(kw) => {
                let kw_code = match kw {
                    TsKeyword::Any => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsAnyKeyword },
                    TsKeyword::Unknown => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsUnknownKeyword },
                    TsKeyword::String => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsStringKeyword },
                    TsKeyword::Number => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsNumberKeyword },
                    TsKeyword::Boolean => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsBooleanKeyword },
                    TsKeyword::Void => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsVoidKeyword },
                    TsKeyword::Null => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsNullKeyword },
                    TsKeyword::Undefined => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsUndefinedKeyword },
                    TsKeyword::Never => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsNeverKeyword },
                    TsKeyword::Object => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsObjectKeyword },
                    TsKeyword::BigInt => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsBigIntKeyword },
                    TsKeyword::Symbol => quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsSymbolKeyword },
                };

                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsType::TsKeywordType(
                        macroforge_ts::swc_core::ecma::ast::TsKeywordType {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            kind: #kw_code,
                        }
                    )
                }
            }

            IrNode::UnionType { types } => {
                let types_code: Vec<TokenStream> = types
                    .iter()
                    .map(|t| {
                        let tc = self.generate_type(t);
                        quote! { Box::new(#tc) }
                    })
                    .collect();

                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsType::TsUnionOrIntersectionType(
                        macroforge_ts::swc_core::ecma::ast::TsUnionOrIntersectionType::TsUnionType(
                            macroforge_ts::swc_core::ecma::ast::TsUnionType {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                types: vec![#(#types_code),*],
                            }
                        )
                    )
                }
            }

            IrNode::ArrayType { elem } => {
                let elem_code = self.generate_type(elem);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsType::TsArrayType(
                        macroforge_ts::swc_core::ecma::ast::TsArrayType {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            elem_type: Box::new(#elem_code),
                        }
                    )
                }
            }

            IrNode::Placeholder {
                kind: PlaceholderKind::Type,
                expr,
            } => {
                quote! { macroforge_ts::macroforge_ts_syn::ToTsType::to_ts_type(#expr) }
            }

            IrNode::Ident(name) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsType::TsTypeRef(
                        macroforge_ts::swc_core::ecma::ast::TsTypeRef {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            type_name: macroforge_ts::swc_core::ecma::ast::TsEntityName::Ident(
                                macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                    #name.into(),
                                    macroforge_ts::swc_core::common::DUMMY_SP,
                                )
                            ),
                            type_params: None,
                        }
                    )
                }
            }

            // Raw text - parse as type at runtime
            IrNode::Raw(text) => {
                quote! {
                    {
                        let __source = #text;
                        macroforge_ts::ts_syn::parse_ts_type(__source)
                            .unwrap_or_else(|_| macroforge_ts::swc_core::ecma::ast::TsType::TsKeywordType(
                                macroforge_ts::swc_core::ecma::ast::TsKeywordType {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    kind: macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsAnyKeyword,
                                }
                            ))
                    }
                }
            }

            // IdentBlock with multiple parts - build string and parse
            IrNode::IdentBlock { parts } => {
                let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .map(|p| match p {
                        IrNode::Raw(text) => quote! { __type_str.push_str(#text); },
                        IrNode::StrLit(text) => quote! { __type_str.push_str(#text); },
                        IrNode::Ident(text) => quote! { __type_str.push_str(#text); },
                        IrNode::Placeholder { kind, expr } => {
                            match kind {
                                PlaceholderKind::Type => {
                                    quote! {
                                        let __ty = macroforge_ts::macroforge_ts_syn::ToTsType::to_ts_type(#expr);
                                        __type_str.push_str(&macroforge_ts::ts_syn::emit_ts_type(&__ty));
                                    }
                                }
                                PlaceholderKind::Ident => {
                                    quote! {
                                        __type_str.push_str(&macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr).sym.to_string());
                                    }
                                }
                                _ => quote! { /* placeholder kind not supported in type context */ },
                            }
                        }
                        _ => quote! {},
                    })
                    .collect();

                quote! {
                    {
                        let mut __type_str = String::new();
                        #(#part_exprs)*
                        macroforge_ts::ts_syn::parse_ts_type(&__type_str)
                            .unwrap_or_else(|_| macroforge_ts::swc_core::ecma::ast::TsType::TsKeywordType(
                                macroforge_ts::swc_core::ecma::ast::TsKeywordType {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    kind: macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsAnyKeyword,
                                }
                            ))
                    }
                }
            }

            // TypeAnnotation wrapper - unwrap and generate the inner type
            IrNode::TypeAnnotation { type_ann } => {
                self.generate_type(type_ann)
            }

            _ => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsType::TsKeywordType(
                        macroforge_ts::swc_core::ecma::ast::TsKeywordType {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            kind: macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsAnyKeyword,
                        }
                    )
                }
            }
        }
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
                        macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr)
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

    fn generate_type_params(&self, _node: &IrNode) -> TokenStream {
        // Simplified - full implementation would parse TypeParams node
        quote! {
            macroforge_ts::swc_core::ecma::ast::TsTypeParamDecl {
                span: macroforge_ts::swc_core::common::DUMMY_SP,
                params: vec![],
            }
        }
    }

    fn generate_type_param_instantiation(&self, _node: &IrNode) -> TokenStream {
        quote! {
            macroforge_ts::swc_core::ecma::ast::TsTypeParamInstantiation {
                span: macroforge_ts::swc_core::common::DUMMY_SP,
                params: vec![],
            }
        }
    }

    fn generate_class_members(&self, members: &[IrNode]) -> TokenStream {
        // Check if any member contains control flow (for loop, if, etc.)
        let has_control_flow = members.iter().any(|n| matches!(n,
            IrNode::For { .. } | IrNode::If { .. } | IrNode::While { .. } |
            IrNode::Match { .. } | IrNode::Let { .. } | IrNode::Do { .. }
        ));

        if has_control_flow {
            // Generate code that dynamically builds the vec
            let member_pushes = self.generate_class_member_pushes(members);
            quote! {
                {
                    let mut __class_members: Vec<macroforge_ts::swc_core::ecma::ast::ClassMember> = Vec::new();
                    #member_pushes
                    __class_members
                }
            }
        } else {
            // No control flow - generate literal vec
            let members_code: Vec<TokenStream> = members
                .iter()
                .filter_map(|m| self.generate_class_member(m))
                .collect();
            quote! { vec![#(#members_code),*] }
        }
    }

    fn generate_class_member_pushes(&self, members: &[IrNode]) -> TokenStream {
        let pushes: Vec<TokenStream> = members
            .iter()
            .filter_map(|m| self.generate_class_member_push(m))
            .collect();
        quote! { #(#pushes)* }
    }

    fn generate_class_member_push(&self, node: &IrNode) -> Option<TokenStream> {
        match node {
            IrNode::For { pattern, iterator, body } => {
                let body_pushes = self.generate_class_member_pushes(body);
                Some(quote! {
                    for #pattern in #iterator { #body_pushes }
                })
            }
            IrNode::If { condition, then_body, else_if_branches, else_body } => {
                let then_pushes = self.generate_class_member_pushes(then_body);

                let else_code = if else_if_branches.is_empty() && else_body.is_none() {
                    quote! {}
                } else {
                    let mut branches = TokenStream::new();
                    for (cond, body) in else_if_branches {
                        let b = self.generate_class_member_pushes(body);
                        branches.extend(quote! { else if #cond { #b } });
                    }
                    if let Some(eb) = else_body {
                        let eb_pushes = self.generate_class_member_pushes(eb);
                        branches.extend(quote! { else { #eb_pushes } });
                    }
                    branches
                };

                Some(quote! {
                    if #condition { #then_pushes } #else_code
                })
            }
            IrNode::While { condition, body } => {
                let body_pushes = self.generate_class_member_pushes(body);
                Some(quote! {
                    while #condition { #body_pushes }
                })
            }
            IrNode::Match { expr, arms } => {
                let arm_tokens: Vec<TokenStream> = arms
                    .iter()
                    .map(|MatchArm { pattern, guard, body }| {
                        let b = self.generate_class_member_pushes(body);
                        if let Some(g) = guard {
                            quote! { #pattern if #g => { #b } }
                        } else {
                            quote! { #pattern => { #b } }
                        }
                    })
                    .collect();
                Some(quote! {
                    match #expr { #(#arm_tokens)* }
                })
            }
            IrNode::Let { pattern, mutable, type_hint, value } => {
                let mutability = if *mutable { quote! { mut } } else { quote! {} };
                if let Some(ty) = type_hint {
                    Some(quote! { let #mutability #pattern: #ty = #value; })
                } else {
                    Some(quote! { let #mutability #pattern = #value; })
                }
            }
            IrNode::Do { code } => Some(quote! { #code }),
            _ => {
                // Regular class member - push to __class_members
                if let Some(member_code) = self.generate_class_member(node) {
                    Some(quote! { __class_members.push(#member_code); })
                } else {
                    None
                }
            }
        }
    }

    fn generate_class_member(&self, node: &IrNode) -> Option<TokenStream> {
        match node {
            IrNode::Constructor { accessibility, params, body } => {
                let params_code = self.generate_constructor_params(params);
                let body_code = body
                    .as_ref()
                    .map(|b| self.generate_block_stmt_opt(b))
                    .unwrap_or(quote! { None });
                let accessibility_code = match accessibility {
                    Some(Accessibility::Public) => quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Public) },
                    Some(Accessibility::Private) => quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Private) },
                    Some(Accessibility::Protected) => quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Protected) },
                    None => quote! { None },
                };
                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::ClassMember::Constructor(
                        macroforge_ts::swc_core::ecma::ast::Constructor {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            key: macroforge_ts::swc_core::ecma::ast::PropName::Ident(
                                macroforge_ts::swc_core::ecma::ast::IdentName::new(
                                    "constructor".into(),
                                    macroforge_ts::swc_core::common::DUMMY_SP,
                                )
                            ),
                            params: #params_code,
                            body: #body_code,
                            accessibility: #accessibility_code,
                            is_optional: false,
                        }
                    )
                })
            }
            IrNode::Method {
                static_,
                accessibility,
                readonly: _,
                async_,
                generator,
                kind,
                name,
                optional,
                type_params,
                params,
                return_type,
                body,
            } => {
                let name_code = self.generate_prop_name(name);
                let params_code = self.generate_params(params);
                let body_code = body
                    .as_ref()
                    .map(|b| self.generate_block_stmt_opt(b))
                    .unwrap_or(quote! { None });
                let return_type_code = return_type
                    .as_ref()
                    .map(|t| {
                        let tc = self.generate_type_ann(t);
                        quote! { Some(Box::new(#tc)) }
                    })
                    .unwrap_or(quote! { None });
                let type_params_code = type_params
                    .as_ref()
                    .map(|tp| {
                        let tpc = self.generate_type_params(tp);
                        quote! { Some(Box::new(#tpc)) }
                    })
                    .unwrap_or(quote! { None });
                let accessibility_code = match accessibility {
                    Some(Accessibility::Public) => quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Public) },
                    Some(Accessibility::Private) => quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Private) },
                    Some(Accessibility::Protected) => quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Protected) },
                    None => quote! { None },
                };
                let kind_code = match kind {
                    MethodKind::Method => quote! { macroforge_ts::swc_core::ecma::ast::MethodKind::Method },
                    MethodKind::Getter => quote! { macroforge_ts::swc_core::ecma::ast::MethodKind::Getter },
                    MethodKind::Setter => quote! { macroforge_ts::swc_core::ecma::ast::MethodKind::Setter },
                };
                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::ClassMember::Method(
                        macroforge_ts::swc_core::ecma::ast::ClassMethod {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            key: #name_code,
                            function: Box::new(macroforge_ts::swc_core::ecma::ast::Function {
                                params: #params_code,
                                decorators: vec![],
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                                body: #body_code,
                                is_generator: #generator,
                                is_async: #async_,
                                type_params: #type_params_code,
                                return_type: #return_type_code,
                            }),
                            kind: #kind_code,
                            is_static: #static_,
                            accessibility: #accessibility_code,
                            is_abstract: false,
                            is_optional: #optional,
                            is_override: false,
                        }
                    )
                })
            }
            IrNode::ClassProp {
                static_,
                accessibility,
                readonly,
                declare,
                optional,
                definite,
                name,
                type_ann,
                value,
            } => {
                let name_code = self.generate_prop_name(name);
                let type_ann_code = type_ann
                    .as_ref()
                    .map(|t| {
                        let tc = self.generate_type_ann(t);
                        quote! { Some(Box::new(#tc)) }
                    })
                    .unwrap_or(quote! { None });
                let value_code = value
                    .as_ref()
                    .map(|v| {
                        let vc = self.generate_expr(v);
                        quote! { Some(Box::new(#vc)) }
                    })
                    .unwrap_or(quote! { None });
                let accessibility_code = match accessibility {
                    Some(Accessibility::Public) => quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Public) },
                    Some(Accessibility::Private) => quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Private) },
                    Some(Accessibility::Protected) => quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Protected) },
                    None => quote! { None },
                };
                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::ClassMember::ClassProp(
                        macroforge_ts::swc_core::ecma::ast::ClassProp {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            key: #name_code,
                            value: #value_code,
                            type_ann: #type_ann_code,
                            is_static: #static_,
                            decorators: vec![],
                            accessibility: #accessibility_code,
                            is_abstract: false,
                            is_optional: #optional,
                            is_override: false,
                            readonly: #readonly,
                            declare: #declare,
                            definite: #definite,
                        }
                    )
                })
            }
            _ => None,
        }
    }

    fn generate_constructor_params(&self, params: &[IrNode]) -> TokenStream {
        let params_code: Vec<TokenStream> = params.iter().filter_map(|p| {
            let param_code = self.generate_param(p);
            Some(quote! {
                macroforge_ts::swc_core::ecma::ast::ParamOrTsParamProp::Param(#param_code)
            })
        }).collect();
        quote! { vec![#(#params_code),*] }
    }

    fn generate_ts_interface_body(&self, body: &[IrNode]) -> TokenStream {
        // Check if body has any control flow
        let has_control_flow = body.iter().any(|n| matches!(n, IrNode::For { .. } | IrNode::If { .. } | IrNode::While { .. } | IrNode::Match { .. }));

        if has_control_flow {
            // Generate code that builds the body dynamically
            let body_stmts: Vec<TokenStream> = body.iter().filter_map(|node| {
                self.generate_interface_member_stmt(node)
            }).collect();

            quote! {
                macroforge_ts::swc_core::ecma::ast::TsInterfaceBody {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    body: {
                        let mut __members: Vec<macroforge_ts::swc_core::ecma::ast::TsTypeElement> = Vec::new();
                        #(#body_stmts)*
                        __members
                    },
                }
            }
        } else {
            // Static body - generate directly
            let members: Vec<TokenStream> = body.iter().filter_map(|node| {
                self.generate_interface_member(node)
            }).collect();

            quote! {
                macroforge_ts::swc_core::ecma::ast::TsInterfaceBody {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    body: vec![#(#members),*],
                }
            }
        }
    }

    fn generate_interface_member_stmt(&self, node: &IrNode) -> Option<TokenStream> {
        match node {
            IrNode::For { pattern, iterator, body } => {
                let body_stmts: Vec<TokenStream> = body.iter().filter_map(|n| {
                    self.generate_interface_member_stmt(n)
                }).collect();

                Some(quote! {
                    for #pattern in #iterator {
                        #(#body_stmts)*
                    }
                })
            }
            IrNode::If { condition, then_body, else_if_branches, else_body } => {
                let then_stmts: Vec<TokenStream> = then_body.iter().filter_map(|n| {
                    self.generate_interface_member_stmt(n)
                }).collect();

                let else_if_code: Vec<TokenStream> = else_if_branches.iter().map(|(cond, body)| {
                    let branch_stmts: Vec<TokenStream> = body.iter().filter_map(|n| {
                        self.generate_interface_member_stmt(n)
                    }).collect();
                    quote! {
                        else if #cond {
                            #(#branch_stmts)*
                        }
                    }
                }).collect();

                let else_code = else_body.as_ref().map(|body| {
                    let else_stmts: Vec<TokenStream> = body.iter().filter_map(|n| {
                        self.generate_interface_member_stmt(n)
                    }).collect();
                    quote! {
                        else {
                            #(#else_stmts)*
                        }
                    }
                });

                Some(quote! {
                    if #condition {
                        #(#then_stmts)*
                    }
                    #(#else_if_code)*
                    #else_code
                })
            }
            IrNode::PropSignature { readonly, name, optional, type_ann } => {
                let member_code = self.generate_prop_signature(*readonly, name, *optional, type_ann.as_deref());
                Some(quote! {
                    __members.push(#member_code);
                })
            }
            IrNode::MethodSignature { name, optional, type_params, params, return_type } => {
                let member_code = self.generate_method_signature(name, *optional, type_params.as_deref(), params, return_type.as_deref());
                Some(quote! {
                    __members.push(#member_code);
                })
            }
            IrNode::Raw(_) => None, // Skip whitespace/raw text
            _ => None,
        }
    }

    fn generate_interface_member(&self, node: &IrNode) -> Option<TokenStream> {
        match node {
            IrNode::PropSignature { readonly, name, optional, type_ann } => {
                Some(self.generate_prop_signature(*readonly, name, *optional, type_ann.as_deref()))
            }
            IrNode::MethodSignature { name, optional, type_params, params, return_type } => {
                Some(self.generate_method_signature(name, *optional, type_params.as_deref(), params, return_type.as_deref()))
            }
            _ => None,
        }
    }

    fn generate_prop_signature(&self, readonly: bool, name: &IrNode, optional: bool, type_ann: Option<&IrNode>) -> TokenStream {
        let name_code = self.generate_expr(name);
        let type_ann_code = type_ann.map(|t| {
            let tc = self.generate_type(t);
            quote! {
                Some(Box::new(macroforge_ts::swc_core::ecma::ast::TsTypeAnn {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    type_ann: Box::new(#tc),
                }))
            }
        }).unwrap_or(quote! { None });

        quote! {
            macroforge_ts::swc_core::ecma::ast::TsTypeElement::TsPropertySignature(
                macroforge_ts::swc_core::ecma::ast::TsPropertySignature {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    readonly: #readonly,
                    key: Box::new(#name_code),
                    computed: false,
                    optional: #optional,
                    type_ann: #type_ann_code,
                }
            )
        }
    }

    fn generate_method_signature(&self, name: &IrNode, optional: bool, type_params: Option<&IrNode>, params: &[IrNode], return_type: Option<&IrNode>) -> TokenStream {
        let name_code = self.generate_expr(name);
        let type_params_code = type_params.map(|tp| {
            let tpc = self.generate_type_params(tp);
            quote! { Some(Box::new(#tpc)) }
        }).unwrap_or(quote! { None });
        let params_code = self.generate_params(params);
        let return_type_code = return_type.map(|rt| {
            let rtc = self.generate_type(rt);
            quote! {
                Some(Box::new(macroforge_ts::swc_core::ecma::ast::TsTypeAnn {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    type_ann: Box::new(#rtc),
                }))
            }
        }).unwrap_or(quote! { None });

        quote! {
            macroforge_ts::swc_core::ecma::ast::TsTypeElement::TsMethodSignature(
                macroforge_ts::swc_core::ecma::ast::TsMethodSignature {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    key: Box::new(#name_code),
                    computed: false,
                    optional: #optional,
                    type_params: #type_params_code,
                    params: #params_code,
                    type_ann: #return_type_code,
                }
            )
        }
    }

    fn generate_implements(&self, _implements: &[IrNode]) -> TokenStream {
        quote! { vec![] }
    }

    fn generate_ts_expr_with_type_args(&self, _extends: &[IrNode]) -> TokenStream {
        quote! { vec![] }
    }

    fn generate_props(&self, props: &[IrNode]) -> TokenStream {
        let props_code: Vec<TokenStream> = props.iter().filter_map(|p| self.generate_prop(p)).collect();
        quote! { vec![#(#props_code),*] }
    }

    fn generate_prop(&self, node: &IrNode) -> Option<TokenStream> {
        match node {
            IrNode::KeyValueProp { key, value } => {
                let key_code = self.generate_prop_name(key);
                let value_code = self.generate_expr(value);
                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::PropOrSpread::Prop(Box::new(
                        macroforge_ts::swc_core::ecma::ast::Prop::KeyValue(
                            macroforge_ts::swc_core::ecma::ast::KeyValueProp {
                                key: #key_code,
                                value: Box::new(#value_code),
                            }
                        )
                    ))
                })
            }
            IrNode::ShorthandProp { key } => {
                let key_code = self.generate_ident(key);
                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::PropOrSpread::Prop(Box::new(
                        macroforge_ts::swc_core::ecma::ast::Prop::Shorthand(#key_code)
                    ))
                })
            }
            IrNode::SpreadElement { expr } => {
                let expr_code = self.generate_expr(expr);
                Some(quote! {
                    macroforge_ts::swc_core::ecma::ast::PropOrSpread::Spread(
                        macroforge_ts::swc_core::ecma::ast::SpreadElement {
                            dot3_token: macroforge_ts::swc_core::common::DUMMY_SP,
                            expr: Box::new(#expr_code),
                        }
                    )
                })
            }
            // For control flow in object literals
            IrNode::If { .. } | IrNode::For { .. } => None, // Handled at higher level
            _ => None,
        }
    }

    fn generate_prop_name(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::Ident(name) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::PropName::Ident(
                        macroforge_ts::swc_core::ecma::ast::IdentName::new(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        )
                    )
                }
            }
            IrNode::StrLit(value) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::PropName::Str(
                        macroforge_ts::swc_core::ecma::ast::Str {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: #value.into(),
                            raw: None,
                        }
                    )
                }
            }
            IrNode::ComputedPropName { expr } => {
                let expr_code = self.generate_expr(expr);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::PropName::Computed(
                        macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            expr: Box::new(#expr_code),
                        }
                    )
                }
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
            } => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::PropName::Ident({
                        let __ident = macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(#expr);
                        macroforge_ts::swc_core::ecma::ast::IdentName::new(
                            __ident.sym,
                            __ident.span,
                        )
                    })
                }
            }
            _ => quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Ident(
                    macroforge_ts::swc_core::ecma::ast::IdentName::new(
                        "".into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                )
            },
        }
    }

    fn generate_assign_target(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::Ident(name) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::AssignTarget::Simple(
                        macroforge_ts::swc_core::ecma::ast::SimpleAssignTarget::Ident(
                            macroforge_ts::swc_core::ecma::ast::BindingIdent {
                                id: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                    #name.into(),
                                    macroforge_ts::swc_core::common::DUMMY_SP,
                                ),
                                type_ann: None,
                            }
                        )
                    )
                }
            }
            IrNode::MemberExpr { obj, prop, computed } => {
                let obj_code = self.generate_expr(obj);
                let prop_code = if *computed {
                    let p = self.generate_expr(prop);
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::MemberProp::Computed(
                            macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(#p),
                            }
                        )
                    }
                } else {
                    let ident_code = self.generate_ident_name(prop);
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::MemberProp::Ident(#ident_code)
                    }
                };

                quote! {
                    macroforge_ts::swc_core::ecma::ast::AssignTarget::Simple(
                        macroforge_ts::swc_core::ecma::ast::SimpleAssignTarget::Member(
                            macroforge_ts::swc_core::ecma::ast::MemberExpr {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                obj: Box::new(#obj_code),
                                prop: #prop_code,
                            }
                        )
                    )
                }
            }
            _ => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::AssignTarget::Simple(
                        macroforge_ts::swc_core::ecma::ast::SimpleAssignTarget::Invalid(
                            macroforge_ts::swc_core::ecma::ast::Invalid {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                            }
                        )
                    )
                }
            }
        }
    }

    fn generate_binary_op(&self, op: &BinaryOp) -> TokenStream {
        match op {
            BinaryOp::Add => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Add },
            BinaryOp::Sub => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Sub },
            BinaryOp::Mul => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Mul },
            BinaryOp::Div => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Div },
            BinaryOp::Mod => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Mod },
            BinaryOp::Exp => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Exp },
            BinaryOp::EqEq => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::EqEq },
            BinaryOp::NotEq => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::NotEq },
            BinaryOp::EqEqEq => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::EqEqEq },
            BinaryOp::NotEqEq => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::NotEqEq },
            BinaryOp::Lt => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Lt },
            BinaryOp::Le => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::LtEq },
            BinaryOp::Gt => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Gt },
            BinaryOp::Ge => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::GtEq },
            BinaryOp::And => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::LogicalAnd },
            BinaryOp::Or => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::LogicalOr },
            BinaryOp::NullishCoalesce => {
                quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::NullishCoalescing }
            }
            BinaryOp::BitAnd => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::BitAnd },
            BinaryOp::BitOr => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::BitOr },
            BinaryOp::BitXor => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::BitXor },
            BinaryOp::Shl => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::LShift },
            BinaryOp::Shr => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::RShift },
            BinaryOp::UShr => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::ZeroFillRShift },
            BinaryOp::In => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::In },
            BinaryOp::InstanceOf => {
                quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::InstanceOf }
            }
        }
    }

    fn generate_assign_op(&self, op: &AssignOp) -> TokenStream {
        match op {
            AssignOp::Assign => quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::Assign },
            AssignOp::AddAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::AddAssign }
            }
            AssignOp::SubAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::SubAssign }
            }
            AssignOp::MulAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::MulAssign }
            }
            AssignOp::DivAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::DivAssign }
            }
            AssignOp::ModAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::ModAssign }
            }
            AssignOp::ExpAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::ExpAssign }
            }
            AssignOp::ShlAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::LShiftAssign }
            }
            AssignOp::ShrAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::RShiftAssign }
            }
            AssignOp::UShrAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::ZeroFillRShiftAssign }
            }
            AssignOp::BitAndAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::BitAndAssign }
            }
            AssignOp::BitOrAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::BitOrAssign }
            }
            AssignOp::BitXorAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::BitXorAssign }
            }
            AssignOp::AndAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::AndAssign }
            }
            AssignOp::OrAssign => quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::OrAssign },
            AssignOp::NullishAssign => {
                quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::NullishAssign }
            }
        }
    }
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

// Suppress unused warnings for now - these will be used when full implementation is complete
#[allow(dead_code)]
fn _unused_variants() {
    let _ = UnaryOp::Minus;
    let _ = UpdateOp::Increment;
    let _ = Accessibility::Public;
    let _ = MethodKind::Method;
}
