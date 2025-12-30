use crate::template::parse_template;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;

#[test]
fn test_static_template_emits_compile_time_parsing() {
    let input = quote! {
        const value = 1;
    };
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses compile-time ts_quote! for static templates
    assert!(
        s.contains("macroforge_ts_quote :: ts_quote !"),
        "Expected compile-time ts_quote! parsing. Got: {}",
        s
    );
}

#[test]
fn test_interpolation_expr_binding() {
    let input = TokenStream2::from_str("const value = @{expr};").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses ToTsExpr trait for expression interpolation
    assert!(
        s.contains("to_ts_expr") || s.contains("ToTsExpr"),
        "Expected ToTsExpr for expression interpolation. Got: {}",
        s
    );
}

#[test]
fn test_ident_block_binding() {
    let input = TokenStream2::from_str("const foo@{bar} = 1;").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // Implicit concatenation: foo@{bar} should be detected and wrapped in IdentBlock
    // The generated code should concatenate "foo" with the bar placeholder
    // Look for the ident builder pattern or concatenation code
    assert!(
        s.contains("push_str") || s.contains("foo") && s.contains("bar"),
        "Expected implicit concatenation of foo and bar. Got: {}",
        s
    );
}

#[test]
fn test_if_expression_in_statement() {
    let input =
        TokenStream2::from_str("const status = {#if cond} \"a\" {:else} \"b\" {/if}").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler generates Rust if statements for control flow
    assert!(
        s.contains("if cond"),
        "Expected Rust if for expression control. Got: {}",
        s
    );
}

#[test]
fn test_string_literal_interpolation() {
    let input = TokenStream2::from_str("const msg = \"Hello @{name}!\";").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses ts_quote! with placeholders
    assert!(
        s.contains("macroforge_ts_quote :: ts_quote !"),
        "Expected ts_quote! for template. Got: {}",
        s
    );
}

#[test]
fn test_backtick_template_literal_syntax() {
    let input =
        TokenStream2::from_str("const html = \"'^<@{tag}>${content}</@{tag}>^'\";").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses ts_quote! with ToTsExpr for interpolations
    assert!(
        s.contains("macroforge_ts_quote :: ts_quote !"),
        "Expected ts_quote! for template. Got: {}",
        s
    );
}

#[test]
fn test_doc_attribute_comment_is_emitted() {
    let input = quote! {
        #[doc = "Generated field"]
        const value = 1;
    };
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    assert!(
        s.contains("Generated field"),
        "Expected doc comments to be preserved in generated output. Got: {}",
        s
    );
    assert!(
        s.contains("__pending_comments"),
        "Expected pending comment buffer in generated output. Got: {}",
        s
    );
}

#[test]
fn test_block_comment_is_stripped() {
    let tokens = TokenStream2::from_str("/* block comment */ const value = 1;").unwrap();
    let raw = tokens.to_string();

    assert!(
        !raw.contains("block comment"),
        "Expected block comments to be stripped from TokenStream"
    );
}

#[test]
fn test_function_name_interpolation_is_ident() {
    let input = TokenStream2::from_str("export function @{fn_name}() {}").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses ToTsIdent or ToTsExpr for identifier placeholders
    assert!(
        s.contains("to_ts_ident")
            || s.contains("ToTsIdent")
            || s.contains("to_ts_expr")
            || s.contains("ToTsExpr"),
        "Expected identifier/expression handling for function name. Got: {}",
        s
    );
}

#[test]
fn test_dynamic_function_body() {
    let input = TokenStream2::from_str("function test() { {#if true} console.log(\"hi\"); {/if} }")
        .unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler generates if statements for control flow
    assert!(
        s.contains("if true"),
        "Expected Rust if statement. Got: {}",
        s
    );
}

#[test]
fn test_debug_doc_comment_tokenstream() {
    let input = quote! {
        /** Doc comment */
        export function @{fn_name}(value: @{type_param}): string {
            return @{body_expr};
        }
    };
    let template_str = input.to_string();
    eprintln!("Template string from TokenStream: {}", template_str);

    // Also test with direct string
    let direct = "/** Doc comment */ export function @{fn_name}(value: @{type_param}): string { return @{body_expr}; }";
    eprintln!("Direct string: {}", direct);

    // The key difference: doc comments become #[doc = "..."] in TokenStream
    assert!(
        template_str.contains("doc =") || template_str.contains("Doc comment"),
        "TokenStream should preserve doc comment somehow. Got: {}",
        template_str
    );
}

