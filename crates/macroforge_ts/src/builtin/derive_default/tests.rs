use super::types::DefaultField;
use crate::macros::ts_template;
use crate::ts_syn::parse_ts_expr;
use crate::ts_syn::ts_ident;

#[test]
fn test_default_macro_output() {
    let class_name = "User";
    let class_ident = ts_ident!(class_name);

    let default_fields: Vec<DefaultField> = vec![
        DefaultField {
            name: "id".to_string(),
            value: "0".to_string(),
        },
        DefaultField {
            name: "name".to_string(),
            value: r#""""#.to_string(),
        },
    ];

    let output = ts_template!(Within {
        static defaultValue(): @{class_ident.clone()} {
            const instance = new @{class_ident.clone()}();
            {#if !default_fields.is_empty()}
                {#for f in default_fields.iter()}
                    instance.@{ts_ident!(f.name.as_str())} = @{*parse_ts_expr(&f.value).expect("should parse")};
                {/for}
            {/if}
            return instance;
        }
    });

    let source = output.source();
    let body_content = source
        .strip_prefix("/* @macroforge:body */")
        .unwrap_or(source);
    let wrapped = format!("class __Temp {{ {} }}", body_content);

    assert!(
        macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
        "Generated Default macro output should parse as class members"
    );
    assert!(
        source.contains("defaultValue"),
        "Should contain defaultValue method"
    );
    assert!(source.contains("static"), "Should be a static method");
}

#[test]
fn test_default_field_assignment() {
    let fields: Vec<DefaultField> = vec![
        DefaultField {
            name: "count".to_string(),
            value: "42".to_string(),
        },
        DefaultField {
            name: "items".to_string(),
            value: "[]".to_string(),
        },
    ];

    let assignments = fields
        .iter()
        .map(|f| format!("instance.{} = {};", f.name, f.value))
        .collect::<Vec<_>>()
        .join("\n");

    assert!(assignments.contains("instance.count = 42;"));
    assert!(assignments.contains("instance.items = [];"));
}
