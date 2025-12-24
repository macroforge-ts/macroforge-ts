use crate::template::parse_template;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;

#[test]
fn test_static_template_emits_string_building() {
    let input = quote! {
        const value = 1;
    };
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses string building and runtime SWC parsing
    assert!(
        s.contains("push_str"),
        "Expected push_str for string building in generated output. Got: {}",
        s
    );
    assert!(
        s.contains("swc_core"),
        "Expected swc_core for runtime parsing. Got: {}",
        s
    );
}

#[test]
fn test_interpolation_expr_binding() {
    let input = TokenStream2::from_str("const value = @{expr};").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses ToTsExpr or ToTsStmt trait depending on context
    assert!(
        s.contains("to_ts_expr") || s.contains("ToTsExpr") ||
        s.contains("to_ts_stmt") || s.contains("ToTsStmt"),
        "Expected ToTs conversion for expression interpolation. Got: {}",
        s
    );
}

#[test]
fn test_ident_block_binding() {
    let input = TokenStream2::from_str("const {|foo@{bar}|} = 1;").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler builds identifiers as strings
    assert!(
        s.contains("push_str"),
        "Expected string building for ident blocks. Got: {}",
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
    assert!(s.contains("push_str"), "Expected string building. Got: {}", s);
}

#[test]
fn test_string_literal_interpolation() {
    let input = TokenStream2::from_str("const msg = \"Hello @{name}!\";").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses string building
    assert!(
        s.contains("push_str"),
        "Expected push_str for string building. Got: {}",
        s
    );
}

#[test]
fn test_backtick_template_literal_syntax() {
    let input = TokenStream2::from_str("const html = \"'^<@{tag}>${content}</@{tag}>^'\";").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    // New compiler uses string building and ToTsExpr for interpolations
    assert!(
        s.contains("push_str"),
        "Expected push_str for string building. Got: {}",
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

    // New compiler uses ToTsIdent for identifier placeholders
    assert!(
        s.contains("to_ts_ident") || s.contains("ToTsIdent") || s.contains("push_str"),
        "Expected identifier handling for function name. Got: {}",
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
    assert!(s.contains("push_str"), "Expected string building. Got: {}", s);
}
