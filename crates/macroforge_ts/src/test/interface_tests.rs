use super::*;

// ============================================================================
// Interface Derive Macro Tests
// ============================================================================

#[test]
fn test_derive_debug_on_interface_generates_namespace() {
    // Debug macro now works on interfaces, generating a namespace with toString function
    let source = r#"
/** @derive(Debug) */
interface Status {
    active: boolean;
}
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

        // Output should contain prefix-style toString function (default naming style)
        assert!(
            result.code.contains("statusToString"),
            "Should generate prefix-style statusToString function. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_clone_on_interface_generates_functions() {
    let source = r#"
/** @derive(Clone) */
interface UserData {
    name: string;
    age: number;
}
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

        // Output should contain prefix-style clone function (default naming style)
        assert!(
            result.code.contains("userDataClone"),
            "Should generate prefix-style userDataClone function"
        );
    });
}

#[test]
fn test_derive_partial_eq_hash_on_interface_generates_functions() {
    let source = r#"
/** @derive(PartialEq, Hash) */
interface Point {
    x: number;
    y: number;
}
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

        // Output should contain prefix-style equals and hashCode functions (default naming style)
        assert!(
            result.code.contains("pointEquals"),
            "Should generate prefix-style pointEquals function"
        );
        assert!(
            result.code.contains("pointHashCode"),
            "Should generate prefix-style pointHashCode function"
        );
    });
}

#[test]
fn test_derive_debug_on_interface_generates_correct_output() {
    // Test that the generated Debug output is correct for interfaces
    let source = r#"
/** @derive(Debug) */
interface Status {
    active: boolean;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no errors
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should succeed without errors");

        // Verify the output contains expected patterns (suffix-style uses value parameter)
        eprintln!("[DEBUG] result.code = {:?}", result.code);
        assert!(
            result.code.contains("value.active"),
            "Should reference value.active"
        );
    });
}

#[test]
fn test_multiple_derives_on_interface_all_succeed() {
    // When multiple derives are used on interface, all should succeed
    let source = r#"
/** @derive(Debug, Clone, PartialEq, Hash) */
interface Status {
    active: boolean;
}
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
            "Should have no errors for derives on interface, got {} errors",
            error_count
        );

        // Should have all prefix-style functions generated
        assert!(
            result.code.contains("statusToString"),
            "Should have Debug's statusToString"
        );
        assert!(
            result.code.contains("statusClone"),
            "Should have Clone's statusClone"
        );
        assert!(
            result.code.contains("statusEquals"),
            "Should have PartialEq's statusEquals"
        );
        assert!(
            result.code.contains("statusHashCode"),
            "Should have Hash's statusHashCode"
        );
    });
}

#[test]
fn test_unknown_derive_macro_produces_error() {
    // A derive macro that doesn't exist should produce an error
    let source = r#"
/** @derive(NonExistentMacro) */
class User {
    name: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have a diagnostic for unknown macro
        let unknown_error = result.diagnostics.iter().find(|d| {
            d.message.contains("NonExistentMacro")
                || d.message.contains("unknown")
                || d.message.contains("not found")
        });

        assert!(
            unknown_error.is_some(),
            "Should produce an error for unknown derive macro. Diagnostics: {:?}",
            result.diagnostics
        );
    });
}

#[test]
fn test_unknown_derive_macro_on_interface_produces_error() {
    // A derive macro that doesn't exist should produce an error for interfaces too
    let source = r#"
/** @derive(Serializable) */
interface Config {
    host: string;
    port: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have a diagnostic for unknown macro
        let unknown_error = result.diagnostics.iter().find(|d| {
            d.message.contains("Serializable")
                || d.message.contains("unknown")
                || d.message.contains("not found")
        });

        assert!(
            unknown_error.is_some(),
            "Should produce an error for unknown derive macro on interface. Diagnostics: {:?}",
            result.diagnostics
        );
    });
}

