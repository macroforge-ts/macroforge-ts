//! Type placeholder code path - handles templates with type placeholders.
//!
//! When type placeholders are present, we use runtime parsing with full TypeScript support.
//! This is needed because SWC's quote! macro doesn't support $placeholder in type positions.

use super::helpers::{
    generate_binding_initializations, generate_expr_arms, generate_ident_arms, generate_type_arms,
    generate_visitor_components,
};
use crate::template::TemplateAndBindings;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

/// Generates code for templates with type placeholders.
///
/// This path uses runtime parsing and visitor-based substitution because
/// SWC's quote! macro doesn't support placeholders in type positions.
pub fn generate_type_placeholder_code(
    template_result: &TemplateAndBindings,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> TokenStream2 {
    let template_str = syn::LitStr::new(&template_result.template, Span::call_site());

    // Generate binding initializations
    let binding_inits = generate_binding_initializations(
        &template_result.bindings,
        &template_result.type_placeholders,
    );

    // Generate visitor struct components
    let (visitor_fields, visitor_inits) =
        generate_visitor_components(&template_result.bindings, &template_result.type_placeholders);

    // Use swc_core directly
    let swc_core_path = quote!(swc_core);

    // Generate match arms
    let ident_arms = generate_ident_arms(&template_result.bindings);
    let expr_arms = generate_expr_arms(&template_result.bindings);
    let type_arms = generate_type_arms(&template_result.type_placeholders, &swc_core_path);

    quote! {{
        #binding_inits
        use #swc_core_path::common::{FileName, SourceMap, sync::Lrc};
        use #swc_core_path::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
        use #swc_core_path::ecma::visit::{VisitMut, VisitMutWith};
        use #swc_core_path::ecma::ast::*;

        let __mf_cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
        let __mf_fm = __mf_cm.new_source_file(
            FileName::Custom("template.ts".into()).into(),
            #template_str.to_string(),
        );
        let __mf_syntax = Syntax::Typescript(TsSyntax {
            tsx: true,
            decorators: true,
            ..Default::default()
        });
        let __mf_lexer = Lexer::new(
            __mf_syntax,
            EsVersion::latest(),
            StringInput::from(&*__mf_fm),
            None,
        );
        let mut __mf_parser = Parser::new_from(__mf_lexer);
        let __mf_module = __mf_parser
            .parse_module()
            .unwrap_or_else(|e| panic!("Failed to parse TypeScript template: {:?}\n\nGenerated source:\n{}", e, #template_str));

        struct __MfSubstitutor {
            #(#visitor_fields,)*
        }

        impl VisitMut for __MfSubstitutor {
            fn visit_mut_ident(&mut self, ident: &mut Ident) {
                let name = ident.sym.as_ref();
                match name {
                    #(#ident_arms)*
                    _ => {}
                }
            }

            fn visit_mut_expr(&mut self, expr: &mut Expr) {
                // First check if this is an ident placeholder that should be replaced
                let replacement = if let Expr::Ident(ident) = &*expr {
                    match ident.sym.as_ref() {
                        #(#expr_arms)*
                        _ => None
                    }
                } else {
                    None
                };

                // Apply replacement if found, otherwise continue visiting children
                if let Some(new_expr) = replacement {
                    *expr = new_expr;
                } else {
                    expr.visit_mut_children_with(self);
                }
            }

            fn visit_mut_ts_type(&mut self, ty: &mut TsType) {
                if let TsType::TsTypeRef(type_ref) = ty {
                    if let TsEntityName::Ident(ident) = &type_ref.type_name {
                        match ident.sym.as_ref() {
                            #(#type_arms)*
                            _ => {}
                        }
                    }
                }
                ty.visit_mut_children_with(self);
            }
        }

        let mut __mf_substitutor = __MfSubstitutor {
            #(#visitor_inits,)*
        };

        for __mf_item in __mf_module.body {
            let mut __mf_stmt = match __mf_item {
                ModuleItem::Stmt(s) => s,
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export)) => Stmt::Decl(export.decl),
                _ => continue,
            };

            __mf_stmt.visit_mut_with(&mut __mf_substitutor);

            let __mf_pos = #swc_core_path::common::BytePos(#pos_ident);
            #pos_ident += 1;
            {
                struct __MfSpanFix {
                    span: #swc_core_path::common::Span,
                }
                impl VisitMut for __MfSpanFix {
                    fn visit_mut_span(&mut self, span: &mut #swc_core_path::common::Span) {
                        *span = self.span;
                    }
                }
                let mut __mf_span_fix = __MfSpanFix {
                    span: #swc_core_path::common::Span::new(__mf_pos, __mf_pos),
                };
                __mf_stmt.visit_mut_with(&mut __mf_span_fix);
            }

            if !#pending_ident.is_empty() {
                use #swc_core_path::common::comments::Comments;
                for __mf_comment in #pending_ident.drain(..) {
                    #comments_ident.add_leading(__mf_pos, __mf_comment);
                }
            }

            #out_ident.push(__mf_stmt);
        }
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{BindingSpec, TemplateAndBindings, TypePlaceholder};
    use quote::quote;

    fn create_test_ident(name: &str) -> proc_macro2::Ident {
        proc_macro2::Ident::new(name, Span::call_site())
    }

    #[test]
    fn test_generate_type_placeholder_code_empty_template() {
        let template_result = TemplateAndBindings {
            template: String::new(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let code = generate_type_placeholder_code(
            &template_result,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(!code.is_empty(), "Should generate code even for empty template");
        let code_str = code.to_string();
        assert!(code_str.contains("SourceMap"), "Should use SourceMap");
        assert!(code_str.contains("Parser"), "Should use Parser");
    }

    #[test]
    fn test_generate_type_placeholder_code_with_type_placeholder() {
        let template_result = TemplateAndBindings {
            template: "const x: __MF_TYPE_0 = value;".to_string(),
            bindings: Vec::new(),
            type_placeholders: vec![TypePlaceholder {
                id: 0,
                expr: quote! { string },
            }],
        };

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let code = generate_type_placeholder_code(
            &template_result,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        let code_str = code.to_string();
        assert!(code_str.contains("__MfSubstitutor"), "Should create substitutor struct");
        assert!(code_str.contains("visit_mut_ts_type"), "Should have type visitor");
        assert!(code_str.contains("__MF_TYPE_0"), "Should reference type placeholder");
    }

    #[test]
    fn test_generate_type_placeholder_code_with_bindings() {
        let template_result = TemplateAndBindings {
            template: "const x = __MF_EXPR_0;".to_string(),
            bindings: vec![BindingSpec {
                name: create_test_ident("__mf_b_0"),
                ty: quote! { Expr },
                expr: quote! { my_expr },
            }],
            type_placeholders: Vec::new(),
        };

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let code = generate_type_placeholder_code(
            &template_result,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        let code_str = code.to_string();
        assert!(code_str.contains("__mf_b_0"), "Should include binding name");
        assert!(code_str.contains("visit_mut_expr"), "Should have expr visitor");
    }

    #[test]
    fn test_generate_type_placeholder_code_uses_swc_core() {
        let template_result = TemplateAndBindings {
            template: "const x = 1;".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let code = generate_type_placeholder_code(
            &template_result,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        let code_str = code.to_string();
        assert!(code_str.contains("swc_core"), "Should use swc_core path");
        assert!(code_str.contains("VisitMut"), "Should use VisitMut trait");
        assert!(code_str.contains("TsSyntax"), "Should use TypeScript syntax");
    }

    #[test]
    fn test_generate_type_placeholder_code_handles_comments() {
        let template_result = TemplateAndBindings {
            template: "const x = 1;".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let code = generate_type_placeholder_code(
            &template_result,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        let code_str = code.to_string();
        assert!(code_str.contains(&pending_ident.to_string()), "Should reference pending_ident");
        assert!(code_str.contains(&comments_ident.to_string()), "Should reference comments_ident");
        assert!(code_str.contains("add_leading"), "Should add leading comments");
    }

    #[test]
    fn test_generate_type_placeholder_code_span_fixing() {
        let template_result = TemplateAndBindings {
            template: "const x = 1;".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let code = generate_type_placeholder_code(
            &template_result,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        let code_str = code.to_string();
        assert!(code_str.contains("__MfSpanFix"), "Should have span fix struct");
        assert!(code_str.contains("visit_mut_span"), "Should fix spans");
        assert!(code_str.contains("BytePos"), "Should use BytePos");
    }

    #[test]
    fn test_generate_type_placeholder_code_multiple_type_placeholders() {
        let template_result = TemplateAndBindings {
            template: "const x: __MF_TYPE_0 = value; const y: __MF_TYPE_1 = other;".to_string(),
            bindings: Vec::new(),
            type_placeholders: vec![
                TypePlaceholder {
                    id: 0,
                    expr: quote! { string },
                },
                TypePlaceholder {
                    id: 1,
                    expr: quote! { number },
                },
            ],
        };

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let code = generate_type_placeholder_code(
            &template_result,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        let code_str = code.to_string();
        assert!(code_str.contains("__MF_TYPE_0"), "Should handle first type placeholder");
        assert!(code_str.contains("__MF_TYPE_1"), "Should handle second type placeholder");
    }

    #[test]
    fn test_generate_type_placeholder_code_export_handling() {
        let template_result = TemplateAndBindings {
            template: "export const x = 1;".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let code = generate_type_placeholder_code(
            &template_result,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        let code_str = code.to_string();
        assert!(code_str.contains("ModuleDecl"), "Should handle module declarations");
        assert!(code_str.contains("ExportDecl"), "Should handle export declarations");
    }
}
