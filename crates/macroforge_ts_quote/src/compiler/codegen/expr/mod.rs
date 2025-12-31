mod r#as;

use super::*;

impl Codegen {
    pub(in super::super) fn generate_expr(&self, node: &IrNode) -> TokenStream {
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

        IrNode::MemberExpr {
            obj,
            prop,
            computed,
        } => {
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
                            Box::new(macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone()))
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
                quote! { macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone()) }
            }
            PlaceholderKind::Ident => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                        macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone())
                    )
                }
            }
            _ => quote! { /* placeholder in wrong context */ },
        },

        // TypeScript type assertion: expr as Type
        IrNode::TsAsExpr { expr, type_ann } => {
            let expr_code = self.generate_expr(expr);
            let type_ann_code = self.generate_ts_type(type_ann);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TsAs(
                    macroforge_ts::swc_core::ecma::ast::TsAsExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                        type_ann: Box::new(#type_ann_code),
                    }
                )
            }
        }

        // TypeScript satisfies: expr satisfies Type
        IrNode::TsSatisfiesExpr { expr, type_ann } => {
            let expr_code = self.generate_expr(expr);
            let type_ann_code = self.generate_ts_type(type_ann);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TsSatisfies(
                    macroforge_ts::swc_core::ecma::ast::TsSatisfiesExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                        type_ann: Box::new(#type_ann_code),
                    }
                )
            }
        }

        // TypeScript non-null assertion: expr!
        IrNode::TsNonNullExpr { expr } => {
            let expr_code = self.generate_expr(expr);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TsNonNull(
                    macroforge_ts::swc_core::ecma::ast::TsNonNullExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            }
        }

        // TypeScript instantiation: expr<Type>
        IrNode::TsInstantiation { expr, type_args } => {
            let expr_code = self.generate_expr(expr);
            let type_args_code = self.generate_type_param_instantiation(type_args);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TsInstantiation(
                    macroforge_ts::swc_core::ecma::ast::TsInstantiation {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                        type_args: Box::new(#type_args_code),
                    }
                )
            }
        }

        // Await expression: await expr
        IrNode::AwaitExpr { arg } => {
            let arg_code = self.generate_expr(arg);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Await(
                    macroforge_ts::swc_core::ecma::ast::AwaitExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: Box::new(#arg_code),
                    }
                )
            }
        }

        // Yield expression: yield expr, yield* expr
        IrNode::YieldExpr { arg, delegate } => {
            let arg_code = arg.as_ref().map(|a| {
                let ac = self.generate_expr(a);
                quote! { Some(Box::new(#ac)) }
            }).unwrap_or(quote! { None });
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Yield(
                    macroforge_ts::swc_core::ecma::ast::YieldExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: #arg_code,
                        delegate: #delegate,
                    }
                )
            }
        }

        // Phase 4: Private name #field
        IrNode::PrivateName(name) => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::PrivateName(
                    macroforge_ts::swc_core::ecma::ast::PrivateName {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        name: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        ),
                    }
                )
            }
        }

        // BigInt literal: 42n
        IrNode::BigIntLit(value) => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                    macroforge_ts::swc_core::ecma::ast::Lit::BigInt(
                        macroforge_ts::swc_core::ecma::ast::BigInt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: Box::new(#value.parse::<num_bigint::BigInt>().unwrap_or_default()),
                            raw: Some(format!("{}n", #value).into()),
                        }
                    )
                )
            }
        }

        // Update expression: ++x, x--, etc.
        IrNode::UpdateExpr { op, prefix, arg } => {
            let arg_code = self.generate_expr(arg);
            let op_code = self.generate_update_op(op);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Update(
                    macroforge_ts::swc_core::ecma::ast::UpdateExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        op: #op_code,
                        prefix: #prefix,
                        arg: Box::new(#arg_code),
                    }
                )
            }
        }

        // Optional chaining expression: obj?.prop, fn?.()
        IrNode::OptChainExpr { base } => {
            let base_code = self.generate_opt_chain_base(base);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::OptChain(
                    macroforge_ts::swc_core::ecma::ast::OptChainExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        optional: true,
                        base: Box::new(#base_code),
                    }
                )
            }
        }

        // Phase 5: Function expression
        IrNode::FnExpr { async_, generator, name, type_params: _, params, return_type: _, body } => {
            let params_code = self.generate_params(params);
            let name_code = name.as_ref().map(|n| {
                let nc = self.generate_ident(n);
                quote! { Some(#nc) }
            }).unwrap_or(quote! { None });
            let body_code = body.as_ref().map(|b| {
                let bc = self.generate_block_stmt(b);
                quote! { Some(#bc) }
            }).unwrap_or(quote! { None });

            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Fn(
                    macroforge_ts::swc_core::ecma::ast::FnExpr {
                        ident: #name_code,
                        function: Box::new(macroforge_ts::swc_core::ecma::ast::Function {
                            params: #params_code,
                            decorators: vec![],
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            body: #body_code,
                            is_generator: #generator,
                            is_async: #async_,
                            type_params: None,
                            return_type: None,
                        }),
                    }
                )
            }
        }

        // Class expression
        IrNode::ClassExpr { name, type_params: _, extends, implements: _, body } => {
            let name_code = name.as_ref().map(|n| {
                let nc = self.generate_ident(n);
                quote! { Some(#nc) }
            }).unwrap_or(quote! { None });
            let extends_code = extends.as_ref().map(|e| {
                let ec = self.generate_expr(e);
                quote! {
                    Some(Box::new(macroforge_ts::swc_core::ecma::ast::ExtendsClause {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        super_class: Box::new(#ec),
                        type_args: None,
                    }))
                }
            }).unwrap_or(quote! { None });
            let body_code = self.generate_class_members(body);

            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Class(
                    macroforge_ts::swc_core::ecma::ast::ClassExpr {
                        ident: #name_code,
                        class: Box::new(macroforge_ts::swc_core::ecma::ast::Class {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            decorators: vec![],
                            body: #body_code,
                            super_class: #extends_code,
                            is_abstract: false,
                            type_params: None,
                            implements: vec![],
                        }),
                    }
                )
            }
        }

        // Parenthesized expression: (expr)
        IrNode::ParenExpr { expr } => {
            let expr_code = self.generate_expr(expr);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Paren(
                    macroforge_ts::swc_core::ecma::ast::ParenExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            }
        }

        // Sequence expression: a, b, c
        IrNode::SeqExpr { exprs } => {
            let exprs_code: Vec<TokenStream> = exprs
                .iter()
                .map(|e| {
                    let ec = self.generate_expr(e);
                    quote! { Box::new(#ec) }
                })
                .collect();

            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Seq(
                    macroforge_ts::swc_core::ecma::ast::SeqExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        exprs: vec![#(#exprs_code),*],
                    }
                )
            }
        }

        // Tagged template literal: tag`template`
        IrNode::TaggedTpl { tag, type_params: _, quasis, exprs } => {
            let tag_code = self.generate_expr(tag);
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
                    let ec = self.generate_expr(e);
                    quote! { Box::new(#ec) }
                })
                .collect();

            quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TaggedTpl(
                    macroforge_ts::swc_core::ecma::ast::TaggedTpl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        tag: Box::new(#tag_code),
                        type_params: None,
                        tpl: Box::new(macroforge_ts::swc_core::ecma::ast::Tpl {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            quasis: vec![#(#quasis_code),*],
                            exprs: vec![#(#exprs_code),*],
                        }),
                    }
                )
            }
        }

        // Raw text - parse as expression at runtime
        // Note: parse_ts_expr returns Result<Box<Expr>, _>, so we dereference to get Expr
        IrNode::Raw(text) => {
            quote! {
                {
                    let __source = #text;
                    *macroforge_ts::ts_syn::parse_ts_expr(__source)
                        .unwrap_or_else(|_| Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Invalid(
                            macroforge_ts::swc_core::ecma::ast::Invalid {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                            }
                        )))
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
                                      let __expr = macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone());
                                      __expr_str.push_str(&macroforge_ts::ts_syn::emit_expr(&__expr));
                                  }
                              }
                              PlaceholderKind::Ident => {
                                  quote! {
                                      __expr_str.push_str(&macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()).sym.to_string());
                                  }
                              }
                              PlaceholderKind::Type => {
                                  // For types (e.g., in "as Type" expressions), emit as type string
                                  quote! {
                                      let __ty = macroforge_ts::ts_syn::ToTsType::to_ts_type((#expr).clone());
                                      __expr_str.push_str(&macroforge_ts::ts_syn::emit_ts_type(&__ty));
                                  }
                              }
                              _ => quote! {},
                          }
                      }
                      // Handle nested IdentBlock (e.g., @{name}Val within a larger expression)
                      IrNode::IdentBlock { parts: inner_parts } => {
                          let inner_exprs: Vec<TokenStream> = inner_parts
                              .iter()
                              .map(|ip| match ip {
                                  IrNode::Raw(text) => quote! { __expr_str.push_str(#text); },
                                  IrNode::StrLit(text) => quote! { __expr_str.push_str(#text); },
                                  IrNode::Ident(text) => quote! { __expr_str.push_str(#text); },
                                  IrNode::Placeholder { kind, expr } => {
                                      match kind {
                                          PlaceholderKind::Ident => {
                                              quote! {
                                                  __expr_str.push_str(&macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()).sym.to_string());
                                              }
                                          }
                                          PlaceholderKind::Expr => {
                                              quote! {
                                                  let __expr = macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone());
                                                  __expr_str.push_str(&macroforge_ts::ts_syn::emit_expr(&__expr));
                                              }
                                          }
                                          PlaceholderKind::Type => {
                                              quote! {
                                                  let __ty = macroforge_ts::ts_syn::ToTsType::to_ts_type((#expr).clone());
                                                  __expr_str.push_str(&macroforge_ts::ts_syn::emit_ts_type(&__ty));
                                              }
                                          }
                                          _ => quote! {},
                                      }
                                  }
                                  _ => quote! {},
                              })
                              .collect();
                          quote! { #(#inner_exprs)* }
                      }
                      // Handle StringInterp (string literals with placeholders like "@{name}")
                      // For placeholders inside strings, we insert the raw value (not quoted expression)
                      IrNode::StringInterp { quote: q, parts: string_parts } => {
                          let quote_char = q.to_string();
                          let inner_exprs: Vec<TokenStream> = string_parts
                              .iter()
                              .map(|sp| match sp {
                                  IrNode::Raw(text) => quote! { __expr_str.push_str(#text); },
                                  IrNode::StrLit(text) => quote! { __expr_str.push_str(#text); },
                                  IrNode::Placeholder { expr, .. } => {
                                      // Inside a string, always insert the raw string value
                                      // (not the quoted expression representation)
                                      quote! {
                                          __expr_str.push_str(&(#expr).to_string());
                                      }
                                  }
                                  _ => quote! {},
                              })
                              .collect();
                          quote! {
                              __expr_str.push_str(#quote_char);
                              #(#inner_exprs)*
                              __expr_str.push_str(#quote_char);
                          }
                      }
                      _ => quote! {},
                  })
                  .collect();

            // Note: parse_ts_expr returns Result<Box<Expr>, _>, so we dereference to get Expr
            quote! {
                {
                    let mut __expr_str = String::new();
                    #(#part_exprs)*
                    #[cfg(debug_assertions)]
                    if std::env::var("MF_DEBUG_EXPR").is_ok() {
                        eprintln!("[MF_DEBUG_EXPR] Expression string: {:?}", __expr_str);
                    }
                    *macroforge_ts::ts_syn::parse_ts_expr(&__expr_str)
                        .unwrap_or_else(|e| {
                            #[cfg(debug_assertions)]
                            if std::env::var("MF_DEBUG_EXPR").is_ok() {
                                eprintln!("[MF_DEBUG_EXPR] Parse failed: {:?}", e);
                            }
                            Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Invalid(
                                macroforge_ts::swc_core::ecma::ast::Invalid {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                }
                            ))
                        })
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
/// Generate code that builds an expression string from an IrNode.
    pub(in super::super) fn generate_expr_string_parts(&self, node: &IrNode) -> TokenStream {
    // Reuse generate_stmt_string_part since it handles the same node types
    self.generate_stmt_string_part(node)
}

pub(in super::super) fn generate_ts_expr_with_type_args(&self, _extends: &[IrNode]) -> TokenStream {
    quote! { vec![] }
}

/// Generate code for update operators (++, --)
fn generate_update_op(&self, op: &UpdateOp) -> TokenStream {
    match op {
        UpdateOp::Increment => quote! { macroforge_ts::swc_core::ecma::ast::UpdateOp::PlusPlus },
        UpdateOp::Decrement => quote! { macroforge_ts::swc_core::ecma::ast::UpdateOp::MinusMinus },
    }
}

/// Generate code for optional chain base (member access or call)
fn generate_opt_chain_base(&self, node: &IrNode) -> TokenStream {
    match node {
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
                macroforge_ts::swc_core::ecma::ast::OptChainBase::Member(
                    macroforge_ts::swc_core::ecma::ast::MemberExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        obj: Box::new(#obj_code),
                        prop: #prop_code,
                    }
                )
            }
        }
        IrNode::CallExpr { callee, args, type_args: _ } => {
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
                macroforge_ts::swc_core::ecma::ast::OptChainBase::Call(
                    macroforge_ts::swc_core::ecma::ast::OptCall {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        callee: Box::new(#callee_code),
                        args: vec![#(#args_code),*],
                        type_args: None,
                    }
                )
            }
        }
        _ => {
            // Fallback - treat as expression and wrap in member access
            let expr_code = self.generate_expr(node);
            quote! {
                macroforge_ts::swc_core::ecma::ast::OptChainBase::Member(
                    macroforge_ts::swc_core::ecma::ast::MemberExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        obj: Box::new(#expr_code),
                        prop: macroforge_ts::swc_core::ecma::ast::MemberProp::Ident(
                            macroforge_ts::swc_core::ecma::ast::IdentName::new(
                                "".into(),
                                macroforge_ts::swc_core::common::DUMMY_SP,
                            )
                        ),
                    }
                )
            }
        }
    }
}
}
