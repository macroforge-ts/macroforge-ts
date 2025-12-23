use std::collections::{HashMap, HashSet};

use proc_macro2::Span;
use quote::quote;

use crate::template::{
    append_part, build_string_interp_expr, build_template_interp_expr, compile_control_expr,
    compile_ident_block, placeholder_name, placeholder_type_tokens, template_error,
    BindingSpec, PlaceholderUse, Segment, TemplateAndBindings, TypePlaceholder,
};

/// Builds the placeholder template string and binding list for a segment run.
pub fn build_template_and_bindings(
    segments: impl IntoIterator<Item = impl std::borrow::Borrow<Segment>>,
    context_map: &HashMap<usize, PlaceholderUse>,
) -> syn::Result<TemplateAndBindings> {
    let mut template = String::new();
    let mut bindings = Vec::new();
    let mut type_placeholders = Vec::new();
    let mut seen = HashSet::new();

    for seg in segments {
        match seg.borrow() {
            Segment::Static(s) => append_part(&mut template, s),
            Segment::Comment { .. } => {
                return Err(template_error(
                    Span::call_site(),
                    "Comments must appear between TypeScript statements",
                    None,
                ));
            }
            Segment::Interpolation { id, expr } => {
                let use_kind = context_map.get(id).cloned().unwrap_or(PlaceholderUse::Expr);

                if matches!(use_kind, PlaceholderUse::Type) {
                    // For type placeholders, use a unique marker identifier that quote! can parse
                    // We'll replace it with the actual type using a visitor after quote! parsing
                    let marker = format!("__MfTypeMarker{}", id);
                    append_part(&mut template, &marker);
                    if seen.insert(*id) {
                        type_placeholders.push(TypePlaceholder {
                            id: *id,
                            expr: expr.clone(),
                        });
                    }
                } else {
                    // Non-type placeholders use $ prefix for quote! substitution
                    let name = placeholder_name(*id);
                    append_part(&mut template, &format!("${name}"));
                    if seen.insert(*id) {
                        let ty = placeholder_type_tokens(&use_kind);
                        let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                        bindings.push(BindingSpec {
                            name: bind_ident,
                            ty,
                            expr: expr.clone(),
                        });
                    }
                }
            }
            Segment::StringInterp { id, parts } => {
                let name = placeholder_name(*id);
                append_part(&mut template, &format!("${name}"));
                if seen.insert(*id) {
                    let expr = build_string_interp_expr(parts, *id);
                    let ty = placeholder_type_tokens(&PlaceholderUse::Expr);
                    let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                    bindings.push(BindingSpec {
                        name: bind_ident,
                        ty,
                        expr,
                    });
                }
            }
            Segment::TemplateInterp { id, parts } => {
                let name = placeholder_name(*id);
                append_part(&mut template, &format!("${name}"));
                if seen.insert(*id) {
                    let expr = build_template_interp_expr(parts, *id);
                    let ty = placeholder_type_tokens(&PlaceholderUse::Expr);
                    let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                    bindings.push(BindingSpec {
                        name: bind_ident,
                        ty,
                        expr,
                    });
                }
            }
            Segment::IdentBlock { id, parts } => {
                let name = placeholder_name(*id);
                append_part(&mut template, &format!("${name}"));
                if seen.insert(*id) {
                    let ident_expr = compile_ident_block(parts);
                    let ty = placeholder_type_tokens(&PlaceholderUse::Ident);
                    let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                    bindings.push(BindingSpec {
                        name: bind_ident,
                        ty,
                        expr: ident_expr,
                    });
                }
            }
            Segment::Control { id, node } => {
                let name = placeholder_name(*id);
                let use_kind = context_map.get(id).cloned().unwrap_or(PlaceholderUse::Expr);
                if matches!(use_kind, PlaceholderUse::Stmt) {
                    return Err(template_error(
                        Span::call_site(),
                        "Statement-level control blocks must stand alone",
                        None,
                    ));
                }
                if matches!(use_kind, PlaceholderUse::Ident | PlaceholderUse::Type) {
                    return Err(template_error(
                        Span::call_site(),
                        "Control blocks cannot appear in identifier/type positions",
                        None,
                    ));
                }
                append_part(&mut template, &format!("${name}"));
                if seen.insert(*id) {
                    let expr = compile_control_expr(node, Span::call_site())?;
                    let ty = placeholder_type_tokens(&PlaceholderUse::Expr);
                    let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                    bindings.push(BindingSpec {
                        name: bind_ident,
                        ty,
                        expr,
                    });
                }
            }
            Segment::Typescript { id, .. } => {
                let use_kind = context_map.get(id).cloned().unwrap_or(PlaceholderUse::Stmt);
                if !matches!(use_kind, PlaceholderUse::Stmt) {
                    return Err(template_error(
                        Span::call_site(),
                        "{$typescript} is only valid at statement boundaries",
                        None,
                    ));
                }
                return Err(template_error(
                    Span::call_site(),
                    "{$typescript} must be placed as its own statement",
                    None,
                ));
            }
            Segment::Let { .. } | Segment::Do { .. } => {
                return Err(template_error(
                    Span::call_site(),
                    "{$let} and {$do} cannot appear inside TypeScript expressions",
                    None,
                ));
            }
            Segment::BraceBlock { id, inner } => {
                // Check if any inner segment contains statement-level control flow or typescript injection
                fn has_stmt_level_control(
                    segments: &[Segment],
                    context_map: &HashMap<usize, PlaceholderUse>,
                ) -> bool {
                    for s in segments {
                        match s {
                            Segment::Control { id, .. } => {
                                if matches!(context_map.get(id), Some(PlaceholderUse::Stmt)) {
                                    return true;
                                }
                            }
                            // {$typescript} requires statement-level compilation
                            Segment::Typescript { .. } => {
                                return true;
                            }
                            Segment::BraceBlock { inner, .. } => {
                                if has_stmt_level_control(inner, context_map) {
                                    return true;
                                }
                            }
                            _ => {}
                        }
                    }
                    false
                }

                let has_stmt_control = has_stmt_level_control(inner, context_map);

                if has_stmt_control {
                    // For blocks with statement-level control, emit a placeholder
                    // that will be replaced with the compiled block body after parsing
                    let block_marker = format!("__mf_block_{}", id);
                    append_part(&mut template, &format!("{{ {}; }}", block_marker));
                    // Note: The BraceBlock is NOT added to bindings here.
                    // It will be compiled and substituted in flush_stmt_run.
                } else {
                    // For blocks without statement-level control, inline the content
                    append_part(&mut template, "{");
                    let inner_result = build_template_and_bindings(inner.iter(), context_map)?;
                    append_part(&mut template, &inner_result.template);
                    bindings.extend(inner_result.bindings);
                    type_placeholders.extend(inner_result.type_placeholders);
                    append_part(&mut template, "}");
                }
            }
            Segment::ObjectPropLoop {
                id,
                pat,
                iter,
                key_expr,
                value_expr,
            } => {
                // Generate an ObjectLit expression at Rust compile time.
                // The for loop iterates over `iter` with pattern `pat`, building
                // key-value properties from `key_expr` and `value_expr`.
                let name = placeholder_name(*id);
                append_part(&mut template, &format!("${name}"));

                if seen.insert(*id) {
                    // Generate Rust code that builds ObjectLit at macro expansion time
                    let obj_expr = quote! {{
                        use swc_core::ecma::ast::*;
                        use swc_core::common::DUMMY_SP;

                        let __mf_props: Vec<PropOrSpread> = (#iter).into_iter().map(|#pat| {
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(IdentName {
                                    span: DUMMY_SP,
                                    sym: (#key_expr).to_string().into(),
                                }),
                                value: Box::new(macroforge_ts_syn::to_ts_expr(#value_expr)),
                            })))
                        }).collect();

                        Expr::Object(ObjectLit {
                            span: DUMMY_SP,
                            props: __mf_props,
                        })
                    }};

                    let ty = placeholder_type_tokens(&PlaceholderUse::Expr);
                    let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                    bindings.push(BindingSpec {
                        name: bind_ident,
                        ty,
                        expr: obj_expr,
                    });
                }
            }
        }
    }

    Ok(TemplateAndBindings {
        template,
        bindings,
        type_placeholders,
    })
}
