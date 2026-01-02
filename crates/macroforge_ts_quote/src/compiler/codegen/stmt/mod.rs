use super::error::{GenError, GenErrorKind, GenResult};
use super::*;

impl Codegen {
    /// Generate a BlockStmt from an IrNode (for function bodies)
    pub(in super::super) fn generate_block_stmt(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::BlockStmt { stmts, .. } => {
                let stmts_code = self.generate_stmts_vec(stmts);
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

    pub(in super::super) fn generate_block_stmt_opt(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::BlockStmt { stmts, .. } => {
            let stmts_code = self.generate_stmts_vec(stmts);
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

pub(in super::super) fn generate_block_stmt_or_expr(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::BlockStmt { stmts, .. } => {
            let stmts_code = self.generate_stmts_vec(stmts);
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
        // Collect results, panicking on errors during code generation
        let stmts: Vec<TokenStream> = nodes
            .iter()
            .filter_map(|n| {
                match self.generate_stmt(n) {
                    Ok(ts) => Some(ts),
                    Err(e) => panic!("Statement generation failed: {}", e.to_message()),
                }
            })
            .collect();
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
                match self.generate_combined_stmt(&pending_nodes) {
                    Ok(Some(stmt)) => pushes.push(stmt),
                    Ok(None) => {}
                    Err(e) => panic!("Combined statement generation failed: {}", e.to_message()),
                }
                pending_nodes.clear();
            }
            // Generate control flow / structured statement directly
            match self.generate_stmt_push(node) {
                Ok(push) => pushes.push(push),
                Err(e) => panic!("Statement push generation failed: {}", e.to_message()),
            }
        } else {
            pending_nodes.push(node);
        }
    }

    // Flush remaining pending nodes
    if !pending_nodes.is_empty() {
        match self.generate_combined_stmt(&pending_nodes) {
            Ok(Some(stmt)) => pushes.push(stmt),
            Ok(None) => {}
            Err(e) => panic!("Combined statement generation failed: {}", e.to_message()),
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

pub(in super::super) fn generate_combined_stmt(&self, nodes: &[&IrNode]) -> GenResult<Option<TokenStream>> {
    if nodes.is_empty() {
        return Ok(None);
    }

    // Check if all nodes are whitespace-only Raw nodes
    let all_whitespace = nodes
        .iter()
        .all(|n| matches!(n, IrNode::Raw { value: text, .. } if text.trim().is_empty()));
    if all_whitespace {
        return Ok(None);
    }

    // Generate code that builds a statement string and parses it
    let part_exprs: Vec<TokenStream> = nodes
        .iter()
        .map(|n| self.generate_stmt_string_part(n))
        .collect();

    Ok(Some(quote! {
        {
            let mut __stmt_str = String::new();
            #(#part_exprs)*
            if !__stmt_str.trim().is_empty() {
                let __parsed = macroforge_ts::ts_syn::parse_ts_stmt(&__stmt_str)
                    .unwrap_or_else(|e| panic!(
                        "Failed to parse generated TypeScript statement:\n\n{}\n\nError: {:?}",
                        __stmt_str, e
                    ));
                __body_stmts.push(__parsed);
            }
        }
    }))
}

pub(in super::super) fn generate_stmt_string_part(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::Raw { value: text, .. } => quote! { __stmt_str.push_str(#text); },
        IrNode::StrLit { value: text, .. } => quote! { __stmt_str.push_str(#text); },
        IrNode::Ident { value: name, .. } => quote! { __stmt_str.push_str(#name); },
        IrNode::IdentBlock { parts, .. } => {
            // Recursively process each part
            let part_strs: Vec<TokenStream> = parts
                .iter()
                .map(|p| self.generate_stmt_string_part(p))
                .collect();
            quote! { #(#part_strs)* }
        }
        IrNode::StringInterp { quote: q, parts, .. } => {
            let quote_char = q.to_string();
            let inner_parts: Vec<TokenStream> = parts
                .iter()
                .map(|p| match p {
                    IrNode::Raw { value: text, .. } | IrNode::StrLit { value: text, .. } => {
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
        IrNode::Placeholder { kind, expr, span } => match kind {
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
            _ => {
                let err = GenError::invalid_placeholder_at(
                    "statement string part",
                    &format!("{:?}", kind),
                    &["Expr", "Ident", "Type"],
                    *span,
                );
                panic!("{}", err.to_message());
            }
        },
        // Control flow nodes - generate Rust code that builds the string dynamically
        IrNode::For {
            pattern,
            iterator,
            body,
            ..
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
            ..
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
        IrNode::While { condition, body, .. } => {
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
        IrNode::Match { expr, arms, .. } => {
            let arm_tokens: Vec<TokenStream> = arms
                .iter()
                .map(|MatchArm { pattern, guard, body, .. }| {
                    let body_parts: Vec<TokenStream> = body
                        .iter()
                        .map(|n| self.generate_stmt_string_part(n))
                        .collect();
                    if let Some(g) = guard {
                        quote! { #pattern if #g => { #(#body_parts)* } }
                    } else {
                        quote! { #pattern => { #(#body_parts)* } }
                    }
                })
                .collect();
            quote! {
                match #expr { #(#arm_tokens)* }
            }
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
                quote! { let #mutability #pattern: #ty = #value; }
            } else {
                quote! { let #mutability #pattern = #value; }
            }
        }
        IrNode::Do { code, .. } => quote! { #code; },
        // TypeScript statement nodes - build as strings
        IrNode::VarDecl {
            exported: _,
            declare: _,
            kind,
            decls,
            ..
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
        IrNode::ExprStmt { expr, .. } => {
            let expr_parts = self.generate_stmt_string_part(expr);
            quote! {
                #expr_parts
                __stmt_str.push_str(";");
            }
        }
        IrNode::ReturnStmt { arg, .. } => {
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
        IrNode::BlockStmt { stmts, .. } => {
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
        IrNode::TsIfStmt { test, cons, alt, .. } => {
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
        IrNode::TsLoopStmt { parts, .. } => {
            let part_exprs: Vec<TokenStream> = parts
                .iter()
                .map(|p| self.generate_stmt_string_part(p))
                .collect();
            quote! { #(#part_exprs)* }
        }
        // Expression nodes - generate as string representations
        IrNode::UnaryExpr { op, arg, .. } => {
            let op_str = match op {
                UnaryOp::Minus => "-",
                UnaryOp::Plus => "+",
                UnaryOp::Not => "!",
                UnaryOp::BitNot => "~",
                UnaryOp::TypeOf => "typeof ",
                UnaryOp::Void => "void ",
                UnaryOp::Delete => "delete ",
            };
            let arg_parts = self.generate_stmt_string_part(arg);
            quote! {
                __stmt_str.push_str(#op_str);
                #arg_parts
            }
        }
        IrNode::BinExpr { left, op, right, .. } => {
            let op_str = match op {
                BinaryOp::Add => " + ",
                BinaryOp::Sub => " - ",
                BinaryOp::Mul => " * ",
                BinaryOp::Div => " / ",
                BinaryOp::Mod => " % ",
                BinaryOp::Exp => " ** ",
                BinaryOp::EqEq => " == ",
                BinaryOp::NotEq => " != ",
                BinaryOp::EqEqEq => " === ",
                BinaryOp::NotEqEq => " !== ",
                BinaryOp::Lt => " < ",
                BinaryOp::Le => " <= ",
                BinaryOp::Gt => " > ",
                BinaryOp::Ge => " >= ",
                BinaryOp::And => " && ",
                BinaryOp::Or => " || ",
                BinaryOp::NullishCoalesce => " ?? ",
                BinaryOp::BitAnd => " & ",
                BinaryOp::BitOr => " | ",
                BinaryOp::BitXor => " ^ ",
                BinaryOp::Shl => " << ",
                BinaryOp::Shr => " >> ",
                BinaryOp::UShr => " >>> ",
                BinaryOp::In => " in ",
                BinaryOp::InstanceOf => " instanceof ",
            };
            let left_parts = self.generate_stmt_string_part(left);
            let right_parts = self.generate_stmt_string_part(right);
            quote! {
                #left_parts
                __stmt_str.push_str(#op_str);
                #right_parts
            }
        }
        IrNode::CondExpr { test, consequent, alternate, .. } => {
            let test_parts = self.generate_stmt_string_part(test);
            let cons_parts = self.generate_stmt_string_part(consequent);
            let alt_parts = self.generate_stmt_string_part(alternate);
            quote! {
                #test_parts
                __stmt_str.push_str(" ? ");
                #cons_parts
                __stmt_str.push_str(" : ");
                #alt_parts
            }
        }
        IrNode::CallExpr { callee, args, .. } => {
            let callee_parts = self.generate_stmt_string_part(callee);
            let args_parts: Vec<TokenStream> = args
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    let arg_part = self.generate_stmt_string_part(a);
                    if i > 0 {
                        quote! {
                            __stmt_str.push_str(", ");
                            #arg_part
                        }
                    } else {
                        arg_part
                    }
                })
                .collect();
            quote! {
                #callee_parts
                __stmt_str.push_str("(");
                #(#args_parts)*
                __stmt_str.push_str(")");
            }
        }
        IrNode::MemberExpr { obj, prop, computed, .. } => {
            let obj_parts = self.generate_stmt_string_part(obj);
            let prop_parts = self.generate_stmt_string_part(prop);
            if *computed {
                quote! {
                    #obj_parts
                    __stmt_str.push_str("[");
                    #prop_parts
                    __stmt_str.push_str("]");
                }
            } else {
                quote! {
                    #obj_parts
                    __stmt_str.push_str(".");
                    #prop_parts
                }
            }
        }
        IrNode::NewExpr { callee, args, .. } => {
            let callee_parts = self.generate_stmt_string_part(callee);
            let args_parts: Vec<TokenStream> = args
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    let arg_part = self.generate_stmt_string_part(a);
                    if i > 0 {
                        quote! {
                            __stmt_str.push_str(", ");
                            #arg_part
                        }
                    } else {
                        arg_part
                    }
                })
                .collect();
            quote! {
                __stmt_str.push_str("new ");
                #callee_parts
                __stmt_str.push_str("(");
                #(#args_parts)*
                __stmt_str.push_str(")");
            }
        }
        IrNode::ParenExpr { expr, .. } => {
            let expr_parts = self.generate_stmt_string_part(expr);
            quote! {
                __stmt_str.push_str("(");
                #expr_parts
                __stmt_str.push_str(")");
            }
        }
        IrNode::ArrayLit { elems, .. } => {
            let elem_parts: Vec<TokenStream> = elems
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    let elem_part = self.generate_stmt_string_part(e);
                    if i > 0 {
                        quote! {
                            __stmt_str.push_str(", ");
                            #elem_part
                        }
                    } else {
                        elem_part
                    }
                })
                .collect();
            quote! {
                __stmt_str.push_str("[");
                #(#elem_parts)*
                __stmt_str.push_str("]");
            }
        }
        IrNode::ObjectLit { props, .. } => {
            let prop_parts: Vec<TokenStream> = props
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let prop_part = self.generate_stmt_string_part(p);
                    if i > 0 {
                        quote! {
                            __stmt_str.push_str(", ");
                            #prop_part
                        }
                    } else {
                        prop_part
                    }
                })
                .collect();
            quote! {
                __stmt_str.push_str("{ ");
                #(#prop_parts)*
                __stmt_str.push_str(" }");
            }
        }
        IrNode::KeyValueProp { key, value, .. } => {
            let key_parts = self.generate_stmt_string_part(key);
            let value_parts = self.generate_stmt_string_part(value);
            quote! {
                #key_parts
                __stmt_str.push_str(": ");
                #value_parts
            }
        }
        IrNode::ShorthandProp { key, .. } => {
            self.generate_stmt_string_part(key)
        }
        IrNode::SpreadElement { expr, .. } => {
            let expr_parts = self.generate_stmt_string_part(expr);
            quote! {
                __stmt_str.push_str("...");
                #expr_parts
            }
        }
        IrNode::NumLit { value, .. } => {
            quote! { __stmt_str.push_str(#value); }
        }
        IrNode::BoolLit { value, .. } => {
            let val_str = if *value { "true" } else { "false" };
            quote! { __stmt_str.push_str(#val_str); }
        }
        IrNode::NullLit { .. } => {
            quote! { __stmt_str.push_str("null"); }
        }
        IrNode::ThisExpr { .. } => {
            quote! { __stmt_str.push_str("this"); }
        }
        IrNode::SuperExpr { .. } => {
            quote! { __stmt_str.push_str("super"); }
        }
        IrNode::AssignExpr { left, op, right, .. } => {
            let op_str = match op {
                AssignOp::Assign => " = ",
                AssignOp::AddAssign => " += ",
                AssignOp::SubAssign => " -= ",
                AssignOp::MulAssign => " *= ",
                AssignOp::DivAssign => " /= ",
                AssignOp::ModAssign => " %= ",
                AssignOp::ExpAssign => " **= ",
                AssignOp::ShlAssign => " <<= ",
                AssignOp::ShrAssign => " >>= ",
                AssignOp::UShrAssign => " >>>= ",
                AssignOp::BitAndAssign => " &= ",
                AssignOp::BitOrAssign => " |= ",
                AssignOp::BitXorAssign => " ^= ",
                AssignOp::AndAssign => " &&= ",
                AssignOp::OrAssign => " ||= ",
                AssignOp::NullishAssign => " ??= ",
            };
            let left_parts = self.generate_stmt_string_part(left);
            let right_parts = self.generate_stmt_string_part(right);
            quote! {
                #left_parts
                __stmt_str.push_str(#op_str);
                #right_parts
            }
        }
        IrNode::UpdateExpr { op, prefix, arg, .. } => {
            let op_str = match op {
                UpdateOp::Increment => "++",
                UpdateOp::Decrement => "--",
            };
            let arg_parts = self.generate_stmt_string_part(arg);
            if *prefix {
                quote! {
                    __stmt_str.push_str(#op_str);
                    #arg_parts
                }
            } else {
                quote! {
                    #arg_parts
                    __stmt_str.push_str(#op_str);
                }
            }
        }
        IrNode::AwaitExpr { arg, .. } => {
            let arg_parts = self.generate_stmt_string_part(arg);
            quote! {
                __stmt_str.push_str("await ");
                #arg_parts
            }
        }
        IrNode::TsAsExpr { expr, type_ann, .. } => {
            let expr_parts = self.generate_stmt_string_part(expr);
            let type_parts = self.generate_stmt_string_part(type_ann);
            quote! {
                #expr_parts
                __stmt_str.push_str(" as ");
                #type_parts
            }
        }
        IrNode::TsNonNullExpr { expr, .. } => {
            let expr_parts = self.generate_stmt_string_part(expr);
            quote! {
                #expr_parts
                __stmt_str.push_str("!");
            }
        }
        // Type nodes for string representation
        IrNode::KeywordType { keyword: kw, .. } => {
            let type_str = match kw {
                TsKeyword::Any => "any",
                TsKeyword::Unknown => "unknown",
                TsKeyword::String => "string",
                TsKeyword::Number => "number",
                TsKeyword::Boolean => "boolean",
                TsKeyword::Void => "void",
                TsKeyword::Null => "null",
                TsKeyword::Undefined => "undefined",
                TsKeyword::Never => "never",
                TsKeyword::Object => "object",
                TsKeyword::BigInt => "bigint",
                TsKeyword::Symbol => "symbol",
            };
            quote! { __stmt_str.push_str(#type_str); }
        }
        IrNode::TypeRef { name, type_params, .. } => {
            let name_parts = self.generate_stmt_string_part(name);
            if let Some(params) = type_params {
                let params_parts = self.generate_stmt_string_part(params);
                quote! {
                    #name_parts
                    #params_parts
                }
            } else {
                name_parts
            }
        }
        IrNode::TypeArgs { args, .. } => {
            let arg_parts: Vec<TokenStream> = args
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    let arg_part = self.generate_stmt_string_part(a);
                    if i > 0 {
                        quote! {
                            __stmt_str.push_str(", ");
                            #arg_part
                        }
                    } else {
                        arg_part
                    }
                })
                .collect();
            quote! {
                __stmt_str.push_str("<");
                #(#arg_parts)*
                __stmt_str.push_str(">");
            }
        }
        IrNode::UnionType { types, .. } => {
            let type_parts: Vec<TokenStream> = types
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    let type_part = self.generate_stmt_string_part(t);
                    if i > 0 {
                        quote! {
                            __stmt_str.push_str(" | ");
                            #type_part
                        }
                    } else {
                        type_part
                    }
                })
                .collect();
            quote! { #(#type_parts)* }
        }
        IrNode::ArrayType { elem, .. } => {
            let elem_parts = self.generate_stmt_string_part(elem);
            quote! {
                #elem_parts
                __stmt_str.push_str("[]");
            }
        }
        _ => {
            let err = GenError::unexpected_node(
                "statement string part",
                node,
                &["Raw", "StrLit", "Ident", "IdentBlock", "StringInterp", "Placeholder", "For", "If", "While", "Let", "Do", "VarDecl", "ExprStmt", "ReturnStmt", "BlockStmt", "TsIfStmt", "TsLoopStmt", "UnaryExpr", "BinExpr", "CallExpr", "MemberExpr", "NewExpr", "ParenExpr", "ArrayLit", "ObjectLit", "NumLit", "BoolLit", "NullLit", "ThisExpr", "etc."],
            );
            panic!("{}", err.to_message());
        }
    }
}
/// Generate code that builds a statement string from an IrNode.
    pub(in super::super) fn generate_stmt_as_string(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::BlockStmt { stmts, .. } => {
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
        IrNode::ExprStmt { expr, .. } => {
            let expr_parts = self.generate_stmt_string_part(expr);
            quote! {
                #expr_parts
                __stmt_str.push_str(";");
            }
        }
        IrNode::ReturnStmt { arg, .. } => {
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
        IrNode::TsIfStmt { test, cons, alt, .. } => {
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
        IrNode::TsLoopStmt { parts, .. } => {
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

pub(in super::super) fn generate_stmt_push(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::For {
            pattern,
            iterator,
            body,
            ..
        } => {
            let body_pushes = self.generate_stmt_pushes(body);
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

            Ok(quote! {
                if #condition { #then_pushes } #else_code
            })
        }
        IrNode::While { condition, body, .. } => {
            let body_pushes = self.generate_stmt_pushes(body);
            Ok(quote! {
                while #condition { #body_pushes }
            })
        }
        IrNode::Match { expr, arms, .. } => {
            let arm_tokens: Vec<TokenStream> = arms
                .iter()
                .map(
                    |MatchArm {
                         pattern,
                         guard,
                         body,
                         ..
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
    match node {
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
            let stmts_code = self.generate_stmts_vec(stmts);
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
        // TypeScript if statement
        IrNode::TsIfStmt { test, cons, alt, .. } => {
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
            Ok(quote! {
                {
                    let mut __stmt_str = String::new();
                    __stmt_str.push_str("if (");
                    #test_code
                    __stmt_str.push_str(") ");
                    #cons_code
                    #alt_code
                    macroforge_ts::ts_syn::parse_ts_stmt(&__stmt_str)
                        .expect("Failed to parse generated if statement")
                }
            })
        }
        // TypeScript loop statement (for/while parsed as raw text with placeholders)
        IrNode::TsLoopStmt { parts, .. } => {
            // Build the loop string from parts and parse at runtime
            // generate_stmt_string_part uses __stmt_str, so we alias it
            let part_exprs: Vec<TokenStream> = parts
                .iter()
                .map(|p| self.generate_stmt_string_part(p))
                .collect();
            Ok(quote! {
                {
                    let mut __stmt_str = String::new();
                    #(#part_exprs)*
                    macroforge_ts::ts_syn::parse_ts_stmt(&__stmt_str)
                        .expect("Failed to parse generated loop statement")
                }
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
                    &["ExprStmt", "ReturnStmt", "BlockStmt", "VarDecl", "TsIfStmt", "TsLoopStmt", "ForInStmt", "ForOfStmt"],
                ))
            }
        }
    }
}

/// Generate code for the left-hand side of a for-in/for-of loop.
/// Returns a ForHead which can be either a VarDecl or a Pat.
pub(in super::super) fn generate_for_head(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::VarDecl { kind, decls, declare, .. } => {
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
