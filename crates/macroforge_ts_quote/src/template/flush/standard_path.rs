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