#[test]
fn test_error_span_covers_macro_name_not_entire_decorator() {
    // Verify the error span is reasonably sized (not covering the entire file)
    // Use an unknown macro to trigger an error
    let source = r#"
/** @derive(NonExistent) */
interface Data {
    value: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        let error_diag = result
            .diagnostics
            .iter()
            .find(|d| d.level == DiagnosticLevel::Error)
            .expect("Should have an error-level diagnostic");

        let span = error_diag.span.as_ref().expect("Error should have a span");
        let span_len = span.end - span.start;

        // The span should be reasonably sized - not the entire source
        // A macro name like "NonExistent" would be around 5-20 characters with context
        assert!(
            span_len < 100,
            "Error span should be focused, not cover entire source. Span length: {}",
            span_len
        );
    });
}

#[test]
fn test_derive_serialize_on_interface_generates_functions() {
    let source = r#"
/** @derive(Serialize) */
interface Point {
    x: number;
    y: number;
}
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

        // Output should contain prefix-style serialize functions (default naming style)
        assert!(
            result.code.contains("pointSerialize"),
            "Should generate prefix-style pointSerialize function"
        );
        assert!(
            result.code.contains("pointSerializeWithContext"),
            "Should generate prefix-style pointSerializeWithContext function"
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
fn test_derive_deserialize_on_interface_generates_functions() {
    let source = r#"
/** @derive(Deserialize) */
interface Point {
    x: number;
    y: number;
}
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

        // Output should contain prefix-style deserialize functions (default naming style)
        assert!(
            result.code.contains("pointDeserialize"),
            "Should generate prefix-style pointDeserialize function"
        );
        assert!(
            result.code.contains("pointDeserializeWithContext"),
            "Should generate prefix-style pointDeserializeWithContextfunction"
        );
    });
}

#[test]
fn test_interface_derive_macros_default_to_prefix_functions() {
    let source = r#"
/** @derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize) */
export interface Point {
    x: number;
    y: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(
            !result.code.contains("export namespace Point"),
            "Default naming style should not emit namespaces"
        );

        for expected in [
            "export function pointToString",
            "export function pointClone",
            "export function pointEquals",
            "export function pointHashCode",
            "export function pointPartialCompare",
            "export function pointCompare",
            "export function pointDefaultValue",
            "export function pointSerializeWithContext",
            "export function pointDeserializeWithContext",
        ] {
            assert!(
                result.code.contains(expected),
                "Expected prefix-style function: {expected}"
            );
        }
    });
}

#[test]
fn test_external_type_function_imports_for_prefix_style() {
    let source = r#"
import { Metadata } from "./metadata.svelte";

/** @derive(Default, Serialize, Deserialize) */
export interface User {
    metadata: Metadata;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // The expansion should call into the imported type's generated functions
        assert!(
            result.code.contains("metadataSerializeWithContext"),
            "Expected User serialization to reference metadataSerializeWithContext"
        );
        assert!(
            result.code.contains("metadataDeserializeWithContext"),
            "Expected User deserialization to reference metadataDeserializeWithContext"
        );
        assert!(
            result.code.contains("metadataDefaultValue"),
            "Expected User defaultValue to reference metadataDefaultValue"
        );

        // ...and it should add corresponding imports from the original module specifier.
        assert!(
            result
                .code
                .contains("import { metadataSerializeWithContext } from \"./metadata.svelte\";"),
            "Expected metadataSerializeWithContext import to be added"
        );
        assert!(
            result
                .code
                .contains("import { metadataDeserializeWithContext } from \"./metadata.svelte\";"),
            "Expected metadataDeserializeWithContext import to be added"
        );
        assert!(
            result
                .code
                .contains("import { metadataDefaultValue } from \"./metadata.svelte\";"),
            "Expected metadataDefaultValue import to be added"
        );
    });
}

#[test]
fn test_derive_default_on_interface_with_object_literal_field() {
    // Regression: interfaces with fields typed as object literals (e.g. index signatures)
    // must not silently fail to generate their defaultValue function.
    let source = r#"
/** @derive(Default) */
export interface Assignment {
    name: string;
    scores: { [key: string]: number };
    active: boolean;
}
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
            "Should have no errors for interface with object literal field. Got: {:?}",
            result.diagnostics
        );

        assert!(
            result.code.contains("assignmentDefaultValue"),
            "Should generate assignmentDefaultValue function. Got:\n{}",
            result.code
        );
        // The object literal field should default to {}
        assert!(
            result.code.contains("scores: {}"),
            "Object literal field should default to {{}}. Got:\n{}",
            result.code
        );
    });
}
