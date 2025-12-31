use swc_core::ecma::ast::*;
use syn::parse_quote;

use crate::ast::ToCode;
use crate::ctxt::{Ctx, VarPos};

// Helper to check if a TsEntityName is a $variable reference
fn get_type_var_name(name: &TsEntityName) -> Option<&str> {
    if let TsEntityName::Ident(ident) = name {
        ident.sym.strip_prefix('$')
    } else {
        None
    }
}

// TsType - the main type enum with variable substitution support
impl ToCode for TsType {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        // Check for $variable pattern in TsTypeRef
        if let TsType::TsTypeRef(type_ref) = self
            && let Some(var_name) = get_type_var_name(&type_ref.type_name)
            && let Some(var) = cx.var(VarPos::TsType, var_name)
        {
            return var.get_expr();
        }

        match self {
            TsType::TsKeywordType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsKeywordType(#val))
            }
            TsType::TsThisType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsThisType(#val))
            }
            TsType::TsFnOrConstructorType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsFnOrConstructorType(#val))
            }
            TsType::TsTypeRef(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsTypeRef(#val))
            }
            TsType::TsTypeQuery(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsTypeQuery(#val))
            }
            TsType::TsTypeLit(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsTypeLit(#val))
            }
            TsType::TsArrayType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsArrayType(#val))
            }
            TsType::TsTupleType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsTupleType(#val))
            }
            TsType::TsOptionalType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsOptionalType(#val))
            }
            TsType::TsRestType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsRestType(#val))
            }
            TsType::TsUnionOrIntersectionType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsUnionOrIntersectionType(#val))
            }
            TsType::TsConditionalType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsConditionalType(#val))
            }
            TsType::TsInferType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsInferType(#val))
            }
            TsType::TsParenthesizedType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsParenthesizedType(#val))
            }
            TsType::TsTypeOperator(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsTypeOperator(#val))
            }
            TsType::TsIndexedAccessType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsIndexedAccessType(#val))
            }
            TsType::TsMappedType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsMappedType(#val))
            }
            TsType::TsLitType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsLitType(#val))
            }
            TsType::TsTypePredicate(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsTypePredicate(#val))
            }
            TsType::TsImportType(v) => {
                let val = v.to_code(cx);
                parse_quote!(macroforge_ts::swc_core::ecma::ast::TsType::TsImportType(#val))
            }
        }
    }
}

// TsTypeAnn - wraps a type with span
impl_struct!(TsTypeAnn, [span, type_ann]);

// TsTypeRef - reference to a named type
impl_struct!(TsTypeRef, [span, type_name, type_params]);

// TsEntityName - qualified or simple name
impl_enum!(TsEntityName, [TsQualifiedName, Ident]);

// TsQualifiedName
impl_struct!(TsQualifiedName, [span, left, right]);

// Keyword types
impl_struct!(TsKeywordType, [span, kind]);

impl ToCode for TsKeywordTypeKind {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        let variant = match self {
            TsKeywordTypeKind::TsAnyKeyword => "TsAnyKeyword",
            TsKeywordTypeKind::TsUnknownKeyword => "TsUnknownKeyword",
            TsKeywordTypeKind::TsNumberKeyword => "TsNumberKeyword",
            TsKeywordTypeKind::TsObjectKeyword => "TsObjectKeyword",
            TsKeywordTypeKind::TsBooleanKeyword => "TsBooleanKeyword",
            TsKeywordTypeKind::TsBigIntKeyword => "TsBigIntKeyword",
            TsKeywordTypeKind::TsStringKeyword => "TsStringKeyword",
            TsKeywordTypeKind::TsSymbolKeyword => "TsSymbolKeyword",
            TsKeywordTypeKind::TsVoidKeyword => "TsVoidKeyword",
            TsKeywordTypeKind::TsUndefinedKeyword => "TsUndefinedKeyword",
            TsKeywordTypeKind::TsNullKeyword => "TsNullKeyword",
            TsKeywordTypeKind::TsNeverKeyword => "TsNeverKeyword",
            TsKeywordTypeKind::TsIntrinsicKeyword => "TsIntrinsicKeyword",
        };
        let ident = syn::Ident::new(variant, proc_macro2::Span::call_site());
        parse_quote!(macroforge_ts::swc_core::ecma::ast::TsKeywordTypeKind::#ident)
    }
}

// This type
impl_struct!(TsThisType, [span]);

// Function and constructor types
impl_enum!(TsFnOrConstructorType, [TsFnType, TsConstructorType]);
impl_struct!(TsFnType, [span, type_params, params, type_ann]);
impl_struct!(
    TsConstructorType,
    [span, type_params, params, type_ann, is_abstract]
);

// Type parameters
impl_struct!(TsTypeParamInstantiation, [span, params]);
impl_struct!(TsTypeParamDecl, [span, params]);
impl_struct!(
    TsTypeParam,
    [span, name, is_in, is_out, is_const, constraint, default]
);

// Function parameter
impl_enum!(TsFnParam, [Ident, Array, Rest, Object]);

// Type query
impl_struct!(TsTypeQuery, [span, expr_name, type_args]);
impl_enum!(TsTypeQueryExpr, [TsEntityName, Import]);

// Type literal
impl_struct!(TsTypeLit, [span, members]);

// Array type
impl_struct!(TsArrayType, [span, elem_type]);

// Tuple type
impl_struct!(TsTupleType, [span, elem_types]);
impl_struct!(TsTupleElement, [span, label, ty]);

// Optional type
impl_struct!(TsOptionalType, [span, type_ann]);

// Rest type
impl_struct!(TsRestType, [span, type_ann]);

// Union and intersection
impl_enum!(TsUnionOrIntersectionType, [TsUnionType, TsIntersectionType]);
impl_struct!(TsUnionType, [span, types]);
impl_struct!(TsIntersectionType, [span, types]);

// Conditional type
impl_struct!(
    TsConditionalType,
    [span, check_type, extends_type, true_type, false_type]
);

// Infer type
impl_struct!(TsInferType, [span, type_param]);

// Parenthesized type
impl_struct!(TsParenthesizedType, [span, type_ann]);

// Type operator
impl_struct!(TsTypeOperator, [span, op, type_ann]);

impl ToCode for TsTypeOperatorOp {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        let variant = match self {
            TsTypeOperatorOp::KeyOf => "KeyOf",
            TsTypeOperatorOp::Unique => "Unique",
            TsTypeOperatorOp::ReadOnly => "ReadOnly",
        };
        let ident = syn::Ident::new(variant, proc_macro2::Span::call_site());
        parse_quote!(macroforge_ts::swc_core::ecma::ast::TsTypeOperatorOp::#ident)
    }
}

// Indexed access type
impl_struct!(TsIndexedAccessType, [span, readonly, obj_type, index_type]);

// Mapped type
impl_struct!(
    TsMappedType,
    [span, readonly, type_param, name_type, optional, type_ann]
);

impl ToCode for TruePlusMinus {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        let variant = match self {
            TruePlusMinus::True => "True",
            TruePlusMinus::Plus => "Plus",
            TruePlusMinus::Minus => "Minus",
        };
        let ident = syn::Ident::new(variant, proc_macro2::Span::call_site());
        parse_quote!(macroforge_ts::swc_core::ecma::ast::TruePlusMinus::#ident)
    }
}

// Literal type
impl_struct!(TsLitType, [span, lit]);
impl_enum!(TsLit, [Number, Str, Bool, BigInt, Tpl]);
impl_struct!(TsTplLitType, [span, types, quasis]);

// Type predicate
impl_struct!(TsTypePredicate, [span, asserts, param_name, type_ann]);
impl_enum!(TsThisTypeOrIdent, [TsThisType, Ident]);

// Import type
impl_struct!(TsImportCallOptions, [span, with]);
impl_struct!(TsImportType, [span, arg, qualifier, type_args, attributes]);

// Type elements (for interfaces and type literals)
impl_enum!(
    TsTypeElement,
    [
        TsCallSignatureDecl,
        TsConstructSignatureDecl,
        TsPropertySignature,
        TsGetterSignature,
        TsSetterSignature,
        TsMethodSignature,
        TsIndexSignature
    ]
);
impl_struct!(TsCallSignatureDecl, [span, params, type_ann, type_params]);
impl_struct!(
    TsConstructSignatureDecl,
    [span, params, type_ann, type_params]
);
impl_struct!(
    TsPropertySignature,
    [span, readonly, key, computed, optional, type_ann]
);
impl_struct!(TsGetterSignature, [span, key, computed, type_ann]);
impl_struct!(TsSetterSignature, [span, key, computed, param]);
impl_struct!(
    TsMethodSignature,
    [span, key, computed, optional, params, type_ann, type_params]
);
impl_struct!(
    TsIndexSignature,
    [span, params, type_ann, readonly, is_static]
);

// Declarations
impl_struct!(
    TsInterfaceDecl,
    [span, id, declare, type_params, extends, body]
);
impl_struct!(TsInterfaceBody, [span, body]);
impl_struct!(TsExprWithTypeArgs, [span, expr, type_args]);

impl_struct!(TsTypeAliasDecl, [span, declare, id, type_params, type_ann]);
impl_struct!(TsEnumDecl, [span, declare, is_const, id, members]);
impl_struct!(TsEnumMember, [span, id, init]);
impl_enum!(TsEnumMemberId, [Ident, Str]);

impl_struct!(TsModuleDecl, [span, declare, global, namespace, id, body]);
impl_enum!(TsModuleName, [Ident, Str]);
impl_enum!(TsNamespaceBody, [TsModuleBlock, TsNamespaceDecl]);
impl_struct!(TsModuleBlock, [span, body]);
impl_struct!(TsNamespaceDecl, [span, declare, global, id, body]);

impl_struct!(
    TsImportEqualsDecl,
    [span, is_export, is_type_only, id, module_ref]
);
impl_enum!(TsModuleRef, [TsEntityName, TsExternalModuleRef]);
impl_struct!(TsExternalModuleRef, [span, expr]);

impl_struct!(TsExportAssignment, [span, expr]);
impl_struct!(TsNamespaceExportDecl, [span, id]);

// Expressions with types
impl_struct!(TsTypeAssertion, [span, expr, type_ann]);
impl_struct!(TsConstAssertion, [span, expr]);
impl_struct!(TsNonNullExpr, [span, expr]);
impl_struct!(TsAsExpr, [span, expr, type_ann]);
impl_struct!(TsInstantiation, [span, expr, type_args]);
impl_struct!(TsSatisfiesExpr, [span, expr, type_ann]);

// Param prop
impl_struct!(
    TsParamProp,
    [
        span,
        decorators,
        accessibility,
        is_override,
        readonly,
        param
    ]
);
impl_enum!(TsParamPropParam, [Ident, Assign]);

// Accessibility is implemented in enums.rs via impl_simple_enum!

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ToCode;
    use crate::ctxt::Ctx;
    use quote::ToTokens;
    use rustc_hash::FxHashMap;
    use swc_core::atoms::Atom;
    use swc_core::common::{Span, SyntaxContext};

    /// Helper to create an empty context for testing
    fn empty_ctx() -> Ctx {
        Ctx {
            vars: FxHashMap::default(),
        }
    }

    // ==================== TsType Tests ====================

    #[test]
    fn test_ts_type_keyword_to_code() {
        let cx = empty_ctx();
        let ts_type = TsType::TsKeywordType(TsKeywordType {
            span: Span::default(),
            kind: TsKeywordTypeKind::TsStringKeyword,
        });
        let code = ts_type.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsType"));
        assert!(code_str.contains("TsKeywordType"));
    }

    #[test]
    fn test_ts_type_this_to_code() {
        let cx = empty_ctx();
        let ts_type = TsType::TsThisType(TsThisType {
            span: Span::default(),
        });
        let code = ts_type.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsType"));
        assert!(code_str.contains("TsThisType"));
    }

    #[test]
    fn test_ts_type_array_to_code() {
        let cx = empty_ctx();
        let ts_type = TsType::TsArrayType(TsArrayType {
            span: Span::default(),
            elem_type: Box::new(TsType::TsKeywordType(TsKeywordType {
                span: Span::default(),
                kind: TsKeywordTypeKind::TsNumberKeyword,
            })),
        });
        let code = ts_type.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsType"));
        assert!(code_str.contains("TsArrayType"));
    }

    // ==================== TsKeywordTypeKind Tests ====================

    #[test]
    fn test_ts_keyword_type_kind_string() {
        let cx = empty_ctx();
        let kind = TsKeywordTypeKind::TsStringKeyword;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsKeywordTypeKind"));
        assert!(code_str.contains("TsStringKeyword"));
    }

    #[test]
    fn test_ts_keyword_type_kind_number() {
        let cx = empty_ctx();
        let kind = TsKeywordTypeKind::TsNumberKeyword;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsNumberKeyword"));
    }

    #[test]
    fn test_ts_keyword_type_kind_boolean() {
        let cx = empty_ctx();
        let kind = TsKeywordTypeKind::TsBooleanKeyword;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsBooleanKeyword"));
    }

    #[test]
    fn test_ts_keyword_type_kind_void() {
        let cx = empty_ctx();
        let kind = TsKeywordTypeKind::TsVoidKeyword;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsVoidKeyword"));
    }

    #[test]
    fn test_ts_keyword_type_kind_never() {
        let cx = empty_ctx();
        let kind = TsKeywordTypeKind::TsNeverKeyword;
        let code = kind.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsNeverKeyword"));
    }

    // ==================== TsTypeAnn Tests ====================

    #[test]
    fn test_ts_type_ann_to_code() {
        let cx = empty_ctx();
        let ann = TsTypeAnn {
            span: Span::default(),
            type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
                span: Span::default(),
                kind: TsKeywordTypeKind::TsStringKeyword,
            })),
        };
        let code = ann.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsTypeAnn"));
        assert!(code_str.contains("type_ann"));
    }

    // ==================== TsTypeRef Tests ====================

    #[test]
    fn test_ts_type_ref_to_code() {
        let cx = empty_ctx();
        let type_ref = TsTypeRef {
            span: Span::default(),
            type_name: TsEntityName::Ident(Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("MyType"),
                optional: false,
            }),
            type_params: None,
        };
        let code = type_ref.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsTypeRef"));
        assert!(code_str.contains("type_name"));
    }

    // ==================== TsEntityName Tests ====================

    #[test]
    fn test_ts_entity_name_ident_to_code() {
        let cx = empty_ctx();
        let name = TsEntityName::Ident(Ident {
            span: Span::default(),
            ctxt: SyntaxContext::empty(),
            sym: Atom::from("TypeName"),
            optional: false,
        });
        let code = name.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsEntityName"));
        assert!(code_str.contains("Ident"));
    }

    // ==================== TsTypeOperatorOp Tests ====================

    #[test]
    fn test_ts_type_operator_op_keyof() {
        let cx = empty_ctx();
        let op = TsTypeOperatorOp::KeyOf;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsTypeOperatorOp"));
        assert!(code_str.contains("KeyOf"));
    }

    #[test]
    fn test_ts_type_operator_op_readonly() {
        let cx = empty_ctx();
        let op = TsTypeOperatorOp::ReadOnly;
        let code = op.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("ReadOnly"));
    }

