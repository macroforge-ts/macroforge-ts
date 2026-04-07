use super::*;

#[test]
fn test_prepare_no_derive() {
    let source = "class User { name: string; }";
    let program = parse_module(source);
    let host = MacroExpander::new().unwrap();
    let result = host.prepare_expansion_context(&program, source).unwrap();
    // Even without decorators, we return Some because we still need to
    // generate method signatures for type output
    assert!(result.is_some());
}

#[test]
fn test_prepare_no_classes() {
    let source = "const x = 1;";
    let program = parse_module(source);
    let host = MacroExpander::new().unwrap();
    let result = host.prepare_expansion_context(&program, source).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_prepare_with_classes() {
    let source = "/** @derive(Debug) */ class User {}";
    let program = parse_module(source);
    let host = MacroExpander::new().unwrap();
    let result = host.prepare_expansion_context(&program, source).unwrap();
    assert!(result.is_some());
    let (_module, items) = result.unwrap();
    assert_eq!(items.classes.len(), 1);
    assert_eq!(items.classes[0].name, "User");
}

#[test]
fn test_process_macro_output_converts_tokens_into_patches() {
    GLOBALS.set(&Default::default(), || {
        let host = MacroExpander::new().unwrap();
        let class_ir = base_class("TokenDriven");
        let ctx = MacroContextIR::new_derive_class(
            "Debug".into(),
            DERIVE_MODULE_PATH.into(),
            SpanIR::new(0, 5),
            class_ir.span,
            "token.ts".into(),
            class_ir.clone(),
            "class TokenDriven {}".into(),
        );

        // Use body marker since default is now "below"
        let mut result = MacroResult {
            tokens: Some(
                r#"/* @macroforge:body */
                        toString() { return `${this.value}`; }
                        constructor(value: string) { this.value = value; }
                    "#
                .into(),
            ),
            ..Default::default()
        };

        let (runtime, type_patches) = host
            .process_macro_output(&mut result, &ctx, &ctx.target_source)
            .expect("tokens should parse");

        assert_eq!(
            runtime.len(),
            2,
            "expected one runtime patch per generated member"
        );
        assert_eq!(
            type_patches.len(),
            2,
            "expected one type patch per generated member"
        );

        for patch in runtime {
            match patch {
                Patch::Insert {
                    code: PatchCode::ClassMember(_),
                    ..
                } => {}
                other => panic!("expected class member insert, got {:?}", other),
            }
        }

        for patch in type_patches {
            if let Patch::Insert {
                code: PatchCode::ClassMember(member),
                ..
            } = patch
            {
                match member {
                    ClassMember::Method(method) => assert!(
                        method.function.body.is_none(),
                        "type patch should strip method body"
                    ),
                    ClassMember::Constructor(cons) => assert!(
                        cons.body.is_none(),
                        "type patch should drop constructor body"
                    ),
                    _ => {}
                }
            } else {
                panic!("expected type patch insert");
            }
        }
    });
}

#[test]
fn test_process_macro_output_reports_parse_errors() {
    GLOBALS.set(&Default::default(), || {
        let host = MacroExpander::new().unwrap();
        let class_ir = base_class("Broken");
        let ctx = MacroContextIR::new_derive_class(
            "Debug".into(),
            DERIVE_MODULE_PATH.into(),
            SpanIR::new(0, 5),
            class_ir.span,
            "broken.ts".into(),
            class_ir.clone(),
            "class Broken {}".into(),
        );

        // Use body marker to trigger class member parsing, which will fail
        let mut result = MacroResult {
            tokens: Some("/* @macroforge:body */this is not valid class member syntax".into()),
            ..Default::default()
        };

        let (_runtime, _types) = host
            .process_macro_output(&mut result, &ctx, &ctx.target_source)
            .expect("process_macro_output should succeed with raw insertion fallback");

        let diag = result
            .diagnostics
            .iter()
            .find(|d| d.message.contains("Failed to parse macro output"))
            .expect("diagnostic should mention parsing failure");
        assert_eq!(diag.level, DiagnosticLevel::Warning);
    });
}

#[test]
fn test_collect_constructor_patch() {
    // Test that a class WITH @derive gets constructor body stripping patches
    let source = "/** @derive(Debug) */ class User { constructor(id: string) { this.id = id; } }";
    let program = parse_module(source);
    let host = MacroExpander::new().unwrap();
    let (module, items) = host
        .prepare_expansion_context(&program, source)
        .unwrap()
        .unwrap();

    let (collector, _) = host.collect_macro_patches(&module, items, "test.ts", source);

    let type_patches = collector.get_type_patches();
    // Should have at least: decorator removal + constructor body stripping + generated methods
    assert!(
        type_patches.len() >= 2,
        "Expected at least 2 patches, got {}",
        type_patches.len()
    );

    // Find the constructor patch
    let constructor_patch = type_patches.iter().find(|p| {
        if let Patch::Replace {
            code: PatchCode::Text(text),
            ..
        } = p
        {
            text.contains("constructor")
        } else {
            false
        }
    });

    assert!(
        constructor_patch.is_some(),
        "Should have a constructor patch"
    );
    if let Some(Patch::Replace { code, .. }) = constructor_patch {
        match code {
            PatchCode::Text(text) => assert_eq!(text, "constructor(id: string);"),
            _ => panic!("Expected textual patch for constructor signature"),
        }
    }
}

#[test]
fn test_no_patches_for_class_without_derive() {
    // Test that a class WITHOUT @derive gets NO patches (fixes .svelte.ts rune issue)
    let source = "class User { constructor(id: string) { this.id = id; } }";
    let program = parse_module(source);
    let host = MacroExpander::new().unwrap();
    let (module, items) = host
        .prepare_expansion_context(&program, source)
        .unwrap()
        .unwrap();

    let (collector, _) = host.collect_macro_patches(&module, items, "test.ts", source);

    let type_patches = collector.get_type_patches();
    assert_eq!(
        type_patches.len(),
        0,
        "Class without @derive should get no type patches"
    );

    // Also verify that has_type_patches is false
    assert!(
        !collector.has_type_patches(),
        "Class without @derive should have no type patches"
    );
}

#[test]
fn test_collect_derive_debug_patch() {
    let source = "/** @derive(Debug) */ class User { name: string; }";
    let program = parse_module(source);
    let host = MacroExpander::new().unwrap();
    let (module, items) = host
        .prepare_expansion_context(&program, source)
        .unwrap()
        .unwrap();
    let (collector, _) = host.collect_macro_patches(&module, items, "test.ts", source);

    let type_patches = collector.get_type_patches();

    // Expecting 3 patches:
    // 1. Decorator removal for /** @derive(Debug) */
    // 2. Static toString method insertion inside class (from Derive(Debug) macro)
    // 3. Standalone function userToString insertion after class
    assert_eq!(
        type_patches.len(),
        3,
        "Expected 3 patches, got {}",
        type_patches.len()
    );

    // check for decorator deletion
    assert!(
        type_patches
            .iter()
            .any(|p| matches!(p, Patch::Delete { .. }))
    );
    // check for method signature insertion
    assert!(
        type_patches
            .iter()
            .any(|p| matches!(p, Patch::Insert { .. }))
    );
}

#[test]
fn test_apply_and_finalize_expansion_no_type_patches() {
    let source = "class User {}";
    let mut collector = PatchCollector::new();
    let mut diagnostics = Vec::new();
    let host = MacroExpander::new().unwrap();
    let result = host
        .apply_and_finalize_expansion(
            source,
            &mut collector,
            &mut diagnostics,
            crate::host::expand::LoweredItems {
                classes: Vec::new(),
                interfaces: Vec::new(),
                enums: Vec::new(),
                type_aliases: Vec::new(),
                imports: crate::host::import_registry::ImportRegistry::new(),
            },
        )
        .unwrap();
    assert!(result.type_output.is_none());
}

#[test]
fn test_default_below_location_for_unmarked_tokens() {
    GLOBALS.set(&Default::default(), || {
        let host = MacroExpander::new().unwrap();
        let class_ir = base_class("BelowTest");
        let ctx = MacroContextIR::new_derive_class(
            "Custom".into(),
            DERIVE_MODULE_PATH.into(),
            SpanIR::new(0, 5),
            class_ir.span,
            "below.ts".into(),
            class_ir.clone(),
            "class BelowTest {}".into(),
        );

        // Tokens without a marker should default to "below" location
        let mut result = MacroResult {
            tokens: Some("const helper = () => {};".into()),
            ..Default::default()
        };

        let (runtime, type_patches) = host
            .process_macro_output(&mut result, &ctx, &ctx.target_source)
            .expect("unmarked tokens should be inserted as text below class");

        // Should have 1 runtime and 1 type patch for "below" insertion
        assert_eq!(runtime.len(), 1, "should have 1 runtime patch for below");
        assert_eq!(type_patches.len(), 1, "should have 1 type patch for below");

        // Verify it's a text patch, not a class member patch
        for patch in runtime {
            match patch {
                Patch::Insert {
                    code: PatchCode::Text(_),
                    at,
                    ..
                } => {
                    // Below location should insert at class span end
                    assert_eq!(at.start, class_ir.span.end, "should insert at class end");
                }
                other => panic!("expected text insert for below, got {:?}", other),
            }
        }
    });
}

#[test]
fn test_explicit_body_marker_parses_as_class_members() {
    GLOBALS.set(&Default::default(), || {
        let host = MacroExpander::new().unwrap();
        let class_ir = base_class("BodyTest");
        let ctx = MacroContextIR::new_derive_class(
            "Custom".into(),
            DERIVE_MODULE_PATH.into(),
            SpanIR::new(0, 5),
            class_ir.span,
            "body.ts".into(),
            class_ir.clone(),
            "class BodyTest {}".into(),
        );

        // Tokens WITH body marker should be parsed as class members
        let mut result = MacroResult {
            tokens: Some("/* @macroforge:body */getValue(): string { return \"test\"; }".into()),
            ..Default::default()
        };

        let (runtime, type_patches) = host
            .process_macro_output(&mut result, &ctx, &ctx.target_source)
            .expect("body-marked tokens should parse as class members");

        // Should have class member patches
        assert_eq!(runtime.len(), 1, "should have 1 runtime patch for body");
        assert_eq!(type_patches.len(), 1, "should have 1 type patch for body");

        for patch in runtime {
            match patch {
                Patch::Insert {
                    code: PatchCode::ClassMember(_),
                    ..
                } => {}
                other => panic!("expected class member insert for body, got {:?}", other),
            }
        }
    });
}
