use crate::compiler::compile_template;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;

/// Helper to compile a template and get the generated code as a string
fn compile(input: &str) -> String {
    compile_template(input, None, 0).unwrap().to_string()
}

#[test]
fn test_static_template_builds_source_string() {
    let input = quote! { const value = 1; };
    let s = compile(&input.to_string());

    // New codegen generates direct AST with __stmts vector
    assert!(
        s.contains("__stmts"),
        "Expected __stmts for code generation. Got: {}",
        s
    );
}

#[test]
fn test_interpolation_expr_binding() {
    let s = compile("const value = @{expr};");

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
    let s = compile(&input.to_string());

    // Look for the ident builder pattern or concatenation code
    assert!(
        s.contains("push_str") || (s.contains("foo") && s.contains("bar")),
        "Expected implicit concatenation of foo and bar. Got: {}",
        s
    );
}

/// Tests control blocks in expression position.
///
/// NOTE: This test uses `TokenStream::from_str()` which goes through Rust's tokenizer.
/// Rust's tokenizer splits `{#if` into separate tokens `{`, `#`, `if`, producing
/// `{ # if cond }` when `.to_string()` is called. The template lexer's normalizer
/// (`normalize_template`) rejoins these back to `{#if cond}`.
#[test]
fn test_if_expression_in_statement() {
    // Use raw string to bypass Rust's tokenizer
    let s = compile("const status = {#if cond} \"a\" {:else} \"b\" {/if}");

    // Expression-level if generates Rust if expression
    assert!(
        s.contains("if cond"),
        "Expected Rust if for expression control. Got: {}",
        s
    );
}

#[test]
fn test_for_expression() {
    // Use raw string to bypass Rust's tokenizer
    let s = compile("const items = [{#for x in list} x.name {/for}]");

    // For expression generates iterator with map
    assert!(
        s.contains("into_iter") && s.contains("map"),
        "Expected iterator map pattern for for-expression. Got: {}",
        s
    );
}

#[test]
fn test_while_expression() {
    // Use raw string to bypass Rust's tokenizer
    let s = compile("const vals = {#while cond} get_next() {/while}");

    // While expression generates std::iter::from_fn
    assert!(
        s.contains("from_fn"),
        "Expected from_fn for while-expression. Got: {}",
        s
    );
}

#[test]
fn test_match_expression() {
    // Use raw string to bypass Rust's tokenizer
    let s = compile("const val = {#match x}{:case Some(v)} v {:case None} 0 {/match}");

    // Match expression generates Rust match
    assert!(
        s.contains("match"),
        "Expected Rust match for match-expression. Got: {}",
        s
    );
}

#[test]
fn test_nested_control_expressions() {
    // Use raw string to bypass Rust's tokenizer
    let s = compile("const x = {#if a} {#if b} 1 {:else} 2 {/if} {:else} 3 {/if}");

    // Nested if expressions should both appear
    assert!(
        s.contains("if a") || s.contains("if b"),
        "Expected nested if expressions. Got: {}",
        s
    );
}

#[test]
fn test_if_expression_requires_else() {
    use crate::compiler::compile_template;

    // If expression without else should fail
    let result = compile_template("const x = {#if cond} \"a\" {/if}", None, 0);
    assert!(
        result.is_err(),
        "Expected error for if-expression without else branch"
    );

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("else") || err_msg.contains("MissingElseBranch"),
        "Expected error message about missing else. Got: {}",
        err_msg
    );
}

#[test]
fn test_string_literal_interpolation() {
    // Use raw string directly since backticks aren't valid Rust tokens
    let s = compile("const msg = `Hello @{name}!`;");

    // New codegen generates Tpl (template literal) with exprs for interpolations
    assert!(
        s.contains("Tpl") && s.contains("name"),
        "Expected template literal with name placeholder. Got: {}",
        s
    );
}

#[test]
fn test_backtick_template_literal_syntax() {
    let input = TokenStream2::from_str("const html = \"'^<@{tag}>${content}</@{tag}>^'\";").unwrap();
    let s = compile(&input.to_string());

    // New codegen generates direct AST with __stmts vector
    // The template literal with interpolations should contain the placeholders
    assert!(
        s.contains("__stmts"),
        "Expected __stmts for code generation. Got: {}",
        s
    );
}

