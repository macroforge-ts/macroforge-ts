#![cfg(feature = "oxc")]
use oxc_ast::ast::*;
use syn::parse_quote;

use super::ToCode;
use super::ctxt::Ctx;

macro_rules! fail_todo {
    ($T:ty) => {
        impl crate::ToCode for $T {
            fn to_code(&self, _: &crate::ctxt::Ctx) -> syn::Expr {
                todo!("ToCode for {}", stringify!($T))
            }
        }
    };
}

impl<T> ToCode for oxc_allocator::Box<'_, T>
where
    T: ToCode,
{
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let inner = (**self).to_code(cx);
        parse_quote! {
            allocator.alloc(#inner)
        }
    }
}

impl<T> ToCode for oxc_allocator::Vec<'_, T>
where
    T: ToCode,
{
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let items: Vec<syn::Expr> = self.iter().map(|item| item.to_code(cx)).collect();
        parse_quote! {
            allocator.alloc_vec(&[#(#items),*])
        }
    }
}

impl<T> ToCode for Option<T>
where
    T: ToCode,
{
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match self {
            Some(v) => {
                let inner = v.to_code(cx);
                parse_quote!(Some(#inner))
            }
            None => parse_quote!(None),
        }
    }
}

impl ToCode for oxc_span::Span {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote!(oxc_span::Span::default())
    }
}

impl ToCode for oxc_span::Atom<'_> {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        let s = self.as_str();
        parse_quote!(allocator.alloc_str(#s).into())
    }
}

impl ToCode for Expression<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match self {
            Expression::BooleanLiteral(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::BooleanLiteral(#inner))
            }
            Expression::NullLiteral(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::NullLiteral(#inner))
            }
            Expression::NumericLiteral(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::NumericLiteral(#inner))
            }
            Expression::StringLiteral(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::StringLiteral(#inner))
            }
            Expression::Identifier(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::Identifier(#inner))
            }
            Expression::BinaryExpression(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::BinaryExpression(#inner))
            }
            Expression::CallExpression(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::CallExpression(#inner))
            }
            Expression::ComputedMemberExpression(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::ComputedMemberExpression(#inner))
            }
            Expression::StaticMemberExpression(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::StaticMemberExpression(#inner))
            }
            Expression::PrivateFieldExpression(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Expression::PrivateFieldExpression(#inner))
            }
            _ => todo!("ToCode for Expression variant"),
        }
    }
}

impl ToCode for BooleanLiteral {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        let value = self.value;
        parse_quote! {
            oxc_ast::ast::BooleanLiteral {
                span: oxc_span::Span::default(),
                value: #value,
            }
        }
    }
}

impl ToCode for NullLiteral {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote! {
            oxc_ast::ast::NullLiteral {
                span: oxc_span::Span::default(),
            }
        }
    }
}

impl ToCode for NumericLiteral<'_> {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        let value = self.value;
        let raw = self.raw.as_ref().map(|a| a.as_str());
        let raw_expr: syn::Expr = match raw {
            Some(s) => parse_quote!(Some(allocator.alloc_str(#s).into())),
            None => parse_quote!(None),
        };
        parse_quote! {
            oxc_ast::ast::NumericLiteral {
                span: oxc_span::Span::default(),
                value: #value,
                raw: #raw_expr,
                base: oxc_ast::ast::NumberBase::Decimal,
            }
        }
    }
}

impl ToCode for StringLiteral<'_> {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        let value = self.value.as_str();
        parse_quote! {
            oxc_ast::ast::StringLiteral {
                span: oxc_span::Span::default(),
                value: allocator.alloc_str(#value).into(),
            }
        }
    }
}

impl ToCode for IdentifierReference<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let name = self.name.to_code(cx);
        parse_quote! {
            oxc_ast::ast::IdentifierReference {
                span: oxc_span::Span::default(),
                name: #name,
                reference_id: std::cell::Cell::new(None),
                reference_flag: oxc_semantic::ReferenceFlags::default(),
            }
        }
    }
}

impl ToCode for IdentifierName<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let name = self.name.to_code(cx);
        parse_quote! {
            oxc_ast::ast::IdentifierName {
                span: oxc_span::Span::default(),
                name: #name,
            }
        }
    }
}

impl ToCode for BindingIdentifier<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let name = self.name.to_code(cx);
        parse_quote! {
            oxc_ast::ast::BindingIdentifier {
                span: oxc_span::Span::default(),
                name: #name,
                symbol_id: std::cell::Cell::new(None),
            }
        }
    }
}

impl ToCode for BinaryExpression<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let left = self.left.to_code(cx);
        let right = self.right.to_code(cx);
        let operator = format!("{:?}", self.operator);
        let operator =
            syn::parse_str::<syn::Path>(&format!("oxc_ast::ast::BinaryOperator::{}", operator))
                .unwrap();
        parse_quote! {
            oxc_ast::ast::BinaryExpression {
                span: oxc_span::Span::default(),
                left: #left,
                right: #right,
                operator: #operator,
            }
        }
    }
}