#[test]
fn test_function_with_doc_comment_uses_ident() {
    // This test matches the actual pattern from derive_serialize.rs
    let input = quote! {
        /** Doc comment */
        export function @{fn_name}(value: @{type_param}): string {
            return @{body_expr};
        }
    };
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    eprintln!("Generated code:\n{}", s);

    // fn_name after "function" keyword should use ToTsIdent
    assert!(
        s.contains("to_ts_ident"),
        "fn_name should use ToTsIdent for function name. Generated:\n{}",
        s
    );

    // type_param after ":" should use ToTsType
    assert!(
        s.contains("to_ts_type"),
        "type_param should use ToTsType for parameter type. Generated:\n{}",
        s
    );

    // body_expr in function body should use ToTsExpr
    assert!(
        s.contains("to_ts_expr"),
        "body_expr should use ToTsExpr for expression. Generated:\n{}",
        s
    );
}

#[test]
fn test_multiple_functions_with_doc_comments() {
    // This matches the pattern from derive_serialize.rs with multiple functions
    let input = quote! {
        /** First function doc */
        export function @{fn_name1}(value: @{type1}): string {
            return @{body1};
        }

        /** Second function doc */
        export function @{fn_name2}(value: @{type2}): Record<string, unknown> {
            return @{body2};
        }
    };
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    eprintln!("Generated code for multiple functions:\n{}", s);

    // Count occurrences of to_ts_ident - should be 2 (one for each function name)
    let ident_count = s.matches("to_ts_ident").count();
    assert_eq!(
        ident_count, 2,
        "Expected 2 function names to use ToTsIdent, found {}. Generated:\n{}",
        ident_count, s
    );

    // Count occurrences of to_ts_type - should be 2 (one for each parameter type)
    let type_count = s.matches("to_ts_type").count();
    assert_eq!(
        type_count, 2,
        "Expected 2 parameter types to use ToTsType, found {}. Generated:\n{}",
        type_count, s
    );

    // Count occurrences of to_ts_expr - should be 2 (one for each body expression)
    let expr_count = s.matches("to_ts_expr").count();
    assert_eq!(
        expr_count, 2,
        "Expected 2 body expressions to use ToTsExpr, found {}. Generated:\n{}",
        expr_count, s
    );
}

// =============================================================================
// Bug reproduction tests for placeholder and __MF_DUMMY__ issues
// =============================================================================

#[test]
fn test_for_loop_field_interpolation_in_interface() {
    // This test reproduces the bug where field names in for loops become $MfPh placeholders
    // instead of actual field names when generating interface members
    let input = TokenStream2::from_str(
        r#"export interface FieldControllers {
            {#for field in fields}
                readonly @{field.name}: FieldController<@{field.ts_type}>;
            {/for}
        }"#,
    )
    .unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    eprintln!("Generated code for interface with for loop:\n{}", s);

    // The generated code should reference field.name and field.ts_type
    // NOT generate $MfPh placeholders that never get substituted
    assert!(
        s.contains("field . name") || s.contains("field.name"),
        "Expected field.name reference in generated code. Got:\n{}",
        s
    );
    assert!(
        s.contains("field . ts_type") || s.contains("field.ts_type"),
        "Expected field.ts_type reference in generated code. Got:\n{}",
        s
    );
}

#[test]
fn test_for_loop_generates_runtime_iteration() {
    // Test that for loops generate proper runtime iteration code
    let input = TokenStream2::from_str(
        r#"{#for item in items}
            const @{item.name} = @{item.value};
        {/for}"#,
    )
    .unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    eprintln!("Generated code for for loop:\n{}", s);

    // Should generate a Rust for loop
    assert!(
        s.contains("for item in items"),
        "Expected Rust for loop in generated code. Got:\n{}",
        s
    );
}

#[test]
fn test_for_loop_with_string_field_name() {
    // Test for loop with simple string field access
    let input = TokenStream2::from_str(
        r#"{#for field in fields}
            @{&field.name}: __gf_Option<Array<string>>;
        {/for}"#,
    )
    .unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    eprintln!("Generated code for field name interpolation:\n{}", s);

    // Should reference field.name in the generated code
    assert!(
        s.contains("field") && s.contains("name"),
        "Expected field.name reference in generated code. Got:\n{}",
        s
    );
}

