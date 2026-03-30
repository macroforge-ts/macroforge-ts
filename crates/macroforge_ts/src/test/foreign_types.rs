use super::*;

// ============================================================================
// Foreign Types in Union Type Alias -- Deserialize
// ============================================================================

#[test]
fn test_derive_deserialize_union_with_foreign_type_uses_has_shape() {
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Deserialize) */
type FlexibleValue = DateTime.DateTime | RegularType;
"#;

    GLOBALS.set(&Default::default(), || {
        set_foreign_types(vec![make_foreign_type(
            "DateTime.DateTime",
            vec!["effect"],
            Some("(v) => DateTime.formatIso(v)"),
            Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
            Some("() => DateTime.unsafeNow()"),
            Some("(v) => typeof v === \"string\""),
            vec!["DateTime"],
        )]);

        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        clear_foreign_types();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Should use the configured deserialize expression, not broken camelCase helpers
        assert!(
            result.code.contains("DateTime.unsafeFromDate")
                || result.code.contains("__mf_DateTime.unsafeFromDate"),
            "Should use foreign type deserialize expression. Got:\n{}",
            result.code
        );

        // Should NOT generate broken dotted identifier
        assert!(
            !result
                .code
                .contains("dateTime.dateTimeDeserializeWithContext"),
            "Should NOT generate broken dotted deserialize fn. Got:\n{}",
            result.code
        );

        // Should use the hasShape expression for shape matching
        assert!(
            result.code.contains("typeof v === \"string\""),
            "Should use foreign type hasShape expression. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_deserialize_union_foreign_only_types() {
    let source = r#"
import type { DateTime, BigDecimal } from 'effect';

/** @derive(Deserialize) */
type FlexValue = DateTime.DateTime | BigDecimal.BigDecimal;
"#;

    GLOBALS.set(&Default::default(), || {
        set_foreign_types(vec![
            make_foreign_type(
                "DateTime.DateTime",
                vec!["effect"],
                Some("(v) => DateTime.formatIso(v)"),
                Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
                Some("() => DateTime.unsafeNow()"),
                Some("(v) => typeof v === \"string\""),
                vec!["DateTime"],
            ),
            make_foreign_type(
                "BigDecimal.BigDecimal",
                vec!["effect"],
                Some("(v) => BigDecimal.format(v)"),
                Some("(raw) => BigDecimal.fromString(String(raw))"),
                Some("() => BigDecimal.unsafeFromNumber(0)"),
                Some("(v) => typeof v === \"string\" || typeof v === \"number\""),
                vec!["BigDecimal"],
            ),
        ]);

        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        clear_foreign_types();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Both foreign deserialize expressions should be present
        assert!(
            result.code.contains("DateTime.unsafeFromDate")
                || result.code.contains("__mf_DateTime.unsafeFromDate"),
            "Should have DateTime foreign deserialize. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("BigDecimal.fromString")
                || result.code.contains("__mf_BigDecimal.fromString"),
            "Should have BigDecimal foreign deserialize. Got:\n{}",
            result.code
        );

        // hasShape functions should be used
        assert!(
            result.code.contains("flexValueHasShape"),
            "Should generate hasShape for union. Got:\n{}",
            result.code
        );

        // Should NOT generate broken camelCase helper calls
        assert!(
            !result
                .code
                .contains("dateTime.dateTimeDeserializeWithContext"),
            "Should NOT generate broken DateTime dotted identifier. Got:\n{}",
            result.code
        );
        assert!(
            !result
                .code
                .contains("bigDecimal.bigDecimalDeserializeWithContext"),
            "Should NOT generate broken BigDecimal dotted identifier. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_deserialize_union_foreign_without_has_shape() {
    // Foreign type without hasShape should still use foreign deserialize for __type dispatch
    // but won't participate in shape matching
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Deserialize) */
type Value = DateTime.DateTime | RegularType;
"#;

    GLOBALS.set(&Default::default(), || {
        set_foreign_types(vec![make_foreign_type(
            "DateTime.DateTime",
            vec!["effect"],
            Some("(v) => DateTime.formatIso(v)"),
            Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
            Some("() => DateTime.unsafeNow()"),
            None, // No hasShape
            vec!["DateTime"],
        )]);

        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        clear_foreign_types();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Should still use foreign deserialize for __type-based dispatch
        assert!(
            result.code.contains("DateTime.unsafeFromDate")
                || result.code.contains("__mf_DateTime.unsafeFromDate"),
            "Should use foreign deserialize even without hasShape. Got:\n{}",
            result.code
        );

        // Should NOT generate broken dotted identifier
        assert!(
            !result
                .code
                .contains("dateTime.dateTimeDeserializeWithContext"),
            "Should NOT generate broken dotted identifier. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_derive_deserialize_union_mixed_foreign_and_primitives() {
    let source = r#"
import type { DateTime } from 'effect';

/** @derive(Deserialize) */
type MaybeDate = DateTime.DateTime | string | number;
"#;

    GLOBALS.set(&Default::default(), || {
        set_foreign_types(vec![make_foreign_type(
            "DateTime.DateTime",
            vec!["effect"],
            Some("(v) => DateTime.formatIso(v)"),
            Some("(raw) => DateTime.unsafeFromDate(new Date(raw))"),
            Some("() => DateTime.unsafeNow()"),
            Some("(v) => typeof v === \"string\""),
            vec!["DateTime"],
        )]);

        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        clear_foreign_types();

        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Should use foreign type deserialize
        assert!(
            result.code.contains("DateTime.unsafeFromDate")
                || result.code.contains("__mf_DateTime.unsafeFromDate"),
            "Should use foreign type deserialize in mixed union. Got:\n{}",
            result.code
        );

        // Should have primitive checks too
        assert!(
            result.code.contains("typeof value === \"string\""),
            "Should have primitive string check. Got:\n{}",
            result.code
        );
        assert!(
            result.code.contains("typeof value === \"number\""),
            "Should have primitive number check. Got:\n{}",
            result.code
        );
    });
}
