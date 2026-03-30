use super::*;

#[test]
fn test_hash_macro_output() {
    let hash_fields: Vec<HashField> = vec![HashField {
        name: "id".to_string(),
        ts_type: "number".to_string(),
    }];
    let has_fields = !hash_fields.is_empty();

    let hash_exprs: Vec<crate::swc_ecma_ast::Expr> = hash_fields
        .iter()
        .map(|f| {
            let expr_src = generate_field_hash_for_interface(f, "value", None, None);
            *crate::ts_syn::parse_ts_expr(&expr_src).expect("hash expr should parse")
        })
        .collect();

    let output = crate::macros::ts_template!(Within {
        hashCode(): number {
            let hash = 17;
            {#if has_fields}
                {#for hash_expr in hash_exprs}
                    hash = (hash * 31 + @{hash_expr}) | 0;
                {/for}
            {/if}
            return hash;
        }
    });

    let source = output.source();
    let body_content = source
        .strip_prefix("/* @macroforge:body */")
        .unwrap_or(source);
    let wrapped = format!("class __Temp {{ {} }}", body_content);

    assert!(
        macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
        "Generated Hash macro output should parse as class members"
    );
    assert!(
        source.contains("hashCode"),
        "Should contain hashCode method"
    );
}

#[test]
fn test_field_hash_number() {
    let field = HashField {
        name: "id".to_string(),
        ts_type: "number".to_string(),
    };
    let result = generate_field_hash_for_interface(&field, "value", None, None);
    assert!(result.contains("Number.isInteger"));
}

#[test]
fn test_field_hash_string() {
    let field = HashField {
        name: "name".to_string(),
        ts_type: "string".to_string(),
    };
    let result = generate_field_hash_for_interface(&field, "value", None, None);
    assert!(result.contains("split"));
    assert!(result.contains("charCodeAt"));
}

#[test]
fn test_field_hash_boolean() {
    let field = HashField {
        name: "active".to_string(),
        ts_type: "boolean".to_string(),
    };
    let result = generate_field_hash_for_interface(&field, "value", None, None);
    assert!(result.contains("1231")); // Java's Boolean.hashCode() constants
    assert!(result.contains("1237"));
}

#[test]
fn test_field_hash_date() {
    let field = HashField {
        name: "createdAt".to_string(),
        ts_type: "Date".to_string(),
    };
    let result = generate_field_hash_for_interface(&field, "value", None, None);
    assert!(result.contains("getTime"));
}

#[test]
fn test_field_hash_object() {
    let field = HashField {
        name: "user".to_string(),
        ts_type: "User".to_string(),
    };
    // Without registry — duck-typing fallback
    let result = generate_field_hash_for_interface(&field, "value", None, None);
    assert!(result.contains("hashCode"));
}
