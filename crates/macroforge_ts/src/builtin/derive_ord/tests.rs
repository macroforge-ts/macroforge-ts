use super::comparison::generate_field_compare_for_interface;
use super::types::OrdField;

use crate::macros::ts_template;
use crate::swc_ecma_ast::{Expr, Ident};
use crate::ts_syn::{parse_ts_expr, ts_ident};

#[test]
fn test_ord_macro_output() {
    let class_name = "User";
    let class_ident = ts_ident!(class_name);
    let ord_fields: Vec<OrdField> = vec![OrdField {
        name: "id".to_string(),
        ts_type: "number".to_string(),
    }];

    let compare_steps: Vec<(Ident, Expr)> = ord_fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let cmp_ident = ts_ident!(format!("cmp{}", i));
            let expr_src = generate_field_compare_for_interface(f, "a", "b", None, None);
            let expr = parse_ts_expr(&expr_src).expect("compare expr should parse");
            (cmp_ident, *expr)
        })
        .collect();

    // Explicitly use compare_steps to satisfy clippy (it's consumed by body! macro)
    let _ = &compare_steps;

    let output = ts_template!(Within {
        compareTo(other: @{class_ident}): number {
            if (a === b) return 0;
            {#for (cmp_ident, cmp_expr) in &compare_steps}
                const @{cmp_ident.clone()} = @{cmp_expr.clone()};
                if (@{cmp_ident.clone()} !== 0) return @{cmp_ident.clone()};
            {/for}
            return 0;
        }
    });

    let source = output.source();
    let body_content = source
        .strip_prefix("/* @macroforge:body */")
        .unwrap_or(source);
    let wrapped = format!("class __Temp {{ {} }}", body_content);

    assert!(
        macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
        "Generated Ord macro output should parse as class members"
    );
    assert!(
        source.contains("compareTo"),
        "Should contain compareTo method"
    );
    // Verify return type doesn't contain null
    assert!(
        !source.contains("number | null"),
        "Should not contain nullable return type"
    );
}

#[test]
fn test_field_compare_number() {
    let field = OrdField {
        name: "id".to_string(),
        ts_type: "number".to_string(),
    };
    let result = generate_field_compare_for_interface(&field, "a", "b", None, None);
    assert!(result.contains("a.id < b.id"));
    assert!(result.contains("a.id > b.id"));
    assert!(!result.contains("null")); // Total ordering - no null
}

#[test]
fn test_field_compare_string() {
    let field = OrdField {
        name: "name".to_string(),
        ts_type: "string".to_string(),
    };
    let result = generate_field_compare_for_interface(&field, "a", "b", None, None);
    assert!(result.contains("localeCompare"));
    // Should clamp localeCompare result to -1, 0, 1
    assert!(result.contains("-1"));
    assert!(result.contains("1"));
}

#[test]
fn test_field_compare_object_no_null() {
    let field = OrdField {
        name: "user".to_string(),
        ts_type: "User".to_string(),
    };
    let result = generate_field_compare_for_interface(&field, "a", "b", None, None);
    assert!(result.contains("compareTo"));
    // Should fallback to 0 instead of null
    assert!(result.contains("?? 0"));
}
