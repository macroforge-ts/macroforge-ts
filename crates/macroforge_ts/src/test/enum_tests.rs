use super::*;

// ==================== ENUM TESTS ====================

#[test]
fn test_derive_debug_on_enum_generates_functions() {
    let source = r#"
/** @derive(Debug) */
enum Status {
    Active,
    Inactive,
    Pending
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

        // Debug macro on enum generates prefix-style toString function
        assert!(
            result.code.contains("statusToString"),
            "Should generate prefix-style statusToString function"
        );
    });
}

#[test]
fn test_derive_clone_on_enum_generates_functions() {
    let source = r#"
/** @derive(Clone) */
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3
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

        // Clone macro on enum generates prefix-style standalone function (default style)
        assert!(
            result.code.contains("priorityClone"),
            "Should generate prefix-style clone function for enum"
        );
        assert!(result.code.contains("Clone"), "Should have Clone function");
    });
}

#[test]
fn test_derive_partial_eq_hash_on_enum_generates_functions() {
    let source = r#"
/** @derive(PartialEq, Hash) */
enum Color {
    Red = "red",
    Green = "green",
    Blue = "blue"
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

        // PartialEq and Hash macros on enum generate prefix-style functions
        assert!(
            result.code.contains("colorEquals"),
            "Should generate prefix-style colorEquals function"
        );
        assert!(
            result.code.contains("colorHashCode"),
            "Should generate prefix-style colorHashCode function"
        );
    });
}

#[test]
fn test_derive_serialize_on_enum_generates_functions() {
    let source = r#"
/** @derive(Serialize) */
enum Direction {
    North,
    South,
    East,
    West
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

        // Serialize macro on enum generates prefix-style serialize function
        assert!(
            result.code.contains("directionSerialize"),
            "Should generate prefix-style directionSerialize function"
        );
    });
}

#[test]
fn test_derive_deserialize_on_enum_generates_functions() {
    let source = r#"
/** @derive(Deserialize) */
enum Role {
    Admin = "admin",
    User = "user",
    Guest = "guest"
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

        // Deserialize macro on enum generates prefix-style deserialize function
        assert!(
            result.code.contains("roleDeserialize"),
            "Should generate prefix-style roleDeserialize function"
        );
    });
}

#[test]
fn test_multiple_derives_on_enum() {
    let source = r#"
/** @derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize) */
enum Status {
    Active = "active",
    Inactive = "inactive"
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
            "Should have no errors, got {} errors",
            error_count
        );

        // All macros should generate prefix-style functions
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
        assert!(
            result.code.contains("statusSerialize"),
            "Should have Serialize's statusSerialize"
        );
        assert!(
            result.code.contains("statusDeserialize"),
            "Should have Deserialize's statusDeserialize"
        );
    });
}