    // ==================== TruePlusMinus Tests ====================

    #[test]
    fn test_true_plus_minus_true() {
        let cx = empty_ctx();
        let val = TruePlusMinus::True;
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TruePlusMinus"));
        assert!(code_str.contains("True"));
    }

    #[test]
    fn test_true_plus_minus_plus() {
        let cx = empty_ctx();
        let val = TruePlusMinus::Plus;
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Plus"));
    }

    #[test]
    fn test_true_plus_minus_minus() {
        let cx = empty_ctx();
        let val = TruePlusMinus::Minus;
        let code = val.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("Minus"));
    }

    // ==================== TsUnionType Tests ====================

    #[test]
    fn test_ts_union_type_to_code() {
        let cx = empty_ctx();
        let union = TsUnionType {
            span: Span::default(),
            types: vec![
                Box::new(TsType::TsKeywordType(TsKeywordType {
                    span: Span::default(),
                    kind: TsKeywordTypeKind::TsStringKeyword,
                })),
                Box::new(TsType::TsKeywordType(TsKeywordType {
                    span: Span::default(),
                    kind: TsKeywordTypeKind::TsNumberKeyword,
                })),
            ],
        };
        let code = union.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsUnionType"));
        assert!(code_str.contains("types"));
    }

    // ==================== TsIntersectionType Tests ====================

    #[test]
    fn test_ts_intersection_type_to_code() {
        let cx = empty_ctx();
        let intersection = TsIntersectionType {
            span: Span::default(),
            types: vec![],
        };
        let code = intersection.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsIntersectionType"));
    }

    // ==================== TsTupleType Tests ====================

    #[test]
    fn test_ts_tuple_type_to_code() {
        let cx = empty_ctx();
        let tuple = TsTupleType {
            span: Span::default(),
            elem_types: vec![],
        };
        let code = tuple.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsTupleType"));
        assert!(code_str.contains("elem_types"));
    }

    // ==================== TsInterfaceDecl Tests ====================

    #[test]
    fn test_ts_interface_decl_to_code() {
        let cx = empty_ctx();
        let decl = TsInterfaceDecl {
            span: Span::default(),
            id: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("MyInterface"),
                optional: false,
            },
            declare: false,
            type_params: None,
            extends: vec![],
            body: TsInterfaceBody {
                span: Span::default(),
                body: vec![],
            },
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsInterfaceDecl"));
        assert!(code_str.contains("id"));
        assert!(code_str.contains("body"));
    }

    // ==================== TsTypeAliasDecl Tests ====================

    #[test]
    fn test_ts_type_alias_decl_to_code() {
        let cx = empty_ctx();
        let decl = TsTypeAliasDecl {
            span: Span::default(),
            declare: false,
            id: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("MyAlias"),
                optional: false,
            },
            type_params: None,
            type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
                span: Span::default(),
                kind: TsKeywordTypeKind::TsStringKeyword,
            })),
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsTypeAliasDecl"));
        assert!(code_str.contains("id"));
        assert!(code_str.contains("type_ann"));
    }

    // ==================== TsEnumDecl Tests ====================

    #[test]
    fn test_ts_enum_decl_to_code() {
        let cx = empty_ctx();
        let decl = TsEnumDecl {
            span: Span::default(),
            declare: false,
            is_const: false,
            id: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("MyEnum"),
                optional: false,
            },
            members: vec![],
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsEnumDecl"));
        assert!(code_str.contains("id"));
        assert!(code_str.contains("members"));
    }

    #[test]
    fn test_ts_enum_decl_const() {
        let cx = empty_ctx();
        let decl = TsEnumDecl {
            span: Span::default(),
            declare: false,
            is_const: true,
            id: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("ConstEnum"),
                optional: false,
            },
            members: vec![],
        };
        let code = decl.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsEnumDecl"));
        assert!(code_str.contains("is_const"));
    }

    // ==================== TsAsExpr Tests ====================

    #[test]
    fn test_ts_as_expr_to_code() {
        let cx = empty_ctx();
        let expr = TsAsExpr {
            span: Span::default(),
            expr: Box::new(Expr::Ident(Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("value"),
                optional: false,
            })),
            type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
                span: Span::default(),
                kind: TsKeywordTypeKind::TsStringKeyword,
            })),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsAsExpr"));
        assert!(code_str.contains("expr"));
        assert!(code_str.contains("type_ann"));
    }

    // ==================== TsNonNullExpr Tests ====================

    #[test]
    fn test_ts_non_null_expr_to_code() {
        let cx = empty_ctx();
        let expr = TsNonNullExpr {
            span: Span::default(),
            expr: Box::new(Expr::Ident(Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("value"),
                optional: false,
            })),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsNonNullExpr"));
        assert!(code_str.contains("expr"));
    }

    // ==================== TsConstAssertion Tests ====================

    #[test]
    fn test_ts_const_assertion_to_code() {
        let cx = empty_ctx();
        let expr = TsConstAssertion {
            span: Span::default(),
            expr: Box::new(Expr::Lit(Lit::Str(Str {
                span: Span::default(),
                value: Atom::from("literal").into(),
                raw: None,
            }))),
        };
        let code = expr.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsConstAssertion"));
        assert!(code_str.contains("expr"));
    }

    // ==================== TsParamProp Tests ====================

    #[test]
    fn test_ts_param_prop_to_code() {
        let cx = empty_ctx();
        let prop = TsParamProp {
            span: Span::default(),
            decorators: vec![],
            accessibility: Some(Accessibility::Public),
            is_override: false,
            readonly: false,
            param: TsParamPropParam::Ident(BindingIdent {
                id: Ident {
                    span: Span::default(),
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from("name"),
                    optional: false,
                },
                type_ann: None,
            }),
        };
        let code = prop.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsParamProp"));
        assert!(code_str.contains("accessibility"));
        assert!(code_str.contains("param"));
    }

    // ==================== TsFnType Tests ====================

    #[test]
    fn test_ts_fn_type_to_code() {
        let cx = empty_ctx();
        let fn_type = TsFnType {
            span: Span::default(),
            type_params: None,
            params: vec![],
            type_ann: Box::new(TsTypeAnn {
                span: Span::default(),
                type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
                    span: Span::default(),
                    kind: TsKeywordTypeKind::TsVoidKeyword,
                })),
            }),
        };
        let code = fn_type.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsFnType"));
        assert!(code_str.contains("params"));
        assert!(code_str.contains("type_ann"));
    }

    // ==================== TsTypeParam Tests ====================

    #[test]
    fn test_ts_type_param_to_code() {
        let cx = empty_ctx();
        let param = TsTypeParam {
            span: Span::default(),
            name: Ident {
                span: Span::default(),
                ctxt: SyntaxContext::empty(),
                sym: Atom::from("T"),
                optional: false,
            },
            is_in: false,
            is_out: false,
            is_const: false,
            constraint: None,
            default: None,
        };
        let code = param.to_code(&cx);
        let code_str = code.to_token_stream().to_string();
        assert!(code_str.contains("TsTypeParam"));
        assert!(code_str.contains("name"));
    }
}
