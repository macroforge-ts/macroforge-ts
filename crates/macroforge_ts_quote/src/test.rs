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

    // New compiler uses compile-time SWC quote! for static templates
    assert!(
        s.contains("swc_core :: quote !"),
        "Expected compile-time SWC quote! parsing. Got: {}",
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
    let input = TokenStream2::from_str("const {|foo@{bar}|} = 1;").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler builds identifiers dynamically
    assert!(
        s.contains("__ident") || s.contains("Ident"),
        "Expected identifier building for ident blocks. Got: {}",
        s
    );
}

#[test]
fn test_if_expression_in_statement() {
    let input = TokenStream2::from_str(
        "const status = {#if cond} \"a\" {:else} \"b\" {/if}",
    )
    .unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler generates Rust if statements for control flow
    assert!(s.contains("if cond"), "Expected Rust if for expression control. Got: {}", s);
}

#[test]
fn test_string_literal_interpolation() {
    let input = TokenStream2::from_str("const msg = \"Hello @{name}!\";").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses swc_core::quote! with placeholders
    assert!(
        s.contains("swc_core :: quote !"),
        "Expected swc_core::quote! for template. Got: {}",
        s
    );
}

#[test]
fn test_backtick_template_literal_syntax() {
    let input = TokenStream2::from_str("const html = \"'^<@{tag}>${content}</@{tag}>^'\";").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses swc_core::quote! with ToTsExpr for interpolations
    assert!(
        s.contains("swc_core :: quote !"),
        "Expected swc_core::quote! for template. Got: {}",
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
        s.contains("to_ts_ident") || s.contains("ToTsIdent") ||
        s.contains("to_ts_expr") || s.contains("ToTsExpr"),
        "Expected identifier/expression handling for function name. Got: {}",
        s
    );
}

#[test]
fn test_dynamic_function_body() {
    let input = TokenStream2::from_str("function test() { {#if true} console.log(\"hi\"); {/if} }").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler generates if statements for control flow
    assert!(s.contains("if true"), "Expected Rust if statement. Got: {}", s);
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
        "TokenStream should preserve doc comment somehow. Got: {}", template_str
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
        "fn_name should use ToTsIdent for function name. Generated:\n{}", s
    );

    // type_param after ":" should use ToTsType
    assert!(
        s.contains("to_ts_type"),
        "type_param should use ToTsType for parameter type. Generated:\n{}", s
    );

    // body_expr in function body should use ToTsExpr
    assert!(
        s.contains("to_ts_expr"),
        "body_expr should use ToTsExpr for expression. Generated:\n{}", s
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
        "Expected 2 function names to use ToTsIdent, found {}. Generated:\n{}", ident_count, s
    );

    // Count occurrences of to_ts_type - should be 2 (one for each parameter type)
    let type_count = s.matches("to_ts_type").count();
    assert_eq!(
        type_count, 2,
        "Expected 2 parameter types to use ToTsType, found {}. Generated:\n{}", type_count, s
    );

    // Count occurrences of to_ts_expr - should be 2 (one for each body expression)
    let expr_count = s.matches("to_ts_expr").count();
    assert_eq!(
        expr_count, 2,
        "Expected 2 body expressions to use ToTsExpr, found {}. Generated:\n{}", expr_count, s
    );
}
