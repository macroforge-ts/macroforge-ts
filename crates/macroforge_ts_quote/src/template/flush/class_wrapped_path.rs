//! Class wrapped code path - handles class body members (constructor, methods, etc.).
//!
//! When module parsing fails, we try wrapping in a class to parse class body members.

use crate::template::{quote_ts, QuoteTsResult, TemplateAndBindings};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use swc_core::common::sync::Lrc;
use swc_core::common::{SourceMap, SourceMapper, Spanned};
use swc_core::ecma::ast::{ClassMember, Decl, ModuleDecl, ModuleItem, Stmt};

/// Generates code for class body members by wrapping content in a class.
///
/// This path is used when module parsing fails, indicating the template
/// contains class body members like constructors or methods.
pub fn generate_class_wrapped_code(
    template_result: &TemplateAndBindings,
    cm: &Lrc<SourceMap>,
    module: &swc_core::ecma::ast::Module,
    out_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    let mut output = TokenStream2::new();

    // Find the class declaration and process its members
    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export)) = item {
            if let Decl::Class(class_decl) = &export.decl {
                process_class_members(
                    &class_decl.class.body,
                    cm,
                    template_result,
                    out_ident,
                    &mut output,
                )?;
                break;
            }
        } else if let ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) = item {
            process_class_members(
                &class_decl.class.body,
                cm,
                template_result,
                out_ident,
                &mut output,
            )?;
            break;
        }
    }

    Ok(output)
}

/// Processes class members, wrapping each in a temporary class for parsing.
fn process_class_members(
    members: &[ClassMember],
    cm: &Lrc<SourceMap>,
    template_result: &TemplateAndBindings,
    out_ident: &proc_macro2::Ident,
    output: &mut TokenStream2,
) -> syn::Result<()> {
    for member in members {
        let snippet = cm.span_to_snippet(member.span()).map_err(|e| {
            syn::Error::new(Span::call_site(), format!("TypeScript span error: {e:?}"))
        })?;
        let snippet = snippet.trim();
        if snippet.is_empty() {
            continue;
        }

        // Wrap in a minimal class to parse as a statement
        let wrapped_snippet = format!("class __Temp {{ {} }}", snippet);
        let quote_ts_result = quote_ts(&wrapped_snippet, quote!(Stmt), &template_result.bindings);
        let QuoteTsResult { bindings, expr } = quote_ts_result;

        output.extend(generate_member_extraction_code(bindings, expr, out_ident));
    }

    Ok(())
}

/// Generates code to extract a class member from a wrapper class.
fn generate_member_extraction_code(
    bindings: TokenStream2,
    expr: TokenStream2,
    out_ident: &proc_macro2::Ident,
) -> TokenStream2 {
    quote! {{
        #bindings
        let __mf_class_stmt = #expr;
        // Extract the class member from the wrapper class
        if let swc_core::ecma::ast::Stmt::Decl(swc_core::ecma::ast::Decl::Class(class_decl)) = __mf_class_stmt {
            for __mf_member in class_decl.class.body {
                // Convert ClassMember to a pseudo-statement for body injection
                // The body! macro outputs raw source which includes class members
                // We need to emit the member as source text
                let __cm = swc_core::common::sync::Lrc::new(swc_core::common::SourceMap::default());
                let mut __buf = Vec::new();
                {
                    use swc_core::ecma::codegen::{text_writer::JsWriter, Emitter};
                    let mut __emitter = Emitter {
                        cfg: swc_core::ecma::codegen::Config::default(),
                        cm: __cm.clone(),
                        comments: None,
                        wr: JsWriter::new(__cm.clone(), "\n", &mut __buf, None),
                    };
                    // Wrap in a temp class to emit
                    let __temp_class = swc_core::ecma::ast::ClassDecl {
                        ident: swc_core::ecma::ast::Ident::new(
                            "__Temp".into(),
                            swc_core::common::DUMMY_SP,
                            Default::default(),
                        ),
                        declare: false,
                        class: Box::new(swc_core::ecma::ast::Class {
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

                    __emitter
                        .emit_class_decl(&__temp_class)
                        .expect("Failed to emit class");
                }
                let __member_source = String::from_utf8(__buf).expect("UTF-8");
                // Extract just the member part (remove "class __Temp {" and "}")
                let __member_only = __member_source
                    .strip_prefix("class __Temp {")
                    .and_then(|s| s.strip_suffix("}"))
                    .map(|s| s.trim())
                    .unwrap_or(&__member_source);
                // Add as raw source - the body! macro handles this
                #out_ident.push(swc_core::ecma::ast::Stmt::Expr(swc_core::ecma::ast::ExprStmt {
                    span: swc_core::common::DUMMY_SP,
                    expr: Box::new(swc_core::ecma::ast::Expr::Ident(
                        swc_core::ecma::ast::Ident::new(
                            format!("/* @macroforge:raw */{}", __member_only).into(),
                            swc_core::common::DUMMY_SP,
                            Default::default(),
                        )
                    )),
                }));
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
        assert!(code_str.contains("__mf_class_stmt"), "Should create class statement");
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
    fn test_process_class_members_empty() {
        let members: Vec<swc_core::ecma::ast::ClassMember> = Vec::new();
        let template_result = TemplateAndBindings {
            template: String::new(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };
        let wrapped_source = "class __Test {}";
        let (_, cm) = parse_ts_module_with_source(wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");
        let mut output = TokenStream2::new();

        let result = process_class_members(&members, &cm, &template_result, &out_ident, &mut output);

        assert!(result.is_ok(), "Should handle empty members");
        assert!(output.is_empty(), "Should produce no output for empty members");
    }

    #[test]
    fn test_generate_member_extraction_code() {
        use quote::quote;

        let bindings = quote! { let x = 1; };
        let expr = quote! { my_stmt };
        let out_ident = create_test_ident("__mf_out");

        let code = generate_member_extraction_code(bindings, expr, &out_ident);

        let code_str = code.to_string();
        assert!(code_str.contains("__mf_class_stmt"), "Should create class statement variable");
        // The code uses swc_core paths, not bare "ClassMember" string
        assert!(code_str.contains("class_decl"), "Should handle class declaration");
        assert!(code_str.contains("__mf_out"), "Should push to output");
        // Check for raw marker format (may be @ macroforge or similar)
        assert!(code_str.contains("macroforge") || code_str.contains("raw"), "Should use raw marker for output");
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
