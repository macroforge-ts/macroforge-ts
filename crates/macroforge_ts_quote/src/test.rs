use crate::template::parse_template;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;

#[test]
fn test_static_template_emits_quote() {
    let input = quote! {
        const value = 1;
    };
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    assert!(
        s.contains("swc_core :: quote"),
        "Expected swc_core::quote! in generated output"
    );
    assert!(
        s.contains("Vec < swc_core :: ecma :: ast :: Stmt >"),
        "Expected Vec<Stmt> output type"
    );
}

#[test]
fn test_interpolation_expr_binding() {
    let input = TokenStream2::from_str("const value = @{expr};").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    assert!(
        s.contains("swc_core :: ecma :: ast :: Expr"),
        "Expected Expr-typed interpolation binding"
    );
}

#[test]
fn test_ident_block_binding() {
    let input = TokenStream2::from_str("const {|foo@{bar}|} = 1;").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    assert!(
        s.contains("swc_core :: ecma :: ast :: Ident :: new"),
        "Expected Ident construction for ident blocks"
    );
}

#[test]
fn test_if_expression_in_statement() {
    let input = TokenStream2::from_str(
        "const status = {#if cond} \"a\" {:else} \"b\" {/if};",
    )
    .unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    assert!(s.contains("if cond"), "Expected Rust if for expression control");
    assert!(s.contains("swc_core :: quote"), "Expected quote! usage");
}

#[test]
fn test_string_literal_interpolation() {
    let input = TokenStream2::from_str("const msg = \"Hello @{name}!\";").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    assert!(
        s.contains("__mf_str_"),
        "Expected nested template literal bindings for string interpolation"
    );
    assert!(
        s.contains("to_ts_expr"),
        "Expected to_ts_expr conversion for string interpolation"
    );
    assert!(
        s.contains("swc_core :: quote"),
        "Expected quote! usage in string interpolation"
    );
}

#[test]
fn test_backtick_template_literal_syntax() {
    let input = TokenStream2::from_str("const html = \"'^<@{tag}>${content}</@{tag}>^'\";").unwrap();
    let output = parse_template(input).unwrap();
    let s = output.to_string();

    assert!(
        s.contains("__mf_tpl_"),
        "Expected template literal placeholder bindings"
    );
    assert!(
        s.contains("to_ts_expr"),
        "Expected to_ts_expr conversion for template literal interpolation"
    );
    assert!(
        s.contains("swc_core :: quote"),
        "Expected quote! usage in template literal interpolation"
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
        "Expected doc comments to be preserved in generated output"
    );
    assert!(
        s.contains("__pending_comments"),
        "Expected pending comment buffer in generated output"
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
