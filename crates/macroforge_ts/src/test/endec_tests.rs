use super::*;
use crate::ts_syn::abi::DiagnosticLevel;

#[test]
fn test_derive_encode_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Encode) */
class User {
    name: string;
    age: number;
}
"#;

    {
        let result = expand_test(source);

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        // Check for new endec methods - static methods + standalone functions
        assert!(
            type_output.contains("static encode(value: User, keepMetadata?: boolean): string"),
            "Should have static encode method"
        );
        assert!(
            type_output.contains("static encodeWithContext(value: User, ctx: __mf_EncodeContext)"),
            "Should have static encodeWithContext method"
        );
        assert!(
            type_output.contains("export function userEncode"),
            "Should have standalone userEncode function"
        );
    }
}

#[test]
fn test_derive_encode_runtime_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Encode) */
class Data {
    val: number;
}
"#;

    {
        let result = expand_test(source);

        assert!(result.changed, "expand() should report changes");
        // Encode macro adds static encode methods and standalone functions
        assert!(
            result
                .code
                .contains("static encode(value: Data, keepMetadata?: boolean): string"),
            "Should have static encode method"
        );
        assert!(
            result.code.contains("encodeWithContext"),
            "Should have encodeWithContext method"
        );
        assert!(
            result.code.contains("export function dataEncode"),
            "Should have standalone dataEncode function"
        );
    }
}

#[test]
fn test_derive_decode_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Decode) */
class User {
    name: string;
    age: number;
}
"#;

    {
        let result = expand_test(source);

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");
        // Check for new endec methods
        assert!(
            type_output.contains("static decode(input: unknown"),
            "Should have decode method"
        );
        assert!(
            type_output.contains("static decodeWithContext(value: any, ctx: __mf_DecodeContext)"),
            "Should have decodeWithContext method"
        );
    }
}

#[test]
fn test_derive_decode_runtime_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Decode) */
class Data {
    val: number;
}
"#;

    {
        let result = expand_test(source);

        assert!(result.changed, "expand() should report changes");
        // Decode macro adds static decode() and decodeWithContext() methods
        assert!(result.code.contains("decode"), "Should have decode method");
        assert!(
            result.code.contains("decodeWithContext"),
            "Should have decodeWithContext method"
        );
        assert!(result.code.contains("static"), "Methods should be static");
        assert!(
            result.code.contains("DecodeContext"),
            "Should use DecodeContext"
        );
    }
}

#[test]
fn test_multiple_derives_with_encode_decode() {
    // When Encode and Decode are combined, both should succeed
    let source = r#"
/** @derive(Encode, Decode) */
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

        // Should have both encode and decode methods (now static)
        assert!(
            result
                .code
                .contains("static encode(value: Config, keepMetadata?: boolean): string"),
            "Should have Encode's static encode"
        );
        assert!(
            result.code.contains("static decode"),
            "Should have Decode's static decode"
        );
    }
}

#[test]
fn test_decode_validation_on_interface() {
    // Verify that @endec validators generate validation code for interfaces
    // Note: Interface fields use JSDoc comments for decorators
    let source = r#"
/** @derive(Decode) */
interface UserProfile {
    /** @endec(email) */
    email: string;

    /** @endec(minLength(2), maxLength(50)) */
    username: string;

    /** @endec(positive) */
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
fn test_decode_validation_on_type_alias() {
    // Verify that @endec validators generate validation code for type alias objects
    // Note: Type alias fields use JSDoc comments for decorators
    let source = r#"
/** @derive(Decode) */
type ContactInfo = {
    /** @endec(email) */
    primaryEmail: string;

    /** @endec(minLength(1), maxLength(100)) */
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
fn test_encode_generates_correct_field_access() {
    // Test that encoding uses correct property access syntax
    let source = r#"
/** @derive(Encode) */
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
}

#[test]
fn test_encode_internally_tagged_union_uses_variant_label_not_payload_type() {
    // Regression: an internally-tagged union whose variant *label* differs from
    // its payload *type name*: `{ variant: 'User' } & PartialUser`: must
    // encode the discriminator as the variant label ("User"), not the
    // payload's type name ("PartialUser").
    //
    // The dispatched per-variant encoder (`partialUserEncodeWithContext`)
    // tags its output with the payload's `__type` ("PartialUser"). The union
    // encoder must then restore the discriminator from the value's own tag
    // field; the old codegen instead re-emitted `__type`, so encode produced
    // `{ variant: "PartialUser" }` while decode dispatched on "User": the
    // round-trip threw. (This broke drag-rescheduling any record whose payload
    // carried such a union, e.g. an event's `activity` Did with an
    // `in: RecordLink<Actor>`.) Unions whose variant label equals the payload
    // type name hid the bug because `__type` happened to match.
    let source = r#"
/** @derive(Encode, Decode) */
interface PartialUser { id: string; }

/** @derive(Encode, Decode) */
interface PartialEmployee { id: string; }

/** @derive(Encode, Decode) */
/** @endec({ tag: "variant" }) */
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
        result.code.contains("actorEncodeWithContext"),
        "Should generate the union encoder. Got:\n{}",
        result.code
    );

    // The bug: the discriminator was emitted straight from the payload type
    // name via `return { "variant": __typeName, ...fields };`.
    assert!(
        !result.code.contains(r#""variant": __typeName,"#),
        "union encode must not use the payload type name as the internal tag. Got:\n{}",
        result.code
    );

    // The fix restores the discriminator from the value's own tag field,
    // falling back to `__type` only when the value carries no tag.
    assert!(
        result.code.contains("?? __typeName"),
        "union encode should restore the internal tag from the value, not the payload type. Got:\n{}",
        result.code
    );
}

#[test]
fn test_decode_external_variant_keeps_a_non_object_payload() {
    // A link-shaped payload: encodable in its own right, but carried as a
    // bare id string as often as an object: the shape `RecordLink<T>` takes.
    let source = r#"
/** @derive(Encode, Decode) */
export type Ref = string | { id: string };

/** @derive(Encode, Decode) */
/** @endec({ externallyTagged: true }) */
export type Stage = 'Active' | { Invoice: Ref };
"#;

    let result = expand_test(source);
    assert!(result.changed, "expand() should report changes");

    // The bug: dispatching an external variant's payload into its own
    // decoder guarded the argument with
    // `typeof __inner === "object" ? __inner : {}`, so a variant holding a
    // bare id decoded to `{}`: the link vanished and the result still
    // reported success, which is worse than failing.
    let invoice_branch = result
        .code
        .lines()
        .find(|line| line.contains(r#"__variantName === "Invoice""#))
        .unwrap_or_else(|| {
            panic!(
                "expected a decode branch for the Invoice variant. Got:\n{}",
                result.code
            )
        });

    assert!(
        !invoice_branch.contains(r#"typeof __inner === "object""#),
        "an external variant's payload must reach its decoder intact, not \
         be coerced away when it is not an object. Got:\n{invoice_branch}"
    );
    assert!(
        invoice_branch.contains("__inner"),
        "the Invoice branch should pass the payload through. Got:\n{invoice_branch}"
    );
}
