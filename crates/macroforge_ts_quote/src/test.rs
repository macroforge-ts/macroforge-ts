use crate::template::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;

// Helper for 'capitalize' function for test setup
// Removed unused capitalize function

#[test]
fn test_at_interpolation_glue() {
    // Rust allows `make@{Name}` naturally without spaces.
    // We verify that `make@{Name}` results in "make" + value (no space).
    let input = quote! {
        make@{name_ident}
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should generate code that pushes "make"
    assert!(s.contains("\"make\""));
    // Should generate code that pushes the interpolated value
    assert!(
        s.contains("name_ident . to_string ()"),
        "Expected generated code to include name_ident.to_string()"
    );

    // The concatenation assertion is based on the logic of template.rs
    // which tries to avoid spaces when not needed.
    // The previous logic was `make ` + `MyName` (from interpolation)
    // The new logic should try to avoid the space.
    // The test cannot directly assert the *final rendered string* because it depends on the runtime value of name_ident.
    // Instead, it asserts the *structure of the generated code*.
    // For the original problem, the output string to be parsed by SWC would have been "make MyNameBaseProps".
    // We are testing that "make" and "MyName" (from interpolation) should be contiguous.
    // But this test specifically checks "make@{name_ident}".
    // `make@{name_ident}` should result in a generated string that does NOT contain "make ".
    // Instead, it should just be "make" followed by the interpolation result.
    // However, the test is asserting about the *generated Rust code*, not the final *TypeScript string*.

    // The intent of this test is to verify the spacing logic around `@{}`.
    // If `is_ident { is_ts_keyword(&s) || !next_is_at }` where `s="make"` `is_ts_keyword` is false, `next_is_at` is true. So `!next_is_at` is false.
    // Thus `is_ts_keyword(&s) || !next_is_at` is `false || false` -> `false`. No space after `make`.
    // So `__out.push_str("make"); __out.push_str(&name_ident.to_string());`
    // The generated string `s` from output.unwrap().to_string() would be `__out.push_str("make"); __out.push_str(& name_ident . to_string ());`
    // This is correct.

    // Let's modify the assertion to reflect what is actually generated.
    assert!(
        s.contains("__out . push_str (\"make\") ;"),
        "Generated code should push \"make\""
    );
    assert!(
        s.contains("__out . push_str (& name_ident . to_string ()) ;"),
        "Generated code should push interpolated name_ident"
    );
    // The "makeMyName" assertion is actually checking if there's no space in the Rust code generating string.
    // This is okay.
    // assert!(s.contains("makeMyName"), "Expected 'make' and 'MyName' to be concatenated without space"); // This is wrong for generated code.
}

#[test]
fn test_let_scope() {
    let input = TokenStream2::from_str(
        r###"            {%let val_local = val}
            @{val_local}
        "###,
    )
    .unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should contain "let val = \"hello\" ;"
    assert!(s.contains("let val_local = val"));

    // Should contain usage
    assert!(s.contains("val_local . to_string"));
}

#[test]
fn test_for_loop() {
    let input = TokenStream2::from_str(
        r###"            {#for item in items}
                @{item}
            {/for}
        "###,
    )
    .unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should generate a for loop
    assert!(s.contains("for item in items"), "Should generate for loop");
    assert!(s.contains("item . to_string"), "Should interpolate item");
}

#[test]
fn test_if_else() {
    let input = TokenStream2::from_str(
        r###"            {#if condition}
                "true"
            {:else}
                "false"
            {/if}
        "###,
    )
    .unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should generate an if-else
    assert!(s.contains("if condition"), "Should generate if condition");
    assert!(s.contains("else"), "Should have else branch");
}

#[test]
fn test_if_else_if() {
    let input = TokenStream2::from_str(
        r###"            {#if a}
                "a"
            {:else if b}
                "b"
            {:else}
                "c"
            {/if}
        "###,
    )
    .unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should generate if/else if/else chain
    assert!(s.contains("if a"), "Should have if a");
    assert!(s.contains("if b"), "Should have else if b");
    assert!(s.contains("else"), "Should have else");
}

#[test]
fn test_string_interpolation_simple() {
    // Test that @{expr} inside strings gets interpolated
    let input = quote! {
        "Hello @{name}!"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should push the opening quote
    // The generated code will look something like: __out . push_str ("\"") ;
    assert!(s.contains("push_str (\"\\\"\")"), "Should push opening quote");
    // Should push "Hello "
    assert!(s.contains("\"Hello \""), "Should push 'Hello '");
    // Should interpolate name
    assert!(s.contains("name . to_string"), "Should interpolate name");
    // Should push "!"
    assert!(s.contains("\"!\""), "Should push '!'");
}

#[test]
fn test_string_no_interpolation() {
    // Strings without @{} should pass through unchanged
    let input = quote! {
        "Just a plain string"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should just push the whole string as-is (escaped in the generated code)
    assert!(
        s.contains("Just a plain string"),
        "Should contain the string content"
    );
}

#[test]
fn test_string_interpolation_multiple() {
    // Test multiple interpolations in one string

    let input = quote! {
        "@{greeting}, @{name}!"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should interpolate both
    assert!(
        s.contains("greeting . to_string"),
        "Should interpolate greeting"
    );
    assert!(s.contains("name . to_string"), "Should interpolate name");
}

#[test]
fn test_string_interpolation_with_method_call() {
    // Test that expressions with method calls work
    let input = quote! {
        "Name: @{name.to_uppercase()}"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should contain the method call
    assert!(s.contains("to_uppercase"), "Should contain method call");
}

#[test]
fn test_backtick_template_simple() {
    // Test that "'^...^'" outputs backtick template literals
    let input = quote! {
        "'^hello ${name}^'"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should push backtick at start
    assert!(s.contains("\"`\""), "Should push opening backtick");
    // Should contain the template content with ${name} passed through
    assert!(
        s.contains("hello ${name}"),
        "Should contain template content"
    );
}

#[test]
fn test_backtick_template_with_rust_interpolation() {
    // Test that @{} works inside backtick templates for Rust expressions
    let input = quote! {
        "'^hello @{rust_var}^'"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should push backtick
    assert!(s.contains("\"`\""), "Should push backtick");
    // Should interpolate the Rust variable
    assert!(
        s.contains("rust_var . to_string"),
        "Should interpolate Rust var"
    );
}

#[test]
fn test_backtick_template_mixed() {
    // Test mixing JS ${} and Rust @{} in backtick templates
    let input = quote! {
        "'^${js_var} and @{rust_var}^'"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should contain JS interpolation passed through
    assert!(
        s.contains("${js_var}"),
        "Should pass through JS interpolation"
    );
    // Should interpolate Rust variable
    assert!(
        s.contains("rust_var . to_string"),
        "Should interpolate Rust var"
    );
}

#[test]
fn test_at_symbol_without_brace_passes_through() {
    // Test that @ not followed by { passes through unchanged
    let input = quote! {
        "email@domain.com"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should contain literal @ (no escaping needed)
    assert!(
        s.contains("email@domain.com"),
        "Should pass through @ unchanged"
    );
}

#[test]
fn test_at_symbol_in_backtick_passes_through() {
    // Test that @ not followed by { passes through in backtick templates
    let input = quote! {
        "'^email@domain.com^'"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should contain backticks and the literal @
    assert!(s.contains("\"`\""), "Should push backtick");
    assert!(
        s.contains("email@domain.com"),
        "Should pass through @ unchanged"
    );
}

#[test]
fn test_escape_at_before_brace() {
    // Test that @@{ produces a literal @{ (not interpolation)
    let input = quote! {
        "use @@{decorators}"
    };
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should contain literal @{decorators} not try to interpolate
    let expected = "@{decorators}";
    assert!(
        s.contains(expected),
        "Should contain literal @{{decorators}}"
    );
    // Should NOT try to interpolate 'decorators'
    assert!(
        !s.contains("decorators . to_string"),
        "Should not interpolate"
    );
}

#[test]
fn test_if_let_simple() {
    let input = TokenStream2::from_str(
        r###"            {#if let Some(value) = option}
                @{value}
            {/if}
        "###,
    )
    .unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should generate if let
    assert!(
        s.contains("if let Some (value)"),
        "Should have if let pattern"
    );
    assert!(s.contains("= option"), "Should have expression");
}

#[test]
fn test_if_let_with_else() {
    let input = TokenStream2::from_str(
        r###"            {#if let Some(x) = maybe}
                "found"
            {:else}
                "not found"
            {/if}
        "###,
    )
    .unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should generate if let with else
    assert!(s.contains("if let Some (x)"), "Should have if let pattern");
    assert!(s.contains("else"), "Should have else branch");
}

#[test]
fn test_match_simple() {
    let input = TokenStream2::from_str(
        r###"            {#match value}
                {:case Some(x)}
                    @{x}
                {:case None}
                    "nothing"
            {/match}
        "###,
    )
    .unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should generate match
    assert!(s.contains("match value"), "Should have match expr");
    assert!(s.contains("Some (x) =>"), "Should have Some case arm");
    assert!(s.contains("None =>"), "Should have None case arm");
}

#[test]
fn test_match_with_wildcard() {
    let input = TokenStream2::from_str(
        r###"            {#match num}
                {:case 1}
                    "one"
                {:case 2}
                    "two"
                {:case _}
                    "other"
            {/match}
        "###,
    )
    .unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should generate match with all arms
    assert!(s.contains("match num"), "Should have match expr");
    assert!(s.contains("1 =>"), "Should have case 1");
    assert!(s.contains("2 =>"), "Should have case 2");
    assert!(s.contains("_ =>"), "Should have wildcard case");
}

#[test]
fn test_match_with_interpolation() {
    let input = TokenStream2::from_str(
        r###"            {#match result}
                {:case Ok(val)}
                    "success: @{val}"
                {:case Err(e)}
                    "error: @{e}"
            {/match}
        "###,
    )
    .unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should interpolate values in match arms
    assert!(s.contains("val . to_string"), "Should interpolate val");
    assert!(s.contains("e . to_string"), "Should interpolate e");
}

// ========== Ident Block {| |} Tests ==========

#[test]
fn test_ident_block_basic() {
    // {| content |} should concatenate without spaces
    let input = TokenStream2::from_str(r#"{|namespace@{suffix}|}"#).unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should have interpolation without spacing
    assert!(s.contains("suffix . to_string"), "Should interpolate suffix");
    // Should emit "namespace" without trailing space
    assert!(
        s.contains(r#"__out . push_str ("namespace")"#),
        "Should emit namespace without space"
    );
}

#[test]
fn test_ident_block_multiple_parts() {
    // Multiple consecutive ident blocks should concatenate
    let input = TokenStream2::from_str(r#"{|@{a}|}{|@{b}|}{|@{c}|}"#).unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should have all three interpolations
    assert!(s.contains("a . to_string"), "Should interpolate a");
    assert!(s.contains("b . to_string"), "Should interpolate b");
    assert!(s.contains("c . to_string"), "Should interpolate c");
}

#[test]
fn test_ident_block_with_surrounding_text() {
    // Ident block within normal template context
    let input = TokenStream2::from_str(r#"function {|get@{name}|}() { }"#).unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should have the function keyword and interpolation
    assert!(s.contains(r#""function""#), "Should have function keyword");
    assert!(s.contains("name . to_string"), "Should interpolate name");
    assert!(s.contains(r#"__out . push_str ("get")"#), "Should have get prefix");
}

#[test]
fn test_ident_block_empty() {
    // Empty ident block {||} should produce nothing
    let input = TokenStream2::from_str(r#"prefix{||}suffix"#).unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should have prefix and suffix
    assert!(s.contains(r#""prefix""#), "Should have prefix");
    assert!(s.contains(r#""suffix""#), "Should have suffix");
}

#[test]
fn test_ident_block_plain_text_only() {
    // Ident block with just plain text (no interpolation)
    let input = TokenStream2::from_str(r#"{|hello|}"#).unwrap();
    let output = parse_template(input);
    let s = output.unwrap().to_string();

    // Should emit "hello" without spaces
    assert!(s.contains(r#"__out . push_str ("hello")"#), "Should emit hello");
}
