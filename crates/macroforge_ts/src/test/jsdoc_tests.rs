use super::*;

#[test]
fn test_inline_jsdoc_with_export_interface() {
    // Test that /** @derive(X) */ export interface works on same line
    let source =
        r#"/** @derive(Deserialize) */ export interface User { name: string; age: number; }"#;

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
            "Should have no errors for inline JSDoc, got {}",
            error_count
        );

        // Should generate deserialize method in User namespace
        assert!(
            result.code.contains("User") && result.code.contains("deserialize"),
            "Should generate deserialize for Deserialize on interface. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_inline_jsdoc_with_export_class() {
    // Test that /** @derive(X) */ export class works on same line
    let source = r#"/** @derive(Debug) */ export class User { name: string; }"#;

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
            "Should have no errors for inline JSDoc on class, got {}",
            error_count
        );

        // Should generate toString method in class
        assert!(
            result.code.contains("toString"),
            "Should generate toString for Debug on class. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_inline_jsdoc_with_export_enum() {
    // Test that /** @derive(X) */ export enum works on same line
    let source = r#"/** @derive(Debug) */ export enum Status { Active, Inactive }"#;

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
            "Should have no errors for inline JSDoc on enum, got {}",
            error_count
        );

        // Should generate toString in Status namespace
        assert!(
            result.code.contains("Status") && result.code.contains("toString"),
            "Should generate toString for Debug on enum. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_inline_jsdoc_with_export_type() {
    // Test that /** @derive(X) */ export type works on same line
    let source = r#"/** @derive(Debug) */ export type Point = { x: number; y: number; }"#;

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
            "Should have no errors for inline JSDoc on type, got {}",
            error_count
        );

        // Should generate toString in Point namespace
        assert!(
            result.code.contains("Point") && result.code.contains("toString"),
            "Should generate toString for Debug on type alias. Got:\n{}",
            result.code
        );
    });
}