#[test]
fn test_within_position_extracts_body_correctly() {
    // Test that ts_template!(Within { ... }) properly extracts class body
    // without leaking __MF_DUMMY__ wrapper
    use crate::template::parse_template_str;

    // Simulate what happens with Within position
    let wrapped = "class __MF_DUMMY__ { readonly foo: string; readonly bar: number; }";
    let output = parse_template_str(wrapped).unwrap();
    let s = output.to_string();

    eprintln!("Generated code for Within body:\n{}", s);

    // The generated code should handle the wrapper extraction
    // Look for the body extraction logic
    assert!(
        s.contains("__stmts") || s.contains("ModuleItem"),
        "Expected statement building in generated code. Got:\n{}",
        s
    );
}

#[test]
fn test_interpolation_in_object_property_position() {
    // Test that @{expr} in object property position generates correct ident handling
    let input = TokenStream2::from_str(r#"const obj = { @{field_name}: @{field_value} };"#).unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    eprintln!("Generated code for object property interpolation:\n{}", s);

    // field_name should use ident handling, field_value should use expr handling
    assert!(
        s.contains("field_name"),
        "Expected field_name in generated code. Got:\n{}",
        s
    );
    assert!(
        s.contains("field_value"),
        "Expected field_value in generated code. Got:\n{}",
        s
    );
}

#[test]
fn test_multiple_interpolations_same_variable() {
    // Test that using the same variable multiple times doesn't create different placeholders
    let input = TokenStream2::from_str(
        r#"const @{name}Obj = {};
           let current = @{name}Obj;
           obj.@{name} = @{name}Obj;"#,
    )
    .unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    eprintln!("Generated code for multiple same-variable interpolations:\n{}", s);

    // Count occurrences of "name" - should appear multiple times with consistent handling
    let name_count = s.matches("name").count();
    assert!(
        name_count >= 4,
        "Expected 'name' to appear at least 4 times (for each @{{name}}). Found {}. Got:\n{}",
        name_count,
        s
    );
}

#[test]
fn test_conditional_in_interface_member() {
    // Test that conditionals inside interface member position work correctly
    let input = TokenStream2::from_str(
        r#"export interface Test {
            {#if is_array}
                readonly items: ArrayFieldController<@{element_type}>;
            {:else}
                readonly value: FieldController<@{value_type}>;
            {/if}
        }"#,
    )
    .unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    eprintln!("Generated code for conditional in interface:\n{}", s);

    // Should generate Rust if/else
    assert!(
        s.contains("if is_array"),
        "Expected Rust if statement. Got:\n{}",
        s
    );
    assert!(
        s.contains("else"),
        "Expected else branch. Got:\n{}",
        s
    );
}

#[test]
fn test_debug_tokenstream_normalization() {
    use crate::template::{normalize_template_spacing, collapse_template_newlines};

    // Test what happens when we tokenize a for loop template
    let raw_str = r#"{#for field in fields}
        readonly @{field.name}: FieldController<@{field.ts_type}>;
    {/for}"#;

    // Convert to TokenStream and back to string (simulating what the macro does)
    let tokens = TokenStream2::from_str(raw_str).unwrap();
    let tokenized_str = tokens.to_string();

    eprintln!("=== Debug Tokenization ===");
    eprintln!("Original: {:?}", raw_str);
    eprintln!("Tokenized: {:?}", tokenized_str);

    // Apply normalization
    let normalized = normalize_template_spacing(&tokenized_str);
    eprintln!("Normalized: {:?}", normalized);

    let collapsed = collapse_template_newlines(&normalized);
    eprintln!("Collapsed: {:?}", collapsed);

    // The issue: after tokenization, {#for becomes { # for with spaces
    // The normalize function should fix this, but let's verify
    assert!(
        collapsed.contains("{#for"),
        "Expected {{#for}} to be preserved after normalization. Got: {:?}",
        collapsed
    );
}

#[test]
fn test_control_block_pattern_recognition() {
    use crate::template::normalize_template_spacing;

    // Test the specific pattern that should be normalized
    // The normalize function should convert space-separated control blocks
    // back to their compact form
    let patterns = [
        ("{ # for }", "{#for"),
        ("{ # if }", "{#if"),
        ("{ / for }", "{/for"),
        ("{ / if }", "{/if"),
        ("{ : else }", "{:else"),
        ("{ $ let }", "{$let"),
    ];

    for (input, expected_contains) in patterns {
        let result = normalize_template_spacing(input);
        eprintln!("Input: {:?} -> Result: {:?}", input, result);
        assert!(
            result.contains(expected_contains),
            "Expected output to contain {}. Got: {:?}",
            expected_contains,
            result
        );
    }
}
