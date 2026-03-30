use super::*;

#[test]
fn test_derive_serialize_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Serialize) */
class User {
    name: string;
    age: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        // Check for new serde methods - static methods + standalone functions
        assert!(
            type_output.contains("static serialize(value: User, keepMetadata?: boolean): string"),
            "Should have static serialize method"
        );
        assert!(
            type_output
                .contains("static serializeWithContext(value: User, ctx: __mf_SerializeContext)"),
            "Should have static serializeWithContext method"
        );
        assert!(
            type_output.contains("export function userSerialize"),
            "Should have standalone userSerialize function"
        );
    });
}

#[test]
fn test_derive_serialize_runtime_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Serialize) */
class Data {
    val: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        // Serialize macro adds static serialize methods and standalone functions
        assert!(
            result
                .code
                .contains("static serialize(value: Data, keepMetadata?: boolean): string"),
            "Should have static serialize method"
        );
        assert!(
            result.code.contains("serializeWithContext"),
            "Should have serializeWithContext method"
        );
        assert!(
            result.code.contains("export function dataSerialize"),
            "Should have standalone dataSerialize function"
        );
    });
}

#[test]
fn test_derive_deserialize_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Deserialize) */
class User {
    name: string;
    age: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");
        // Check for new serde methods
        assert!(
            type_output.contains("static deserialize(input: unknown"),
            "Should have deserialize method"
        );
        assert!(
            type_output.contains(
                "static deserializeWithContext(value: any, ctx: __mf_DeserializeContext)"
            ),
            "Should have deserializeWithContext method"
        );
    });
}

#[test]
fn test_derive_deserialize_runtime_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Deserialize) */
class Data {
    val: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        // Deserialize macro adds static deserialize() and deserializeWithContext() methods
        assert!(
            result.code.contains("deserialize"),
            "Should have deserialize method"
        );
        assert!(
            result.code.contains("deserializeWithContext"),
            "Should have deserializeWithContext method"
        );
        assert!(result.code.contains("static"), "Methods should be static");
        assert!(
            result.code.contains("DeserializeContext"),
            "Should use DeserializeContext"
        );
    });
}

#[test]
fn test_multiple_derives_with_serialize_deserialize() {
    // When Serialize and Deserialize are combined, both should succeed
    let source = r#"
/** @derive(Serialize, Deserialize) */
class Config {
    host: string;
    port: number;
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

        // Should have both serialize and deserialize methods (now static)
        assert!(
            result
                .code
                .contains("static serialize(value: Config, keepMetadata?: boolean): string"),
            "Should have Serialize's static serialize"
        );
        assert!(
            result.code.contains("static deserialize"),
            "Should have Deserialize's static deserialize"
        );
    });
}

#[test]
fn test_deserialize_validation_on_interface() {
    // Verify that @serde validators generate validation code for interfaces
    // Note: Interface fields use JSDoc comments for decorators
    let source = r#"
/** @derive(Deserialize) */
interface UserProfile {
    /** @serde(email) */
    email: string;

    /** @serde(minLength(2), maxLength(50)) */
    username: string;

    /** @serde(positive) */
    age?: number;
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

        // Should generate validation code for email
        assert!(
            result.code.contains("test(__raw_email)"),
            "Should generate email validation. Got:\n{}",
            result.code
        );

        // Should generate validation code for username length
        assert!(
            result.code.contains("__raw_username.length < 2")
                || result.code.contains("__raw_username.length > 50"),
            "Should generate length validation. Got:\n{}",
            result.code
        );

        // Should generate validation code for positive number
        assert!(
            result.code.contains("__raw_age <= 0"),
            "Should generate positive validation. Got:\n{}",
            result.code
        );

        // Should push errors to the errors array
        assert!(
            result.code.contains("errors.push"),
            "Should push validation errors. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_deserialize_validation_on_type_alias() {
    // Verify that @serde validators generate validation code for type alias objects
    // Note: Type alias fields use JSDoc comments for decorators
    let source = r#"
/** @derive(Deserialize) */
type ContactInfo = {
    /** @serde(email) */
    primaryEmail: string;

    /** @serde(minLength(1), maxLength(100)) */
    address: string;
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

        // Should generate validation code for email
        assert!(
            result.code.contains("test(__raw_primaryEmail)"),
            "Should generate email validation for type alias. Got:\n{}",
            result.code
        );

        // Should generate validation code for address length
        assert!(
            result.code.contains("__raw_address.length < 1")
                || result.code.contains("__raw_address.length > 100"),
            "Should generate length validation for type alias. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_serialize_generates_correct_field_access() {
    // Test that serialization uses correct property access syntax
    use crate::expand_inner;

    let source = r#"
/** @derive(Serialize) */
class Point {
    x: number;
    y: number;
}
"#;

    let result = expand_inner(source, "test.ts", None).unwrap();

    // Should have __type marker with class name
    assert!(
        result.code.contains("__type"),
        "Should have __type marker. Got:\n{}",
        result.code
    );

    // Should use direct property access (result.x) not computed (result["x"])
    assert!(
        result.code.contains("result.x =") || result.code.contains("result.x="),
        "Should use direct property access for x. Got:\n{}",
        result.code
    );

    // Should NOT have template literal property access
    assert!(
        !result.code.contains("`${"),
        "Should not have template literal syntax. Got:\n{}",
        result.code
    );

    // Should NOT have #0 syntax context markers
    assert!(
        !result.code.contains("#0"),
        "Should not have SWC syntax context markers. Got:\n{}",
        result.code
    );
}
