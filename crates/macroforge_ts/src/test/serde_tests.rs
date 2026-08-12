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

    {
        let result = expand_test(source);

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
    }
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

    {
        let result = expand_test(source);

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
    }
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

    {
        let result = expand_test(source);

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
    }
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

    {
        let result = expand_test(source);

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
    }
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

    {
        let result = expand_test(source);

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
    }
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

    {
        let result = expand_test(source);

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
    }
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

    {
        let result = expand_test(source);

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
    }
}

#[test]
fn test_serialize_generates_correct_field_access() {
    // Test that serialization uses correct property access syntax
    let source = r#"
/** @derive(Serialize) */
class Point {
    x: number;
    y: number;
}
"#;

    let result = expand_test(source);

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

#[test]
fn test_serialize_internally_tagged_union_uses_variant_label_not_payload_type() {
    // Regression: an internally-tagged union whose variant *label* differs from
    // its payload *type name* — `{ variant: 'User' } & PartialUser` — must
    // serialize the discriminator as the variant label ("User"), not the
    // payload's type name ("PartialUser").
    //
    // The dispatched per-variant serializer (`partialUserSerializeWithContext`)
    // tags its output with the payload's `__type` ("PartialUser"). The union
    // serializer must then restore the discriminator from the value's own tag
    // field; the old codegen instead re-emitted `__type`, so serialize produced
    // `{ variant: "PartialUser" }` while deserialize dispatched on "User" — the
    // round-trip threw. (This broke drag-rescheduling any record whose payload
    // carried such a union, e.g. an event's `activity` Did with an
    // `in: RecordLink<Actor>`.) Unions whose variant label equals the payload
    // type name hid the bug because `__type` happened to match.
    let source = r#"
/** @derive(Serialize, Deserialize) */
interface PartialUser { id: string; }

/** @derive(Serialize, Deserialize) */
interface PartialEmployee { id: string; }

/** @derive(Serialize, Deserialize) */
/** @serde({ tag: "variant" }) */
export type Actor =
    | ({ variant: 'User' } & PartialUser)
    | ({ variant: 'Employee' } & PartialEmployee);
"#;

    let result = expand_test(source);

    let error_count = result
        .diagnostics
        .iter()
        .filter(|d| d.level == DiagnosticLevel::Error)
        .count();
    assert_eq!(
        error_count, 0,
        "internally-tagged union should expand without errors. Got: {:?}",
        result.diagnostics
    );

    // The per-variant dispatch + `__type` rewrap path must actually be
    // exercised, or the guards below pass vacuously.
    assert!(
        result.code.contains("actorSerializeWithContext"),
        "Should generate the union serializer. Got:\n{}",
        result.code
    );

    // The bug: the discriminator was emitted straight from the payload type
    // name via `return { "variant": __typeName, ...fields };`.
    assert!(
        !result.code.contains(r#""variant": __typeName,"#),
        "union serialize must not use the payload type name as the internal tag. Got:\n{}",
        result.code
    );

    // The fix restores the discriminator from the value's own tag field,
    // falling back to `__type` only when the value carries no tag.
    assert!(
        result.code.contains("?? __typeName"),
        "union serialize should restore the internal tag from the value, not the payload type. Got:\n{}",
        result.code
    );
}

#[test]
fn test_deserialize_external_variant_keeps_a_non_object_payload() {
    // A link-shaped payload: serializable in its own right, but carried as a
    // bare id string as often as an object — the shape `RecordLink<T>` takes.
    let source = r#"
/** @derive(Serialize, Deserialize) */
export type Ref = string | { id: string };

/** @derive(Serialize, Deserialize) */
/** @serde({ externallyTagged: true }) */
export type Stage = 'Active' | { Invoice: Ref };
"#;

    let result = expand_test(source);
    assert!(result.changed, "expand() should report changes");

    // The bug: dispatching an external variant's payload into its own
    // deserializer guarded the argument with
    // `typeof __inner === "object" ? __inner : {}`, so a variant holding a
    // bare id decoded to `{}` — the link vanished and the result still
    // reported success, which is worse than failing.
    let invoice_branch = result
        .code
        .lines()
        .find(|line| line.contains(r#"__variantName === "Invoice""#))
        .unwrap_or_else(|| {
            panic!(
                "expected a deserialize branch for the Invoice variant. Got:\n{}",
                result.code
            )
        });

    assert!(
        !invoice_branch.contains(r#"typeof __inner === "object""#),
        "an external variant's payload must reach its deserializer intact, not \
         be coerced away when it is not an object. Got:\n{invoice_branch}"
    );
    assert!(
        invoice_branch.contains("__inner"),
        "the Invoice branch should pass the payload through. Got:\n{invoice_branch}"
    );
}