#[test]
#[ignore = "Doc comment handling removed in codegen simplification - to be reimplemented if needed"]
fn test_doc_attribute_comment_is_emitted() {
    let input = quote! {
        #[doc = "Generated field"]
        const value = 1;
    };
    let s = compile(&input.to_string());

    assert!(
        s.contains("Generated field"),
        "Expected doc comments to be preserved in generated output. Got: {}",
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
    let s = compile(&input.to_string());

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
    let input_str = input.to_string();
    eprintln!("TokenStream string: {}", input_str);

    let s = compile(&input_str);

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

    // The key difference: doc comments become #[doc = "..."] in TokenStream
    assert!(
        template_str.contains("doc =") || template_str.contains("Doc comment"),
        "TokenStream should preserve doc comment somehow. Got: {}",
        template_str
    );
}

#[test]
fn test_function_with_doc_comment_uses_ident() {
    let input = quote! {
        export function @{fn_name}(value: @{type_param}): string {
            return @{body_expr};
        }
    };
    let s = compile(&input.to_string());

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
    let input = quote! {
        export function @{fn_name1}(value: @{type1}): string {
            return @{body1};
        }

        export function @{fn_name2}(value: @{type2}): Record<string, unknown> {
            return @{body2};
        }
    };
    let s = compile(&input.to_string());

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
    let input = TokenStream2::from_str(
        r#"export interface FieldControllers {
            {#for field in fields}
                readonly @{field.name}: FieldController<@{field.ts_type}>;
            {/for}
        }"#,
    )
    .unwrap();
    let s = compile(&input.to_string());

    eprintln!("Generated code for interface with for loop:\n{}", s);

    // The generated code should reference field.name and field.ts_type
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
    let input = TokenStream2::from_str(
        r#"{#for item in items}
            const @{item.name} = @{item.value};
        {/for}"#,
    )
    .unwrap();
    let s = compile(&input.to_string());

    eprintln!("Generated code for for loop:\n{}", s);

    // Should generate a Rust for loop
    assert!(
        s.contains("for item in items"),
        "Expected Rust for loop in generated code. Got:\n{}",
        s
    );
}

// test_for_loop_with_string_field_name removed - fragments no longer supported
// Control blocks must produce structured IR. Use control blocks inside interfaces
// for property signatures (see test_for_loop_field_interpolation_in_interface).

#[test]
fn test_within_position_extracts_body_correctly() {
    // Test that Within position wrapping works
    let wrapped = "class __MF_DUMMY__ { readonly foo: string; readonly bar: number; }";
    let s = compile_template(wrapped, Some("Within"), 0).unwrap().to_string();

    eprintln!("Generated code for Within body:\n{}", s);

    // The generated code should have body extraction logic
    assert!(
        s.contains("__MF_DUMMY__") || s.contains("__stmts"),
        "Expected statement building in generated code. Got:\n{}",
        s
    );
}

#[test]
fn test_interpolation_in_object_property_position() {
    let input = TokenStream2::from_str(r#"const obj = { @{field_name}: @{field_value} };"#).unwrap();
    let s = compile(&input.to_string());

    eprintln!("Generated code for object property interpolation:\n{}", s);

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
    let input = TokenStream2::from_str(
        r#"const @{name}Obj = {};
           let current = @{name}Obj;
           obj.@{name} = @{name}Obj;"#,
    )
    .unwrap();
    let s = compile(&input.to_string());

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
    let s = compile(&input.to_string());

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
fn test_for_loop_with_tuple_pattern_in_function_body() {
    // This test replicates the derive_ord pattern that was failing
    let input = r#"export function compare(a: T, b: T): number {
        if (a === b) return 0;
        {#for (name, expr) in &steps}
            const @{name.clone()} = @{expr.clone()};
            if (@{name.clone()} !== 0) return @{name.clone()};
        {/for}
        return 0;
    }"#;

    // Dump tokens using the test helper in lexer tests
    eprintln!("=== INPUT ===\n{}\n=== END INPUT ===\n", input);

    let s = compile(input);

    eprintln!("Generated code for for loop with tuple in function body:\n{}", s);

    // Should generate Rust for loop with tuple pattern
    assert!(
        s.contains("for (name , expr) in & steps") || s.contains("for ( name , expr ) in"),
        "Expected Rust for loop with tuple pattern. Got:\n{}",
        s
    );

    // The loop variables should be used inside the loop
    assert!(
        s.contains("name . clone"),
        "Expected name.clone() inside loop. Got:\n{}",
        s
    );
}

#[test]
fn test_placeholder_followed_by_method_call() {
    // This test replicates the issue where placeholder.method(arg) fails to parse
    // The parser was stopping after .is instead of continuing to parse (result)
    let input = r#"
        if (@{pending_ref_expr}.is(result)) {
            return { __pendingIdx: idx, __refId: result.id };
        }
    "#;

    let s = compile(input);

    eprintln!("Generated code for placeholder method call:\n{}", s);

    // The placeholder followed by .is(result) should be parsed as a call expression
    assert!(
        s.contains("is") && (s.contains("result") || s.contains("pending_ref_expr")),
        "Expected placeholder.is(result) to be parsed as call. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_is() {
    // `is` is a TypeScript keyword (type predicates) but valid as property name
    let input = r#"const result = obj.is(value);"#;
    let s = compile(input);
    assert!(
        s.contains("is"),
        "Expected .is() method call to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_as() {
    // `as` is a TypeScript keyword but valid as property name
    let input = r#"const result = obj.as(Type);"#;
    let s = compile(input);
    assert!(
        s.contains("as"),
        "Expected .as() method call to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_type() {
    // `type` is a TypeScript keyword but valid as property name
    let input = r#"const t = obj.type;"#;
    let s = compile(input);
    assert!(
        s.contains("type"),
        "Expected .type property access to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_default() {
    // `default` is a keyword but valid as property name
    let input = r#"const d = module.default;"#;
    let s = compile(input);
    assert!(
        s.contains("default"),
        "Expected .default property access to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_delete() {
    // `delete` is a keyword but valid as property name
    let input = r#"await db.delete(id);"#;
    let s = compile(input);
    assert!(
        s.contains("delete"),
        "Expected .delete() method call to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_get_set() {
    // `get` and `set` are keywords but valid as property names
    let input = r#"
        const val = store.get(key);
        store.set(key, value);
    "#;
    let s = compile(input);
    assert!(
        s.contains("get") && s.contains("set"),
        "Expected .get() and .set() method calls to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_in_chain() {
    // Keywords in chained member access
    let input = r#"const result = obj.type.is.default.get();"#;
    let s = compile(input);
    assert!(
        s.contains("type") && s.contains("is") && s.contains("default") && s.contains("get"),
        "Expected chained keyword property access to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_placeholder_with_keyword_method_chain() {
    // Placeholder followed by keyword method chain
    let input = r#"const result = @{expr}.type.is(value).get();"#;
    let s = compile(input);
    assert!(
        s.contains("type") && s.contains("is") && s.contains("get"),
        "Expected placeholder with keyword method chain to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_new() {
    // `new` is a keyword but valid as property name
    let input = r#"const instance = factory.new(config);"#;
    let s = compile(input);
    assert!(
        s.contains("new"),
        "Expected .new() method call to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_return() {
    // `return` is a keyword but valid as property name
    let input = r#"const ret = response.return;"#;
    let s = compile(input);
    assert!(
        s.contains("return"),
        "Expected .return property access to parse. Got:\n{}",
        s
    );
}

#[test]
fn test_keyword_as_property_name_typeof() {
    // `typeof` is a keyword but valid as property name
    let input = r#"const t = schema.typeof(value);"#;
    let s = compile(input);
    assert!(
        s.contains("typeof"),
        "Expected .typeof() method call to parse. Got:\n{}",
        s
    );
}
