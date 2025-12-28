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
