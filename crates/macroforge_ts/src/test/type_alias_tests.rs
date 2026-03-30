use super::*;

// ==================== TYPE ALIAS TESTS ====================

#[test]
fn test_derive_debug_on_type_alias_generates_functions() {
    let source = r#"
/** @derive(Debug) */
type Point = {
    x: number;
    y: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Debug macro on type alias generates standalone pointToString function
        assert!(
            result.code.contains("pointToString"),
            "Should generate pointToString function for type"
        );
    });
}

#[test]
fn test_derive_clone_on_type_alias_generates_functions() {
    let source = r#"
/** @derive(Clone) */
type Config = {
    host: string;
    port: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Clone macro on type alias generates standalone configClone function
        assert!(
            result.code.contains("configClone"),
            "Should generate configClone function for type"
        );
    });
}

#[test]
fn test_derive_partial_eq_hash_on_type_alias_generates_functions() {
    let source = r#"
/** @derive(PartialEq, Hash) */
type Vector = {
    x: number;
    y: number;
    z: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // PartialEq and Hash macros on type alias generate standalone functions
        assert!(
            result.code.contains("vectorEquals"),
            "Should generate vectorEquals function for type"
        );
        assert!(
            result.code.contains("vectorHashCode"),
            "Should generate vectorHashCode function for type"
        );
    });
}

#[test]
fn test_derive_serialize_on_type_alias_generates_functions() {
    let source = r#"
/** @derive(Serialize) */
type User = {
    name: string;
    age: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Serialize macro on type alias generates standalone userSerialize function
        assert!(
            result.code.contains("userSerialize"),
            "Should generate userSerialize function for type"
        );

        // Verify no syntax context markers in output (would indicate Ident emission bug)
        assert!(
            !result.code.contains("#0"),
            "Output should not contain SWC syntax context markers like #0. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_deserialize_on_type_alias_generates_functions() {
    let source = r#"
/** @derive(Deserialize) */
type Settings = {
    theme: string;
    language: string;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Deserialize macro on type alias generates standalone settingsDeserialize function
        assert!(
            result.code.contains("settingsDeserialize"),
            "Should generate settingsDeserialize function for type"
        );
    });
}

#[test]
fn test_multiple_derives_on_type_alias() {
    let source = r#"
/** @derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize) */
type Coordinate = {
    lat: number;
    lng: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors, got {} errors",
            error_count
        );

        // All macros should generate standalone functions with prefix naming
        assert!(
            result.code.contains("coordinateToString"),
            "Should have Debug's coordinateToString"
        );
        assert!(
            result.code.contains("coordinateClone"),
            "Should have Clone's coordinateClone"
        );
        assert!(
            result.code.contains("coordinateEquals"),
            "Should have PartialEq's coordinateEquals"
        );
        assert!(
            result.code.contains("coordinateHashCode"),
            "Should have Hash's coordinateHashCode"
        );
        assert!(
            result.code.contains("coordinateSerialize"),
            "Should have Serialize's coordinateSerialize"
        );
        assert!(
            result.code.contains("coordinateDeserialize"),
            "Should have Deserialize's coordinateDeserialize"
        );
    });
}

#[test]
fn test_derive_on_union_type_alias() {
    let source = r#"
/** @derive(Debug, PartialEq, Hash) */
type Status = "active" | "inactive" | "pending";
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Macros should generate standalone functions for union type alias
        assert!(
            result.code.contains("statusToString"),
            "Should generate statusToString function for union type"
        );
        assert!(
            result.code.contains("statusEquals"),
            "Should generate statusEquals function for union type"
        );
        assert!(
            result.code.contains("statusHashCode"),
            "Should generate statusHashCode function for union type"
        );
    });
}

#[test]
fn test_derive_default_on_union_type_alias_with_explicit_default() {
    // Union types with @derive(Default) require @default to specify the default variant
    let source = r#"
/** @derive(Default) @default(VariantA.defaultValue()) */
export type UnionType = VariantA | VariantB;
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors with @default specified, got {}",
            error_count
        );

        // Should generate standalone unionTypeDefaultValue function
        assert!(
            result.code.contains("unionTypeDefaultValue"),
            "Should generate unionTypeDefaultValue function. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("VariantA.defaultValue()"),
            "Should call VariantA.defaultValue(). Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_default_on_union_type_alias_without_default_errors() {
    // Union types with @derive(Default) without @default should error
    let source = r#"
/** @derive(Default) */
export type UnionType = VariantA | VariantB;
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have an error diagnostic
        let error = result
            .diagnostics
            .iter()
            .find(|d| d.level == DiagnosticLevel::Error);

        assert!(
            error.is_some(),
            "Should produce an error when @default is missing for union type. Got diagnostics: {:?}",
            result.diagnostics
        );

        let error_msg = &error.unwrap().message;
        assert!(
            error_msg.contains("@default") || error_msg.contains("union"),
            "Error should mention @default or union. Got: {}",
            error_msg
        );
    });
}

