use super::*;

impl Codegen {
    /// Generate a BlockStmt from an IrNode (for function bodies)
    pub(in super::super) fn generate_block_stmt(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::BlockStmt { stmts } => {
                let stmts_code = self.generate_stmts_vec(stmts);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::BlockStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        stmts: #stmts_code,
                    }
                }
            }
            _ => {
                // Fallback: wrap single statement in a block
                if let Some(stmt_code) = self.generate_stmt(node) {
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::BlockStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            stmts: vec![#stmt_code],
                        }
                    }
                } else {
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::BlockStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            stmts: vec![],
                        }
                    }
                }
            }
        }
    }

    pub(in super::super) fn generate_block_stmt_opt(&self, node: &IrNode) -> TokenStream {
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

pub(in super::super) fn generate_block_stmt_or_expr(&self, node: &IrNode) -> TokenStream {
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

pub(in super::super) fn generate_stmts_vec(&self, nodes: &[IrNode]) -> TokenStream {
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
        let stmts: Vec<TokenStream> = nodes.iter().filter_map(|n| self.generate_stmt(n)).collect();
        quote! { vec![#(#stmts),*] }
    }
}

pub(in super::super) fn generate_stmt_pushes(&self, nodes: &[IrNode]) -> TokenStream {
    // Group adjacent non-control-flow nodes into combined statements
    let mut pushes = Vec::new();
    let mut pending_nodes: Vec<&IrNode> = Vec::new();

    for node in nodes {
        if self.is_control_flow_node(node) || self.is_structured_stmt(node) {
            // Flush pending nodes as combined statement
            if !pending_nodes.is_empty() {
                if let Some(stmt) = self.generate_combined_stmt(&pending_nodes) {
                    pushes.push(stmt);
                }
                pending_nodes.clear();
            }
            // Generate control flow / structured statement directly
            if let Some(push) = self.generate_stmt_push(node) {
                pushes.push(push);
            }
        } else {
            pending_nodes.push(node);
        }
    }

    // Flush remaining pending nodes
    if !pending_nodes.is_empty() {
        if let Some(stmt) = self.generate_combined_stmt(&pending_nodes) {
            pushes.push(stmt);
        }
    }

    quote! { #(#pushes)* }
}

pub(in super::super) fn is_structured_stmt(&self, node: &IrNode) -> bool {
    matches!(
        node,
        IrNode::VarDecl { .. }
            | IrNode::ReturnStmt { .. }
            | IrNode::BlockStmt { .. }
            | IrNode::ExprStmt { .. }
            | IrNode::TsIfStmt { .. }
            | IrNode::TsLoopStmt { .. }
            | IrNode::ForInStmt { .. }
            | IrNode::ForOfStmt { .. }
    )
}

pub(in super::super) fn generate_combined_stmt(&self, nodes: &[&IrNode]) -> Option<TokenStream> {
    if nodes.is_empty() {
        return None;
    }

    // Check if all nodes are whitespace-only Raw nodes
    let all_whitespace = nodes
        .iter()
        .all(|n| matches!(n, IrNode::Raw(text) if text.trim().is_empty()));
    if all_whitespace {
        return None;
    }

    // Generate code that builds a statement string and parses it
    let part_exprs: Vec<TokenStream> = nodes
        .iter()
        .map(|n| self.generate_stmt_string_part(n))
        .collect();

    Some(quote! {
        {
            let mut __stmt_str = String::new();
            #(#part_exprs)*
            if !__stmt_str.trim().is_empty() {
                if let Ok(__parsed) = macroforge_ts::ts_syn::parse_ts_stmt(&__stmt_str) {
                    __body_stmts.push(__parsed);
                }
            }
        }
    })
}

pub(in super::super) fn generate_stmt_string_part(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::Raw(text) => quote! { __stmt_str.push_str(#text); },
        IrNode::StrLit(text) => quote! { __stmt_str.push_str(#text); },
        IrNode::Ident(name) => quote! { __stmt_str.push_str(#name); },
        IrNode::IdentBlock { parts } => {
            // Recursively process each part
            let part_strs: Vec<TokenStream> = parts
                .iter()
                .map(|p| self.generate_stmt_string_part(p))
                .collect();
            quote! { #(#part_strs)* }
        }
        IrNode::StringInterp { quote: q, parts } => {
            let quote_char = q.to_string();
            let inner_parts: Vec<TokenStream> = parts
                .iter()
                .map(|p| match p {
                    IrNode::Raw(text) | IrNode::StrLit(text) => {
                        quote! { __stmt_str.push_str(#text); }
                    }
                    IrNode::Placeholder { expr, .. } => {
                        quote! { __stmt_str.push_str(&(#expr).to_string()); }
                    }
                    _ => self.generate_stmt_string_part(p),
                })
                .collect();
            quote! {
                __stmt_str.push_str(#quote_char);
                #(#inner_parts)*
                __stmt_str.push_str(#quote_char);
            }
        }
        IrNode::Placeholder { kind, expr } => match kind {
            PlaceholderKind::Expr => {
                quote! {
                    let __expr = macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone());
                    __stmt_str.push_str(&macroforge_ts::ts_syn::emit_expr(&__expr));
                }
            }
            PlaceholderKind::Ident => {
                quote! {
                    __stmt_str.push_str(&macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()).sym.to_string());
                }
            }
            PlaceholderKind::Type => {
                quote! {
                    let __ty = macroforge_ts::ts_syn::ToTsType::to_ts_type((#expr).clone());
                    __stmt_str.push_str(&macroforge_ts::ts_syn::emit_ts_type(&__ty));
                }
            }
            _ => quote! {},
        },
        // Control flow nodes - generate Rust code that builds the string dynamically
        IrNode::For {
            pattern,
            iterator,
            body,
        } => {
            let body_parts: Vec<TokenStream> = body
                .iter()
                .map(|n| self.generate_stmt_string_part(n))
                .collect();
            quote! {
                for #pattern in #iterator {
                    #(#body_parts)*
                }
            }
        }
        IrNode::If {
            condition,
            then_body,
            else_if_branches,
            else_body,
        } => {
            let then_parts: Vec<TokenStream> = then_body
                .iter()
                .map(|n| self.generate_stmt_string_part(n))
                .collect();

            let else_ifs: Vec<TokenStream> = else_if_branches
                .iter()
                .map(|(c, body)| {
                    let b: Vec<TokenStream> = body
                        .iter()
                        .map(|n| self.generate_stmt_string_part(n))
                        .collect();
                    quote! { else if #c { #(#b)* } }
                })
                .collect();

            let else_part = else_body.as_ref().map(|body| {
                let b: Vec<TokenStream> = body
                    .iter()
                    .map(|n| self.generate_stmt_string_part(n))
                    .collect();
                quote! { else { #(#b)* } }
            });

            quote! {
                if #condition { #(#then_parts)* }
                #(#else_ifs)*
                #else_part
            }
        }
        IrNode::While { condition, body } => {
            let body_parts: Vec<TokenStream> = body
                .iter()
                .map(|n| self.generate_stmt_string_part(n))
                .collect();
            quote! {
                while #condition {
                    #(#body_parts)*
                }
            }
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
                quote! { let #mutability #pattern: #ty = #value; }
            } else {
                quote! { let #mutability #pattern = #value; }
            }
        }
        IrNode::Do { code } => quote! { #code; },
        // TypeScript statement nodes - build as strings
        IrNode::VarDecl {
            exported: _,
            declare: _,
            kind,
            decls,
        } => {
            let kind_str = match kind {
                VarKind::Const => "const",
                VarKind::Let => "let",
                VarKind::Var => "var",
            };
            let decl_parts: Vec<TokenStream> = decls
                .iter()
                .map(|decl| {
                    let name_parts = self.generate_stmt_string_part(&decl.name);
                    let type_part = decl.type_ann.as_ref().map(|t| {
                        let tp = self.generate_stmt_string_part(t);
                        quote! {
                            __stmt_str.push_str(": ");
                            #tp
                        }
                    });
                    let init_part = decl.init.as_ref().map(|i| {
                        let ip = self.generate_stmt_string_part(i);
                        quote! {
                            __stmt_str.push_str(" = ");
                            #ip
                        }
                    });
                    quote! {
                        #name_parts
                        #type_part
                        #init_part
                    }
                })
                .collect();

            quote! {
                __stmt_str.push_str(#kind_str);
                __stmt_str.push_str(" ");
                #(#decl_parts)*
                __stmt_str.push_str(";");
            }
        }
        IrNode::ExprStmt { expr } => {
            let expr_parts = self.generate_stmt_string_part(expr);
            quote! {
                #expr_parts
                __stmt_str.push_str(";");
            }
        }
        IrNode::ReturnStmt { arg } => {
            if let Some(a) = arg {
                let arg_parts = self.generate_stmt_string_part(a);
                quote! {
                    __stmt_str.push_str("return ");
                    #arg_parts
                    __stmt_str.push_str(";");
                }
            } else {
                quote! {
                    __stmt_str.push_str("return;");
                }
            }
        }
        IrNode::BlockStmt { stmts } => {
            let stmt_parts: Vec<TokenStream> = stmts
                .iter()
                .map(|s| self.generate_stmt_string_part(s))
                .collect();
            quote! {
                __stmt_str.push_str("{ ");
                #(#stmt_parts)*
                __stmt_str.push_str(" }");
            }
        }
        IrNode::TsIfStmt { test, cons, alt } => {
            let test_parts = self.generate_stmt_string_part(test);
            let cons_parts = self.generate_stmt_string_part(cons);
            let alt_parts = alt.as_ref().map(|a| {
                let ac = self.generate_stmt_string_part(a);
                quote! {
                    __stmt_str.push_str(" else ");
                    #ac
                }
            });
            quote! {
                __stmt_str.push_str("if (");
                #test_parts
                __stmt_str.push_str(") ");
                #cons_parts
                #alt_parts
            }
        }
        IrNode::TsLoopStmt { parts } => {
            let part_exprs: Vec<TokenStream> = parts
                .iter()
                .map(|p| self.generate_stmt_string_part(p))
                .collect();
            quote! { #(#part_exprs)* }
        }
        _ => quote! {},
    }
}
/// Generate code that builds a statement string from an IrNode.
    pub(in super::super) fn generate_stmt_as_string(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::BlockStmt { stmts } => {
            let part_exprs: Vec<TokenStream> = stmts
                .iter()
                .map(|s| self.generate_stmt_string_part(s))
                .collect();
            quote! {
                __stmt_str.push_str("{ ");
                #(#part_exprs)*
                __stmt_str.push_str(" }");
            }
        }
        IrNode::ExprStmt { expr } => {
            let expr_parts = self.generate_stmt_string_part(expr);
            quote! {
                #expr_parts
                __stmt_str.push_str(";");
            }
        }
        IrNode::ReturnStmt { arg } => {
            if let Some(a) = arg {
                let arg_parts = self.generate_stmt_string_part(a);
                quote! {
                    __stmt_str.push_str("return ");
                    #arg_parts
                    __stmt_str.push_str(";");
                }
            } else {
                quote! {
                    __stmt_str.push_str("return;");
                }
            }
        }
        IrNode::TsIfStmt { test, cons, alt } => {
            let test_parts = self.generate_stmt_string_part(test);
            let cons_parts = self.generate_stmt_as_string(cons);
            let alt_parts = alt.as_ref().map(|a| {
                let ac = self.generate_stmt_as_string(a);
                quote! {
                    __stmt_str.push_str(" else ");
                    #ac
                }
            });
            quote! {
                __stmt_str.push_str("if (");
                #test_parts
                __stmt_str.push_str(") ");
                #cons_parts
                #alt_parts
            }
        }
        IrNode::TsLoopStmt { parts } => {
            let part_exprs: Vec<TokenStream> = parts
                .iter()
                .map(|p| self.generate_stmt_string_part(p))
                .collect();
            quote! { #(#part_exprs)* }
        }
        // For other nodes, just use generate_stmt_string_part
        _ => self.generate_stmt_string_part(node),
    }
}

pub(in super::super) fn generate_stmt_push(&self, node: &IrNode) -> Option<TokenStream> {
    match node {
        IrNode::For {
            pattern,
            iterator,
            body,
        } => {
            let body_pushes = self.generate_stmt_pushes(body);
            Some(quote! {
                for #pattern in #iterator { #body_pushes }
            })
        }
        IrNode::If {
            condition,
            then_body,
            else_if_branches,
            else_body,
        } => {
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
                .map(
                    |MatchArm {
                         pattern,
                         guard,
                         body,
                     }| {
                        let b = self.generate_stmt_pushes(body);
                        if let Some(g) = guard {
                            quote! { #pattern if #g => { #b } }
                        } else {
                            quote! { #pattern => { #b } }
                        }
                    },
                )
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
        IrNode::Do { code } => Some(quote! { #code; }),
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

pub(in super::super) fn generate_stmt(&self, node: &IrNode) -> Option<TokenStream> {
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
        // TypeScript if statement
        IrNode::TsIfStmt { test, cons, alt } => {
            // Build if statement string from parts
            let test_code = self.generate_expr_string_parts(test);
            let cons_code = self.generate_stmt_as_string(cons);
            let alt_code = alt.as_ref().map(|a| {
                let ac = self.generate_stmt_as_string(a);
                quote! {
                    __stmt_str.push_str(" else ");
                    #ac
                }
            });
            Some(quote! {
                {
                    let mut __stmt_str = String::new();
                    __stmt_str.push_str("if (");
                    #test_code
                    __stmt_str.push_str(") ");
                    #cons_code
                    #alt_code
                    macroforge_ts::ts_syn::parse_ts_stmt(&__stmt_str)
                        .unwrap_or_else(|_| macroforge_ts::swc_core::ecma::ast::Stmt::Empty(
                            macroforge_ts::swc_core::ecma::ast::EmptyStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                            }
                        ))
                }
            })
        }
        // TypeScript loop statement (for/while parsed as raw text with placeholders)
        IrNode::TsLoopStmt { parts } => {
            // Build the loop string from parts and parse at runtime
            // generate_stmt_string_part uses __stmt_str, so we alias it
            let part_exprs: Vec<TokenStream> = parts
                .iter()
                .map(|p| self.generate_stmt_string_part(p))
                .collect();
            Some(quote! {
                {
                    let mut __stmt_str = String::new();
                    #(#part_exprs)*
                    macroforge_ts::ts_syn::parse_ts_stmt(&__stmt_str)
                        .unwrap_or_else(|_| macroforge_ts::swc_core::ecma::ast::Stmt::Empty(
                            macroforge_ts::swc_core::ecma::ast::EmptyStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                            }
                        ))
                }
            })
        }
        // Structured for-in statement
        IrNode::ForInStmt { left, right, body } => {
            let left_code = self.generate_for_head(left);
            let right_code = self.generate_expr(right);
            let body_code = self.generate_stmt(body).unwrap_or_else(|| {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Stmt::Empty(
                        macroforge_ts::swc_core::ecma::ast::EmptyStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                        }
                    )
                }
            });
            Some(quote! {
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
        IrNode::ForOfStmt { await_, left, right, body } => {
            let left_code = self.generate_for_head(left);
            let right_code = self.generate_expr(right);
            let body_code = self.generate_stmt(body).unwrap_or_else(|| {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Stmt::Empty(
                        macroforge_ts::swc_core::ecma::ast::EmptyStmt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                        }
                    )
                }
            });
            Some(quote! {
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

/// Generate code for the left-hand side of a for-in/for-of loop.
/// Returns a ForHead which can be either a VarDecl or a Pat.
pub(in super::super) fn generate_for_head(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::VarDecl { kind, decls, declare, .. } => {
            let kind_code = match kind {
                VarKind::Const => {
                    quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Const }
                }
                VarKind::Let => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Let },
                VarKind::Var => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Var },
            };
            let decls_code = self.generate_var_declarators(decls);
            quote! {
                macroforge_ts::swc_core::ecma::ast::ForHead::VarDecl(Box::new(
                    macroforge_ts::swc_core::ecma::ast::VarDecl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        kind: #kind_code,
                        declare: #declare,
                        decls: #decls_code,
                    }
                ))
            }
        }
        // For patterns (destructuring without var/let/const)
        _ => {
            let pat_code = self.generate_pat(node);
            quote! {
                macroforge_ts::swc_core::ecma::ast::ForHead::Pat(Box::new(#pat_code))
            }
        }
    }
}
}