impl ToCode for CallExpression<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let callee = self.callee.to_code(cx);
        let arguments = self.arguments.to_code(cx);
        let type_parameters = self.type_parameters.to_code(cx);
        parse_quote! {
            oxc_ast::ast::CallExpression {
                span: oxc_span::Span::default(),
                callee: #callee,
                arguments: #arguments,
                type_parameters: #type_parameters,
                optional: false,
            }
        }
    }
}

impl ToCode for Argument<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match self {
            Argument::SpreadElement(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Argument::SpreadElement(#inner))
            }
            _ => {
                let inner = self.to_expression().to_code(cx);
                parse_quote!(#inner.into())
            }
        }
    }
}

impl ToCode for SpreadElement<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let argument = self.argument.to_code(cx);
        parse_quote! {
            oxc_ast::ast::SpreadElement {
                span: oxc_span::Span::default(),
                argument: #argument,
            }
        }
    }
}

impl ToCode for MemberExpression<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match self {
            MemberExpression::ComputedMemberExpression(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::MemberExpression::ComputedMemberExpression(#inner))
            }
            MemberExpression::StaticMemberExpression(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::MemberExpression::StaticMemberExpression(#inner))
            }
            MemberExpression::PrivateFieldExpression(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::MemberExpression::PrivateFieldExpression(#inner))
            }
        }
    }
}

impl ToCode for ComputedMemberExpression<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let object = self.object.to_code(cx);
        let expression = self.expression.to_code(cx);
        parse_quote! {
            oxc_ast::ast::ComputedMemberExpression {
                span: oxc_span::Span::default(),
                object: #object,
                expression: #expression,
                optional: false,
            }
        }
    }
}

impl ToCode for StaticMemberExpression<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let object = self.object.to_code(cx);
        let property = self.property.to_code(cx);
        parse_quote! {
            oxc_ast::ast::StaticMemberExpression {
                span: oxc_span::Span::default(),
                object: #object,
                property: #property,
                optional: false,
            }
        }
    }
}

impl ToCode for PrivateFieldExpression<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let object = self.object.to_code(cx);
        let field = self.field.to_code(cx);
        parse_quote! {
            oxc_ast::ast::PrivateFieldExpression {
                span: oxc_span::Span::default(),
                object: #object,
                field: #field,
                optional: false,
            }
        }
    }
}

impl ToCode for PrivateIdentifier<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let name = self.name.to_code(cx);
        parse_quote! {
            oxc_ast::ast::PrivateIdentifier {
                span: oxc_span::Span::default(),
                name: #name,
            }
        }
    }
}

impl ToCode for Statement<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match self {
            Statement::ExpressionStatement(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Statement::ExpressionStatement(#inner))
            }
            Statement::BlockStatement(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Statement::BlockStatement(#inner))
            }
            Statement::ReturnStatement(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Statement::ReturnStatement(#inner))
            }
            Statement::IfStatement(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Statement::IfStatement(#inner))
            }
            Statement::VariableDeclaration(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::Statement::VariableDeclaration(#inner))
            }
            _ => todo!("ToCode for Statement variant"),
        }
    }
}

impl ToCode for ExpressionStatement<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let expression = self.expression.to_code(cx);
        parse_quote! {
            oxc_ast::ast::ExpressionStatement {
                span: oxc_span::Span::default(),
                expression: #expression,
            }
        }
    }
}

impl ToCode for BlockStatement<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let body = self.body.to_code(cx);
        parse_quote! {
            oxc_ast::ast::BlockStatement {
                span: oxc_span::Span::default(),
                body: #body,
            }
        }
    }
}

impl ToCode for ReturnStatement<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let argument = self.argument.to_code(cx);
        parse_quote! {
            oxc_ast::ast::ReturnStatement {
                span: oxc_span::Span::default(),
                argument: #argument,
            }
        }
    }
}

impl ToCode for IfStatement<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let test = self.test.to_code(cx);
        let consequent = self.consequent.to_code(cx);
        let alternate = self.alternate.to_code(cx);
        parse_quote! {
            oxc_ast::ast::IfStatement {
                span: oxc_span::Span::default(),
                test: #test,
                consequent: #consequent,
                alternate: #alternate,
            }
        }
    }
}

impl ToCode for VariableDeclaration<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let kind = format!("{:?}", self.kind);
        let kind = syn::parse_str::<syn::Path>(&format!(
            "oxc_ast::ast::VariableDeclarationKind::{}",
            kind
        ))
        .unwrap();
        let declarations = self.declarations.to_code(cx);
        parse_quote! {
            oxc_ast::ast::VariableDeclaration {
                span: oxc_span::Span::default(),
                kind: #kind,
                declarations: #declarations,
                declare: false,
            }
        }
    }
}

impl ToCode for VariableDeclarator<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let id = self.id.to_code(cx);
        let init = self.init.to_code(cx);
        parse_quote! {
            oxc_ast::ast::VariableDeclarator {
                span: oxc_span::Span::default(),
                kind: oxc_ast::ast::VariableDeclarationKind::Var,
                id: #id,
                init: #init,
                definite: false,
            }
        }
    }
}