#[test]
fn test_derive_default_on_multiline_union_type() {
    // Test multi-line JSDoc with @derive(Default) and @default on separate lines
    let source = r#"
/**
 * @derive(Default)
 * @default(TypeA.defaultValue())
 */
export type MultilineUnion = TypeA | TypeB | TypeC;
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors for multi-line JSDoc with @default. Got: {:?}",
            result.diagnostics
        );

        // Should generate standalone multilineUnionDefaultValue function
        assert!(
            result.code.contains("multilineUnionDefaultValue"),
            "Should generate multilineUnionDefaultValue function. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_default_with_default_on_union_variant() {
    // Test Rust-like syntax: @default decorator on the variant itself
    let source = r#"
/** @derive(Default) */
export type UnionWithVariantDefault =
  | /** @default */ VariantA
  | VariantB
  | VariantC;
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors with @default on variant. Got: {:?}",
            result.diagnostics
        );

        // Should generate standalone unionWithVariantDefaultDefaultValue function
        assert!(
            result.code.contains("unionWithVariantDefaultDefaultValue"),
            "Should generate unionWithVariantDefaultDefaultValue function. Got:\n{}",
            result.code
        );
        // With default prefix naming style, should call variantADefaultValue() not VariantA.defaultValue()
        assert!(
            result.code.contains("variantADefaultValue()"),
            "Should call variantADefaultValue(). Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_default_inline_union_without_leading_pipes() {
    // Test inline union without leading | for first variant (common format)
    // This matches: export type ActivityType = /** @default */ Created | Edited | Sent;
    let source = r#"
/** @derive(Default) */
export type ActivityType = /** @default */ Created | Edited | Sent;
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Debug: Print what we got
        eprintln!("[TEST] Expanded code:\n{}", result.code);
        eprintln!("[TEST] Diagnostics: {:?}", result.diagnostics);

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors with /** @default */ before first variant. Got: {:?}",
            result.diagnostics
        );

        // Should generate standalone activityTypeDefaultValue function
        assert!(
            result.code.contains("activityTypeDefaultValue"),
            "Should generate activityTypeDefaultValue function. Got:\n{}",
            result.code
        );
        // With default prefix naming style, should call createdDefaultValue() not Created.defaultValue()
        assert!(
            result.code.contains("createdDefaultValue()"),
            "Should call createdDefaultValue(). Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_default_inline_object_union_with_default() {
    // Regression test: union of inline object types with @default on first member.
    // This matches the PropValue pattern where all members are inline objects
    // (adjacently-tagged serde unions).
    let source = r#"
/** @derive(Default, Serialize, Deserialize) */
/** @serde({ tag: "type", content: "value" }) */
export type PropValue = /** @default */ { type: 'String'; value: string } | { type: 'Number'; value: number } | { type: 'Boolean'; value: boolean } | { type: 'Json'; value: string } | { type: 'Asset'; value: string } | { type: 'Page'; value: string } | { type: 'Expression'; value: string };
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors for inline object union with @default. Got: {:?}",
            result.diagnostics
        );

        assert!(
            result.code.contains("propValueDefaultValue"),
            "Should generate propValueDefaultValue function. Got:\n{}",
            result.code
        );

        assert!(
            result.code.contains("'String'") || result.code.contains("\"String\""),
            "Should contain 'String' literal value in default. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_default_inline_object_union_without_default() {
    // All-object union without @default: should fallback to first member
    let source = r#"
/** @derive(Default) */
export type Status = { kind: 'Active'; data: string } | { kind: 'Inactive'; reason: string };
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors for all-object union (fallback to first). Got: {:?}",
            result.diagnostics
        );

        assert!(
            result.code.contains("statusDefaultValue"),
            "Should generate statusDefaultValue function. Got:\n{}",
            result.code
        );

        assert!(
            result.code.contains("'Active'") || result.code.contains("\"Active\""),
            "Should contain 'Active' from first variant. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_default_inline_object_union_default_on_non_first() {
    // @default on a non-first object member
    let source = r#"
/** @derive(Default) */
export type PropValue = { type: 'String'; value: string } | /** @default */ { type: 'Number'; value: number };
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors with @default on non-first object member. Got: {:?}",
            result.diagnostics
        );

        assert!(
            result.code.contains("propValueDefaultValue"),
            "Should generate propValueDefaultValue function. Got:\n{}",
            result.code
        );

        // Should use Number variant, not String
        assert!(
            result.code.contains("'Number'") || result.code.contains("\"Number\""),
            "Should contain 'Number' from @default variant. Got:\n{}",
            result.code
        );
    });
}
