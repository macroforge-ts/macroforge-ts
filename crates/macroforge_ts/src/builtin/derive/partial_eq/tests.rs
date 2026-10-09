use super::*;

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;

#[test]
fn test_partial_eq_macro_output() {
    // Test that the template compiles and produces valid output
    let eq_fields: Vec<EqField> = vec![
        EqField {
            name: "id".to_string(),
            ts_type: "number".to_string(),
            optional: false,
        },
        EqField {
            name: "name".to_string(),
            ts_type: "string".to_string(),
            optional: false,
        },
    ];

    let comparison = eq_fields
        .iter()
        .map(|f| generate_field_equality_for_interface(f, "a", "b", None, &TypeRegistry::default()))
        .collect::<Vec<_>>()
        .join(" && ");
    let comparison_expr = Expr::parse(&comparison).expect("comparison expr should parse");

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
        macroforge_ts_syn::parse_statement(&oxc::allocator::Allocator::default(), &wrapped).is_ok(),
        "Generated PartialEq macro output should parse as class members"
    );
    assert!(source.contains("equals"), "Should contain equals method");
}

#[test]
fn test_field_equality_primitive() {
    let field = EqField {
        name: "id".to_string(),
        ts_type: "number".to_string(),
        optional: false,
    };
    let result =
        generate_field_equality_for_interface(&field, "a", "b", None, &TypeRegistry::default());
    assert!(result.contains("a.id === b.id"));
}

#[test]
fn test_field_equality_object() {
    let field = EqField {
        name: "user".to_string(),
        ts_type: "User".to_string(),
        optional: false,
    };
    let result =
        generate_field_equality_for_interface(&field, "a", "b", None, &TypeRegistry::default());
    assert_eq!(result, "__mf_structuralEquals(a.user, b.user)");
}

#[test]
fn test_field_equality_array() {
    let field = EqField {
        name: "items".to_string(),
        ts_type: "string[]".to_string(),
        optional: false,
    };
    let result =
        generate_field_equality_for_interface(&field, "a", "b", None, &TypeRegistry::default());
    assert_eq!(
        result,
        "(a.items.length === b.items.length && a.items.every((__item0, __index0) => { \
         const __other0 = b.items[__index0]; \
         return __other0 === undefined ? __item0 === undefined : __item0 === __other0; \
         }))"
    );
}

#[test]
fn test_field_equality_date() {
    let field = EqField {
        name: "createdAt".to_string(),
        ts_type: "Date".to_string(),
        optional: false,
    };
    let result =
        generate_field_equality_for_interface(&field, "a", "b", None, &TypeRegistry::default());
    assert!(result.contains("getTime"));
}

#[test]
fn nested_arrays_bind_each_level_apart() {
    let field = EqField {
        name: "grid".to_string(),
        ts_type: "number[][]".to_string(),
        optional: false,
    };
    let result =
        generate_field_equality_for_interface(&field, "a", "b", None, &TypeRegistry::default());
    // The inner level reads its own bindings, never the outer index.
    assert!(
        result.contains("const __other0 = b.grid[__index0];"),
        "{result}"
    );
    assert!(
        result.contains("(__item0.length === __other0.length"),
        "{result}"
    );
    assert!(
        result.contains("const __other1 = __other0[__index1];"),
        "{result}"
    );
    assert!(result.contains("__item1 === __other1"), "{result}");
}

#[test]
fn optional_field_guards_absent_values() {
    let field = EqField {
        name: "at".to_string(),
        ts_type: "Date".to_string(),
        optional: true,
    };
    let result =
        generate_field_equality_for_interface(&field, "a", "b", None, &TypeRegistry::default());
    assert_eq!(
        result,
        "(a.at == null || b.at == null ? a.at === b.at : a.at.getTime() === b.at.getTime())"
    );
}
