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
                    let block_marker = format!("__mf_hole_block_{}", id);
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

                        let __mf_hole_props: Vec<PropOrSpread> = (#iter).into_iter().map(|#pat| {
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
                            props: __mf_hole_props,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{CommentStyle, ControlNode, IdentPart, StringPart};
    use quote::quote;

    fn empty_context() -> HashMap<usize, PlaceholderUse> {
        HashMap::new()
    }

    #[test]
    fn test_empty_segments() {
        let segments: Vec<Segment> = vec![];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, "");
        assert!(result.bindings.is_empty());
        assert!(result.type_placeholders.is_empty());
    }

    #[test]
    fn test_static_only() {
        let segments = vec![
            Segment::Static("const x = ".to_string()),
            Segment::Static("42;".to_string()),
        ];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, "const x = 42;");
        assert!(result.bindings.is_empty());
    }

    #[test]
    fn test_comment_returns_error() {
        let segments = vec![Segment::Comment {
            style: CommentStyle::Line,
            text: "comment".to_string(),
        }];
        let result = build_template_and_bindings(segments, &empty_context());

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Comments must appear between TypeScript statements"));
        }
    }

    #[test]
    fn test_simple_interpolation() {
        let segments = vec![
            Segment::Static("x = ".to_string()),
            Segment::Interpolation {
                id: 0,
                expr: quote!(value),
            },
        ];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"x = $__mf_hole_0");
        assert_eq!(result.bindings.len(), 1);
        assert_eq!(result.bindings[0].name.to_string(), "__mf_hole_0");
    }

    #[test]
    fn test_interpolation_as_type() {
        let mut context = HashMap::new();
        context.insert(0, PlaceholderUse::Type);

        let segments = vec![Segment::Interpolation {
            id: 0,
            expr: quote!(MyType),
        }];
        let result = build_template_and_bindings(segments, &context).unwrap();

        // Type placeholders use marker instead of $ prefix
        assert_eq!(result.template, "__MfTypeMarker0");
        assert!(result.bindings.is_empty());
        assert_eq!(result.type_placeholders.len(), 1);
        assert_eq!(result.type_placeholders[0].id, 0);
    }

    #[test]
    fn test_interpolation_as_ident() {
        let mut context = HashMap::new();
        context.insert(0, PlaceholderUse::Ident);

        let segments = vec![Segment::Interpolation {
            id: 0,
            expr: quote!(name),
        }];
        let result = build_template_and_bindings(segments, &context).unwrap();

        assert_eq!(result.template, r"$__mf_hole_0");
        assert_eq!(result.bindings.len(), 1);
    }

    #[test]
    fn test_duplicate_interpolations_only_one_binding() {
        let segments = vec![
            Segment::Interpolation {
                id: 5,
                expr: quote!(x),
            },
            Segment::Static(" + ".to_string()),
            Segment::Interpolation {
                id: 5,
                expr: quote!(x),
            },
        ];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"$__mf_hole_5 + $__mf_hole_5");
        // Should only have one binding for the duplicate ID
        assert_eq!(result.bindings.len(), 1);
    }

    #[test]
    fn test_string_interp() {
        let segments = vec![Segment::StringInterp {
            id: 1,
            parts: vec![
                StringPart::Text("hello ".to_string()),
                StringPart::Expr(quote!(name)),
            ],
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"$__mf_hole_1");
        assert_eq!(result.bindings.len(), 1);
        assert_eq!(result.bindings[0].name.to_string(), "__mf_hole_1");
    }

    #[test]
    fn test_template_interp() {
        let segments = vec![Segment::TemplateInterp {
            id: 2,
            parts: vec![StringPart::Text("template".to_string())],
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"$__mf_hole_2");
        assert_eq!(result.bindings.len(), 1);
    }

    #[test]
    fn test_ident_block() {
        let segments = vec![Segment::IdentBlock {
            id: 3,
            parts: vec![
                IdentPart::Static("get".to_string()),
                IdentPart::Interpolation { expr: quote!(name) },
            ],
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"$__mf_hole_3");
        assert_eq!(result.bindings.len(), 1);
    }

    #[test]
    fn test_control_expr_context() {
        let segments = vec![Segment::Control {
            id: 4,
            node: ControlNode::If {
                cond: quote!(true),
                then_branch: vec![],
                else_branch: Some(vec![]),  // Expression-level if requires else branch
            },
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"$__mf_hole_4");
        assert_eq!(result.bindings.len(), 1);
    }

    #[test]
    fn test_control_stmt_context_returns_error() {
        let mut context = HashMap::new();
        context.insert(4, PlaceholderUse::Stmt);

        let segments = vec![Segment::Control {
            id: 4,
            node: ControlNode::If {
                cond: quote!(true),
                then_branch: vec![],
                else_branch: None,
            },
        }];
        let result = build_template_and_bindings(segments, &context);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains(r"Statement-level control blocks must stand alone"));
        }
    }

    #[test]
    fn test_control_ident_context_returns_error() {
        let mut context = HashMap::new();
        context.insert(4, PlaceholderUse::Ident);

        let segments = vec![Segment::Control {
            id: 4,
            node: ControlNode::If {
                cond: quote!(true),
                then_branch: vec![],
                else_branch: None,
            },
        }];
        let result = build_template_and_bindings(segments, &context);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains(r"Control blocks cannot appear in identifier/type positions"));
        }
    }

    #[test]
    fn test_control_type_context_returns_error() {
        let mut context = HashMap::new();
        context.insert(4, PlaceholderUse::Type);

        let segments = vec![Segment::Control {
            id: 4,
            node: ControlNode::If {
                cond: quote!(true),
                then_branch: vec![],
                else_branch: None,
            },
        }];
        let result = build_template_and_bindings(segments, &context);

        assert!(result.is_err());
    }

    #[test]
    fn test_typescript_segment_returns_error() {
        let segments = vec![Segment::Typescript {
            id: 5,
            expr: quote!(stream),
        }];
        let result = build_template_and_bindings(segments, &empty_context());

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("{$typescript} must be placed as its own statement"));
        }
    }

    #[test]
    fn test_let_segment_returns_error() {
        let segments = vec![Segment::Let {
            tokens: quote!(x = 5),
        }];
        let result = build_template_and_bindings(segments, &empty_context());

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("{$let} and {$do} cannot appear inside TypeScript expressions"));
        }
    }

    #[test]
    fn test_do_segment_returns_error() {
        let segments = vec![Segment::Do {
            expr: quote!(side_effect()),
        }];
        let result = build_template_and_bindings(segments, &empty_context());

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("{$let} and {$do} cannot appear inside TypeScript expressions"));
        }
    }

    #[test]
    fn test_brace_block_simple() {
        let segments = vec![Segment::BraceBlock {
            id: 6,
            inner: vec![Segment::Static("return 42".to_string())],
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, "{ return 42}");
        assert!(result.bindings.is_empty());
    }

    #[test]
    fn test_brace_block_with_interpolation() {
        let segments = vec![Segment::BraceBlock {
            id: 7,
            inner: vec![
                Segment::Static("return ".to_string()),
                Segment::Interpolation {
                    id: 8,
                    expr: quote!(value),
                },
            ],
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"{ return $__mf_hole_8}");
        assert_eq!(result.bindings.len(), 1);
    }

    #[test]
    fn test_nested_brace_blocks() {
        let segments = vec![Segment::BraceBlock {
            id: 9,
            inner: vec![Segment::BraceBlock {
                id: 10,
                inner: vec![Segment::Static("nested".to_string())],
            }],
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, "{ { nested}}");
    }

    #[test]
    fn test_brace_block_with_stmt_control() {
        let mut context = HashMap::new();
        context.insert(11, PlaceholderUse::Stmt);

        let segments = vec![Segment::BraceBlock {
            id: 10,
            inner: vec![Segment::Control {
                id: 11,
                node: ControlNode::For {
                    pat: quote!(item),
                    iter: quote!(items),
                    body: vec![],
                },
            }],
        }];
        let result = build_template_and_bindings(segments, &context).unwrap();

        // Should emit a block marker when containing stmt-level control
        assert!(result.template.contains("__mf_hole_block_10"));
        assert!(result.template.contains("{"));
        assert!(result.template.contains("}"));
    }

    #[test]
    fn test_brace_block_with_typescript() {
        let segments = vec![Segment::BraceBlock {
            id: 12,
            inner: vec![Segment::Typescript {
                id: 13,
                expr: quote!(stream),
            }],
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        // BraceBlock with typescript should emit block marker
        assert!(result.template.contains("__mf_hole_block_12"));
    }

    #[test]
    fn test_object_prop_loop() {
        let segments = vec![Segment::ObjectPropLoop {
            id: 14,
            pat: quote!((k, v)),
            iter: quote!(items),
            key_expr: quote!(k),
            value_expr: quote!(v),
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"$__mf_hole_14");
        assert_eq!(result.bindings.len(), 1);
        // Check that the binding contains ObjectLit construction
        let binding_str = result.bindings[0].expr.to_string();
        assert!(binding_str.contains("ObjectLit"));
        assert!(binding_str.contains("KeyValueProp"));
    }

    #[test]
    fn test_multiple_segment_types() {
        let segments = vec![
            Segment::Static("const x = ".to_string()),
            Segment::Interpolation {
                id: 15,
                expr: quote!(a),
            },
            Segment::Static(" + ".to_string()),
            Segment::StringInterp {
                id: 16,
                parts: vec![StringPart::Text("test".to_string())],
            },
        ];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"const x = $__mf_hole_15 + $__mf_hole_16");
        assert_eq!(result.bindings.len(), 2);
    }

    #[test]
    fn test_mixed_type_and_expr_placeholders() {
        let mut context = HashMap::new();
        context.insert(20, PlaceholderUse::Type);
        context.insert(21, PlaceholderUse::Expr);

        let segments = vec![
            Segment::Interpolation {
                id: 20,
                expr: quote!(TypeA),
            },
            Segment::Static(" | ".to_string()),
            Segment::Interpolation {
                id: 21,
                expr: quote!(value),
            },
        ];
        let result = build_template_and_bindings(segments, &context).unwrap();

        assert_eq!(result.template, r"__MfTypeMarker20 | $__mf_hole_21");
        assert_eq!(result.type_placeholders.len(), 1);
        assert_eq!(result.bindings.len(), 1);
    }

    #[test]
    fn test_control_for_loop() {
        let segments = vec![Segment::Control {
            id: 22,
            node: ControlNode::For {
                pat: quote!(item),
                iter: quote!(items),
                body: vec![Segment::Static("item".to_string())],
            },
        }];
        // For loops cannot be used in expression context
        let result = build_template_and_bindings(segments, &empty_context());
        assert!(result.is_err());
    }

    #[test]
    fn test_control_if_let() {
        let segments = vec![Segment::Control {
            id: 23,
            node: ControlNode::IfLet {
                pattern: quote!(Some(x)),
                expr: quote!(opt),
                then_branch: vec![],
                else_branch: Some(vec![]),  // Expression-level if-let requires else branch
            },
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"$__mf_hole_23");
        assert_eq!(result.bindings.len(), 1);
    }

    #[test]
    fn test_control_while() {
        let segments = vec![Segment::Control {
            id: 24,
            node: ControlNode::While {
                cond: quote!(true),
                body: vec![],
            },
        }];
        // While loops cannot be used in expression context
        let result = build_template_and_bindings(segments, &empty_context());
        assert!(result.is_err());
    }

    #[test]
    fn test_control_while_let() {
        let segments = vec![Segment::Control {
            id: 25,
            node: ControlNode::WhileLet {
                pattern: quote!(Some(x)),
                expr: quote!(iter.next()),
                body: vec![],
            },
        }];
        // While-let loops cannot be used in expression context
        let result = build_template_and_bindings(segments, &empty_context());
        assert!(result.is_err());
    }

    #[test]
    fn test_control_match() {
        use crate::template::MatchCase;

        let segments = vec![Segment::Control {
            id: 26,
            node: ControlNode::Match {
                expr: quote!(value),
                cases: vec![MatchCase {
                    pattern: quote!(Some(x)),
                    body: vec![],
                }],
            },
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"$__mf_hole_26");
        assert_eq!(result.bindings.len(), 1);
    }

    #[test]
    fn test_deeply_nested_brace_blocks_with_interpolation() {
        let segments = vec![Segment::BraceBlock {
            id: 27,
            inner: vec![Segment::BraceBlock {
                id: 28,
                inner: vec![
                    Segment::Static("x = ".to_string()),
                    Segment::Interpolation {
                        id: 29,
                        expr: quote!(val),
                    },
                ],
            }],
        }];
        let result = build_template_and_bindings(segments, &empty_context()).unwrap();

        assert_eq!(result.template, r"{ { x = $__mf_hole_29}}");
        assert_eq!(result.bindings.len(), 1);
    }
}