impl ToCode for BindingPattern<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match &self.kind {
            BindingPatternKind::BindingIdentifier(e) => {
                let inner = e.to_code(cx);
                parse_quote! {
                    oxc_ast::ast::BindingPattern {
                        kind: oxc_ast::ast::BindingPatternKind::BindingIdentifier(#inner),
                        type_annotation: None,
                        optional: false,
                    }
                }
            }
            _ => todo!("ToCode for BindingPattern variant"),
        }
    }
}

impl ToCode for TSTypeParameterInstantiation<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let params = self.params.to_code(cx);
        parse_quote! {
            oxc_ast::ast::TSTypeParameterInstantiation {
                span: oxc_span::Span::default(),
                params: #params,
            }
        }
    }
}

impl ToCode for TSType<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match self {
            TSType::TSAnyKeyword(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSType::TSAnyKeyword(#inner))
            }
            TSType::TSUnknownKeyword(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSType::TSUnknownKeyword(#inner))
            }
            TSType::TSNumberKeyword(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSType::TSNumberKeyword(#inner))
            }
            TSType::TSBooleanKeyword(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSType::TSBooleanKeyword(#inner))
            }
            TSType::TSStringKeyword(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSType::TSStringKeyword(#inner))
            }
            TSType::TSVoidKeyword(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSType::TSVoidKeyword(#inner))
            }
            TSType::TSNullKeyword(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSType::TSNullKeyword(#inner))
            }
            TSType::TSUndefinedKeyword(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSType::TSUndefinedKeyword(#inner))
            }
            TSType::TSTypeReference(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSType::TSTypeReference(#inner))
            }
            _ => todo!("ToCode for TSType variant"),
        }
    }
}

impl ToCode for TSAnyKeyword {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote! {
            oxc_ast::ast::TSAnyKeyword {
                span: oxc_span::Span::default(),
            }
        }
    }
}

impl ToCode for TSUnknownKeyword {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote! {
            oxc_ast::ast::TSUnknownKeyword {
                span: oxc_span::Span::default(),
            }
        }
    }
}

impl ToCode for TSNumberKeyword {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote! {
            oxc_ast::ast::TSNumberKeyword {
                span: oxc_span::Span::default(),
            }
        }
    }
}

impl ToCode for TSBooleanKeyword {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote! {
            oxc_ast::ast::TSBooleanKeyword {
                span: oxc_span::Span::default(),
            }
        }
    }
}

impl ToCode for TSStringKeyword {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote! {
            oxc_ast::ast::TSStringKeyword {
                span: oxc_span::Span::default(),
            }
        }
    }
}

impl ToCode for TSVoidKeyword {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote! {
            oxc_ast::ast::TSVoidKeyword {
                span: oxc_span::Span::default(),
            }
        }
    }
}

impl ToCode for TSNullKeyword {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote! {
            oxc_ast::ast::TSNullKeyword {
                span: oxc_span::Span::default(),
            }
        }
    }
}

impl ToCode for TSUndefinedKeyword {
    fn to_code(&self, _: &Ctx) -> syn::Expr {
        parse_quote! {
            oxc_ast::ast::TSUndefinedKeyword {
                span: oxc_span::Span::default(),
            }
        }
    }
}

impl ToCode for TSTypeReference<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let type_name = self.type_name.to_code(cx);
        let type_parameters = self.type_parameters.to_code(cx);
        parse_quote! {
            oxc_ast::ast::TSTypeReference {
                span: oxc_span::Span::default(),
                type_name: #type_name,
                type_parameters: #type_parameters,
            }
        }
    }
}

impl ToCode for TSTypeName<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match self {
            TSTypeName::IdentifierReference(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSTypeName::IdentifierReference(#inner))
            }
            TSTypeName::QualifiedName(e) => {
                let inner = e.to_code(cx);
                parse_quote!(oxc_ast::ast::TSTypeName::QualifiedName(#inner))
            }
        }
    }
}

impl ToCode for TSQualifiedName<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let left = self.left.to_code(cx);
        let right = self.right.to_code(cx);
        parse_quote! {
            oxc_ast::ast::TSQualifiedName {
                span: oxc_span::Span::default(),
                left: #left,
                right: #right,
            }
        }
    }
}

impl ToCode for TSTypeAnnotation<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let type_annotation = self.type_annotation.to_code(cx);
        parse_quote! {
            oxc_ast::ast::TSTypeAnnotation {
                span: oxc_span::Span::default(),
                type_annotation: #type_annotation,
            }
        }
    }
}

impl ToCode for Program<'_> {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let body = self.body.to_code(cx);
        parse_quote! {
            oxc_ast::ast::Program {
                span: oxc_span::Span::default(),
                source_type: oxc_span::SourceType::default(),
                source_text: "",
                comments: allocator.alloc_vec(&[]),
                directives: allocator.alloc_vec(&[]),
                body: #body,
                hashbang: None,
                scope_id: std::cell::Cell::new(None),
            }
        }
    }
}
