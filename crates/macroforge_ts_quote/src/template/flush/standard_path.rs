//! Standard code path - handles normal module statements and exports.
//!
//! This is the main path for templates without type placeholders that parse as valid modules.

use crate::template::{quote_ts, template_error, QuoteTsResult, TemplateAndBindings};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use swc_core::common::sync::Lrc;
use swc_core::common::{SourceMap, SourceMapper, Spanned};
use swc_core::ecma::ast::{ModuleDecl, ModuleItem, Stmt};

/// Context for standard code generation, grouping related parameters.
pub struct StandardCodeContext<'a> {
    pub cm: &'a Lrc<SourceMap>,
    pub template_result: &'a TemplateAndBindings,
    pub ident_name_fix: &'a TokenStream2,
    pub type_fix: &'a TokenStream2,
    pub block_compilations: &'a [(usize, TokenStream2)],
    pub out_ident: &'a proc_macro2::Ident,
    pub comments_ident: &'a proc_macro2::Ident,
    pub pending_ident: &'a proc_macro2::Ident,
    pub pos_ident: &'a proc_macro2::Ident,
}

/// Generates code for normal module statements.
///
/// This is the standard path when the template parses as a valid module
/// and has no type placeholders.
pub fn generate_standard_code(
    module: &swc_core::ecma::ast::Module,
    ctx: &StandardCodeContext,
) -> syn::Result<TokenStream2> {
    let mut output = TokenStream2::new();

    for item in &module.body {
        match item {
            ModuleItem::Stmt(stmt) => {
                let code = generate_stmt_code(stmt, ctx)?;
                output.extend(code);
            }
            ModuleItem::ModuleDecl(decl) => {
                let code = generate_module_decl_code(decl, ctx)?;
                output.extend(code);
            }
        }
    }

    Ok(output)
}

/// Generates code for a single statement.
fn generate_stmt_code(stmt: &Stmt, ctx: &StandardCodeContext) -> syn::Result<TokenStream2> {
    let snippet = ctx.cm.span_to_snippet(stmt.span()).map_err(|e| {
        syn::Error::new(Span::call_site(), format!("TypeScript span error: {e:?}"))
    })?;
    let snippet = snippet.trim();
    if snippet.is_empty() {
        return Ok(TokenStream2::new());
    }

    let quote_ts = quote_ts(snippet, quote!(Stmt), &ctx.template_result.bindings);
    let QuoteTsResult { bindings, expr } = quote_ts;

    let block_replacement = generate_block_replacement_code(ctx.block_compilations);
    let ident_name_fix = ctx.ident_name_fix;
    let type_fix = ctx.type_fix;
    let pos_ident = ctx.pos_ident;
    let pending_ident = ctx.pending_ident;
    let comments_ident = ctx.comments_ident;
    let out_ident = ctx.out_ident;

    Ok(quote! {{
        #bindings
        let mut __mf_stmt = #expr;
        #ident_name_fix
        #type_fix
        #block_replacement
        let __mf_pos = swc_core::common::BytePos(#pos_ident);
        #pos_ident += 1;
        {
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};
            struct __MfSpanFix {
                span: swc_core::common::Span,
            }
            impl VisitMut for __MfSpanFix {
                fn visit_mut_span(&mut self, span: &mut swc_core::common::Span) {
                    *span = self.span;
                }
            }
            let mut __mf_span_fix = __MfSpanFix {
                span: swc_core::common::Span::new(__mf_pos, __mf_pos),
            };
            __mf_stmt.visit_mut_with(&mut __mf_span_fix);
        }
        if !#pending_ident.is_empty() {
            use swc_core::common::comments::Comments;
            for __mf_comment in #pending_ident.drain(..) {
                #comments_ident.add_leading(__mf_pos, __mf_comment);
            }
        }
        #out_ident.push(__mf_stmt);
    }})
}

