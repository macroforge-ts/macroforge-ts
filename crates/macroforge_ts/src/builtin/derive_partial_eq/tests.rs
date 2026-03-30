use super::*;

use crate::macros::ts_template;
use crate::ts_syn::parse_ts_expr;

#[test]
fn test_partial_eq_macro_output() {
    // Test that the template compiles and produces valid output
    let eq_fields: Vec<EqField> = vec![
        EqField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
        },
        EqField {
            name: "name".to_string(),
            ts_type: "string".to_string(),
        },
    ];

    let comparison = eq_fields
        .iter()
        .map(|f| generate_field_equality_for_interface(f, "a", "b", None, None))
        .collect::<Vec<_>>()
        .join(" && ");
    let comparison_expr = parse_ts_expr(&comparison).expect("comparison expr should parse");

    let output = ts_template!(Within {
        equals(other: unknown): boolean {
            if (a === b) return true;
            return @{comparison_expr};
        }
    });

    let source = output.source();
    let body_content = source
        .strip_prefix("/* @macroforge:body */")
        .unwrap_or(source);
    let wrapped = format!("class __Temp {{ {} }}", body_content);

    assert!(
        macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
        "Generated PartialEq macro output should parse as class members"
    );
    assert!(source.contains("equals"), "Should contain equals method");
}

#[test]
fn test_field_equality_primitive() {
    let field = EqField {
        name: "id".to_string(),
        ts_type: "number".to_string(),
    };
    let result = generate_field_equality_for_interface(&field, "a", "b", None, None);
    assert!(result.contains("a.id === b.id"));
}

#[test]
fn test_field_equality_object() {
    let field = EqField {
        name: "user".to_string(),
        ts_type: "User".to_string(),
    };
    let result = generate_field_equality_for_interface(&field, "a", "b", None, None);
    assert!(result.contains("equals"));
}

#[test]
fn test_field_equality_array() {
    let field = EqField {
        name: "items".to_string(),
        ts_type: "string[]".to_string(),
    };
    let result = generate_field_equality_for_interface(&field, "a", "b", None, None);
    assert!(result.contains("Array.isArray"));
    assert!(result.contains("every"));
}

#[test]
fn test_field_equality_date() {
    let field = EqField {
        name: "createdAt".to_string(),
        ts_type: "Date".to_string(),
    };
    let result = generate_field_equality_for_interface(&field, "a", "b", None, None);
    assert!(result.contains("getTime"));
}
