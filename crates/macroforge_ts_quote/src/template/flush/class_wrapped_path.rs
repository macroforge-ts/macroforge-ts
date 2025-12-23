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
