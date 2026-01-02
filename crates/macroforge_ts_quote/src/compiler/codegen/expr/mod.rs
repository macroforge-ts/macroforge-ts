mod r#as;

use super::error::{GenError, GenErrorKind, GenResult};
use super::*;

impl Codegen {
    pub(in super::super) fn generate_expr(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::Ident { value: name, .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                    macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                        #name.into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                )
            })
        }

        IrNode::StrLit { value, .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                    macroforge_ts::swc_core::ecma::ast::Lit::Str(
                        macroforge_ts::swc_core::ecma::ast::Str {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: #value.into(),
                            raw: None,
                        }
                    )
                )
            })
        }

        IrNode::NumLit { span, value } => {
            let num: f64 = value.parse().map_err(|_| {
                GenError::new(GenErrorKind::InvalidNumericLiteral)
                    .with_context("numeric literal")
                    .with_found(value)
                    .with_span(*span)
            })?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                    macroforge_ts::swc_core::ecma::ast::Lit::Num(
                        macroforge_ts::swc_core::ecma::ast::Number {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: #num,
                            raw: Some(#value.into()),
                        }
                    )
                )
            })
        }

        IrNode::BoolLit { value, .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                    macroforge_ts::swc_core::ecma::ast::Lit::Bool(
                        macroforge_ts::swc_core::ecma::ast::Bool {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: #value,
                        }
                    )
                )
            })
        }

        IrNode::NullLit { .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                    macroforge_ts::swc_core::ecma::ast::Lit::Null(
                        macroforge_ts::swc_core::ecma::ast::Null {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                        }
                    )
                )
            })
        }

        IrNode::ThisExpr { .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::This(
                    macroforge_ts::swc_core::ecma::ast::ThisExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                    }
                )
            })
        }

        IrNode::SuperExpr { .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Super(
                    macroforge_ts::swc_core::ecma::ast::Super {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                    }
                )
            })
        }

        IrNode::CallExpr {
            callee,
            type_args,
            args,
            ..
        } => {
            let callee_code = self.generate_expr(callee)?;
            let args_code: Vec<TokenStream> = args
                .iter()
                .map(|a| {
                    let expr = self.generate_expr(a)?;
                    Ok(quote! {
                        macroforge_ts::swc_core::ecma::ast::ExprOrSpread {
                            spread: None,
                            expr: Box::new(#expr),
                        }
                    })
                })
                .collect::<GenResult<Vec<_>>>()?;

            let type_args_code = match type_args {
                Some(ta) => {
                    let tpc = self.generate_type_param_instantiation(ta)?;
                    quote! { Some(Box::new(#tpc)) }
                }
                None => quote! { None },
            };

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Call(
                    macroforge_ts::swc_core::ecma::ast::CallExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        callee: macroforge_ts::swc_core::ecma::ast::Callee::Expr(Box::new(#callee_code)),
                        args: vec![#(#args_code),*],
                        type_args: #type_args_code,
                    }
                )
            })
        }

        IrNode::MemberExpr {
            obj,
            prop,
            computed,
            ..
        } => {
            let obj_code = self.generate_expr(obj)?;
            let prop_code = if *computed {
                let p = self.generate_expr(prop)?;
                quote! {
                    macroforge_ts::swc_core::ecma::ast::MemberProp::Computed(
                        macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            expr: Box::new(#p),
                        }
                    )
                }
            } else {
                let ident_code = self.generate_ident_name(prop)?;
                quote! {
                    macroforge_ts::swc_core::ecma::ast::MemberProp::Ident(#ident_code)
                }
            };

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Member(
                    macroforge_ts::swc_core::ecma::ast::MemberExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        obj: Box::new(#obj_code),
                        prop: #prop_code,
                    }
                )
            })
        }

        IrNode::ObjectLit { props, .. } => {
            let props_code = self.generate_props(props)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Object(
                    macroforge_ts::swc_core::ecma::ast::ObjectLit {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        props: #props_code,
                    }
                )
            })
        }

        IrNode::ArrayLit { elems, .. } => {
            let elems_code: Vec<TokenStream> = elems
                .iter()
                .map(|e| {
                    let expr = self.generate_expr(e)?;
                    Ok(quote! {
                        Some(macroforge_ts::swc_core::ecma::ast::ExprOrSpread {
                            spread: None,
                            expr: Box::new(#expr),
                        })
                    })
                })
                .collect::<GenResult<Vec<_>>>()?;

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Array(
                    macroforge_ts::swc_core::ecma::ast::ArrayLit {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        elems: vec![#(#elems_code),*],
                    }
                )
            })
        }

        IrNode::BinExpr { left, op, right, .. } => {
            let left_code = self.generate_expr(left)?;
            let right_code = self.generate_expr(right)?;
            let op_code = self.generate_binary_op(op);

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Bin(
                    macroforge_ts::swc_core::ecma::ast::BinExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        op: #op_code,
                        left: Box::new(#left_code),
                        right: Box::new(#right_code),
                    }
                )
            })
        }

        IrNode::AssignExpr { left, op, right, .. } => {
            let left_code = self.generate_assign_target(left)?;
            let right_code = self.generate_expr(right)?;
            let op_code = self.generate_assign_op(op);

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Assign(
                    macroforge_ts::swc_core::ecma::ast::AssignExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        op: #op_code,
                        left: #left_code,
                        right: Box::new(#right_code),
                    }
                )
            })
        }

        IrNode::CondExpr { test, consequent, alternate, .. } => {
            let test_code = self.generate_expr(test)?;
            let cons_code = self.generate_expr(consequent)?;
            let alt_code = self.generate_expr(alternate)?;

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Cond(
                    macroforge_ts::swc_core::ecma::ast::CondExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        test: Box::new(#test_code),
                        cons: Box::new(#cons_code),
                        alt: Box::new(#alt_code),
                    }
                )
            })
        }

        IrNode::NewExpr {
            callee,
            type_args,
            args,
            ..
        } => {
            let callee_code = self.generate_expr(callee)?;
            let args_code: Vec<TokenStream> = args
                .iter()
                .map(|a| {
                    let expr = self.generate_expr(a)?;
                    Ok(quote! {
                        macroforge_ts::swc_core::ecma::ast::ExprOrSpread {
                            spread: None,
                            expr: Box::new(#expr),
                        }
                    })
                })
                .collect::<GenResult<Vec<_>>>()?;

            let type_args_code = match type_args {
                Some(ta) => {
                    let tpc = self.generate_type_param_instantiation(ta)?;
                    quote! { Some(Box::new(#tpc)) }
                }
                None => quote! { None },
            };

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::New(
                    macroforge_ts::swc_core::ecma::ast::NewExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        callee: Box::new(#callee_code),
                        args: Some(vec![#(#args_code),*]),
                        type_args: #type_args_code,
                    }
                )
            })
        }

        IrNode::ArrowExpr {
            async_,
            type_params,
            params,
            return_type,
            body,
            ..
        } => {
            let params_code = self.generate_pats(params)?;
            let body_code = self.generate_block_stmt_or_expr(body)?;

            let type_params_code = match type_params {
                Some(tp) => {
                    let tpc = self.generate_type_params(tp)?;
                    quote! { Some(Box::new(#tpc)) }
                }
                None => quote! { None },
            };

            let return_type_code = match return_type {
                Some(rt) => {
                    let rtc = self.generate_type_ann(rt)?;
                    quote! { Some(Box::new(#rtc)) }
                }
                None => quote! { None },
            };

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Arrow(
                    macroforge_ts::swc_core::ecma::ast::ArrowExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        params: #params_code,
                        body: Box::new(#body_code),
                        is_async: #async_,
                        is_generator: false,
                        type_params: #type_params_code,
                        return_type: #return_type_code,
                    }
                )
            })
        }

        IrNode::TplLit { quasis, exprs, .. } => {
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
                    let expr = self.generate_expr(e)?;
                    Ok(quote! { Box::new(#expr) })
                })
                .collect::<GenResult<Vec<_>>>()?;

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Tpl(
                    macroforge_ts::swc_core::ecma::ast::Tpl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        quasis: vec![#(#quasis_code),*],
                        exprs: vec![#(#exprs_code),*],
                    }
                )
            })
        }

        // StringInterp from template literals - convert to Tpl expression
        IrNode::StringInterp { quote: _, parts, .. } => {
            // Convert parts into quasis and exprs
            let mut quasis = Vec::new();
            let mut exprs = Vec::new();
            let mut current_text = String::new();

            for part in parts {
                match part {
                    IrNode::Raw { value: text, .. } | IrNode::StrLit { value: text, .. } => current_text.push_str(text),
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

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Tpl(
                    macroforge_ts::swc_core::ecma::ast::Tpl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        quasis: vec![#(#quasis_code),*],
                        exprs: vec![#(#exprs),*],
                    }
                )
            })
        }

        IrNode::Placeholder { kind, expr, span } => match kind {
            PlaceholderKind::Expr => {
                Ok(quote! { macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone()) })
            }
            PlaceholderKind::Ident => {
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                        macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone())
                    )
                })
            }
            other => Err(GenError::invalid_placeholder_at(
                "expression",
                &format!("{:?}", other),
                &["Expr", "Ident"],
                *span,
            )),
        },

        // TypeScript type assertion: expr as Type
        IrNode::TsAsExpr { expr, type_ann, .. } => {
            let expr_code = self.generate_expr(expr)?;
            let type_ann_code = self.generate_ts_type(type_ann)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TsAs(
                    macroforge_ts::swc_core::ecma::ast::TsAsExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                        type_ann: Box::new(#type_ann_code),
                    }
                )
            })
        }

        // TypeScript satisfies: expr satisfies Type
        IrNode::TsSatisfiesExpr { expr, type_ann, .. } => {
            let expr_code = self.generate_expr(expr)?;
            let type_ann_code = self.generate_ts_type(type_ann)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TsSatisfies(
                    macroforge_ts::swc_core::ecma::ast::TsSatisfiesExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                        type_ann: Box::new(#type_ann_code),
                    }
                )
            })
        }

        // TypeScript non-null assertion: expr!
        IrNode::TsNonNullExpr { expr, .. } => {
            let expr_code = self.generate_expr(expr)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TsNonNull(
                    macroforge_ts::swc_core::ecma::ast::TsNonNullExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            })
        }

        // TypeScript const assertion: expr as const
        IrNode::TsConstAssertion { expr, .. } => {
            let expr_code = self.generate_expr(expr)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TsConstAssertion(
                    macroforge_ts::swc_core::ecma::ast::TsConstAssertion {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            })
        }

        // TypeScript instantiation: expr<Type>
        IrNode::TsInstantiation { expr, type_args, .. } => {
            let expr_code = self.generate_expr(expr)?;
            let type_args_code = self.generate_type_param_instantiation(type_args)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TsInstantiation(
                    macroforge_ts::swc_core::ecma::ast::TsInstantiation {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                        type_args: Box::new(#type_args_code),
                    }
                )
            })
        }

        // Await expression: await expr
        IrNode::AwaitExpr { arg, .. } => {
            let arg_code = self.generate_expr(arg)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Await(
                    macroforge_ts::swc_core::ecma::ast::AwaitExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: Box::new(#arg_code),
                    }
                )
            })
        }

        // Yield expression: yield expr, yield* expr
        // Note: unwrap_or for arg is valid - None is a valid yield (yield without value)
        IrNode::YieldExpr { arg, delegate, .. } => {
            let arg_code = match arg.as_ref() {
                Some(a) => {
                    let ac = self.generate_expr(a)?;
                    quote! { Some(Box::new(#ac)) }
                }
                None => quote! { None },
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Yield(
                    macroforge_ts::swc_core::ecma::ast::YieldExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: #arg_code,
                        delegate: #delegate,
                    }
                )
            })
        }

        // Phase 4: Private name #field
        IrNode::PrivateName { value: name, .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::PrivateName(
                    macroforge_ts::swc_core::ecma::ast::PrivateName {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        name: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        ),
                    }
                )
            })
        }

        // BigInt literal: 42n
        // Note: BigInt parsing at runtime - the macro generates code that parses at runtime
        // Keep unwrap_or_default since parsing happens at runtime, not compile-time
        IrNode::BigIntLit { value, .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Lit(
                    macroforge_ts::swc_core::ecma::ast::Lit::BigInt(
                        macroforge_ts::swc_core::ecma::ast::BigInt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: Box::new(#value.parse::<num_bigint::BigInt>().unwrap_or_default()),
                            raw: Some(format!("{}n", #value).into()),
                        }
                    )
                )
            })
        }

        // Update expression: ++x, x--, etc.
        IrNode::UpdateExpr { op, prefix, arg, .. } => {
            let arg_code = self.generate_expr(arg)?;
            let op_code = self.generate_update_op(op);
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Update(
                    macroforge_ts::swc_core::ecma::ast::UpdateExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        op: #op_code,
                        prefix: #prefix,
                        arg: Box::new(#arg_code),
                    }
                )
            })
        }

        // Unary expression: -x, !x, typeof x, etc.
        IrNode::UnaryExpr { op, arg, .. } => {
            let arg_code = self.generate_expr(arg)?;
            let op_code = self.generate_unary_op(op);
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Unary(
                    macroforge_ts::swc_core::ecma::ast::UnaryExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        op: #op_code,
                        arg: Box::new(#arg_code),
                    }
                )
            })
        }

        // Optional chaining expression: obj?.prop, fn?.()
        IrNode::OptChainExpr { base, expr, .. } => {
            // `base` is the object being accessed (e.g., `x` in `x?.y`)
            // `expr` contains the chain operation with a placeholder for the object
            let base_obj_code = self.generate_expr(base)?;
            let chain_base_code = match expr.as_ref() {
                // x?.y or x?.[y] - member access
                IrNode::MemberExpr { obj: _, prop, computed, .. } => {
                    // obj is a placeholder, use base instead
                    let prop_code = if *computed {
                        let p = self.generate_expr(prop)?;
                        quote! {
                            macroforge_ts::swc_core::ecma::ast::MemberProp::Computed(
                                macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    expr: Box::new(#p),
                                }
                            )
                        }
                    } else {
                        let ident_code = self.generate_ident_name(prop)?;
                        quote! {
                            macroforge_ts::swc_core::ecma::ast::MemberProp::Ident(#ident_code)
                        }
                    };

                    quote! {
                        macroforge_ts::swc_core::ecma::ast::OptChainBase::Member(
                            macroforge_ts::swc_core::ecma::ast::MemberExpr {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                obj: Box::new(#base_obj_code),
                                prop: #prop_code,
                            }
                        )
                    }
                }
                // x?.() - optional call
                IrNode::CallExpr { callee: _, args, type_args: _, .. } => {
                    // callee is a placeholder, use base instead
                    let args_code: Vec<TokenStream> = args
                        .iter()
                        .map(|a| {
                            let expr = self.generate_expr(a)?;
                            Ok(quote! {
                                macroforge_ts::swc_core::ecma::ast::ExprOrSpread {
                                    spread: None,
                                    expr: Box::new(#expr),
                                }
                            })
                        })
                        .collect::<GenResult<Vec<_>>>()?;

                    quote! {
                        macroforge_ts::swc_core::ecma::ast::OptChainBase::Call(
                            macroforge_ts::swc_core::ecma::ast::OptCall {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                                callee: Box::new(#base_obj_code),
                                args: vec![#(#args_code),*],
                                type_args: None,
                            }
                        )
                    }
                }
                // Fallback for unexpected expr types
                _ => self.generate_opt_chain_base(base)?,
            };

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::OptChain(
                    macroforge_ts::swc_core::ecma::ast::OptChainExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        optional: true,
                        base: Box::new(#chain_base_code),
                    }
                )
            })
        }

        // Phase 5: Function expression
        // Note: unwrap_or for name is valid - anonymous functions are valid
        IrNode::FnExpr { async_, generator, name, type_params, params, return_type, body, .. } => {
            let params_code = self.generate_params(params)?;
            let name_code = match name.as_ref() {
                Some(n) => {
                    let nc = self.generate_ident(n)?;
                    quote! { Some(#nc) }
                }
                None => quote! { None },
            };
            let body_code = match body.as_ref() {
                Some(b) => {
                    let bc = self.generate_block_stmt(b)?;
                    quote! { Some(#bc) }
                }
                None => quote! { None },
            };

            let type_params_code = match type_params.as_ref() {
                Some(tp) => {
                    let tpc = self.generate_type_params(tp)?;
                    quote! { Some(Box::new(#tpc)) }
                }
                None => quote! { None },
            };

            let return_type_code = match return_type.as_ref() {
                Some(rt) => {
                    let rtc = self.generate_type_ann(rt)?;
                    quote! { Some(Box::new(#rtc)) }
                }
                None => quote! { None },
            };

            Ok(quote! {
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
                            type_params: #type_params_code,
                            return_type: #return_type_code,
                        }),
                    }
                )
            })
        }

        // Class expression
        // Note: unwrap_or for name is valid - anonymous classes are valid
        IrNode::ClassExpr { name, type_params, extends, implements, body, .. } => {
            let name_code = match name.as_ref() {
                Some(n) => {
                    let nc = self.generate_ident(n)?;
                    quote! { Some(#nc) }
                }
                None => quote! { None },
            };
            let extends_code = match extends.as_ref() {
                Some(e) => {
                    let ec = self.generate_expr(e)?;
                    quote! {
                        Some(Box::new(macroforge_ts::swc_core::ecma::ast::ExtendsClause {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            super_class: Box::new(#ec),
                            type_args: None,
                        }))
                    }
                }
                None => quote! { None },
            };
            let body_code = self.generate_class_members(body)?;

            let type_params_code = match type_params.as_ref() {
                Some(tp) => {
                    let tpc = self.generate_type_params(tp)?;
                    quote! { Some(Box::new(#tpc)) }
                }
                None => quote! { None },
            };

            let implements_code: Vec<TokenStream> = implements
                .iter()
                .filter_map(|i| {
                    let expr_code = self.generate_type(i).ok()?;
                    Some(quote! {
                        macroforge_ts::swc_core::ecma::ast::TsExprWithTypeArgs {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            expr: Box::new(#expr_code),
                            type_args: None,
                        }
                    })
                })
                .collect();

            Ok(quote! {
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
                            type_params: #type_params_code,
                            implements: vec![#(#implements_code),*],
                        }),
                    }
                )
            })
        }

        // Parenthesized expression: (expr)
        IrNode::ParenExpr { expr, .. } => {
            let expr_code = self.generate_expr(expr)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Paren(
                    macroforge_ts::swc_core::ecma::ast::ParenExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            })
        }

        // Sequence expression: a, b, c
        IrNode::SeqExpr { exprs, .. } => {
            let exprs_code: Vec<TokenStream> = exprs
                .iter()
                .map(|e| {
                    let ec = self.generate_expr(e)?;
                    Ok(quote! { Box::new(#ec) })
                })
                .collect::<GenResult<Vec<_>>>()?;

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::Seq(
                    macroforge_ts::swc_core::ecma::ast::SeqExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        exprs: vec![#(#exprs_code),*],
                    }
                )
            })
        }

        // Tagged template literal: tag`template`
        IrNode::TaggedTpl { tag, type_args, tpl, .. } => {
            let tag_code = self.generate_expr(tag)?;

            // Generate type_args if present
            let type_args_code = match type_args {
                Some(ta) => {
                    let tpc = self.generate_type_param_instantiation(ta)?;
                    quote! { Some(Box::new(#tpc)) }
                }
                None => quote! { None },
            };

            // Extract quasis and exprs from the TplLit node
            let (quasis, exprs) = match tpl.as_ref() {
                IrNode::TplLit { quasis, exprs, .. } => (quasis.clone(), exprs.clone()),
                _ => (vec![], vec![]),
            };

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
                    let ec = self.generate_expr(e)?;
                    Ok(quote! { Box::new(#ec) })
                })
                .collect::<GenResult<Vec<_>>>()?;

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Expr::TaggedTpl(
                    macroforge_ts::swc_core::ecma::ast::TaggedTpl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        tag: Box::new(#tag_code),
                        type_params: #type_args_code,
                        tpl: Box::new(macroforge_ts::swc_core::ecma::ast::Tpl {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            quasis: vec![#(#quasis_code),*],
                            exprs: vec![#(#exprs_code),*],
                        }),
                    }
                )
            })
        }

        // Raw text - parse as expression at runtime
        // Note: parse_ts_expr returns Result<Box<Expr>, _>, so we dereference to get Expr
        // Runtime parsing fallback is intentional - allows dynamic expression parsing
        IrNode::Raw { value: text, .. } => {
            Ok(quote! {
                {
                    let __source = #text;
                    *macroforge_ts::ts_syn::parse_ts_expr(__source)
                        .unwrap_or_else(|_| Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Invalid(
                            macroforge_ts::swc_core::ecma::ast::Invalid {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                            }
                        )))
                }
            })
        }

        // IdentBlock with multiple parts - build string and parse
        IrNode::IdentBlock { parts, .. } => {
            let part_exprs: Vec<TokenStream> = parts
                  .iter()
                  .map(|p| match p {
                      IrNode::Raw { value: text, .. } => quote! { __expr_str.push_str(#text); },
                      IrNode::StrLit { value: text, .. } => quote! { __expr_str.push_str(#text); },
                      IrNode::Ident { value: text, .. } => quote! { __expr_str.push_str(#text); },
                      IrNode::Placeholder { kind, expr, .. } => {
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
                      IrNode::IdentBlock { parts: inner_parts, .. } => {
                          let inner_exprs: Vec<TokenStream> = inner_parts
                              .iter()
                              .map(|ip| match ip {
                                  IrNode::Raw { value: text, .. } => quote! { __expr_str.push_str(#text); },
                                  IrNode::StrLit { value: text, .. } => quote! { __expr_str.push_str(#text); },
                                  IrNode::Ident { value: text, .. } => quote! { __expr_str.push_str(#text); },
                                  IrNode::Placeholder { kind, expr, .. } => {
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
                      IrNode::StringInterp { quote: q, parts: string_parts, .. } => {
                          let quote_char = q.to_string();
                          let inner_exprs: Vec<TokenStream> = string_parts
                              .iter()
                              .map(|sp| match sp {
                                  IrNode::Raw { value: text, .. } => quote! { __expr_str.push_str(#text); },
                                  IrNode::StrLit { value: text, .. } => quote! { __expr_str.push_str(#text); },
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
            // Runtime parsing fallback is intentional - allows dynamic expression construction
            Ok(quote! {
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
            })
        }

        // =========================================================================
        // Expression-Level Control Flow
        // These generate Rust control flow that produces TypeScript expressions
        // =========================================================================

        IrNode::IfExpr {
            condition,
            then_expr,
            else_if_branches,
            else_expr,
            ..
        } => {
            let then_code = self.generate_expr(then_expr)?;
            let else_code = self.generate_expr(else_expr)?;

            let else_if_codes: Vec<TokenStream> = else_if_branches
                .iter()
                .map(|(cond, expr)| {
                    let e = self.generate_expr(expr)?;
                    Ok(quote! { else if #cond { #e } })
                })
                .collect::<GenResult<_>>()?;

            Ok(quote! {
                if #condition { #then_code } #(#else_if_codes)* else { #else_code }
            })
        }

        IrNode::ForExpr {
            pattern,
            iterator,
            body_expr,
            ..
        } => {
            let body_code = self.generate_expr(body_expr)?;

            // Lazy iterator - no collect(), caller decides collection type
            Ok(quote! {
                (#iterator).into_iter().map(|#pattern| #body_code)
            })
        }

        IrNode::WhileExpr {
            condition,
            body_expr,
            ..
        } => {
            let body_code = self.generate_expr(body_expr)?;

            // While as iterator using std::iter::from_fn
            Ok(quote! {
                std::iter::from_fn(|| if #condition { Some(#body_code) } else { None })
            })
        }

        IrNode::MatchExpr { expr, arms, .. } => {
            let arm_codes: Vec<TokenStream> = arms
                .iter()
                .map(|arm| {
                    let pattern = &arm.pattern;
                    let body = self.generate_expr(&arm.body_expr).map_err(|e| {
                        e.with_context("match arm body").with_span(arm.span)
                    })?;
                    if let Some(guard) = &arm.guard {
                        Ok(quote! { #pattern if #guard => #body })
                    } else {
                        Ok(quote! { #pattern => #body })
                    }
                })
                .collect::<GenResult<_>>()?;

            Ok(quote! {
                match #expr { #(#arm_codes),* }
            })
        }

        // Default: return error for unknown node types
        other => Err(GenError::unexpected_node(
            "expression",
            other,
            &[
                "Ident", "StrLit", "NumLit", "BoolLit", "NullLit", "ThisExpr", "SuperExpr",
                "CallExpr", "MemberExpr", "ObjectLit", "ArrayLit", "BinExpr", "AssignExpr",
                "CondExpr", "NewExpr", "ArrowExpr", "TplLit", "StringInterp", "Placeholder",
                "TsAsExpr", "TsSatisfiesExpr", "TsNonNullExpr", "TsConstAssertion", "TsInstantiation",
                "AwaitExpr", "YieldExpr", "PrivateName", "BigIntLit", "UpdateExpr", "UnaryExpr",
                "OptChainExpr", "FnExpr", "ClassExpr", "ParenExpr", "SeqExpr", "TaggedTpl",
                "Raw", "IdentBlock", "IfExpr", "ForExpr", "WhileExpr", "MatchExpr",
            ],
        )),
    }
}
/// Generate code that builds an expression string from an IrNode.
    pub(in super::super) fn generate_expr_string_parts(&self, node: &IrNode) -> GenResult<TokenStream> {
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

/// Generate code for unary operators (-, +, !, ~, typeof, void, delete)
fn generate_unary_op(&self, op: &UnaryOp) -> TokenStream {
    match op {
        UnaryOp::Minus => quote! { macroforge_ts::swc_core::ecma::ast::UnaryOp::Minus },
        UnaryOp::Plus => quote! { macroforge_ts::swc_core::ecma::ast::UnaryOp::Plus },
        UnaryOp::Not => quote! { macroforge_ts::swc_core::ecma::ast::UnaryOp::Bang },
        UnaryOp::BitNot => quote! { macroforge_ts::swc_core::ecma::ast::UnaryOp::Tilde },
        UnaryOp::TypeOf => quote! { macroforge_ts::swc_core::ecma::ast::UnaryOp::TypeOf },
        UnaryOp::Void => quote! { macroforge_ts::swc_core::ecma::ast::UnaryOp::Void },
        UnaryOp::Delete => quote! { macroforge_ts::swc_core::ecma::ast::UnaryOp::Delete },
    }
}

/// Generate code for optional chain base (member access or call)
fn generate_opt_chain_base(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::MemberExpr { obj, prop, computed, .. } => {
            let obj_code = self.generate_expr(obj)?;
            let prop_code = if *computed {
                let p = self.generate_expr(prop)?;
                quote! {
                    macroforge_ts::swc_core::ecma::ast::MemberProp::Computed(
                        macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            expr: Box::new(#p),
                        }
                    )
                }
            } else {
                let ident_code = self.generate_ident_name(prop)?;
                quote! {
                    macroforge_ts::swc_core::ecma::ast::MemberProp::Ident(#ident_code)
                }
            };

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::OptChainBase::Member(
                    macroforge_ts::swc_core::ecma::ast::MemberExpr {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        obj: Box::new(#obj_code),
                        prop: #prop_code,
                    }
                )
            })
        }
        IrNode::CallExpr { callee, args, type_args: _, .. } => {
            let callee_code = self.generate_expr(callee)?;
            let args_code: Vec<TokenStream> = args
                .iter()
                .map(|a| {
                    let expr = self.generate_expr(a)?;
                    Ok(quote! {
                        macroforge_ts::swc_core::ecma::ast::ExprOrSpread {
                            spread: None,
                            expr: Box::new(#expr),
                        }
                    })
                })
                .collect::<GenResult<Vec<_>>>()?;

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::OptChainBase::Call(
                    macroforge_ts::swc_core::ecma::ast::OptCall {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        callee: Box::new(#callee_code),
                        args: vec![#(#args_code),*],
                        type_args: None,
                    }
                )
            })
        }
        other => Err(GenError::unexpected_node(
            "optional chain base",
            other,
            &["MemberExpr", "CallExpr"],
        )),
    }
}
}
