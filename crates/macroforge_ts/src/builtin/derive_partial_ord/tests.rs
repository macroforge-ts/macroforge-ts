use crate::builtin::derive_partial_ord::comparison::generate_field_compare_for_interface;
use crate::builtin::derive_partial_ord::types::OrdField;
use crate::builtin::return_types::partial_ord_return_type;
use crate::macros::ts_template;
use crate::ts_syn::TsStream;
use crate::ts_syn::ts_ident;

#[test]
fn test_partial_ord_macro_output_vanilla() {
    let ord_fields: Vec<OrdField> = vec![OrdField {
        name: "id".to_string(),
        ts_type: "number".to_string(),
    }];

    // Build comparison steps for each field
    let mut compare_body_str = String::new();
    for (i, f) in ord_fields.iter().enumerate() {
        let cmp_var = format!("cmp{}", i);
        let expr_src = generate_field_compare_for_interface(f, "a", "b", true, None, None);
        compare_body_str.push_str(&format!(
            "const {} = {};\nif ({} === null) return null;\nif ({} !== 0) return {};\n",
            cmp_var, expr_src, cmp_var, cmp_var, cmp_var
        ));
    }

    let return_type = partial_ord_return_type();
    let return_type_ident = ts_ident!(return_type);
    let output = ts_template!(Within {
        compareTo(other: unknown): @{return_type_ident} {
            if (a === b) return 0;
            {$typescript TsStream::from_string(compare_body_str)}
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
        "Generated PartialOrd macro output should parse as class members"
    );
    assert!(
        source.contains("compareTo"),
        "Should contain compareTo method"
    );
    // Return type is "number | null"
    assert!(
        source.contains("number | null"),
        "Should have number | null return type"
    );
}

#[test]
fn test_field_compare_number() {
    let field = OrdField {
        name: "id".to_string(),
        ts_type: "number".to_string(),
    };
    let result = generate_field_compare_for_interface(&field, "a", "b", true, None, None);
    assert!(result.contains("a.id < b.id"));
    assert!(result.contains("a.id > b.id"));
}

#[test]
fn test_field_compare_string() {
    let field = OrdField {
        name: "name".to_string(),
        ts_type: "string".to_string(),
    };
    let result = generate_field_compare_for_interface(&field, "a", "b", true, None, None);
    assert!(result.contains("localeCompare"));
}

#[test]
fn test_field_compare_boolean() {
    let field = OrdField {
        name: "active".to_string(),
        ts_type: "boolean".to_string(),
    };
    let result = generate_field_compare_for_interface(&field, "a", "b", true, None, None);
    // false < true: false returns -1, true returns 1
    assert!(result.contains("-1"));
    assert!(result.contains("1"));
}

#[test]
fn test_field_compare_date() {
    let field = OrdField {
        name: "createdAt".to_string(),
        ts_type: "Date".to_string(),
    };
    let result = generate_field_compare_for_interface(&field, "a", "b", true, None, None);
    assert!(result.contains("getTime"));
}

#[test]
fn test_field_compare_object_vanilla() {
    let field = OrdField {
        name: "user".to_string(),
        ts_type: "User".to_string(),
    };
    let result = generate_field_compare_for_interface(&field, "a", "b", true, None, None);
    assert!(result.contains("compareTo"));
    // We check for null directly
    assert!(result.contains("=== null"));
}

#[test]
fn test_field_compare_array_vanilla() {
    let field = OrdField {
        name: "items".to_string(),
        ts_type: "Item[]".to_string(),
    };
    let result = generate_field_compare_for_interface(&field, "a", "b", true, None, None);
    // optResult is already the value
    assert!(result.contains("cmp = optResult"));
}
