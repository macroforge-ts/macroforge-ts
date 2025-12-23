use crate::template::{
    Segment, PlaceholderUse, QuoteTsResult,
    build_template_and_bindings, collect_ident_name_ids, collect_block_compilations,
    ident_name_fix_block, generate_type_placeholder_fix, parse_ts_module_with_source,
    quote_ts, template_error,
};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, format_ident};
use std::collections::HashMap;
use swc_core::common::{SourceMapper, Spanned};
use swc_core::ecma::ast::{ModuleDecl, ModuleItem, Stmt};

/// Emits a run of statement segments as parsed SWC statements, attaching comments.
pub fn flush_stmt_run(
    run: &[&Segment],
    context_map: &HashMap<usize, PlaceholderUse>,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    let template_result = build_template_and_bindings(run.iter().copied(), context_map)?;
    if template_result.template.trim().is_empty() {
        return Ok(TokenStream2::new());
    }

    let ident_name_ids = collect_ident_name_ids(run.iter().copied(), context_map);
    let ident_name_fix = ident_name_fix_block(
        &proc_macro2::Ident::new("__mf_stmt", Span::call_site()),
        &ident_name_ids,
    );

    // Generate type placeholder fix for replacing __MfTypeN with actual types
    let type_fix = generate_type_placeholder_fix(&template_result.type_placeholders);

    // Collect BraceBlocks that need block substitution
    let block_compilations =
        collect_block_compilations(run, context_map, comments_ident, pending_ident, pos_ident)?;

    let mut output = TokenStream2::new();

    // If we have type placeholders, use runtime parsing with full TypeScript support.
    // This is needed because SWC's quote! macro doesn't support $placeholder in type positions.
    if !template_result.type_placeholders.is_empty() {
        let template_str = syn::LitStr::new(&template_result.template, Span::call_site());

        // Generate binding initializations.
        // We use type-specific conversion to handle various input types:
        // - For Expr bindings: use `to_ts_expr()` which handles Box<Expr>, Ident, String, etc.
        // - For Ident bindings: use `.into()` which handles String -> Ident via SWC's From impls
        // This prevents type mismatch errors when the same placeholder appears in multiple contexts,
        // and correctly handles Box<Expr> which doesn't have a From impl for Expr.
        let mut binding_inits = TokenStream2::new();
        for binding in &template_result.bindings {
            let name = &binding.name;
            let ty = &binding.ty;
            let expr = &binding.expr;
            let ty_str = ty.to_string();
            if ty_str == "Expr" {
                binding_inits.extend(quote! {
                    let #name: #ty = macroforge_ts_syn::to_ts_expr(#expr);
                });
            } else {
                binding_inits.extend(quote! {
                    let #name: #ty = (#expr).into();
                });
            }
        }

        // Generate type placeholder initializations
        for tp in &template_result.type_placeholders {
            let field_name = format_ident!("__mf_type_{}", tp.id);
            let expr = &tp.expr;
            binding_inits.extend(quote! {
                let #field_name: String = (#expr).to_string();
            });
        }

        // Generate visitor struct fields
        let mut visitor_fields = Vec::new();
        let mut visitor_inits = Vec::new();

        // Ident substitution fields
        for binding in &template_result.bindings {
            let name = &binding.name;
            let field_name = format_ident!("binding_{}", name);
            let ty = &binding.ty;
            visitor_fields.push(quote! { #field_name: #ty });
            visitor_inits.push(quote! { #field_name: #name.clone() });
        }

        // Type substitution fields
        for tp in &template_result.type_placeholders {
            let field_name = format_ident!("__mf_type_{}", tp.id);
            visitor_fields.push(quote! { #field_name: String });
            visitor_inits.push(quote! { #field_name: #field_name.clone() });
        }

        // Generate ident match arms - only for Ident-typed bindings (not Expr)
        // Template uses $__mf_hole_N format, which in TypeScript parses as ident with sym="$__mf_hole_N"
        let ident_arms: Vec<_> = template_result
            .bindings
            .iter()
            .filter(|binding| binding.ty.to_string() == "Ident")
            .map(|binding| {
                let name = &binding.name;
                // Include $ prefix since template uses $__mf_hole_N format
                let placeholder = format!("${}", name);
                let field_name = format_ident!("binding_{}", name);
                quote! {
                    #placeholder => {
                        ident.sym = self.#field_name.sym.clone();
                    }
                }
            })
            .collect();

        // Generate expr match arms - for Expr-typed bindings
        // When the placeholder appears as an expression (e.g., function name), replace the entire expr
        let expr_arms: Vec<_> = template_result
            .bindings
            .iter()
            .filter(|binding| binding.ty.to_string() == "Expr")
            .map(|binding| {
                let name = &binding.name;
                // Include $ prefix since template uses $__mf_hole_N format
                let placeholder = format!("${}", name);
                let field_name = format_ident!("binding_{}", name);
                quote! {
                    #placeholder => Some(self.#field_name.clone()),
                }
            })
            .collect();

        // Use swc_core directly - macroforge_ts has it as a direct dependency
        // just like macroforge_ts_syn's macros (ident!, fn_expr!, etc.) do
        let swc_core_path = quote!(swc_core);

        // Generate type match arms
        // Uses types from `use #swc_core_path::ecma::ast::*;` and common imports
        let type_arms: Vec<_> = template_result
            .type_placeholders
            .iter()
            .map(|tp| {
                let marker = format!("__MfTypeMarker{}", tp.id);
                let field_name = format_ident!("__mf_type_{}", tp.id);
                let swc_path = &swc_core_path;
                quote! {
                    #marker => {
                        *ty = TsType::TsTypeRef(TsTypeRef {
                            span: #swc_path::common::DUMMY_SP,
                            type_name: TsEntityName::Ident(Ident::new(
                                self.#field_name.clone().into(),
                                #swc_path::common::DUMMY_SP,
                                Default::default(),
                            )),
                            type_params: None,
                        });
                    }
                }
            })
            .collect();

        output.extend(quote! {{
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
                .expect("Failed to parse TypeScript template");

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
        }});
        return Ok(output);
    }

    // Standard path: no type placeholders, use quote! for better performance
    // Try parsing as a module first
    let parse_result = parse_ts_module_with_source(&template_result.template);

    // If module parsing fails, try wrapping in a class (for class body members like constructor)
    let (module, cm, is_class_wrapped) = match parse_result {
        Ok((module, cm)) => (module, cm, false),
        Err(_) => {
            let wrapped_source = format!("class __MfWrapper {{ {} }}", &template_result.template);
            let (module, cm) = parse_ts_module_with_source(&wrapped_source)?;
            (module, cm, true)
        }
    };

    // For class-wrapped content, we need to extract the class body members
    // and process each member as a class member snippet
    if is_class_wrapped {
        use swc_core::ecma::ast::{Decl, ModuleDecl};

        // Find the class declaration
        for item in module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export)) = &item {
                if let Decl::Class(class_decl) = &export.decl {
                    for member in &class_decl.class.body {
                        let snippet = cm.span_to_snippet(member.span()).map_err(|e| {
                            syn::Error::new(
                                Span::call_site(),
                                format!("TypeScript span error: {e:?}"),
                            )
                        })?;
                        let snippet = snippet.trim();
                        if snippet.is_empty() {
                            continue;
                        }
                        // For class members, we wrap in a minimal class to parse as a statement
                        let wrapped_snippet = format!("class __Temp {{ {} }}", snippet);
                        let quote_ts =
                            quote_ts(&wrapped_snippet, quote!(Stmt), &template_result.bindings);
                        let QuoteTsResult { bindings, expr } = quote_ts;

                        // Extract the class body from the generated AST
                        output.extend(quote! {{
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
                        }});
                    }
                    break;
                }
            } else if let ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) = &item {
                for member in &class_decl.class.body {
                    let snippet = cm.span_to_snippet(member.span()).map_err(|e| {
                        syn::Error::new(Span::call_site(), format!("TypeScript span error: {e:?}"))
                    })?;
                    let snippet = snippet.trim();
                    if snippet.is_empty() {
                        continue;
                    }
                    let wrapped_snippet = format!("class __Temp {{ {} }}", snippet);
                    let quote_ts =
                        quote_ts(&wrapped_snippet, quote!(Stmt), &template_result.bindings);
                    let QuoteTsResult { bindings, expr } = quote_ts;

                    output.extend(quote! {{
                        #bindings
                        let __mf_class_stmt = #expr;
                        if let swc_core::ecma::ast::Stmt::Decl(swc_core::ecma::ast::Decl::Class(class_decl)) = __mf_class_stmt {
                            for __mf_member in class_decl.class.body {
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

                                    __emitter.emit_class_decl(&__temp_class).expect("Failed to emit class");
                                }
                                let __member_source = String::from_utf8(__buf).expect("UTF-8");
                                let __member_only = __member_source
                                    .strip_prefix("class __Temp {")
                                    .and_then(|s| s.strip_suffix("}"))
                                    .map(|s| s.trim())
                                    .unwrap_or(&__member_source);
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
                    }});
                }
                break;
            }
        }

        return Ok(output);
    }

    for item in module.body {
        match item {
            ModuleItem::Stmt(stmt) => {
                let snippet = cm.span_to_snippet(stmt.span()).map_err(|e| {
                    syn::Error::new(Span::call_site(), format!("TypeScript span error: {e:?}"))
                })?;
                let snippet = snippet.trim();
                if snippet.is_empty() {
                    continue;
                }
                let quote_ts = quote_ts(snippet, quote!(Stmt), &template_result.bindings);
                let QuoteTsResult { bindings, expr } = quote_ts;

                // Generate block replacement code if we have blocks to replace
                let block_replacement = if block_compilations.is_empty() {
                    TokenStream2::new()
                } else {
                    let mut block_replacements = TokenStream2::new();
                    for (block_id, block_code) in &block_compilations {
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
                };

                output.extend(quote! {{
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
                }});
            }
            ModuleItem::ModuleDecl(decl) => {
                // Handle export declarations (export function, export const, etc.)
                // Since ts_template outputs Vec<Stmt>, we convert exports to their inner declarations
                match &decl {
                    ModuleDecl::ExportDecl(_) => {
                        // Get the FULL snippet including "export" keyword
                        // We quote as ModuleItem (which supports TypeScript) then extract the Decl
                        let snippet = cm.span_to_snippet(decl.span()).map_err(|e| {
                            syn::Error::new(
                                Span::call_site(),
                                format!("TypeScript span error: {e:?}"),
                            )
                        })?;
                        let snippet = snippet.trim();
                        if snippet.is_empty() {
                            continue;
                        }

                        // Quote as ModuleItem - this supports full TypeScript syntax including
                        // type annotations in function parameters (unlike quote!(... as Stmt))
                        let quote_ts =
                            quote_ts(snippet, quote!(ModuleItem), &template_result.bindings);
                        let QuoteTsResult {
                            bindings: quote_bindings,
                            expr,
                        } = quote_ts;

                        // Generate block replacement code if we have blocks to replace
                        let block_replacement = if block_compilations.is_empty() {
                            TokenStream2::new()
                        } else {
                            let mut block_replacements = TokenStream2::new();
                            for (block_id, block_code) in &block_compilations {
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
                                                    return;
                                                }
                                            }

                                            block.visit_mut_children_with(self);
                                        }
                                    }

                                    let mut __mf_block_replacer = __MfBlockReplacer {
                                        blocks: [#block_replacements].into_iter().collect(),
                                    };
                                    __mf_stmt.visit_mut_with(&mut __mf_block_replacer);
                                }
                            }
                        };

                        output.extend(quote! {{
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
                        }});
                    }
                    ModuleDecl::ExportDefaultDecl(_) | ModuleDecl::ExportDefaultExpr(_) => {
                        return Err(template_error(
                            Span::call_site(),
                            "Export default declarations are not supported in ts_template. Use export without default.",
                            None,
                        ));
                    }
                    _ => {
                        return Err(template_error(
                            Span::call_site(),
                            "Import declarations are not supported in ts_template",
                            None,
                        ));
                    }
                }
            }
        }
    }

    Ok(output)
}
