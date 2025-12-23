//! Function wrapped code path - handles method body statements.
//!
//! When module parsing and class-wrapped parsing both fail, we try wrapping
//! in a function to parse method body statements like `this.x = y;`.

use crate::template::{quote_ts, QuoteTsResult, TemplateAndBindings};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use swc_core::common::sync::Lrc;
use swc_core::common::{SourceMap, SourceMapper, Spanned};
use swc_core::ecma::ast::{Decl, FnDecl, ModuleDecl, ModuleItem, Stmt};

/// Generates code for method body statements by wrapping content in a function.
///
/// This path is used when both module and class-wrapped parsing fail,
/// indicating the template contains method body statements like `this.x = y;`.
pub fn generate_function_wrapped_code(
    template_result: &TemplateAndBindings,
    cm: &Lrc<SourceMap>,
    module: &swc_core::ecma::ast::Module,
    out_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    let mut output = TokenStream2::new();

    // Find the function declaration and process its body
    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export)) = item {
            if let Decl::Fn(fn_decl) = &export.decl {
                process_function_body(fn_decl, cm, template_result, out_ident, &mut output)?;
                break;
            }
        } else if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) = item {
            process_function_body(fn_decl, cm, template_result, out_ident, &mut output)?;
            break;
        }
    }

    Ok(output)
}

/// Processes function body statements.
fn process_function_body(
    fn_decl: &FnDecl,
    cm: &Lrc<SourceMap>,
    template_result: &TemplateAndBindings,
    out_ident: &proc_macro2::Ident,
    output: &mut TokenStream2,
) -> syn::Result<()> {
    if let Some(body) = &fn_decl.function.body {
        for stmt in &body.stmts {
            let snippet = cm.span_to_snippet(stmt.span()).map_err(|e| {
                syn::Error::new(Span::call_site(), format!("TypeScript span error: {e:?}"))
            })?;
            let snippet = snippet.trim();
            if snippet.is_empty() {
                continue;
            }

            // Parse as a statement directly
            let quote_ts_result = quote_ts(snippet, quote!(Stmt), &template_result.bindings);
            let QuoteTsResult { bindings, expr } = quote_ts_result;

            output.extend(quote! {{
                #bindings
                let __mf_stmt = #expr;
                #out_ident.push(swc_core::ecma::ast::ModuleItem::Stmt(__mf_stmt));
            }});
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{parse_ts_module_with_source, TemplateAndBindings};

    fn create_test_ident(name: &str) -> proc_macro2::Ident {
        proc_macro2::Ident::new(name, Span::call_site())
    }

    #[test]
    fn test_generate_function_wrapped_code_this_assignment() {
        let template_result = TemplateAndBindings {
            template: "this.x = 1;".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!(
            "function __MfWrapper() {{ {} }}",
            &template_result.template
        );
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_function_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(
            result.is_ok(),
            "Should successfully generate code for this.x = 1"
        );
        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(
            code_str.contains("__mf_stmt"),
            "Should create statement variable"
        );
        assert!(
            code_str.contains("__mf_out"),
            "Should push to output"
        );
    }

    #[test]
    fn test_generate_function_wrapped_code_multiple_statements() {
        let template_result = TemplateAndBindings {
            template: "this.x = 1; this.y = 2; return;".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!(
            "function __MfWrapper() {{ {} }}",
            &template_result.template
        );
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_function_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok(), "Should handle multiple statements");
        let code = result.unwrap();
        let code_str = code.to_string();
        // Should have multiple pushes
        let push_count = code_str.matches("__mf_out . push").count();
        assert!(
            push_count >= 2,
            "Should push multiple statements, got {}",
            push_count
        );
    }

    #[test]
    fn test_generate_function_wrapped_code_with_bindings() {
        use crate::template::BindingSpec;
        use quote::quote;

        let template_result = TemplateAndBindings {
            template: "this.x = $__mf_hole_0;".to_string(),
            bindings: vec![BindingSpec {
                name: create_test_ident("__mf_b_0"),
                ty: quote! { Expr },
                expr: quote! { my_value },
            }],
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!(
            "function __MfWrapper() {{ {} }}",
            &template_result.template
        );
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_function_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok(), "Should handle bindings");
    }

    #[test]
    fn test_generate_function_wrapped_code_uses_out_ident() {
        let template_result = TemplateAndBindings {
            template: "return 42;".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = format!(
            "function __MfWrapper() {{ {} }}",
            &template_result.template
        );
        let (module, cm) = parse_ts_module_with_source(&wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__custom_out");

        let result = generate_function_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok());
        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(
            code_str.contains("__custom_out"),
            "Should use provided out_ident"
        );
    }

    #[test]
    fn test_generate_function_wrapped_code_empty_body() {
        let template_result = TemplateAndBindings {
            template: "".to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let wrapped_source = "function __MfWrapper() {}";
        let (module, cm) = parse_ts_module_with_source(wrapped_source).expect("Failed to parse");
        let out_ident = create_test_ident("__mf_out");

        let result = generate_function_wrapped_code(&template_result, &cm, &module, &out_ident);

        assert!(result.is_ok(), "Should handle empty body");
        let code = result.unwrap();
        assert!(code.is_empty(), "Should produce no output for empty body");
    }
}
