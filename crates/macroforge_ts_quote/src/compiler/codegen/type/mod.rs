use super::*;

impl Codegen {
    pub(in super::super) fn generate_type_params(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::TypeParams { params } => {
            // Generate each type param
            let params_code: Vec<TokenStream> = params
                .iter()
                .filter_map(|p| match p {
                    IrNode::Raw(text) => {
                        let name = text.trim();
                        if name.is_empty() {
                            None
                        } else {
                            Some(quote! {
                                macroforge_ts::swc_core::ecma::ast::TsTypeParam {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    name: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                        #name.into(),
                                        macroforge_ts::swc_core::common::DUMMY_SP,
                                    ),
                                    is_in: false,
                                    is_out: false,
                                    is_const: false,
                                    constraint: None,
                                    default: None,
                                }
                            })
                        }
                    }
                    IrNode::Placeholder { expr, .. } => Some(quote! {
                        macroforge_ts::swc_core::ecma::ast::TsTypeParam {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            name: macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()),
                            is_in: false,
                            is_out: false,
                            is_const: false,
                            constraint: None,
                            default: None,
                        }
                    }),
                    _ => None,
                })
                .collect();

            quote! {
                macroforge_ts::swc_core::ecma::ast::TsTypeParamDecl {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    params: vec![#(#params_code),*],
                }
            }
        }
        _ => {
            // Fallback - empty type params
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsTypeParamDecl {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    params: vec![],
                }
            }
        }
    }
}

pub(in super::super) fn generate_type_param_instantiation(&self, _node: &IrNode) -> TokenStream {
    quote! {
        macroforge_ts::swc_core::ecma::ast::TsTypeParamInstantiation {
            span: macroforge_ts::swc_core::common::DUMMY_SP,
            params: vec![],
        }
    }
}

pub(in super::super) fn generate_type_ann(&self, node: &IrNode) -> TokenStream {
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

pub(in super::super) fn generate_type(&self, node: &IrNode) -> TokenStream {
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
                TsKeyword::Any => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsAnyKeyword }
                }
                TsKeyword::Unknown => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsUnknownKeyword }
                }
                TsKeyword::String => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsStringKeyword }
                }
                TsKeyword::Number => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsNumberKeyword }
                }
                TsKeyword::Boolean => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsBooleanKeyword }
                }
                TsKeyword::Void => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsVoidKeyword }
                }
                TsKeyword::Null => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsNullKeyword }
                }
                TsKeyword::Undefined => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsUndefinedKeyword }
                }
                TsKeyword::Never => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsNeverKeyword }
                }
                TsKeyword::Object => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsObjectKeyword }
                }
                TsKeyword::BigInt => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsBigIntKeyword }
                }
                TsKeyword::Symbol => {
                    quote! { macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::TsSymbolKeyword }
                }
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
            quote! { macroforge_ts::ts_syn::ToTsType::to_ts_type((#expr).clone()) }
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
                .filter_map(|p| self.generate_type_str_part(p))
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
        IrNode::TypeAnnotation { type_ann } => self.generate_type(type_ann),

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

/// Generate code that pushes to __type_str for building types with control flow
    pub(in super::super) fn generate_type_str_part(&self, node: &IrNode) -> Option<TokenStream> {
    match node {
        IrNode::Raw(text) => Some(quote! { __type_str.push_str(#text); }),
        IrNode::StrLit(text) => Some(quote! { __type_str.push_str(#text); }),
        IrNode::Ident(text) => Some(quote! { __type_str.push_str(#text); }),
        IrNode::Placeholder { kind, expr } => {
            match kind {
                PlaceholderKind::Type => Some(quote! {
                    let __ty = macroforge_ts::ts_syn::ToTsType::to_ts_type((#expr).clone());
                    __type_str.push_str(&macroforge_ts::ts_syn::emit_ts_type(&__ty));
                }),
                PlaceholderKind::Ident => Some(quote! {
                    __type_str.push_str(&macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()).sym.to_string());
                }),
                PlaceholderKind::Expr => {
                    // In type context, expressions are typically used for property names
                    // Quote the string value for proper TypeScript object type syntax
                    Some(quote! {
                        __type_str.push_str("\"");
                        __type_str.push_str(&(#expr).to_string());
                        __type_str.push_str("\"");
                    })
                }
                PlaceholderKind::Stmt => {
                    // Statements in type context - rare, but handle gracefully
                    Some(quote! { /* stmt placeholder in type context */ })
                }
            }
        }
        IrNode::IdentBlock { parts } => {
            // Nested IdentBlock - recursively generate parts
            let inner_parts: Vec<TokenStream> = parts
                .iter()
                .filter_map(|p| self.generate_type_str_part(p))
                .collect();
            Some(quote! { #(#inner_parts)* })
        }
        IrNode::For {
            pattern,
            iterator,
            body,
        } => {
            // Control flow: for loop inside type
            let body_parts: Vec<TokenStream> = body
                .iter()
                .filter_map(|p| self.generate_type_str_part(p))
                .collect();
            Some(quote! {
                for #pattern in #iterator {
                    #(#body_parts)*
                }
            })
        }
        IrNode::If {
            condition,
            then_body,
            else_if_branches,
            else_body,
        } => {
            // Control flow: if/else inside type
            let then_parts: Vec<TokenStream> = then_body
                .iter()
                .filter_map(|p| self.generate_type_str_part(p))
                .collect();

            let else_if_code: Vec<TokenStream> = else_if_branches
                .iter()
                .map(|(cond, body)| {
                    let branch_parts: Vec<TokenStream> = body
                        .iter()
                        .filter_map(|p| self.generate_type_str_part(p))
                        .collect();
                    quote! {
                        else if #cond {
                            #(#branch_parts)*
                        }
                    }
                })
                .collect();

            let else_code = else_body.as_ref().map(|body| {
                let else_parts: Vec<TokenStream> = body
                    .iter()
                    .filter_map(|p| self.generate_type_str_part(p))
                    .collect();
                quote! {
                    else {
                        #(#else_parts)*
                    }
                }
            });

            Some(quote! {
                if #condition {
                    #(#then_parts)*
                }
                #(#else_if_code)*
                #else_code
            })
        }
        IrNode::While { condition, body } => {
            let body_parts: Vec<TokenStream> = body
                .iter()
                .filter_map(|p| self.generate_type_str_part(p))
                .collect();
            Some(quote! {
                while #condition {
                    #(#body_parts)*
                }
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
                        let body_parts: Vec<TokenStream> = body
                            .iter()
                            .filter_map(|p| self.generate_type_str_part(p))
                            .collect();
                        if let Some(g) = guard {
                            quote! { #pattern if #g => { #(#body_parts)* } }
                        } else {
                            quote! { #pattern => { #(#body_parts)* } }
                        }
                    },
                )
                .collect();
            Some(quote! {
                match #expr {
                    #(#arm_tokens)*
                }
            })
        }
        IrNode::Let { pattern, value, .. } => Some(quote! { let #pattern = #value; }),
        IrNode::TypeAnnotation { type_ann } => self.generate_type_str_part(type_ann),
        _ => None,
    }
}
}
