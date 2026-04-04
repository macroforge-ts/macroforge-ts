use super::error::{GenError, GenErrorKind, GenResult};
use super::*;

impl Codegen {
    /// Generate a BlockStmt from an IrNode (for function bodies)
    pub(in super::super) fn generate_block_stmt(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::BlockStmt { stmts, .. } => {
                let stmts_code = self.generate_stmts_vec(stmts)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::BlockStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        stmts: #stmts_code,
                    }
                })
            }
            _ => {
                // Fallback: wrap single statement in a block
                let stmt_code = self.generate_stmt(node)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::BlockStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        stmts: vec![#stmt_code],
                    }
                })
            }
        }
    }

    pub(in super::super) fn generate_block_stmt_opt(
        &self,
        node: &IrNode,
    ) -> GenResult<TokenStream> {
        match node {
            IrNode::BlockStmt { stmts, .. } => {
                let stmts_code = self.generate_stmts_vec(stmts)?;
                Ok(quote! {
                    Some(macroforge_ts::swc_core::ecma::ast::BlockStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        stmts: #stmts_code,
                    })
                })
            }
            _ => Err(GenError::unexpected_node(
                "optional block statement",
                node,
                &["BlockStmt"],
            )),
        }
    }

    pub(in super::super) fn generate_block_stmt_or_expr(
        &self,
        node: &IrNode,
    ) -> GenResult<TokenStream> {
        match node {
            IrNode::BlockStmt { stmts, .. } => {
                let stmts_code = self.generate_stmts_vec(stmts)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::BlockStmtOrExpr::BlockStmt(
                        macroforge_ts::swc_core::ecma::ast::BlockStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            stmts: #stmts_code,
                        }
                    )
                })
            }
            _ => {
                let expr_code = self.generate_expr(node)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::BlockStmtOrExpr::Expr(Box::new(#expr_code))
                })
            }
        }
    }

    pub(in super::super) fn generate_stmts_vec(&self, nodes: &[IrNode]) -> GenResult<TokenStream> {
        // Check if any node requires dynamic generation (control flow)
        let has_control_flow = nodes.iter().any(|n| {
            matches!(
                n,
                IrNode::For { .. }
                    | IrNode::If { .. }
                    | IrNode::While { .. }
                    | IrNode::Match { .. }
                    | IrNode::Let { .. }
                    | IrNode::Do { .. }
            )
        });

        if has_control_flow {
            // Generate code that dynamically builds the vec
            let stmt_pushes = self.generate_stmt_pushes(nodes)?;
            Ok(quote! {
                {
                    let mut __body_stmts: Vec<macroforge_ts::swc_core::ecma::ast::Stmt> = Vec::new();
                    #stmt_pushes
                    __body_stmts
                }
            })
        } else {
            // No control flow - generate literal vec
            let stmts: Vec<TokenStream> = nodes
                .iter()
                .map(|n| self.generate_stmt(n))
                .collect::<GenResult<_>>()?;
            Ok(quote! { vec![#(#stmts),*] })
        }
    }

    pub(in super::super) fn generate_stmt_pushes(
        &self,
        nodes: &[IrNode],
    ) -> GenResult<TokenStream> {
        // Control block bodies now produce structured IR (statements or interface members)
        // so we can generate each node directly
        let mut pushes = Vec::new();

        for node in nodes {
            pushes.push(self.generate_stmt_push(node)?);
        }

        Ok(quote! { #(#pushes)* })
    }

    pub(in super::super) fn generate_stmt_push(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::For {
                pattern,
                iterator,
                body,
                ..
            } => {
                let body_pushes = self.generate_stmt_pushes(body)?;
                Ok(quote! {
                    for #pattern in #iterator { #body_pushes }
                })
            }
            IrNode::If {
                condition,
                then_body,
                else_if_branches,
                else_body,
                ..
            } => {
                let then_pushes = self.generate_stmt_pushes(then_body)?;

                let else_code = if else_if_branches.is_empty() && else_body.is_none() {
                    quote! {}
                } else {
                    let mut branches = TokenStream::new();
                    for (cond, body) in else_if_branches {
                        let b = self.generate_stmt_pushes(body)?;
                        branches.extend(quote! { else if #cond { #b } });
                    }
                    if let Some(eb) = else_body {
                        let eb_pushes = self.generate_stmt_pushes(eb)?;
                        branches.extend(quote! { else { #eb_pushes } });
                    }
                    branches
                };

                Ok(quote! {
                    if #condition { #then_pushes } #else_code
                })
            }
            IrNode::While {
                condition, body, ..
            } => {
                let body_pushes = self.generate_stmt_pushes(body)?;
                Ok(quote! {
                    while #condition { #body_pushes }
                })
            }
            IrNode::Match { expr, arms, .. } => {
                let arm_tokens: Vec<TokenStream> = arms
                    .iter()
                    .map(
                        |MatchArm {
                             span,
                             pattern,
                             guard,
                             body,
                         }| {
                            let b = self
                                .generate_stmt_pushes(body)
                                .map_err(|e| e.with_context("match arm body").with_span(*span))?;
                            if let Some(g) = guard {
                                Ok(quote! { #pattern if #g => { #b } })
                            } else {
                                Ok(quote! { #pattern => { #b } })
                            }
                        },
                    )
                    .collect::<GenResult<_>>()?;
                Ok(quote! {
                    match #expr { #(#arm_tokens)* }
                })
            }
            IrNode::Let {
                pattern,
                mutable,
                type_hint,
                value,
                ..
            } => {
                let mutability = if *mutable {
                    quote! { mut }
                } else {
                    quote! {}
                };
                if let Some(ty) = type_hint {
                    Ok(quote! { let #mutability #pattern: #ty = #value; })
                } else {
                    Ok(quote! { let #mutability #pattern = #value; })
                }
            }
            IrNode::Do { code, .. } => Ok(quote! { #code; }),
            _ => {
                // Regular statement - push to __body_stmts
                let stmt_code = self.generate_stmt(node)?;
                Ok(quote! { __body_stmts.push(#stmt_code); })
            }
        }
    }

    pub(in super::super) fn generate_stmt(&self, node: &IrNode) -> GenResult<TokenStream> {
        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_AST").is_ok() {
            eprintln!(
                "[MF_DEBUG_AST] generate_stmt: {:?}",
                std::mem::discriminant(node)
            );
        }

        match node {
        IrNode::EmptyStmt { .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::Empty(
                    macroforge_ts::swc_core::ecma::ast::EmptyStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                    }
                )
            })
        }
        IrNode::ExprStmt { expr, .. } => {
            let expr_code = self.generate_expr(expr)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                    macroforge_ts::swc_core::ecma::ast::ExprStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            })
        }
        IrNode::ReturnStmt { arg, .. } => {
            let arg_code = match arg.as_ref() {
                Some(a) => {
                    let ac = self.generate_expr(a)?;
                    quote! { Some(Box::new(#ac)) }
                }
                None => quote! { None },
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::Return(
                    macroforge_ts::swc_core::ecma::ast::ReturnStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: #arg_code,
                    }
                )
            })
        }
        IrNode::ThrowStmt { arg, .. } => {
            let arg_code = self.generate_expr(arg)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::Throw(
                    macroforge_ts::swc_core::ecma::ast::ThrowStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: Box::new(#arg_code),
                    }
                )
            })
        }
        IrNode::BlockStmt { stmts, .. } => {
            let stmts_code = self.generate_stmts_vec(stmts)?;
            Ok(quote! {
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
            ..
        } => {
            let kind_code = match kind {
                VarKind::Const => {
                    quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Const }
                }
                VarKind::Let => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Let },
                VarKind::Var => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Var },
            };
            let decls_code = self.generate_var_declarators(decls)?;

            Ok(quote! {
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
        // TypeScript if statement - direct AST generation
        IrNode::TsIfStmt { test, cons, alt, .. } => {
            let test_code = self.generate_expr(test)?;
            let cons_code = self.generate_stmt(cons)?;
            let alt_code = match alt.as_ref() {
                Some(a) => {
                    let ac = self.generate_stmt(a)?;
                    quote! { Some(Box::new(#ac)) }
                }
                None => quote! { None },
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::If(
                    macroforge_ts::swc_core::ecma::ast::IfStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        test: Box::new(#test_code),
                        cons: Box::new(#cons_code),
                        alt: #alt_code,
                    }
                )
            })
        }
        // Structured for-in statement
        IrNode::ForInStmt { left, right, body, .. } => {
            let left_code = self.generate_for_head(left)?;
            let right_code = self.generate_expr(right)?;
            let body_code = self.generate_stmt(body)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::ForIn(
                    macroforge_ts::swc_core::ecma::ast::ForInStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        left: #left_code,
                        right: Box::new(#right_code),
                        body: Box::new(#body_code),
                    }
                )
            })
        }
        // Structured for-of statement
        IrNode::ForOfStmt { await_, left, right, body, .. } => {
            let left_code = self.generate_for_head(left)?;
            let right_code = self.generate_expr(right)?;
            let body_code = self.generate_stmt(body)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::ForOf(
                    macroforge_ts::swc_core::ecma::ast::ForOfStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        is_await: #await_,
                        left: #left_code,
                        right: Box::new(#right_code),
                        body: Box::new(#body_code),
                    }
                )
            })
        }
        // C-style for statement: for (init; test; update) body
        IrNode::TsForStmt { init, test, update, body, .. } => {
            let init_code = match init.as_ref() {
                Some(init_node) => {
                    match init_node.as_ref() {
                        IrNode::VarDecl { kind, decls, declare, .. } => {
                            let kind_code = match kind {
                                VarKind::Const => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Const },
                                VarKind::Let => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Let },
                                VarKind::Var => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Var },
                            };
                            let decls_code = self.generate_var_declarators(decls)?;
                            quote! {
                                Some(macroforge_ts::swc_core::ecma::ast::VarDeclOrExpr::VarDecl(Box::new(
                                    macroforge_ts::swc_core::ecma::ast::VarDecl {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                                        kind: #kind_code,
                                        declare: #declare,
                                        decls: #decls_code,
                                    }
                                )))
                            }
                        }
                        _ => {
                            let expr_code = self.generate_expr(init_node)?;
                            quote! {
                                Some(macroforge_ts::swc_core::ecma::ast::VarDeclOrExpr::Expr(Box::new(#expr_code)))
                            }
                        }
                    }
                }
                None => quote! { None },
            };
            let test_code = match test.as_ref() {
                Some(t) => {
                    let tc = self.generate_expr(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };
            let update_code = match update.as_ref() {
                Some(u) => {
                    let uc = self.generate_expr(u)?;
                    quote! { Some(Box::new(#uc)) }
                }
                None => quote! { None },
            };
            let body_code = self.generate_stmt(body)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::For(
                    macroforge_ts::swc_core::ecma::ast::ForStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        init: #init_code,
                        test: #test_code,
                        update: #update_code,
                        body: Box::new(#body_code),
                    }
                )
            })
        }
        // While statement: while (test) body
        IrNode::TsWhileStmt { test, body, .. } => {
            let test_code = self.generate_expr(test)?;
            let body_code = self.generate_stmt(body)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::While(
                    macroforge_ts::swc_core::ecma::ast::WhileStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        test: Box::new(#test_code),
                        body: Box::new(#body_code),
                    }
                )
            })
        }
        // Do-while statement: do body while (test)
        IrNode::TsDoWhileStmt { body, test, .. } => {
            let body_code = self.generate_stmt(body)?;
            let test_code = self.generate_expr(test)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::DoWhile(
                    macroforge_ts::swc_core::ecma::ast::DoWhileStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        body: Box::new(#body_code),
                        test: Box::new(#test_code),
                    }
                )
            })
        }
        // Try-catch-finally statement
        IrNode::TsTryStmt { block, handler, finalizer, .. } => {
            let block_code = self.generate_block_stmt(block)?;

            let handler_code = match handler {
                Some(catch_clause) => {
                    let param_code = match &catch_clause.param {
                        Some(p) => {
                            let pat_code = self.generate_pat(p)?;
                            quote! { Some(#pat_code) }
                        }
                        None => quote! { None },
                    };
                    let body_code = self.generate_block_stmt(&catch_clause.body)?;
                    quote! {
                        Some(macroforge_ts::swc_core::ecma::ast::CatchClause {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            param: #param_code,
                            body: #body_code,
                        })
                    }
                }
                None => quote! { None },
            };

            let finalizer_code = match finalizer {
                Some(f) => {
                    let fc = self.generate_block_stmt(f)?;
                    quote! { Some(#fc) }
                }
                None => quote! { None },
            };

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Stmt::Try(Box::new(
                    macroforge_ts::swc_core::ecma::ast::TryStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        block: #block_code,
                        handler: #handler_code,
                        finalizer: #finalizer_code,
                    }
                ))
            })
        }
        // TypeScript injection - the stream is a Rust expression that produces a Stmt
        IrNode::TypeScript { stream, .. } => {
            Ok(quote! { #stream })
        }
        // Control flow that generates Rust code - these are not TS statements
        IrNode::If { .. } | IrNode::For { .. } | IrNode::While { .. } | IrNode::Match { .. } => {
            Err(GenError::new(GenErrorKind::UnsupportedControlFlowPosition)
                .with_context("statement")
                .with_ir_node(node)
                .with_help("Rust control flow nodes (If, For, While, Match) cannot be used as direct TypeScript statements. They generate Rust code that produces TS statements."))
        }
        _ => {
            // Try to generate as expression statement
            if let Some(expr_code) = self.try_generate_as_expr(node)? {
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                        macroforge_ts::swc_core::ecma::ast::ExprStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            expr: Box::new(#expr_code),
                        }
                    )
                })
            } else {
                Err(GenError::unexpected_node(
                    "statement",
                    node,
                    &["ExprStmt", "ReturnStmt", "BlockStmt", "VarDecl", "TsIfStmt", "ForInStmt", "ForOfStmt", "TsForStmt", "TsWhileStmt", "TsDoWhileStmt", "TsTryStmt"],
                ))
            }
        }
    }
    }

    /// Generate code for the left-hand side of a for-in/for-of loop.
    /// Returns a ForHead which can be either a VarDecl or a Pat.
    pub(in super::super) fn generate_for_head(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::VarDecl {
                kind,
                decls,
                declare,
                ..
            } => {
                let kind_code = match kind {
                    VarKind::Const => {
                        quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Const }
                    }
                    VarKind::Let => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Let },
                    VarKind::Var => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Var },
                };
                let decls_code = self.generate_var_declarators(decls)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::ForHead::VarDecl(Box::new(
                        macroforge_ts::swc_core::ecma::ast::VarDecl {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            kind: #kind_code,
                            declare: #declare,
                            decls: #decls_code,
                        }
                    ))
                })
            }
            // For patterns (destructuring without var/let/const)
            _ => {
                let pat_code = self.generate_pat(node)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::ForHead::Pat(Box::new(#pat_code))
                })
            }
        }
    }
}
