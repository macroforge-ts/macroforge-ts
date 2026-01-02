use super::error::{GenError, GenErrorKind, GenResult};
use super::*;

impl Codegen {
    /// Generate code that builds a decorator string at runtime.
    /// Handles Raw, IdentBlock with StringInterp, and placeholders.
    fn generate_decorator_string(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::Raw { value: text, .. } => Ok(quote! { #text.to_string() }),
            IrNode::IdentBlock { parts, .. } => {
                let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .map(|p| self.generate_decorator_string_part(p))
                    .collect::<GenResult<Vec<_>>>()?;
                Ok(quote! {
                    {
                        let mut __s = String::new();
                        #(#part_exprs)*
                        __s
                    }
                })
            }
            _ => Err(GenError::unexpected_node(
                "decorator string",
                node,
                &["Raw", "IdentBlock"],
            )),
        }
    }

    /// Generate code for a single part of a decorator string.
    fn generate_decorator_string_part(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::Raw { value: text, .. } => Ok(quote! { __s.push_str(#text); }),
            IrNode::StrLit { value: text, .. } => {
                // String literal - include with quotes
                Ok(quote! {
                    __s.push_str("\"");
                    __s.push_str(#text);
                    __s.push_str("\"");
                })
            }
            IrNode::Ident { value: name, .. } => Ok(quote! { __s.push_str(#name); }),
            IrNode::NumLit { value: num, .. } => Ok(quote! { __s.push_str(#num); }),
            IrNode::BoolLit { value: b, .. } => {
                let val = if *b { "true" } else { "false" };
                Ok(quote! { __s.push_str(#val); })
            }
            IrNode::NullLit { .. } => Ok(quote! { __s.push_str("null"); }),
            IrNode::StringInterp { quote: q, parts, .. } => {
                let quote_char = match q {
                    '"' => "\"",
                    '\'' => "'",
                    _ => "\"",
                };
                let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .map(|p| match p {
                        IrNode::Raw { value: t, .. } => Ok(quote! { __s.push_str(#t); }),
                        IrNode::Placeholder { expr, .. } => {
                            Ok(quote! { __s.push_str(&(#expr).to_string()); })
                        }
                        _ => Err(GenError::unexpected_node(
                            "decorator string interpolation part",
                            p,
                            &["Raw", "Placeholder"],
                        )),
                    })
                    .collect::<GenResult<Vec<_>>>()?;
                Ok(quote! {
                    __s.push_str(#quote_char);
                    #(#part_exprs)*
                    __s.push_str(#quote_char);
                })
            }
            IrNode::Placeholder { expr, .. } => {
                Ok(quote! { __s.push_str(&(#expr).to_string()); })
            }
            _ => Err(GenError::unexpected_node(
                "decorator string part",
                node,
                &["Raw", "StrLit", "Ident", "NumLit", "BoolLit", "NullLit", "StringInterp", "Placeholder"],
            )),
        }
    }

    /// Generate code for a list of module-level items.
    /// Groups adjacent fragment nodes (Raw, Placeholder, etc.) together and parses them as a combined string.
    pub(in super::super) fn generate_module_items(&self, nodes: &[IrNode]) -> GenResult<TokenStream> {
        let mut pushes = Vec::new();
        let mut pending_fragments: Vec<&IrNode> = Vec::new();

        for node in nodes {
            if self.is_fragment_node(node) {
                pending_fragments.push(node);
            } else {
                // Flush pending fragments as combined module item
                if !pending_fragments.is_empty() {
                    if let Some(stmt) = self.generate_combined_module_item(&pending_fragments)? {
                        pushes.push(stmt);
                    }
                    pending_fragments.clear();
                }
                // Generate structured node directly
                if let Some(push) = self.generate_module_item(node)? {
                    pushes.push(push);
                }
            }
        }

        // Flush remaining pending fragments
        if !pending_fragments.is_empty() {
            if let Some(stmt) = self.generate_combined_module_item(&pending_fragments)? {
                pushes.push(stmt);
            }
        }

        Ok(quote! { #(#pushes)* })
    }

    /// Generate code for combined fragment nodes at module level.
    pub(in super::super) fn generate_combined_module_item(
        &self,
        nodes: &[&IrNode],
    ) -> GenResult<Option<TokenStream>> {
        if nodes.is_empty() {
            return Ok(None);
        }

        let output_var = format_ident!("{}", self.config.output_var);

        // Check if all nodes are whitespace-only Raw nodes
        let all_whitespace = nodes
            .iter()
            .all(|n| matches!(n, IrNode::Raw { value: text, .. } if text.trim().is_empty()));
        if all_whitespace {
            return Ok(None);
        }

        // Generate code that builds a module item string and parses it
        let part_exprs: Vec<TokenStream> = nodes
            .iter()
            .map(|n| self.generate_module_item_string_part(n))
            .collect::<GenResult<Vec<_>>>()?;

        Ok(Some(quote! {
            {
                let mut __module_str = String::new();
                #(#part_exprs)*
                if !__module_str.trim().is_empty() {
                    let __parsed = macroforge_ts::ts_syn::parse_ts_module(&__module_str)
                        .unwrap_or_else(|e| panic!(
                            "Failed to parse generated TypeScript module:\n\n{}\n\nError: {:?}",
                            __module_str, e
                        ));
                    #output_var.extend(__parsed.body);
                }
            }
        }))
    }

    /// Generate code to append a fragment node to the module string.
    pub(in super::super) fn generate_module_item_string_part(
        &self,
        node: &IrNode,
    ) -> GenResult<TokenStream> {
        match node {
            IrNode::Raw { value: text, .. } => Ok(quote! { __module_str.push_str(#text); }),
            IrNode::StrLit { value: text, .. } => Ok(quote! { __module_str.push_str(#text); }),
            IrNode::Ident { value: name, .. } => Ok(quote! { __module_str.push_str(#name); }),
            IrNode::Placeholder { kind, expr, span } => match kind {
                PlaceholderKind::Expr => Ok(quote! {
                    let __expr = macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone());
                    __module_str.push_str(&macroforge_ts::ts_syn::emit_expr(&__expr));
                }),
                PlaceholderKind::Ident => Ok(quote! {
                    __module_str.push_str(&macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()).sym.to_string());
                }),
                PlaceholderKind::Type => Ok(quote! {
                    let __ty = macroforge_ts::ts_syn::ToTsType::to_ts_type((#expr).clone());
                    __module_str.push_str(&macroforge_ts::ts_syn::emit_ts_type(&__ty));
                }),
                PlaceholderKind::Stmt => Err(GenError::invalid_placeholder_at(
                    "module item string",
                    "Stmt",
                    &["Expr", "Ident", "Type"],
                    *span,
                )
                .with_help(
                    "Statement placeholders cannot be used in string interpolation context",
                )),
            },
            IrNode::IdentBlock { parts, .. } => {
                let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .map(|p| self.generate_module_item_string_part(p))
                    .collect::<GenResult<Vec<_>>>()?;
                Ok(quote! { #(#part_exprs)* })
            }
            IrNode::StringInterp { parts, .. } => {
                let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .map(|p| self.generate_module_item_string_part(p))
                    .collect::<GenResult<Vec<_>>>()?;
                Ok(quote! { #(#part_exprs)* })
            }
            _ => Err(GenError::unexpected_node(
                "module item string part",
                node,
                &[
                    "Raw",
                    "StrLit",
                    "Ident",
                    "Placeholder",
                    "IdentBlock",
                    "StringInterp",
                ],
            )),
        }
    }

    /// Generate code for a single module-level item.
    pub(in super::super) fn generate_module_item(
        &self,
        node: &IrNode,
    ) -> GenResult<Option<TokenStream>> {
        let output_var = format_ident!("{}", self.config.output_var);

        match node {
            // =================================================================
            // Declarations
            // =================================================================
            IrNode::FnDecl {
                exported,
                declare,
                async_,
                generator,
                name,
                type_params,
                params,
                return_type,
                body,
                ..
            } => {
                let name_code = self.generate_ident(name)?;
                let params_code = self.generate_params(params)?;
                let body_code = match body.as_ref() {
                    Some(b) => self.generate_block_stmt_opt(b)?,
                    None => quote! { None },
                };
                let return_type_code = match return_type.as_ref() {
                    Some(t) => {
                        let tc = self.generate_type_ann(t)?;
                        quote! { Some(Box::new(#tc)) }
                    }
                    None => quote! { None },
                };
                let type_params_code = match type_params.as_ref() {
                    Some(tp) => {
                        let tpc = self.generate_type_params(tp)?;
                        quote! { Some(Box::new(#tpc)) }
                    }
                    None => quote! { None },
                };

                let fn_decl = quote! {
                    macroforge_ts::swc_core::ecma::ast::FnDecl {
                        ident: #name_code,
                        declare: #declare,
                        function: Box::new(macroforge_ts::swc_core::ecma::ast::Function {
                            params: #params_code,
                            decorators: vec![],
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            body: #body_code,
                            is_generator: #generator,
                            is_async: #async_,
                            type_params: #type_params_code,
                            return_type: #return_type_code,
                        }),
                    }
                };

                if *exported {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::Fn(#fn_decl),
                                }
                            )
                        ));
                    }))
                } else {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::Fn(#fn_decl)
                            )
                        ));
                    }))
                }
            }

            IrNode::ClassDecl {
                exported,
                declare,
                abstract_,
                decorators,
                name,
                type_params,
                extends,
                implements,
                body,
                ..
            } => {
                let name_code = self.generate_ident(name)?;
                let body_code = self.generate_class_members(body)?;
                let extends_code = match extends.as_ref() {
                    Some(e) => {
                        let ec = self.generate_expr(e)?;
                        quote! {
                            Some(Box::new(macroforge_ts::swc_core::ecma::ast::ExtendsClause {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                super_class: Box::new(#ec),
                                type_args: None,
                            }))
                        }
                    }
                    None => quote! { None },
                };
                let type_params_code = match type_params.as_ref() {
                    Some(tp) => {
                        let tpc = self.generate_type_params(tp)?;
                        quote! { Some(Box::new(#tpc)) }
                    }
                    None => quote! { None },
                };
                let implements_code = self.generate_implements(implements)?;

                // Generate decorators - build the decorator expression dynamically
                let decorators_code = if decorators.is_empty() {
                    quote! { vec![] }
                } else {
                    let decorator_exprs: Vec<TokenStream> = decorators
                        .iter()
                        .map(|d| {
                            // Generate code to build decorator string at runtime
                            let build_str = self.generate_decorator_string(d)?;

                            Ok(quote! {
                                {
                                    let __dec_full_str = #build_str;
                                    // Remove leading @ if present
                                    let __dec_expr_str = __dec_full_str.trim_start().strip_prefix('@').unwrap_or(&__dec_full_str);
                                    if let Ok(__dec_expr) = macroforge_ts::ts_syn::parse_ts_expr(__dec_expr_str) {
                                        Some(macroforge_ts::swc_core::ecma::ast::Decorator {
                                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                                            expr: __dec_expr,
                                        })
                                    } else {
                                        None
                                    }
                                }
                            })
                        })
                        .collect::<GenResult<Vec<_>>>()?;

                    quote! {
                        {
                            let __decorators: Vec<Option<macroforge_ts::swc_core::ecma::ast::Decorator>> = vec![#(#decorator_exprs),*];
                            __decorators.into_iter().flatten().collect::<Vec<_>>()
                        }
                    }
                };

                let class_decl = quote! {
                    macroforge_ts::swc_core::ecma::ast::ClassDecl {
                        ident: #name_code,
                        declare: #declare,
                        class: Box::new(macroforge_ts::swc_core::ecma::ast::Class {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            decorators: #decorators_code,
                            body: #body_code,
                            super_class: #extends_code,
                            is_abstract: #abstract_,
                            type_params: #type_params_code,
                            super_type_params: None,
                            implements: #implements_code,
                        }),
                    }
                };

                if *exported {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::Class(#class_decl),
                                }
                            )
                        ));
                    }))
                } else {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::Class(#class_decl)
                            )
                        ));
                    }))
                }
            }

            IrNode::InterfaceDecl {
                exported,
                declare,
                name,
                type_params,
                extends,
                body,
                ..
            } => {
                let name_code = self.generate_ident(name)?;
                let body_code = self.generate_ts_interface_body(body)?;
                let type_params_code = match type_params.as_ref() {
                    Some(tp) => {
                        let tpc = self.generate_type_params(tp)?;
                        quote! { Some(Box::new(#tpc)) }
                    }
                    None => quote! { None },
                };
                let extends_code = self.generate_ts_expr_with_type_args(extends);

                let interface_decl = quote! {
                    macroforge_ts::swc_core::ecma::ast::TsInterfaceDecl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        id: #name_code,
                        declare: #declare,
                        type_params: #type_params_code,
                        extends: #extends_code,
                        body: #body_code,
                    }
                };

                if *exported {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::TsInterface(Box::new(#interface_decl)),
                                }
                            )
                        ));
                    }))
                } else {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::TsInterface(Box::new(#interface_decl))
                            )
                        ));
                    }))
                }
            }

            IrNode::TypeAliasDecl {
                exported,
                declare,
                name,
                type_params,
                type_ann,
                ..
            } => {
                let name_code = self.generate_ident(name)?;
                let type_ann_code = self.generate_type(type_ann)?;
                let type_params_code = match type_params.as_ref() {
                    Some(tp) => {
                        let tpc = self.generate_type_params(tp)?;
                        quote! { Some(Box::new(#tpc)) }
                    }
                    None => quote! { None },
                };

                let type_alias = quote! {
                    macroforge_ts::swc_core::ecma::ast::TsTypeAliasDecl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        id: #name_code,
                        declare: #declare,
                        type_params: #type_params_code,
                        type_ann: Box::new(#type_ann_code),
                    }
                };

                if *exported {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::TsTypeAlias(Box::new(#type_alias)),
                                }
                            )
                        ));
                    }))
                } else {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::TsTypeAlias(Box::new(#type_alias))
                            )
                        ));
                    }))
                }
            }

            IrNode::VarDecl {
                exported,
                declare,
                kind,
                decls,
                ..
            } => {
                let kind_code = match kind {
                    VarKind::Const => {
                        quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Const }
                    }
                    VarKind::Let => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Let },
                    VarKind::Var => quote! { macroforge_ts::swc_core::ecma::ast::VarDeclKind::Var },
                };
                let decls_code = self.generate_var_declarators(decls)?;

                let var_decl = quote! {
                    macroforge_ts::swc_core::ecma::ast::VarDecl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        kind: #kind_code,
                        declare: #declare,
                        decls: #decls_code,
                    }
                };

                if *exported {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::Var(Box::new(#var_decl)),
                                }
                            )
                        ));
                    }))
                } else {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::Var(Box::new(#var_decl))
                            )
                        ));
                    }))
                }
            }

            // =================================================================
            // Import/Export Declarations
            // =================================================================
            IrNode::ImportDecl {
                type_only,
                specifiers,
                src,
                ..
            } => {
                let specifiers_code: GenResult<Vec<TokenStream>> = specifiers
                    .iter()
                    .map(|spec| match spec {
                        IrNode::NamedImport { local, imported, .. } => {
                            let local_code = self.generate_ident(local)?;
                            let imported_code = match imported.as_ref() {
                                Some(i) => {
                                    let ic = self.generate_ident(i)?;
                                    quote! { Some(macroforge_ts::swc_core::ecma::ast::ModuleExportName::Ident(#ic)) }
                                }
                                None => quote! { None },
                            };
                            Ok(quote! {
                                macroforge_ts::swc_core::ecma::ast::ImportSpecifier::Named(
                                    macroforge_ts::swc_core::ecma::ast::ImportNamedSpecifier {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        local: #local_code,
                                        imported: #imported_code,
                                        is_type_only: false,
                                    }
                                )
                            })
                        }
                        IrNode::DefaultImport { local, .. } => {
                            let local_code = self.generate_ident(local)?;
                            Ok(quote! {
                                macroforge_ts::swc_core::ecma::ast::ImportSpecifier::Default(
                                    macroforge_ts::swc_core::ecma::ast::ImportDefaultSpecifier {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        local: #local_code,
                                    }
                                )
                            })
                        }
                        IrNode::NamespaceImport { local, .. } => {
                            let local_code = self.generate_ident(local)?;
                            Ok(quote! {
                                macroforge_ts::swc_core::ecma::ast::ImportSpecifier::Namespace(
                                    macroforge_ts::swc_core::ecma::ast::ImportStarAsSpecifier {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        local: #local_code,
                                    }
                                )
                            })
                        }
                        _ => Err(GenError::unexpected_node(
                            "import specifier",
                            spec,
                            &["NamedImport", "DefaultImport", "NamespaceImport"],
                        )),
                    })
                    .collect();
                let specifiers_code = specifiers_code?;

                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                        macroforge_ts::swc_core::ecma::ast::ModuleDecl::Import(
                            macroforge_ts::swc_core::ecma::ast::ImportDecl {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                specifiers: vec![#(#specifiers_code),*],
                                src: Box::new(macroforge_ts::swc_core::ecma::ast::Str {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    value: #src.into(),
                                    raw: None,
                                }),
                                type_only: #type_only,
                                with: None,
                                phase: macroforge_ts::swc_core::ecma::ast::ImportPhase::Evaluation,
                            }
                        )
                    ));
                }))
            }

            IrNode::NamedExport {
                specifiers,
                src,
                type_only,
                ..
            } => {
                let specifiers_code: GenResult<Vec<TokenStream>> = specifiers
                    .iter()
                    .map(|spec| {
                        if let IrNode::ExportSpecifier { local, exported, .. } = spec {
                            let local_code = self.generate_ident(local)?;
                            let local_name = quote! { macroforge_ts::swc_core::ecma::ast::ModuleExportName::Ident(#local_code) };
                            let exported_code = match exported.as_ref() {
                                Some(e) => {
                                    let ec = self.generate_ident(e)?;
                                    quote! { Some(macroforge_ts::swc_core::ecma::ast::ModuleExportName::Ident(#ec)) }
                                }
                                None => quote! { None },
                            };
                            Ok(quote! {
                                macroforge_ts::swc_core::ecma::ast::ExportSpecifier::Named(
                                    macroforge_ts::swc_core::ecma::ast::ExportNamedSpecifier {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        orig: #local_name,
                                        exported: #exported_code,
                                        is_type_only: false,
                                    }
                                )
                            })
                        } else {
                            Err(GenError::unexpected_node(
                                "export specifier",
                                spec,
                                &["ExportSpecifier"],
                            ))
                        }
                    })
                    .collect();
                let specifiers_code = specifiers_code?;

                let src_code = src
                    .as_ref()
                    .map(|s| {
                        quote! {
                            Some(Box::new(macroforge_ts::swc_core::ecma::ast::Str {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                value: #s.into(),
                                raw: None,
                            }))
                        }
                    })
                    .unwrap_or(quote! { None });

                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                        macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportNamed(
                            macroforge_ts::swc_core::ecma::ast::NamedExport {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                specifiers: vec![#(#specifiers_code),*],
                                src: #src_code,
                                type_only: #type_only,
                                with: None,
                            }
                        )
                    ));
                }))
            }

            IrNode::ExportAll { src, type_only, .. } => {
                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                        macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportAll(
                            macroforge_ts::swc_core::ecma::ast::ExportAll {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                src: Box::new(macroforge_ts::swc_core::ecma::ast::Str {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    value: #src.into(),
                                    raw: None,
                                }),
                                type_only: #type_only,
                                with: None,
                            }
                        )
                    ));
                }))
            }

            IrNode::ExportDefaultExpr { expr, .. } => {
                // Check if it's a declaration that needs ExportDefaultDecl
                match expr.as_ref() {
                    IrNode::ClassDecl {
                        name,
                        type_params,
                        extends,
                        implements,
                        body,
                        decorators,
                        abstract_,
                        ..
                    } => {
                        // Convert ClassDecl to ClassExpr for DefaultDecl::Class
                        let name_code = self.generate_ident(name)?;
                        let body_code = self.generate_class_members(body)?;
                        let extends_code = extends
                            .as_ref()
                            .map(|e| {
                                let ec = self.generate_expr(e)?;
                                Ok(quote! {
                                    Some(Box::new(macroforge_ts::swc_core::ecma::ast::ExtendsClause {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        super_class: Box::new(#ec),
                                        type_args: None,
                                    }))
                                })
                            })
                            .transpose()?
                            .unwrap_or(quote! { None });
                        let type_params_code = type_params
                            .as_ref()
                            .map(|tp| {
                                let tpc = self.generate_type_params(tp)?;
                                Ok(quote! { Some(Box::new(#tpc)) })
                            })
                            .transpose()?
                            .unwrap_or(quote! { None });
                        let implements_code = self.generate_implements(implements)?;

                        // Generate decorators
                        let decorators_code = if decorators.is_empty() {
                            quote! { vec![] }
                        } else {
                            let decorator_exprs: Vec<TokenStream> = decorators
                                .iter()
                                .map(|d| {
                                    let build_str = self.generate_decorator_string(d)?;
                                    Ok(quote! {
                                        {
                                            let __dec_full_str = #build_str;
                                            let __dec_expr_str = __dec_full_str.trim_start().strip_prefix('@').unwrap_or(&__dec_full_str);
                                            if let Ok(__dec_expr) = macroforge_ts::ts_syn::parse_ts_expr(__dec_expr_str) {
                                                Some(macroforge_ts::swc_core::ecma::ast::Decorator {
                                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                                    expr: __dec_expr,
                                                })
                                            } else {
                                                None
                                            }
                                        }
                                    })
                                })
                                .collect::<GenResult<Vec<_>>>()?;

                            quote! {
                                {
                                    let __decorators: Vec<Option<macroforge_ts::swc_core::ecma::ast::Decorator>> = vec![#(#decorator_exprs),*];
                                    __decorators.into_iter().flatten().collect::<Vec<_>>()
                                }
                            }
                        };

                        Ok(Some(quote! {
                            #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                                macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDefaultDecl(
                                    macroforge_ts::swc_core::ecma::ast::ExportDefaultDecl {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        decl: macroforge_ts::swc_core::ecma::ast::DefaultDecl::Class(
                                            Box::new(macroforge_ts::swc_core::ecma::ast::ClassExpr {
                                                ident: Some(#name_code),
                                                class: Box::new(macroforge_ts::swc_core::ecma::ast::Class {
                                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                                    ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                                                    decorators: #decorators_code,
                                                    body: #body_code,
                                                    super_class: #extends_code,
                                                    is_abstract: #abstract_,
                                                    type_params: #type_params_code,
                                                    super_type_params: None,
                                                    implements: #implements_code,
                                                }),
                                            })
                                        ),
                                    }
                                )
                            ));
                        }))
                    }
                    IrNode::FnDecl {
                        name,
                        type_params,
                        params,
                        return_type,
                        body,
                        async_,
                        generator,
                        ..
                    } => {
                        // Convert FnDecl to FnExpr for DefaultDecl::Fn
                        let name_code = self.generate_ident(name)?;
                        let params_code = self.generate_params(params)?;
                        let body_code = match body.as_ref() {
                            Some(b) => self.generate_block_stmt_opt(b)?,
                            None => quote! { None },
                        };
                        let return_type_code = match return_type.as_ref() {
                            Some(t) => {
                                let tc = self.generate_type_ann(t)?;
                                quote! { Some(Box::new(#tc)) }
                            }
                            None => quote! { None },
                        };
                        let type_params_code = match type_params.as_ref() {
                            Some(tp) => {
                                let tpc = self.generate_type_params(tp)?;
                                quote! { Some(Box::new(#tpc)) }
                            }
                            None => quote! { None },
                        };

                        Ok(Some(quote! {
                            #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                                macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDefaultDecl(
                                    macroforge_ts::swc_core::ecma::ast::ExportDefaultDecl {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        decl: macroforge_ts::swc_core::ecma::ast::DefaultDecl::Fn(
                                            Box::new(macroforge_ts::swc_core::ecma::ast::FnExpr {
                                                ident: Some(#name_code),
                                                function: Box::new(macroforge_ts::swc_core::ecma::ast::Function {
                                                    params: #params_code,
                                                    decorators: vec![],
                                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                                    ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                                                    body: #body_code,
                                                    is_generator: #generator,
                                                    is_async: #async_,
                                                    type_params: #type_params_code,
                                                    return_type: #return_type_code,
                                                }),
                                            })
                                        ),
                                    }
                                )
                            ));
                        }))
                    }
                    _ => {
                        // Regular expression export
                        let expr_code = self.generate_expr(expr)?;
                        Ok(Some(quote! {
                            #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                                macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDefaultExpr(
                                    macroforge_ts::swc_core::ecma::ast::ExportDefaultExpr {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        expr: Box::new(#expr_code),
                                    }
                                )
                            ));
                        }))
                    }
                }
            }

            // =================================================================
            // Enum Declaration
            // =================================================================
            IrNode::EnumDecl {
                exported,
                declare,
                const_,
                name,
                members,
                ..
            } => {
                let name_code = self.generate_ident(name)?;
                let members_code: Vec<TokenStream> = members
                    .iter()
                    .filter_map(|m| {
                        if let IrNode::EnumMember { name, init, .. } = m {
                            Some((name, init))
                        } else {
                            None
                        }
                    })
                    .map(|(name, init)| {
                        let member_name_code = self.generate_ident(name)?;
                        let init_code = match init.as_ref() {
                            Some(i) => {
                                let ic = self.generate_expr(i)?;
                                quote! { Some(Box::new(#ic)) }
                            }
                            None => quote! { None },
                        };
                        Ok(quote! {
                            macroforge_ts::swc_core::ecma::ast::TsEnumMember {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                id: macroforge_ts::swc_core::ecma::ast::TsEnumMemberId::Ident(#member_name_code),
                                init: #init_code,
                            }
                        })
                    })
                    .collect::<GenResult<_>>()?;

                let enum_decl = quote! {
                    macroforge_ts::swc_core::ecma::ast::TsEnumDecl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        declare: #declare,
                        is_const: #const_,
                        id: #name_code,
                        members: vec![#(#members_code),*],
                    }
                };

                if *exported {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                            macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    decl: macroforge_ts::swc_core::ecma::ast::Decl::TsEnum(Box::new(#enum_decl)),
                                }
                            )
                        ));
                    }))
                } else {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                macroforge_ts::swc_core::ecma::ast::Decl::TsEnum(Box::new(#enum_decl))
                            )
                        ));
                    }))
                }
            }

            // =================================================================
            // Decorator (standalone - attached to next declaration)
            // =================================================================
            IrNode::Decorator { expr, .. } => {
                // Generate the decorator expression - this will be collected and applied
                // to the next class/method declaration
                let decorator_expr = self.generate_expr(expr)?;

                // For module-level decorators, emit a comment for now
                // Full support would require modifying how we collect decorators
                // and apply them to subsequent class declarations
                Ok(Some(quote! {
                    // Decorator expression generated: #decorator_expr
                }))
            }

            // =================================================================
            // Statements as module items
            // =================================================================
            IrNode::ExprStmt { expr, .. } => {
                let expr_code = self.generate_expr(expr)?;
                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(#expr_code),
                            }
                        )
                    ));
                }))
            }

            IrNode::ReturnStmt { arg, .. } => {
                let arg_code = match arg.as_ref() {
                    Some(a) => {
                        let ac = self.generate_expr(a)?;
                        quote! { Some(Box::new(#ac)) }
                    }
                    None => quote! { None },
                };
                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Return(
                            macroforge_ts::swc_core::ecma::ast::ReturnStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                arg: #arg_code,
                            }
                        )
                    ));
                }))
            }

            IrNode::BlockStmt { stmts, .. } => {
                let stmts_code = self.generate_stmts_vec(stmts)?;
                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Block(
                            macroforge_ts::swc_core::ecma::ast::BlockStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                                stmts: #stmts_code,
                            }
                        )
                    ));
                }))
            }

            IrNode::ThrowStmt { arg, .. } => {
                let arg_code = self.generate_expr(arg)?;
                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Throw(
                            macroforge_ts::swc_core::ecma::ast::ThrowStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                arg: Box::new(#arg_code),
                            }
                        )
                    ));
                }))
            }

            // =================================================================
            // Template Constructs - Control Flow (Rust)
            // =================================================================
            IrNode::If {
                condition,
                then_body,
                else_if_branches,
                else_body,
                ..
            } => {
                let cond = condition;
                let then_stmts = self.generate_module_items(then_body)?;

                let else_ifs: GenResult<Vec<TokenStream>> = else_if_branches
                    .iter()
                    .map(|(c, body)| {
                        let b = self.generate_module_items(body)?;
                        Ok(quote! { else if #c { #b } })
                    })
                    .collect();
                let else_ifs = else_ifs?;

                let else_part = match else_body.as_ref() {
                    Some(body) => {
                        let b = self.generate_module_items(body)?;
                        Some(quote! { else { #b } })
                    }
                    None => None,
                };

                Ok(Some(quote! {
                    if #cond { #then_stmts }
                    #(#else_ifs)*
                    #else_part
                }))
            }

            IrNode::For {
                pattern,
                iterator,
                body,
                ..
            } => {
                let body_stmts = self.generate_module_items(body)?;
                Ok(Some(quote! {
                    for #pattern in #iterator { #body_stmts }
                }))
            }

            IrNode::While { condition, body, .. } => {
                let body_stmts = self.generate_module_items(body)?;
                Ok(Some(quote! {
                    while #condition { #body_stmts }
                }))
            }

            IrNode::Match { expr, arms, .. } => {
                let arm_tokens: GenResult<Vec<TokenStream>> = arms
                    .iter()
                    .map(
                        |MatchArm {
                             span,
                             pattern,
                             guard,
                             body,
                         }| {
                            let _ = span; // Consume span field
                            let b = self.generate_module_items(body)?;
                            if let Some(g) = guard {
                                Ok(quote! { #pattern if #g => { #b } })
                            } else {
                                Ok(quote! { #pattern => { #b } })
                            }
                        },
                    )
                    .collect();
                let arm_tokens = arm_tokens?;

                Ok(Some(quote! {
                    match #expr { #(#arm_tokens)* }
                }))
            }

            IrNode::Let {
                pattern,
                mutable,
                type_hint,
                value,
                ..
            } => {
                let mutability = if *mutable {
                    quote! { mut }
                } else {
                    quote! {}
                };

                if let Some(ty) = type_hint {
                    Ok(Some(quote! { let #mutability #pattern: #ty = #value; }))
                } else {
                    Ok(Some(quote! { let #mutability #pattern = #value; }))
                }
            }

            IrNode::Do { code, .. } => Ok(Some(quote! { #code; })),

            IrNode::TypeScript { stream, .. } => {
                // Handle both module-level and class body streams.
                // Class body streams are marked with /* @macroforge:body */
                Ok(Some(quote! {
                    {
                        let __ts: macroforge_ts::ts_syn::TsStream = #stream;
                        let __source = __ts.source();
                        // Check if this is a class body stream (marked with @macroforge:body)
                        if __source.starts_with("/* @macroforge:body */") {
                            // For class body content, strip the marker and wrap in a class to parse
                            let __content = __source.trim_start_matches("/* @macroforge:body */");
                            if !__content.trim().is_empty() {
                                // Wrap content in a dummy class to parse it
                                let __wrapped = format!("class __MF_CLASS_BODY__ {{ {} }}", __content);
                                if let Ok(__class_module) = macroforge_ts::ts_syn::parse_ts_module(&__wrapped) {
                                    for item in __class_module.body {
                                        // Emit the wrapped class (it will be named __MF_CLASS_BODY__)
                                        #output_var.push(item);
                                    }
                                }
                            }
                        } else {
                            // Normal module-level stream
                            let __parsed = macroforge_ts::ts_syn::parse_ts_module(&__source)
                                .expect("Failed to parse TypeScript stream");
                            #output_var.extend(__parsed.body);
                        }
                    }
                }))
            }

            // =================================================================
            // Placeholders
            // =================================================================
            IrNode::Placeholder { kind, expr, .. } => match kind {
                PlaceholderKind::Expr => Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone())),
                            }
                        )
                    ));
                })),
                PlaceholderKind::Stmt => Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::ts_syn::ToTsStmt::to_ts_stmt(#expr)
                    ));
                })),
                PlaceholderKind::Ident => Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                                    macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone())
                                )),
                            }
                        )
                    ));
                })),
                PlaceholderKind::Type => {
                    // Type placeholders in flat IR need to be emitted as expressions
                    // We use a type assertion (undefined as T) to include the type
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                                macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::TsAs(
                                        macroforge_ts::swc_core::ecma::ast::TsAsExpr {
                                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                                            expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                                                macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                                    "undefined".into(),
                                                    macroforge_ts::swc_core::common::DUMMY_SP,
                                                )
                                            )),
                                            type_ann: Box::new(macroforge_ts::ts_syn::ToTsType::to_ts_type((#expr).clone())),
                                        }
                                    )),
                                }
                            )
                        ));
                    }))
                }
            },

            // =================================================================
            // Comments
            // =================================================================
            IrNode::LineComment { text, .. } => {
                // Comments are emitted as empty statements for now
                // SWC doesn't have inline comment AST nodes - comments are stored separately
                // The text is consumed here to satisfy the "never read" warning
                let _ = text;
                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Empty(
                            macroforge_ts::swc_core::ecma::ast::EmptyStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                            }
                        )
                    ));
                }))
            }

            IrNode::BlockComment { text, .. } => {
                // Block comments are emitted as empty statements for now
                // The text is consumed here to satisfy the "never read" warning
                let _ = text;
                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Empty(
                            macroforge_ts::swc_core::ecma::ast::EmptyStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                            }
                        )
                    ));
                }))
            }

            IrNode::DocComment { text, .. } => {
                // Doc comments are preserved but don't generate AST nodes
                let _ = text; // Consume the field
                Ok(None)
            }

            IrNode::Documented { doc, inner, .. } => {
                // Documented nodes carry JSDoc - for now we preserve the inner and note the doc
                let _ = doc; // Consume the doc field - could be used for JSDoc generation in future
                self.generate_module_item(inner)
            }

            // =================================================================
            // Special constructs
            // =================================================================
            IrNode::IdentBlock { parts, .. } => {
                let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .filter_map(|p| match p {
                        IrNode::Raw { value: text, .. } => Some(quote! { __ident.push_str(#text); }),
                        IrNode::StrLit { value: text, .. } => Some(quote! { __ident.push_str(#text); }),
                        IrNode::Ident { value: text, .. } => Some(quote! { __ident.push_str(#text); }),
                        IrNode::Placeholder { expr, .. } => {
                            Some(quote! {
                                __ident.push_str(&macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()).sym.to_string());
                            })
                        }
                        _ => None,
                    })
                    .collect();

                Ok(Some(quote! {
                    {
                        let mut __ident = String::new();
                        #(#part_exprs)*
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                                macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(
                                        macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                            __ident.into(),
                                            macroforge_ts::swc_core::common::DUMMY_SP,
                                        )
                                    )),
                                }
                            )
                        ));
                    }
                }))
            }

            IrNode::StringInterp { quote: _, parts, .. } => {
                let mut quasi_parts = Vec::new();
                let mut expr_parts = Vec::new();
                let mut current_text = String::new();

                for part in parts {
                    match part {
                        IrNode::Raw { value: text, .. } | IrNode::StrLit { value: text, .. } => current_text.push_str(text),
                        IrNode::Placeholder { expr, .. } => {
                            let text = std::mem::take(&mut current_text);
                            quasi_parts.push(quote! {
                                macroforge_ts::swc_core::ecma::ast::TplElement {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    tail: false,
                                    cooked: Some(#text.into()),
                                    raw: #text.into(),
                                }
                            });
                            expr_parts.push(quote! {
                                Box::new(macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone()))
                            });
                        }
                        _ => {}
                    }
                }

                let final_text = current_text;
                quasi_parts.push(quote! {
                    macroforge_ts::swc_core::ecma::ast::TplElement {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        tail: true,
                        cooked: Some(#final_text.into()),
                        raw: #final_text.into(),
                    }
                });

                Ok(Some(quote! {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                        macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                            macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Tpl(
                                    macroforge_ts::swc_core::ecma::ast::Tpl {
                                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                                        quasis: vec![#(#quasi_parts),*],
                                        exprs: vec![#(#expr_parts),*],
                                    }
                                )),
                            }
                        )
                    ));
                }))
            }

            // Raw text - parse at runtime since it might be a fragment
            IrNode::Raw { value: text, .. } => {
                if text.trim().is_empty() {
                    return Ok(None);
                }
                // Use runtime parsing with parse_ts_module, falling back to storing raw text
                // if it can't be parsed (e.g., partial fragments that will be combined with placeholders)
                Ok(Some(quote! {
                    {
                        let __raw_text = #text;
                        let __parsed = macroforge_ts::ts_syn::parse_ts_module(__raw_text)
                            .unwrap_or_else(|e| panic!(
                                "Failed to parse raw TypeScript fragment:\n\n{}\n\nError: {:?}",
                                __raw_text, e
                            ));
                        #output_var.extend(__parsed.body);
                    }
                }))
            }

            // TypeScript if statement at module level
            IrNode::TsIfStmt { test, cons, alt, .. } => {
                let test_code = self.generate_expr_string_parts(test)?;
                let cons_code = self.generate_stmt_as_string(cons)?;
                let alt_code = alt.as_ref().map(|a| {
                    let ac = self.generate_stmt_as_string(a)?;
                    Ok(quote! {
                        __stmt_str.push_str(" else ");
                        #ac
                    })
                }).transpose()?;
                Ok(Some(quote! {
                    {
                        let mut __stmt_str = String::new();
                        __stmt_str.push_str("if (");
                        #test_code
                        __stmt_str.push_str(") ");
                        #cons_code
                        #alt_code
                        let __parsed = macroforge_ts::ts_syn::parse_ts_stmt(&__stmt_str)
                            .unwrap_or_else(|e| panic!(
                                "Failed to parse generated TypeScript if statement:\n\n{}\n\nError: {:?}",
                                __stmt_str, e
                            ));
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(__parsed));
                    }
                }))
            }

            // TypeScript loop statement at module level
            IrNode::TsLoopStmt { parts, .. } => {
                let part_exprs: Vec<TokenStream> =
                    parts.iter().map(|p| self.generate_stmt_string_part(p)).collect::<GenResult<_>>()?;
                Ok(Some(quote! {
                    {
                        let mut __stmt_str = String::new();
                        #(#part_exprs)*
                        let __parsed = macroforge_ts::ts_syn::parse_ts_stmt(&__stmt_str)
                            .unwrap_or_else(|e| panic!(
                                "Failed to parse generated TypeScript loop statement:\n\n{}\n\nError: {:?}",
                                __stmt_str, e
                            ));
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(__parsed));
                    }
                }))
            }

            // All other node types that don't make sense at module level
            _ => {
                // For nodes that should be expressions or statements,
                // wrap them appropriately
                if let Some(expr_code) = self.try_generate_as_expr(node)? {
                    Ok(Some(quote! {
                        #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                            macroforge_ts::swc_core::ecma::ast::Stmt::Expr(
                                macroforge_ts::swc_core::ecma::ast::ExprStmt {
                                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                                    expr: Box::new(#expr_code),
                                }
                            )
                        ));
                    }))
                } else {
                    Err(GenError::unexpected_node(
                        "module item",
                        node,
                        &[
                            "FnDecl",
                            "ClassDecl",
                            "InterfaceDecl",
                            "TypeAliasDecl",
                            "VarDecl",
                            "ImportDecl",
                            "NamedExport",
                            "ExportAll",
                            "ExportDefaultExpr",
                            "EnumDecl",
                            "ExprStmt",
                            "ReturnStmt",
                            "BlockStmt",
                            "If",
                            "For",
                            "While",
                            "Match",
                            "Let",
                            "Do",
                            "TypeScript",
                            "Placeholder",
                            "Raw",
                        ],
                    ))
                }
            }
        }
    }
}
