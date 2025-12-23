use std::collections::HashMap;

use crate::template::{append_part, placeholder_name, ControlNode, Segment};

/// Placeholder source kind for parsing.
#[derive(Clone, Copy)]
pub enum PlaceholderSourceKind {
    Module,
    Expr,
}

/// Checks if a branch contains statement-level content.
///
/// Returns true if the branch contains statement-level constructs (loops, TS injections, etc.)
/// or any static segment with a semicolon. Returns false for inline expression-only content.
fn is_statement_level_branch(branch: &[Segment]) -> bool {
    fn control_has_statement_level(node: &ControlNode) -> bool {
        match node {
            ControlNode::For { .. }
            | ControlNode::While { .. }
            | ControlNode::WhileLet { .. } => true,
            ControlNode::If {
                then_branch,
                else_branch,
                ..
            }
            | ControlNode::IfLet {
                then_branch,
                else_branch,
                ..
            } => {
                is_statement_level_branch(then_branch)
                    || else_branch
                        .as_ref()
                        .is_some_and(|b| is_statement_level_branch(b))
            }
            ControlNode::Match { cases, .. } => {
                cases.iter().any(|c| is_statement_level_branch(&c.body))
            }
        }
    }

    for seg in branch {
        match seg {
            Segment::Static(s) => {
                let trimmed = s.trim();
                if !trimmed.is_empty() && trimmed.contains(';') {
                    return true;
                }
            }
            Segment::Control { node, .. } => {
                if control_has_statement_level(node) {
                    return true;
                }
            }
            Segment::Typescript { .. } | Segment::Let { .. } | Segment::Do { .. } => {
                return true;
            }
            Segment::BraceBlock { inner, .. } => {
                if is_statement_level_branch(inner) {
                    return true;
                }
            }
            _ => {}
        }
    }

    // Empty or inline-only = expression-level
    false
}

/// Builds a placeholder-only source string and placeholder ID map.
pub fn build_placeholder_source(
    segments: &[Segment],
    kind: PlaceholderSourceKind,
) -> (String, HashMap<String, usize>) {
    let mut src = String::new();
    let mut map = HashMap::new();

    for seg in segments {
        match seg {
            Segment::Static(s) => append_part(&mut src, s),
            Segment::Comment { .. } => {}
            Segment::Interpolation { id, .. }
            | Segment::StringInterp { id, .. }
            | Segment::TemplateInterp { id, .. }
            | Segment::IdentBlock { id, .. }
            | Segment::Typescript { id, .. }
            | Segment::ObjectPropLoop { id, .. } => {
                let name = placeholder_name(*id);
                append_part(&mut src, &name);
                if matches!(kind, PlaceholderSourceKind::Module)
                    && matches!(seg, Segment::Typescript { .. })
                {
                    src.push(';');
                }
                map.insert(name, *id);
            }
            Segment::Control { id, node, .. } => {
                // Determine if control is statement-level based on its content.
                //
                // A control is statement-level if:
                // - It's a loop (For/While/WhileLet), OR
                // - Its branch contains statement-level content (ends with `;`, has Typescript, etc.)
                //
                // Otherwise, it's inline and we recurse into branches for classification.
                match node {
                    ControlNode::For { .. }
                    | ControlNode::While { .. }
                    | ControlNode::WhileLet { .. } => {
                        // Statement-level loops: emit placeholder for classification
                        let name = placeholder_name(*id);
                        append_part(&mut src, &name);
                        if matches!(kind, PlaceholderSourceKind::Module) {
                            src.push(';');
                        }
                        map.insert(name, *id);
                    }
                    ControlNode::If {
                        then_branch,
                        else_branch,
                        ..
                    }
                    | ControlNode::IfLet {
                        then_branch,
                        else_branch,
                        ..
                    } => {
                        // Statement-level if:
                        // - No else branch (expressions need both branches for a value)
                        // - Branch contains statement-level content (ends with `;`, has Typescript, etc.)
                        let is_statement_level = else_branch.is_none()
                            || is_statement_level_branch(then_branch)
                            || else_branch
                                .as_ref()
                                .is_some_and(|b| is_statement_level_branch(b));

                        if is_statement_level {
                            // Statement-level: emit placeholder for classification
                            let name = placeholder_name(*id);
                            append_part(&mut src, &name);
                            if matches!(kind, PlaceholderSourceKind::Module) {
                                src.push(';');
                            }
                            map.insert(name, *id);
                        } else {
                            // Inline conditional: recursively process branches
                            let (ctrl_src, ctrl_map) = build_control_placeholder_source(node, kind);
                            if !ctrl_src.is_empty() {
                                append_part(&mut src, &ctrl_src);
                                map.extend(ctrl_map);
                            }
                        }
                    }
                    ControlNode::Match { cases, .. } => {
                        // Statement-level if any case has statement content
                        if cases.iter().any(|c| is_statement_level_branch(&c.body)) {
                            let name = placeholder_name(*id);
                            append_part(&mut src, &name);
                            if matches!(kind, PlaceholderSourceKind::Module) {
                                src.push(';');
                            }
                            map.insert(name, *id);
                        } else {
                            let (ctrl_src, ctrl_map) = build_control_placeholder_source(node, kind);
                            if !ctrl_src.is_empty() {
                                append_part(&mut src, &ctrl_src);
                                map.extend(ctrl_map);
                            }
                        }
                    }
                }
            }
            Segment::Let { .. } | Segment::Do { .. } => {
                // Rust-only constructs are ignored for TS parsing.
            }
            Segment::BraceBlock { inner, .. } => {
                // Recursively process inner segments with braces
                append_part(&mut src, "{");
                let (inner_src, inner_map) = build_placeholder_source(inner, kind);
                append_part(&mut src, &inner_src);
                map.extend(inner_map);
                append_part(&mut src, "}");
            }
        }
    }

    (src, map)
}