/// Generates code for a module declaration (exports).
fn generate_module_decl_code(
    decl: &ModuleDecl,
    ctx: &StandardCodeContext,
) -> syn::Result<TokenStream2> {
    match decl {
        ModuleDecl::ExportDecl(_) => generate_export_decl_code(decl, ctx),
        ModuleDecl::ExportDefaultDecl(_) | ModuleDecl::ExportDefaultExpr(_) => {
            Err(template_error(
                Span::call_site(),
                "Export default declarations are not supported in ts_template. Use export without default.",
                None,
            ))
        }
        _ => Err(template_error(
            Span::call_site(),
            "Import declarations are not supported in ts_template",
            None,
        )),
    }
}

/// Generates code for export declarations.
fn generate_export_decl_code(
    decl: &ModuleDecl,
    ctx: &StandardCodeContext,
) -> syn::Result<TokenStream2> {
    let snippet = ctx.cm.span_to_snippet(decl.span()).map_err(|e| {
        syn::Error::new(Span::call_site(), format!("TypeScript span error: {e:?}"))
    })?;
    let snippet = snippet.trim();
    if snippet.is_empty() {
        return Ok(TokenStream2::new());
    }

    // Quote as ModuleItem - this supports full TypeScript syntax including
    // type annotations in function parameters (unlike quote!(... as Stmt))
    let quote_ts_result = quote_ts(snippet, quote!(ModuleItem), &ctx.template_result.bindings);
    let QuoteTsResult {
        bindings: quote_bindings,
        expr,
    } = quote_ts_result;

    let block_replacement = generate_block_replacement_code(ctx.block_compilations);
    let ident_name_fix = ctx.ident_name_fix;
    let type_fix = ctx.type_fix;
    let pos_ident = ctx.pos_ident;
    let pending_ident = ctx.pending_ident;
    let comments_ident = ctx.comments_ident;
    let out_ident = ctx.out_ident;

    Ok(quote! {{
        #quote_bindings
        // Quote as ModuleItem, then extract the inner Decl and convert to Stmt
        let __mf_module_item = #expr;
        let mut __mf_stmt = match __mf_module_item {
            swc_core::ecma::ast::ModuleItem::ModuleDecl(
                swc_core::ecma::ast::ModuleDecl::ExportDecl(export)
            ) => swc_core::ecma::ast::Stmt::Decl(export.decl),
            swc_core::ecma::ast::ModuleItem::Stmt(s) => s,
            _ => panic!("unexpected module item type in ts_template"),
        };
        #block_replacement
        #ident_name_fix
        #type_fix
        let __mf_pos = swc_core::common::BytePos(#pos_ident);
        #pos_ident += 1;
        {
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};
            struct __MfSpanFix {
                span: swc_core::common::Span,
            }
            impl VisitMut for __MfSpanFix {
                fn visit_mut_span(&mut self, span: &mut swc_core::common::Span) {
                    *span = self.span;
                }
            }
            let mut __mf_span_fix = __MfSpanFix {
                span: swc_core::common::Span::new(__mf_pos, __mf_pos),
            };
            __mf_stmt.visit_mut_with(&mut __mf_span_fix);
        }
        if !#pending_ident.is_empty() {
            use swc_core::common::comments::Comments;
            for __mf_comment in #pending_ident.drain(..) {
                #comments_ident.add_leading(__mf_pos, __mf_comment);
            }
        }
        #out_ident.push(__mf_stmt);
    }})
}

