use super::*;

impl Codegen {
    /// Generate a TsType from an IrNode (alias for generate_type)
    pub(in super::super) fn generate_ts_type(&self, node: &IrNode) -> TokenStream {
        self.generate_type(node)
    }

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

        // ThisType
        IrNode::ThisType => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsThisType(
                    macroforge_ts::swc_core::ecma::ast::TsThisType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                    }
                )
            }
        }

        // LiteralType
        IrNode::LiteralType { lit } => {
            let lit_code = self.generate_ts_lit(lit);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsLitType(
                    macroforge_ts::swc_core::ecma::ast::TsLitType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        lit: #lit_code,
                    }
                )
            }
        }

        // ParenType
        IrNode::ParenType { type_ann } => {
            let type_code = self.generate_type(type_ann);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsParenthesizedType(
                    macroforge_ts::swc_core::ecma::ast::TsParenthesizedType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        type_ann: Box::new(#type_code),
                    }
                )
            }
        }

        // TupleType
        IrNode::TupleType { elems } => {
            let elems_code: Vec<TokenStream> = elems
                .iter()
                .map(|e| {
                    let elem_code = self.generate_tuple_element(e);
                    quote! { #elem_code }
                })
                .collect();
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsTupleType(
                    macroforge_ts::swc_core::ecma::ast::TsTupleType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        elem_types: vec![#(#elems_code),*],
                    }
                )
            }
        }

        // IntersectionType
        IrNode::IntersectionType { types } => {
            let types_code: Vec<TokenStream> = types
                .iter()
                .map(|t| {
                    let tc = self.generate_type(t);
                    quote! { Box::new(#tc) }
                })
                .collect();

            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsUnionOrIntersectionType(
                    macroforge_ts::swc_core::ecma::ast::TsUnionOrIntersectionType::TsIntersectionType(
                        macroforge_ts::swc_core::ecma::ast::TsIntersectionType {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            types: vec![#(#types_code),*],
                        }
                    )
                )
            }
        }

        // TypeofType
        IrNode::TypeofType { expr } => {
            let expr_code = self.generate_entity_name(expr);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsTypeQuery(
                    macroforge_ts::swc_core::ecma::ast::TsTypeQuery {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr_name: macroforge_ts::swc_core::ecma::ast::TsTypeQueryExpr::TsEntityName(#expr_code),
                        type_args: None,
                    }
                )
            }
        }

        // KeyofType
        IrNode::KeyofType { type_ann } => {
            let type_code = self.generate_type(type_ann);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsTypeOperator(
                    macroforge_ts::swc_core::ecma::ast::TsTypeOperator {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        op: macroforge_ts::swc_core::ecma::ast::TsTypeOperatorOp::KeyOf,
                        type_ann: Box::new(#type_code),
                    }
                )
            }
        }

        // IndexedAccessType
        IrNode::IndexedAccessType { obj, index } => {
            let obj_code = self.generate_type(obj);
            let index_code = self.generate_type(index);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsIndexedAccessType(
                    macroforge_ts::swc_core::ecma::ast::TsIndexedAccessType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        readonly: false,
                        obj_type: Box::new(#obj_code),
                        index_type: Box::new(#index_code),
                    }
                )
            }
        }

        // ConditionalType
        IrNode::ConditionalType { check, extends, true_type, false_type } => {
            let check_code = self.generate_type(check);
            let extends_code = self.generate_type(extends);
            let true_code = self.generate_type(true_type);
            let false_code = self.generate_type(false_type);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsConditionalType(
                    macroforge_ts::swc_core::ecma::ast::TsConditionalType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        check_type: Box::new(#check_code),
                        extends_type: Box::new(#extends_code),
                        true_type: Box::new(#true_code),
                        false_type: Box::new(#false_code),
                    }
                )
            }
        }

        // InferType
        IrNode::InferType { type_param } => {
            let param_code = self.generate_type_param(type_param);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsInferType(
                    macroforge_ts::swc_core::ecma::ast::TsInferType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        type_param: Box::new(#param_code),
                    }
                )
            }
        }

        // MappedType
        IrNode::MappedType { readonly, type_param, name_type, optional, type_ann } => {
            let readonly_code = match readonly {
                Some(true) => quote! { Some(macroforge_ts::swc_core::ecma::ast::TruePlusMinus::Plus) },
                Some(false) => quote! { Some(macroforge_ts::swc_core::ecma::ast::TruePlusMinus::Minus) },
                None => quote! { None },
            };
            let optional_code = match optional {
                Some(true) => quote! { Some(macroforge_ts::swc_core::ecma::ast::TruePlusMinus::Plus) },
                Some(false) => quote! { Some(macroforge_ts::swc_core::ecma::ast::TruePlusMinus::Minus) },
                None => quote! { None },
            };
            let type_param_code = self.generate_type_param(type_param);
            let name_type_code = name_type.as_ref().map(|n| {
                let nc = self.generate_type(n);
                quote! { Some(Box::new(#nc)) }
            }).unwrap_or(quote! { None });
            let type_ann_code = type_ann.as_ref().map(|t| {
                let tc = self.generate_type(t);
                quote! { Some(Box::new(#tc)) }
            }).unwrap_or(quote! { None });
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsMappedType(
                    macroforge_ts::swc_core::ecma::ast::TsMappedType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        readonly: #readonly_code,
                        type_param: Box::new(#type_param_code),
                        name_type: #name_type_code,
                        optional: #optional_code,
                        type_ann: #type_ann_code,
                    }
                )
            }
        }

        // ConstructorType
        IrNode::ConstructorType { type_params: _, params, return_type } => {
            let params_code = self.generate_fn_type_params(params);
            let return_code = self.generate_type(return_type);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsConstructorType(
                    macroforge_ts::swc_core::ecma::ast::TsConstructorType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        params: #params_code,
                        type_params: None,
                        type_ann: Box::new(macroforge_ts::swc_core::ecma::ast::TsTypeAnn {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            type_ann: Box::new(#return_code),
                        }),
                        is_abstract: false,
                    }
                )
            }
        }

        // ImportType
        IrNode::ImportType { arg, qualifier, type_args: _ } => {
            let arg_code = self.generate_type(arg);
            let qualifier_code = qualifier.as_ref().map(|q| {
                let qc = self.generate_entity_name(q);
                quote! { Some(#qc) }
            }).unwrap_or(quote! { None });
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsImportType(
                    macroforge_ts::swc_core::ecma::ast::TsImportType {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: Box::new(#arg_code),
                        qualifier: #qualifier_code,
                        type_args: None,
                    }
                )
            }
        }

        // QualifiedName (handled via TypeRef for now)
        IrNode::QualifiedName { left, right } => {
            let left_code = self.generate_entity_name(left);
            let right_code = self.generate_ident(right);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsTypeRef(
                    macroforge_ts::swc_core::ecma::ast::TsTypeRef {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        type_name: macroforge_ts::swc_core::ecma::ast::TsEntityName::TsQualifiedName(
                            Box::new(macroforge_ts::swc_core::ecma::ast::TsQualifiedName {
                                left: #left_code,
                                right: macroforge_ts::swc_core::ecma::ast::IdentName::new(
                                    #right_code.sym.clone(),
                                    macroforge_ts::swc_core::common::DUMMY_SP,
                                ),
                            })
                        ),
                        type_params: None,
                    }
                )
            }
        }

        // ObjectType
        IrNode::ObjectType { members } => {
            let members_code = self.generate_type_members(members);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsType::TsTypeLit(
                    macroforge_ts::swc_core::ecma::ast::TsTypeLit {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        members: #members_code,
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

/// Generate a TsLit for literal types
    pub(in super::super) fn generate_ts_lit(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::StrLit(value) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsLit::Str(
                        macroforge_ts::swc_core::ecma::ast::Str {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: #value.into(),
                            raw: None,
                        }
                    )
                }
            }
            IrNode::NumLit(value) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsLit::Number(
                        macroforge_ts::swc_core::ecma::ast::Number {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: #value,
                            raw: None,
                        }
                    )
                }
            }
            IrNode::BoolLit(value) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsLit::Bool(
                        macroforge_ts::swc_core::ecma::ast::Bool {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: #value,
                        }
                    )
                }
            }
            IrNode::BigIntLit(value) => {
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsLit::BigInt(
                        macroforge_ts::swc_core::ecma::ast::BigInt {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: Box::new(#value.parse::<num_bigint::BigInt>().unwrap_or_default()),
                            raw: None,
                        }
                    )
                }
            }
            _ => {
                // Fallback - string literal
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsLit::Str(
                        macroforge_ts::swc_core::ecma::ast::Str {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            value: "".into(),
                            raw: None,
                        }
                    )
                }
            }
        }
    }

    /// Generate a TsTupleElement
    pub(in super::super) fn generate_tuple_element(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::OptionalType { type_ann } => {
                let type_code = self.generate_type(type_ann);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsTupleElement {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        label: None,
                        ty: Box::new(macroforge_ts::swc_core::ecma::ast::TsType::TsOptionalType(
                            macroforge_ts::swc_core::ecma::ast::TsOptionalType {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                type_ann: Box::new(#type_code),
                            }
                        )),
                    }
                }
            }
            IrNode::RestType { type_ann } => {
                let type_code = self.generate_type(type_ann);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsTupleElement {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        label: None,
                        ty: Box::new(macroforge_ts::swc_core::ecma::ast::TsType::TsRestType(
                            macroforge_ts::swc_core::ecma::ast::TsRestType {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                type_ann: Box::new(#type_code),
                            }
                        )),
                    }
                }
            }
            _ => {
                // Regular tuple element
                let type_code = self.generate_type(node);
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsTupleElement {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        label: None,
                        ty: Box::new(#type_code),
                    }
                }
            }
        }
    }

    /// Generate a single TsTypeParam
    pub(in super::super) fn generate_type_param(&self, node: &IrNode) -> TokenStream {
        match node {
            IrNode::TypeParam { name, constraint, default } => {
                let constraint_code = constraint.as_ref().map(|c| {
                    let cc = self.generate_type(c);
                    quote! { Some(Box::new(#cc)) }
                }).unwrap_or(quote! { None });
                let default_code = default.as_ref().map(|d| {
                    let dc = self.generate_type(d);
                    quote! { Some(Box::new(#dc)) }
                }).unwrap_or(quote! { None });
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsTypeParam {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        name: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        ),
                        is_in: false,
                        is_out: false,
                        is_const: false,
                        constraint: #constraint_code,
                        default: #default_code,
                    }
                }
            }
            IrNode::Ident(name) => {
                quote! {
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
                }
            }
            _ => {
                // Fallback
                quote! {
                    macroforge_ts::swc_core::ecma::ast::TsTypeParam {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        name: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            "T".into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        ),
                        is_in: false,
                        is_out: false,
                        is_const: false,
                        constraint: None,
                        default: None,
                    }
                }
            }
        }
    }

    /// Generate function type parameters (TsFnParam vec)
    pub(in super::super) fn generate_fn_type_params(&self, params: &[IrNode]) -> TokenStream {
        let params_code: Vec<TokenStream> = params.iter().map(|p| {
            match p {
                IrNode::Param { decorators: _, pat } => {
                    let pat_code = self.generate_pat(pat);
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::TsFnParam::Ident(
                            macroforge_ts::swc_core::ecma::ast::BindingIdent {
                                id: {
                                    let pat = #pat_code;
                                    match pat {
                                        macroforge_ts::swc_core::ecma::ast::Pat::Ident(i) => i.id,
                                        _ => macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                            "param".into(),
                                            macroforge_ts::swc_core::common::DUMMY_SP,
                                        ),
                                    }
                                },
                                type_ann: None,
                            }
                        )
                    }
                }
                IrNode::Ident(name) => {
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::TsFnParam::Ident(
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
                _ => {
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::TsFnParam::Ident(
                            macroforge_ts::swc_core::ecma::ast::BindingIdent {
                                id: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                    "param".into(),
                                    macroforge_ts::swc_core::common::DUMMY_SP,
                                ),
                                type_ann: None,
                            }
                        )
                    }
                }
            }
        }).collect();
        quote! { vec![#(#params_code),*] }
    }

    /// Generate type element members (TsTypeElement vec)
    pub(in super::super) fn generate_type_members(&self, members: &[IrNode]) -> TokenStream {
        let members_code: Vec<TokenStream> = members.iter().filter_map(|m| {
            match m {
                IrNode::PropSignature { name, type_ann, optional, readonly: _ } => {
                    let key_code = self.generate_prop_name(name);
                    let type_ann_code = type_ann.as_ref().map(|t| {
                        let tc = self.generate_type_ann(t);
                        quote! { Some(Box::new(#tc)) }
                    }).unwrap_or(quote! { None });
                    Some(quote! {
                        macroforge_ts::swc_core::ecma::ast::TsTypeElement::TsPropertySignature(
                            macroforge_ts::swc_core::ecma::ast::TsPropertySignature {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                readonly: false,
                                key: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident({
                                    match #key_code {
                                        macroforge_ts::swc_core::ecma::ast::PropName::Ident(i) =>
                                            macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(i.sym, i.span),
                                        _ => macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                            "prop".into(),
                                            macroforge_ts::swc_core::common::DUMMY_SP,
                                        ),
                                    }
                                })),
                                computed: false,
                                optional: #optional,
                                type_ann: #type_ann_code,
                            }
                        )
                    })
                }
                IrNode::MethodSignature { name, type_params: _, params, return_type, optional: _ } => {
                    let key_code = self.generate_prop_name(name);
                    let params_code = self.generate_fn_type_params(params);
                    let return_code = return_type.as_ref().map(|r| {
                        let rc = self.generate_type_ann(r);
                        quote! { Some(Box::new(#rc)) }
                    }).unwrap_or(quote! { None });
                    Some(quote! {
                        macroforge_ts::swc_core::ecma::ast::TsTypeElement::TsMethodSignature(
                            macroforge_ts::swc_core::ecma::ast::TsMethodSignature {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                key: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident({
                                    match #key_code {
                                        macroforge_ts::swc_core::ecma::ast::PropName::Ident(i) =>
                                            macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(i.sym, i.span),
                                        _ => macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                            "method".into(),
                                            macroforge_ts::swc_core::common::DUMMY_SP,
                                        ),
                                    }
                                })),
                                computed: false,
                                optional: false,
                                params: #params_code,
                                type_ann: #return_code,
                                type_params: None,
                            }
                        )
                    })
                }
                IrNode::IndexSignature { readonly, params, type_ann } => {
                    let params_code = self.generate_fn_type_params(params);
                    let return_code = {
                        let tc = self.generate_type_ann(type_ann);
                        quote! { Some(Box::new(#tc)) }
                    };
                    Some(quote! {
                        macroforge_ts::swc_core::ecma::ast::TsTypeElement::TsIndexSignature(
                            macroforge_ts::swc_core::ecma::ast::TsIndexSignature {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                params: #params_code,
                                type_ann: #return_code,
                                readonly: #readonly,
                                is_static: false,
                            }
                        )
                    })
                }
                _ => None,
            }
        }).collect();
        quote! { vec![#(#members_code),*] }
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
