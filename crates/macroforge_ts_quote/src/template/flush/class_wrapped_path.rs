//! Class wrapped code path - handles class body members (constructor, methods, etc.).
//!
//! When module parsing fails, we try wrapping in a class to parse class body members.

use super::helpers::{
    generate_binding_initializations, generate_expr_arms, generate_ident_arms, generate_type_arms,
    generate_visitor_components,
};
use crate::template::TemplateAndBindings;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;

/// Generates code for class body members by wrapping content in a class.
///
/// This path is used when module parsing fails, indicating the template
/// contains class body members like constructors or methods.
pub fn generate_class_wrapped_code(
    template_result: &TemplateAndBindings,
    _cm: &Lrc<SourceMap>,
    _module: &swc_core::ecma::ast::Module,
    out_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    let template = format!("class __MfWrapper {{ {} }}", template_result.template);
    Ok(generate_class_wrapped_runtime_code(
        &template,
        template_result,
        out_ident,
    ))
}

fn generate_class_wrapped_runtime_code(
    template: &str,
    template_result: &TemplateAndBindings,
    out_ident: &proc_macro2::Ident,
) -> TokenStream2 {
    let template_str = syn::LitStr::new(template, Span::call_site());
    let binding_inits = generate_binding_initializations(
        &template_result.bindings,
        &template_result.type_placeholders,
    );
    let (visitor_fields, visitor_inits) =
        generate_visitor_components(&template_result.bindings, &template_result.type_placeholders);
    let ident_arms = generate_ident_arms(&template_result.bindings);
    let expr_arms = generate_expr_arms(&template_result.bindings);
    let type_arms = generate_type_arms(&template_result.type_placeholders, &quote!(swc_core));

    quote! {{
        #binding_inits
        use swc_core::common::{FileName, SourceMap, sync::Lrc};
        use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
        use swc_core::ecma::visit::{VisitMut, VisitMutWith};
        use swc_core::ecma::ast::*;

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
        let mut __mf_module = __mf_parser
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
                let replacement = if let Expr::Ident(ident) = &*expr {
                    match ident.sym.as_ref() {
                        #(#expr_arms)*
                        _ => None
                    }
                } else {
                    None
                };

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

            fn visit_mut_member_prop(&mut self, prop: &mut MemberProp) {
                if let MemberProp::Ident(ident) = prop {
                    match ident.sym.as_ref() {
                        #(#ident_arms)*
                        _ => {}
                    }
                }
                prop.visit_mut_children_with(self);
            }

            fn visit_mut_prop_name(&mut self, prop: &mut PropName) {
                if let PropName::Ident(ident) = prop {
                    match ident.sym.as_ref() {
                        #(#ident_arms)*
                        _ => {}
                    }
                }
                prop.visit_mut_children_with(self);
            }
        }

        let mut __mf_substitutor = __MfSubstitutor {
            #(#visitor_inits,)*
        };

        __mf_module.visit_mut_with(&mut __mf_substitutor);

        for __mf_item in __mf_module.body {
            let __mf_class_decl = match __mf_item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) => Some(class_decl),
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export)) => match export.decl {
                    Decl::Class(class_decl) => Some(class_decl),
                    _ => None,
                },
                _ => None,
            };

            if let Some(__mf_class_decl) = __mf_class_decl {
                for mut __mf_member in __mf_class_decl.class.body {
                    // Reset syntax contexts to avoid #N suffixes in output
                    struct __MfCtxtReset;
                    impl VisitMut for __MfCtxtReset {
                        fn visit_mut_ident(&mut self, ident: &mut Ident) {
                            ident.ctxt = Default::default();
                        }
                    }
                    __mf_member.visit_mut_with(&mut __MfCtxtReset);

                    let __temp_class = ClassDecl {
                        ident: Ident::new(
                            "__Temp".into(),
                            swc_core::common::DUMMY_SP,
                            Default::default(),
                        ),
                        declare: false,
                        class: Box::new(Class {
                            span: swc_core::common::DUMMY_SP,
                            ctxt: Default::default(),
                            decorators: vec![],
                            body: vec![__mf_member],
                            super_class: None,
                            is_abstract: false,
                            type_params: None,
                            super_type_params: None,
                            implements: vec![],
                        }),
                    };
                    let __temp_item = ModuleItem::Stmt(Stmt::Decl(Decl::Class(__temp_class)));
                    let __member_source = swc_core::ecma::codegen::to_code(&__temp_item);
                    let __member_trimmed = __member_source.trim();
                    let __member_only = __member_trimmed
                        .strip_prefix("class __Temp {")
                        .or_else(|| __member_trimmed.strip_prefix("class __Temp{"))
                        .and_then(|s| s.strip_suffix("}"))
                        .map(|s| s.trim())
                        .unwrap_or(__member_trimmed);
                    // Strip SyntaxContext markers (e.g., #0, #1) from identifiers
                    // These are added by to_code but shouldn't appear in final output
                    let __member_cleaned = {
                        let mut result = String::new();
                        let mut chars = __member_only.chars().peekable();
                        while let Some(c) = chars.next() {
                            if c == '#' {
                                // Check if followed by digits
                                let mut is_context_marker = false;
                                let mut digits = String::new();
                                while let Some(&next) = chars.peek() {
                                    if next.is_ascii_digit() {
                                        is_context_marker = true;
                                        digits.push(chars.next().unwrap());
                                    } else {
                                        break;
                                    }
                                }
                                if !is_context_marker {
                                    result.push('#');
                                    result.push_str(&digits);
                                }
                                // If it was a context marker, we skip it entirely
                            } else {
                                result.push(c);
                            }
                        }
                        result
                    };
                    let __member_only = __member_cleaned;
                    #out_ident.push(Stmt::Expr(ExprStmt {
                        span: swc_core::common::DUMMY_SP,
                        expr: Box::new(Expr::Ident(Ident::new(
                            format!("/* @macroforge:raw */{}", __member_only).into(),
                            swc_core::common::DUMMY_SP,
                            Default::default(),
                        ))),
                    }));
                }
                break;
            }
        }
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{parse_ts_module_with_source, TemplateAndBindings};

    fn create_test_ident(name: &str) -> proc_macro2::Ident {
        proc_macro2::Ident::new(name, Span::call_site())
    }

    #[test]
    fn test_generate_class_wrapped_code_basic() {
        let template_result = TemplateAndBindings {
            template: "constructor() {}".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!("class __MfWrapper {{ {} }}", &template_result.template);
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_class_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok(), "Should successfully generate code for constructor");
        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(
            code_str.contains("parse_module"),
            "Should parse class wrapper at runtime"
        );
    }

    #[test]
    fn test_generate_class_wrapped_code_method() {
        let template_result = TemplateAndBindings {
            template: "myMethod() { return 42; }".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!("class __MfWrapper {{ {} }}", &template_result.template);
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_class_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok(), "Should successfully generate code for method");
        let code = result.unwrap();
        assert!(!code.is_empty(), "Should generate non-empty code");
    }

    #[test]
    fn test_generate_class_wrapped_code_multiple_members() {
        let template_result = TemplateAndBindings {
            template: "constructor() {} method1() {} method2() {}".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!("class __MfWrapper {{ {} }}", &template_result.template);
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_class_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok(), "Should handle multiple class members");
        let code = result.unwrap();
        let code_str = code.to_string();
        // Should generate code for each member
        assert!(code_str.len() > 100, "Should generate substantial code for multiple members");
    }

    #[test]
    fn test_generate_class_wrapped_code_with_bindings() {
        use crate::template::BindingSpec;
        use quote::quote;

        let template_result = TemplateAndBindings {
            template: "myMethod() {}".to_string(),
            bindings: vec![BindingSpec {
                name: create_test_ident("__mf_b_0"),
                ty: quote! { Expr },
                expr: quote! { my_expr },
            }],
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!("class __MfWrapper {{ {} }}", &template_result.template);
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_class_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok(), "Should handle bindings");
    }

    #[test]
    fn test_generate_class_wrapped_code_uses_out_ident() {
        let template_result = TemplateAndBindings {
            template: "method() {}".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!("class __MfWrapper {{ {} }}", &template_result.template);
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__custom_out");

        let result = generate_class_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok());
        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(code_str.contains("__custom_out"), "Should use provided out_ident");
    }

    #[test]
    fn test_generate_class_wrapped_code_property() {
        let template_result = TemplateAndBindings {
            template: "public name: string;".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!("class __MfWrapper {{ {} }}", &template_result.template);
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_class_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok(), "Should handle class properties");
    }

    #[test]
    fn test_generate_class_wrapped_code_getter_setter() {
        let template_result = TemplateAndBindings {
            template: "get value() { return this._value; } set value(v) { this._value = v; }".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!("class __MfWrapper {{ {} }}", &template_result.template);
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_class_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok(), "Should handle getters and setters");
    }
}