/// Generates block replacement code if there are blocks to replace.
fn generate_block_replacement_code(block_compilations: &[(usize, TokenStream2)]) -> TokenStream2 {
    if block_compilations.is_empty() {
        return TokenStream2::new();
    }

    let mut block_replacements = TokenStream2::new();
    for (block_id, block_code) in block_compilations {
        let marker = format!("__mf_block_{}", block_id);
        block_replacements.extend(quote! {
            (#marker, {
                let mut __mf_block_stmts: Vec<swc_core::ecma::ast::Stmt> = Vec::new();
                #block_code
                __mf_block_stmts
            }),
        });
    }

    quote! {
        {
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};
            use swc_core::ecma::ast::{Stmt, Expr, ExprStmt, Ident};

            struct __MfBlockReplacer {
                blocks: std::collections::HashMap<String, Vec<Stmt>>,
            }

            impl VisitMut for __MfBlockReplacer {
                fn visit_mut_block_stmt(&mut self, block: &mut swc_core::ecma::ast::BlockStmt) {
                    // First, check if this block contains a marker statement
                    let marker_id = block.stmts.iter().find_map(|stmt| {
                        if let Stmt::Expr(ExprStmt { expr, .. }) = stmt {
                            if let Expr::Ident(ident) = &**expr {
                                let name = ident.sym.as_ref();
                                if name.starts_with("__mf_block_") {
                                    return Some(name.to_string());
                                }
                            }
                        }
                        None
                    });

                    if let Some(marker) = marker_id {
                        if let Some(compiled_stmts) = self.blocks.remove(&marker) {
                            block.stmts = compiled_stmts;
                            return; // Don't recurse into replaced block
                        }
                    }

                    // Recurse into children
                    block.visit_mut_children_with(self);
                }
            }

            let mut __mf_block_replacer = __MfBlockReplacer {
                blocks: [#block_replacements].into_iter().collect(),
            };
            __mf_stmt.visit_mut_with(&mut __mf_block_replacer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{parse_ts_module_with_source, BindingSpec, TemplateAndBindings};
    use quote::quote;

    /// Groups test identifiers to reduce function argument count
    struct TestIdents {
        out: proc_macro2::Ident,
        comments: proc_macro2::Ident,
        pending: proc_macro2::Ident,
        pos: proc_macro2::Ident,
    }

    impl TestIdents {
        fn new() -> Self {
            Self {
                out: proc_macro2::Ident::new("__mf_out", Span::call_site()),
                comments: proc_macro2::Ident::new("__mf_comments", Span::call_site()),
                pending: proc_macro2::Ident::new("__mf_pending", Span::call_site()),
                pos: proc_macro2::Ident::new("__mf_pos", Span::call_site()),
            }
        }
    }

    fn create_test_ident(name: &str) -> proc_macro2::Ident {
        proc_macro2::Ident::new(name, Span::call_site())
    }

    fn create_test_context<'a>(
        cm: &'a Lrc<SourceMap>,
        template_result: &'a TemplateAndBindings,
        ident_name_fix: &'a TokenStream2,
        type_fix: &'a TokenStream2,
        block_compilations: &'a [(usize, TokenStream2)],
        idents: &'a TestIdents,
    ) -> StandardCodeContext<'a> {
        StandardCodeContext {
            cm,
            template_result,
            ident_name_fix,
            type_fix,
            block_compilations,
            out_ident: &idents.out,
            comments_ident: &idents.comments,
            pending_ident: &idents.pending,
            pos_ident: &idents.pos,
        }
    }

    #[test]
    fn test_generate_standard_code_single_statement() {
        let source = "const x = 1;";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = TokenStream2::new();
        let type_fix = TokenStream2::new();
        let block_compilations = Vec::new();
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok(), "Should successfully generate code for single statement");

        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(code_str.contains("__mf_stmt"), "Should create statement variable");
        assert!(code_str.contains("__mf_out"), "Should reference output");
    }

    #[test]
    fn test_generate_standard_code_multiple_statements() {
        let source = "const x = 1; const y = 2; const z = 3;";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = TokenStream2::new();
        let type_fix = TokenStream2::new();
        let block_compilations = Vec::new();
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok(), "Should handle multiple statements");

        let code = result.unwrap();
        let code_str = code.to_string();
        // Should have code for each statement
        assert!(code_str.matches("__mf_stmt").count() >= 3, "Should process all statements");
    }

    #[test]
    fn test_generate_standard_code_export_declaration() {
        let source = "export const x = 1;";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = TokenStream2::new();
        let type_fix = TokenStream2::new();
        let block_compilations = Vec::new();
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok(), "Should handle export declarations");

        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(code_str.contains("ModuleItem"), "Should handle module items");
        assert!(code_str.contains("ExportDecl"), "Should handle exports");
    }

    #[test]
    fn test_generate_standard_code_with_bindings() {
        let source = "const x = 1;";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: vec![BindingSpec {
                name: create_test_ident("__mf_b_0"),
                ty: quote! { Expr },
                expr: quote! { my_expr },
            }],
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = TokenStream2::new();
        let type_fix = TokenStream2::new();
        let block_compilations = Vec::new();
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok(), "Should handle bindings");
    }

    #[test]
    fn test_generate_standard_code_with_ident_name_fix() {
        let source = "const x = 1;";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = quote! { println!("fixing ident names"); };
        let type_fix = TokenStream2::new();
        let block_compilations = Vec::new();
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok());

        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(code_str.contains("fixing ident names"), "Should include ident_name_fix");
    }

    #[test]
    fn test_generate_standard_code_with_block_compilations() {
        let source = "const x = 1;";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = TokenStream2::new();
        let type_fix = TokenStream2::new();
        let block_compilations = vec![
            (0, quote! { println!("block 0"); }),
            (1, quote! { println!("block 1"); }),
        ];
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok());

        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(code_str.contains("__MfBlockReplacer"), "Should include block replacer");
        assert!(code_str.contains("__mf_block_0"), "Should reference first block");
        assert!(code_str.contains("__mf_block_1"), "Should reference second block");
    }

    #[test]
    fn test_generate_standard_code_span_fixing() {
        let source = "const x = 1;";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = TokenStream2::new();
        let type_fix = TokenStream2::new();
        let block_compilations = Vec::new();
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok());

        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(code_str.contains("__MfSpanFix"), "Should include span fix struct");
        assert!(code_str.contains("visit_mut_span"), "Should fix spans");
        assert!(code_str.contains("BytePos"), "Should use BytePos");
    }

    #[test]
    fn test_generate_standard_code_comment_handling() {
        let source = "const x = 1;";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = TokenStream2::new();
        let type_fix = TokenStream2::new();
        let block_compilations = Vec::new();
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok());

        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(code_str.contains(&idents.pending.to_string()), "Should check pending comments");
        assert!(code_str.contains(&idents.comments.to_string()), "Should add comments");
        assert!(code_str.contains("add_leading"), "Should add leading comments");
    }

    #[test]
    fn test_generate_block_replacement_code_empty() {
        let block_compilations: Vec<(usize, TokenStream2)> = Vec::new();
        let code = generate_block_replacement_code(&block_compilations);

        assert!(code.is_empty(), "Should return empty for no block compilations");
    }

    #[test]
    fn test_generate_block_replacement_code_single_block() {
        let block_compilations = vec![
            (0, quote! { println!("block"); }),
        ];
        let code = generate_block_replacement_code(&block_compilations);

        let code_str = code.to_string();
        assert!(code_str.contains("__MfBlockReplacer"), "Should create replacer");
        assert!(code_str.contains("__mf_block_0"), "Should reference block 0");
    }

    #[test]
    fn test_generate_block_replacement_code_multiple_blocks() {
        let block_compilations = vec![
            (0, quote! { println!("block 0"); }),
            (5, quote! { println!("block 5"); }),
            (10, quote! { println!("block 10"); }),
        ];
        let code = generate_block_replacement_code(&block_compilations);

        let code_str = code.to_string();
        assert!(code_str.contains("__mf_block_0"), "Should reference block 0");
        assert!(code_str.contains("__mf_block_5"), "Should reference block 5");
        assert!(code_str.contains("__mf_block_10"), "Should reference block 10");
    }

    #[test]
    fn test_generate_export_decl_code_function() {
        let source = "export function foo() { return 1; }";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = TokenStream2::new();
        let type_fix = TokenStream2::new();
        let block_compilations = Vec::new();
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok(), "Should handle export function");
    }

    #[test]
    fn test_generate_standard_code_mixed_statements_and_exports() {
        let source = "const x = 1; export const y = 2; const z = 3;";
        let (module, cm) = parse_ts_module_with_source(source).expect("Failed to parse");

        let template_result = TemplateAndBindings {
            template: source.to_string(),
            bindings: Vec::new(),
            type_placeholders: Vec::new(),
        };

        let ident_name_fix = TokenStream2::new();
        let type_fix = TokenStream2::new();
        let block_compilations = Vec::new();
        let idents = TestIdents::new();

        let ctx = create_test_context(
            &cm,
            &template_result,
            &ident_name_fix,
            &type_fix,
            &block_compilations,
            &idents,
        );

        let result = generate_standard_code(&module, &ctx);
        assert!(result.is_ok(), "Should handle mix of statements and exports");
    }
}