/// Build placeholder source for a control node by recursively processing its branches.
fn build_control_placeholder_source(
    node: &ControlNode,
    kind: PlaceholderSourceKind,
) -> (String, HashMap<String, usize>) {
    match node {
        ControlNode::If {
            then_branch,
            else_branch,
            ..
        }
        | ControlNode::IfLet {
            then_branch,
            else_branch,
            ..
        } => {
            // Try then_branch first, fall back to else_branch if then is empty
            let (then_src, then_map) = build_placeholder_source(then_branch, kind);
            if !then_src.trim().is_empty() {
                return (then_src, then_map);
            }
            if let Some(else_segments) = else_branch {
                return build_placeholder_source(else_segments, kind);
            }
            (String::new(), HashMap::new())
        }
        ControlNode::For { body, .. }
        | ControlNode::While { body, .. }
        | ControlNode::WhileLet { body, .. } => {
            // For loops, process the body
            build_placeholder_source(body, kind)
        }
        ControlNode::Match { cases, .. } => {
            // For match, try to find the first case with non-empty content
            for case in cases {
                let (case_src, case_map) = build_placeholder_source(&case.body, kind);
                if !case_src.trim().is_empty() {
                    return (case_src, case_map);
                }
            }
            (String::new(), HashMap::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::ControlNode;
    use proc_macro2::TokenStream as TokenStream2;

    // ========================================
    // Basic functionality tests
    // ========================================

    #[test]
    fn test_empty_segments() {
        let (src, map) = build_placeholder_source(&[], PlaceholderSourceKind::Module);
        assert_eq!(src, "");
        assert!(map.is_empty());
    }

    #[test]
    fn test_static_only() {
        let segments = vec![Segment::Static("const x = 1;".to_string())];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert_eq!(src, "const x = 1;");
        assert!(map.is_empty());
    }

    #[test]
    fn test_multiple_statics() {
        let segments = vec![
            Segment::Static("const".to_string()),
            Segment::Static("x".to_string()),
            Segment::Static("=".to_string()),
            Segment::Static("1".to_string()),
        ];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        // append_part adds spaces between parts
        assert!(src.contains("const"));
        assert!(src.contains("x"));
        assert!(map.is_empty());
    }

    // ========================================
    // Interpolation tests
    // ========================================

    #[test]
    fn test_single_interpolation() {
        let segments = vec![Segment::Interpolation {
            id: 0,
            expr: TokenStream2::new(),
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_0"));
        assert_eq!(map.get("__mf_hole_0"), Some(&0));
    }

    #[test]
    fn test_multiple_interpolations() {
        let segments = vec![
            Segment::Interpolation {
                id: 0,
                expr: TokenStream2::new(),
            },
            Segment::Interpolation {
                id: 1,
                expr: TokenStream2::new(),
            },
            Segment::Interpolation {
                id: 2,
                expr: TokenStream2::new(),
            },
        ];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_0"));
        assert!(src.contains("__mf_hole_1"));
        assert!(src.contains("__mf_hole_2"));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_interpolation_with_static() {
        let segments = vec![
            Segment::Static("const x = ".to_string()),
            Segment::Interpolation {
                id: 0,
                expr: TokenStream2::new(),
            },
            Segment::Static(";".to_string()),
        ];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("const x ="));
        assert!(src.contains("__mf_hole_0"));
        assert!(src.ends_with(";"));
        assert_eq!(map.len(), 1);
    }

    // ========================================
    // Module vs Expr mode tests
    // ========================================

    #[test]
    fn test_control_without_else_is_statement_level() {
        // Control without else branch is always statement-level (emits placeholder)
        let segments = vec![Segment::Control {
            id: 0,
            node: ControlNode::If {
                cond: TokenStream2::new(),
                then_branch: vec![],
                else_branch: None,
            },
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        // No else = statement-level, emits placeholder for classification
        assert!(src.contains("__mf_hole_0"));
        assert_eq!(map.get("__mf_hole_0"), Some(&0));
    }

    #[test]
    fn test_control_without_else_emits_placeholder() {
        // Control without else branch emits placeholder (statement-level)
        // even if then_branch has inline-looking content
        let segments = vec![Segment::Control {
            id: 0,
            node: ControlNode::If {
                cond: TokenStream2::new(),
                then_branch: vec![
                    Segment::Static(" as ".to_string()),
                    Segment::Interpolation {
                        id: 1,
                        expr: TokenStream2::new(),
                    },
                ],
                else_branch: None,
            },
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        // No else = statement-level, emits placeholder for the control itself
        assert!(src.contains("__mf_hole_0"));
        assert_eq!(map.get("__mf_hole_0"), Some(&0));
        // Branch content is NOT directly in the source (processed separately during compilation)
        assert!(!src.contains("__mf_hole_1"));
    }

    #[test]
    fn test_control_with_else_recursively_processes() {
        // Control WITH else branch and inline content recurses into branches
        let segments = vec![Segment::Control {
            id: 0,
            node: ControlNode::If {
                cond: TokenStream2::new(),
                then_branch: vec![
                    Segment::Static(" as ".to_string()),
                    Segment::Interpolation {
                        id: 1,
                        expr: TokenStream2::new(),
                    },
                ],
                else_branch: Some(vec![Segment::Interpolation {
                    id: 2,
                    expr: TokenStream2::new(),
                }]),
            },
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        // With else and inline content = expression-level, recurses into then_branch
        assert!(src.contains("as"));
        assert!(src.contains("__mf_hole_1"));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("__mf_hole_1"), Some(&1));
    }

    #[test]
    fn test_control_with_else_statement_content_emits_placeholder() {
        // If any branch contains statement-level content, emit a placeholder for the control.
        let segments = vec![Segment::Control {
            id: 0,
            node: ControlNode::If {
                cond: TokenStream2::new(),
                then_branch: vec![Segment::Interpolation {
                    id: 1,
                    expr: TokenStream2::new(),
                }],
                else_branch: Some(vec![Segment::Control {
                    id: 2,
                    node: ControlNode::For {
                        pat: TokenStream2::new(),
                        iter: TokenStream2::new(),
                        body: vec![],
                    },
                }]),
            },
        }];

        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_0"));
        assert_eq!(map.get("__mf_hole_0"), Some(&0));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_control_with_else_and_semicolon_in_branch_is_statement_level() {
        // Semicolons anywhere in a branch should mark it statement-level.
        let segments = vec![Segment::Control {
            id: 0,
            node: ControlNode::If {
                cond: TokenStream2::new(),
                then_branch: vec![
                    Segment::Static("const x = 1;".to_string()),
                    Segment::Interpolation {
                        id: 1,
                        expr: TokenStream2::new(),
                    },
                ],
                else_branch: Some(vec![Segment::Interpolation {
                    id: 2,
                    expr: TokenStream2::new(),
                }]),
            },
        }];

        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_0"));
        assert_eq!(map.get("__mf_hole_0"), Some(&0));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_control_falls_back_to_else_branch() {
        // If then_branch is empty, should use else_branch
        let segments = vec![Segment::Control {
            id: 0,
            node: ControlNode::If {
                cond: TokenStream2::new(),
                then_branch: vec![],
                else_branch: Some(vec![Segment::Interpolation {
                    id: 2,
                    expr: TokenStream2::new(),
                }]),
            },
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_2"));
        assert_eq!(map.get("__mf_hole_2"), Some(&2));
    }

    #[test]
    fn test_control_for_loop_emits_placeholder() {
        // For loops are statement-level and emit a placeholder for classification
        let segments = vec![Segment::Control {
            id: 0,
            node: ControlNode::For {
                pat: TokenStream2::new(),
                iter: TokenStream2::new(),
                body: vec![Segment::Interpolation {
                    id: 3,
                    expr: TokenStream2::new(),
                }],
            },
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        // For loop emits its own placeholder, not the body content
        assert!(src.contains("__mf_hole_0;"));
        assert_eq!(map.get("__mf_hole_0"), Some(&0));
        // Body content is not directly in the source
        assert!(!map.contains_key("__mf_hole_3"));
    }

    #[test]
    fn test_typescript_in_module_mode_adds_semicolon() {
        let segments = vec![Segment::Typescript {
            id: 0,
            expr: TokenStream2::new(),
        }];
        let (src, _map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_0;"));
    }

    #[test]
    fn test_typescript_in_expr_mode_no_semicolon() {
        let segments = vec![Segment::Typescript {
            id: 0,
            expr: TokenStream2::new(),
        }];
        let (src, _map) = build_placeholder_source(&segments, PlaceholderSourceKind::Expr);
        assert!(src.contains("__mf_hole_0"));
        assert!(!src.contains("__mf_hole_0;"));
    }

    // ========================================
    // Comment handling tests
    // ========================================

    #[test]
    fn test_comments_are_ignored() {
        let segments = vec![
            Segment::Static("const x = ".to_string()),
            Segment::Comment {
                style: crate::template::CommentStyle::Line,
                text: "this is a comment".to_string(),
            },
            Segment::Interpolation {
                id: 0,
                expr: TokenStream2::new(),
            },
        ];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        // Comment should not appear in source
        assert!(!src.contains("comment"));
        // But interpolation should
        assert!(src.contains("__mf_hole_0"));
        assert_eq!(map.len(), 1);
    }

    // ========================================
    // Let/Do handling tests
    // ========================================

    #[test]
    fn test_let_is_ignored() {
        let segments = vec![
            Segment::Let {
                tokens: TokenStream2::new(),
            },
            Segment::Static("const x = 1;".to_string()),
        ];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("const x = 1;"));
        assert!(map.is_empty());
    }

    #[test]
    fn test_do_is_ignored() {
        let segments = vec![
            Segment::Do {
                expr: TokenStream2::new(),
            },
            Segment::Static("const x = 1;".to_string()),
        ];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("const x = 1;"));
        assert!(map.is_empty());
    }

    // ========================================
    // BraceBlock tests
    // ========================================

    #[test]
    fn test_brace_block_empty() {
        let segments = vec![Segment::BraceBlock {
            id: 0,
            inner: vec![],
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("{"));
        assert!(src.contains("}"));
        assert!(map.is_empty());
    }

    #[test]
    fn test_brace_block_with_static() {
        let segments = vec![Segment::BraceBlock {
            id: 0,
            inner: vec![Segment::Static("a: 1".to_string())],
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("{"));
        assert!(src.contains("a: 1"));
        assert!(src.contains("}"));
        assert!(map.is_empty());
    }

    #[test]
    fn test_brace_block_with_interpolation() {
        let segments = vec![Segment::BraceBlock {
            id: 0,
            inner: vec![Segment::Interpolation {
                id: 5,
                expr: TokenStream2::new(),
            }],
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("{"));
        assert!(src.contains("__mf_hole_5"));
        assert!(src.contains("}"));
        assert_eq!(map.get("__mf_hole_5"), Some(&5));
    }

    #[test]
    fn test_nested_brace_blocks() {
        let segments = vec![Segment::BraceBlock {
            id: 0,
            inner: vec![Segment::BraceBlock {
                id: 1,
                inner: vec![Segment::Interpolation {
                    id: 10,
                    expr: TokenStream2::new(),
                }],
            }],
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        // Should have nested braces (append_part adds space between braces)
        assert!(src.contains("{"));
        assert!(src.contains("}"));
        assert!(src.contains("__mf_hole_10"));
        assert_eq!(map.len(), 1);
        // Note: The braces have spaces between them due to append_part spacing rules
        // This is intentional - "{ {" instead of "{{"
    }

    // ========================================
    // StringInterp/TemplateInterp/IdentBlock tests
    // ========================================

    #[test]
    fn test_string_interp() {
        let segments = vec![Segment::StringInterp {
            id: 3,
            parts: vec![],
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_3"));
        assert_eq!(map.get("__mf_hole_3"), Some(&3));
    }

    #[test]
    fn test_template_interp() {
        let segments = vec![Segment::TemplateInterp {
            id: 4,
            parts: vec![],
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_4"));
        assert_eq!(map.get("__mf_hole_4"), Some(&4));
    }

    #[test]
    fn test_ident_block() {
        let segments = vec![Segment::IdentBlock {
            id: 6,
            parts: vec![],
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_6"));
        assert_eq!(map.get("__mf_hole_6"), Some(&6));
    }

    #[test]
    fn test_object_prop_loop() {
        let segments = vec![Segment::ObjectPropLoop {
            id: 7,
            pat: TokenStream2::new(),
            iter: TokenStream2::new(),
            key_expr: TokenStream2::new(),
            value_expr: TokenStream2::new(),
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_7"));
        assert_eq!(map.get("__mf_hole_7"), Some(&7));
    }

    // ========================================
    // Edge cases and stress tests
    // ========================================

    #[test]
    fn test_large_id() {
        let segments = vec![Segment::Interpolation {
            id: 999999,
            expr: TokenStream2::new(),
        }];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("__mf_hole_999999"));
        assert_eq!(map.get("__mf_hole_999999"), Some(&999999));
    }

    #[test]
    fn test_duplicate_ids() {
        // Same ID appearing multiple times - map should have latest
        let segments = vec![
            Segment::Interpolation {
                id: 0,
                expr: TokenStream2::new(),
            },
            Segment::Interpolation {
                id: 0,
                expr: TokenStream2::new(),
            },
        ];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        // Both should appear in source
        let count = src.matches("__mf_hole_0").count();
        assert_eq!(count, 2);
        // Map only has one entry per name
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_mixed_segment_types() {
        let segments = vec![
            Segment::Static("const obj = ".to_string()),
            Segment::BraceBlock {
                id: 0,
                inner: vec![
                    Segment::Static("a: ".to_string()),
                    Segment::Interpolation {
                        id: 1,
                        expr: TokenStream2::new(),
                    },
                    Segment::Static(", b: ".to_string()),
                    Segment::StringInterp {
                        id: 2,
                        parts: vec![],
                    },
                ],
            },
            Segment::Static(";".to_string()),
        ];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
        assert!(src.contains("const obj ="));
        assert!(src.contains("{"));
        assert!(src.contains("__mf_hole_1"));
        assert!(src.contains("__mf_hole_2"));
        assert!(src.contains("}"));
        assert!(src.ends_with(";"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_all_segment_types_together() {
        let segments = vec![
            Segment::Static("start ".to_string()),
            Segment::Interpolation {
                id: 0,
                expr: TokenStream2::new(),
            },
            Segment::StringInterp {
                id: 1,
                parts: vec![],
            },
            Segment::TemplateInterp {
                id: 2,
                parts: vec![],
            },
            Segment::IdentBlock {
                id: 3,
                parts: vec![],
            },
            // Control WITH else branch - recursively includes the then_branch content
            Segment::Control {
                id: 100, // Control id not used in output when inline (has else)
                node: ControlNode::If {
                    cond: TokenStream2::new(),
                    then_branch: vec![Segment::Interpolation {
                        id: 4,
                        expr: TokenStream2::new(),
                    }],
                    else_branch: Some(vec![Segment::Interpolation {
                        id: 44,
                        expr: TokenStream2::new(),
                    }]),
                },
            },
            Segment::Typescript {
                id: 5,
                expr: TokenStream2::new(),
            },
            Segment::ObjectPropLoop {
                id: 6,
                pat: TokenStream2::new(),
                iter: TokenStream2::new(),
                key_expr: TokenStream2::new(),
                value_expr: TokenStream2::new(),
            },
            Segment::Comment {
                style: crate::template::CommentStyle::Line,
                text: "ignored".to_string(),
            },
            Segment::Let {
                tokens: TokenStream2::new(),
            },
            Segment::Do {
                expr: TokenStream2::new(),
            },
            Segment::BraceBlock {
                id: 8,
                inner: vec![Segment::Interpolation {
                    id: 9,
                    expr: TokenStream2::new(),
                }],
            },
            Segment::Static(" end".to_string()),
        ];
        let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);

        // Verify all non-ignored segments are present
        assert!(src.contains("__mf_hole_0"));
        assert!(src.contains("__mf_hole_1"));
        assert!(src.contains("__mf_hole_2"));
        assert!(src.contains("__mf_hole_3"));
        assert!(src.contains("__mf_hole_4")); // From Control's then_branch
        assert!(src.contains("__mf_hole_5;")); // Typescript has semicolon in module mode
        assert!(src.contains("__mf_hole_6"));
        assert!(src.contains("__mf_hole_9")); // Inside BraceBlock

        // Comment has no id, Let, and Do should NOT produce placeholders
        // BraceBlock id (8) should not be in map - only its contents
        // Control id (100) should not be in map - only its branch content
        assert!(!map.contains_key("__mf_hole_8"));
        assert!(!map.contains_key("__mf_hole_100"));

        // Total: 0, 1, 2, 3, 4, 5, 6, 9 = 8 entries
        assert_eq!(map.len(), 8);
    }
}
