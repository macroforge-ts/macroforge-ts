#[cfg(test)]
mod tests {
    use macroforge_ts_quote::{above, below, body, ident as ident_macro, signature, ts_quote, ts_template};
    use macroforge_ts_syn::parse_ts_expr;
    use swc_core::common::{SyntaxContext, DUMMY_SP};
    use swc_core::ecma::ast::{Decl, Expr, FnDecl, Ident as TsIdent, Lit, Number, Stmt, VarDeclKind};
    use std::fs;
    use proc_macro2::{
        Ident as PmIdent, Spacing, Span as PmSpan, TokenStream as TokenStream2, TokenTree,
    };
    //! Comprehensive tests for macroforge_ts_quote macros.
    //!
    //! This test module provides thorough coverage of all macro functionality
    //! including edge cases, complex constructs, and integration scenarios.

    // =============================================================================
    // Helper Functions
    // =============================================================================


    trait PascalCase {
        fn to_pascal_case(&self) -> String;
    }

    impl PascalCase for &str {
        fn to_pascal_case(&self) -> String {
            let mut chars = self.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            let mut out = String::new();
            out.push(first.to_ascii_uppercase());
            out.push_str(chars.as_str());
            out
        }
    }

    #[test]
    fn doc_ts_quote_simple_class() {
        let name = ident("MyClass");
        let stmt: Stmt = ts_quote!(class $(name) {} as Stmt);
        assert!(matches!(stmt, Stmt::Decl(Decl::Class(_))));
    }

    #[test]
    fn doc_ts_template_for_fields() {
        let fields = vec!["name", "age"];
        let stream = ts_template! {
            {#for field in &fields}
                this.@{ident(field)} = @{expr_ident(field)};
            {/for}
        };
        let source = stream.source();
        assert!(source.contains("this.name = name"));
        assert!(source.contains("this.age = age"));
    }

    #[test]
    fn doc_ts_quote_formatted_ident() {
        let field = "userName";
        let expr: Expr = ts_quote!(
            this.$(ident!("get{}", field.to_pascal_case()))()
        as Expr);
        assert!(matches!(expr, Expr::Call(_)));
    }

    #[test]
    fn doc_ts_quote_object_literal() {
        let key = "status";
        let value = expr_ident("active");
        let expr: Expr = ts_quote!({ $(ident!("{}", key)): $(value: Expr) } as Expr);
        let is_object = matches!(expr, Expr::Object(_))
            || matches!(expr, Expr::Paren(paren) if matches!(*paren.expr, Expr::Object(_)));
        assert!(is_object);
    }

    #[test]
    fn doc_quote_ident() {
        let ts = quote_ident("swc_core");
        assert_eq!(ts.to_string(), "swc_core");
    }

    #[test]
    fn doc_quote_punct() {
        let ts = quote_punct("::");
        assert_eq!(ts.to_string(), "::");
    }

    #[test]
    fn doc_ts_template_tojson() {
        let class_name = "User";
        let stream = ts_template! {
            @{expr_ident(class_name)}.prototype.toJSON = function() {
                return { ok: true };
            };
        };
        let source = stream.source();
        assert!(source.contains("User.prototype.toJSON"));
        assert!(source.contains("return {"));
        assert!(source.contains("ok: true"));
    }

    #[test]
    fn doc_ts_template_backtick() {
        let tag = "div";
        let stream = ts_template! {
            const html = "'^<@{tag}>${content}</@{tag}>^'";
        };
        let source = stream.source();
        assert!(source.contains(r#"`<${"div"}>${content}</${"div"}>`"#));
    }

    #[test]
    fn doc_above_macro() {
        let stream = above! {
            const lodash = "lodash";
        };
        let source = stream.source();
        assert!(source.starts_with("/* @macroforge:above */"));
        assert!(source.contains("const lodash = \"lodash\""));
    }

    #[test]
    fn doc_below_macro() {
        let class_name = "User";
        let stream = below! {
            @{expr_ident(class_name)}.prototype.toJSON = function() {
                return { ...this };
            };
        };
        let source = stream.source();
        assert!(source.starts_with("/* @macroforge:below */"));
        assert!(source.contains("User.prototype.toJSON"));
    }

    #[test]
    fn doc_body_macro() {
        let fields = vec!["name", "age"];
        let stream = body! {
            {#for field in &fields}
                this.@{ident(field)} = @{expr_ident(field)};
            {/for}
        };
        let source = stream.source();
        assert!(source.starts_with("/* @macroforge:body */"));
        assert!(source.contains("this.name = name"));
        assert!(source.contains("this.age = age"));
    }

    #[test]
    fn doc_signature_macro() {
        let param_name = "context";
        let param_type = "RequestContext";
        let stream = signature! {
            @{ident(param_name)}: @{expr_ident(param_type)}
        };
        let source = stream.source();
        assert!(source.starts_with("/* @macroforge:signature */"));
        assert!(source.contains("context: RequestContext"));
    }
    fn quote_ident(s: &str) -> TokenStream2 {
        let ident = PmIdent::new(s, PmSpan::call_site());
        TokenStream2::from(TokenTree::Ident(ident))
    }

    fn quote_punct(s: &str) -> TokenStream2 {
        s.chars()
            .map(|c| TokenTree::Punct(proc_macro2::Punct::new(c, Spacing::Joint)))
            .collect()
    }

    fn ident(name: &str) -> TsIdent {
        TsIdent::new(name.into(), DUMMY_SP, SyntaxContext::empty())
    }

    fn expr_ident(name: &str) -> Expr {
        Expr::Ident(ident(name))
    }

    fn ident(name: &str) -> TsIdent {
        TsIdent::new(name.into(), DUMMY_SP, SyntaxContext::empty())
    }

    fn expr_ident(name: &str) -> Expr {
        Expr::Ident(ident(name))
    }

    fn expr_num(n: f64) -> Expr {
        Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: n,
            raw: None,
        }))
    }

    trait PascalCase {
        fn to_pascal_case(&self) -> String;
    }

    impl PascalCase for &str {
        fn to_pascal_case(&self) -> String {
            let mut chars = self.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            let mut out = String::new();
            out.push(first.to_ascii_uppercase());
            out.push_str(chars.as_str());
            out
        }
    }

    // =============================================================================
    // ts_quote! - Basic AST Type Tests
    // =============================================================================

    mod ts_quote_ast_types {
        use super::*;

        #[test]
        fn interface_declaration() {
            let name = ident("User");
            let stmt: Stmt = ts_quote!(interface $(name) { name: string; } as Stmt);
            assert!(
                matches!(stmt, Stmt::Decl(Decl::TsInterface(_))),
                "Expected TsInterface declaration"
            );
        }

        #[test]
        fn function_declaration() {
            let name = ident("greet");
            let stmt: Stmt = ts_quote!(function $(name)(x: number): void {} as Stmt);
            if let Stmt::Decl(Decl::Fn(FnDecl { ident, .. })) = stmt {
                assert_eq!(ident.sym.as_ref(), "greet");
            } else {
                panic!("Expected function declaration");
            }
        }

        #[test]
        fn arrow_function_expression() {
            let expr: Expr = ts_quote!((x: number) => x * 2 as Expr);
            assert!(
                matches!(expr, Expr::Arrow(_)),
                "Expected Arrow function expression"
            );
        }

        #[test]
        fn enum_declaration() {
            let name = ident("Status");
            let stmt: Stmt = ts_quote!(enum $(name) { Active, Inactive } as Stmt);
            assert!(
                matches!(stmt, Stmt::Decl(Decl::TsEnum(_))),
                "Expected TsEnum declaration"
            );
        }

        #[test]
        fn type_alias_declaration() {
            let name = ident("UserId");
            let stmt: Stmt = ts_quote!(type $(name) = string as Stmt);
            assert!(
                matches!(stmt, Stmt::Decl(Decl::TsTypeAlias(_))),
                "Expected TsTypeAlias declaration"
            );
        }

        #[test]
        fn const_declaration() {
            let name = ident("MAX_VALUE");
            let stmt: Stmt = ts_quote!(const $(name) = 100 as Stmt);
            if let Stmt::Decl(Decl::Var(var_decl)) = stmt {
                assert_eq!(var_decl.kind, VarDeclKind::Const);
            } else {
                panic!("Expected const declaration");
            }
        }

        #[test]
        fn let_declaration() {
            let name = ident("counter");
            let stmt: Stmt = ts_quote!(let $(name) = 0 as Stmt);
            if let Stmt::Decl(Decl::Var(var_decl)) = stmt {
                assert_eq!(var_decl.kind, VarDeclKind::Let);
            } else {
                panic!("Expected let declaration");
            }
        }

        #[test]
        fn class_with_method() {
            let class_name = ident("Calculator");
            let stmt: Stmt = ts_quote!(
                class $(class_name) {
                    add(a: number, b: number): number {
                        return a + b;
                    }
                }
            as Stmt);
            assert!(
                matches!(stmt, Stmt::Decl(Decl::Class(_))),
                "Expected Class declaration"
            );
        }

        #[test]
        fn class_with_constructor() {
            let class_name = ident("Person");
            let stmt: Stmt = ts_quote!(
                class $(class_name) {
                    constructor(public name: string) {}
                }
            as Stmt);
            assert!(
                matches!(stmt, Stmt::Decl(Decl::Class(_))),
                "Expected Class declaration with constructor"
            );
        }

    }

    #[test]
    fn doc_line_comment_roundtrip() {
        let stream = ts_template! {
            /// Generated field
            const value = 1;
        };
        let source = stream.source();
        println!("doc-line output: {source}");
        println!(
            "contains doc line comment: {}",
            source.contains("/// Generated field")
        );
        assert!(
            source.contains("const value = 1"),
            "Expected output to contain the statement"
        );
    }

    #[test]
    fn block_comment_roundtrip() {
        let stream = ts_template! {
            /* Block comment */
            const value = 1;
        };
        let source = stream.source();
        println!("block output: {source}");
        println!(
            "contains block comment: {}",
            source.contains("/* Block comment */")
        );
        assert!(
            source.contains("const value = 1"),
            "Expected output to contain the statement"
        );
    }

    #[test]
    fn doc_block_comment_roundtrip() {
        let stream = ts_template! {
            /** Generated field */
            const value = 1;
        };
        let source = stream.source();
        println!("doc-block output: {source}");
        println!(
            "contains doc block comment: {}",
            source.contains("/** Generated field */")
        );
        assert!(
            source.contains("const value = 1"),
            "Expected output to contain the statement"
        );
    }

    #[test]
    fn doc_tag_comment_roundtrip() {
        let stream = ts_template! {
            {>> "Generated field" <<}
            const value = 1;
        };
        let source = stream.source();
        println!("doc-tag output: {source}");
        println!(
            "contains doc tag comment: {}",
            source.contains("/* Generated field */")
        );
        assert!(
            source.contains("/* Generated field */"),
            "Expected block comment to be emitted"
        );
        assert!(
            source.contains("const value = 1"),
            "Expected output to contain the statement"
        );
    }

    #[test]
    fn serialize() {
        let stream = ts_template! {
            {>> "Generated field" <<}
            const value = 1;
        };
        let source = stream.source();
        println!("doc-tag output: {source}");
        println!(
            "contains doc tag comment: {}",
            source.contains("/* Generated field */")
        );
        assert!(
            source.contains("/* Generated field */"),
            "Expected block comment to be emitted"
        );
        assert!(
            source.contains("const value = 1"),
            "Expected output to contain the statement"
        );
    }

    #[test]
    fn write_comment_outputs() {
        let doc_block = ts_template! {
            /** Generated field */
            const value = 1;
        };
        let block = ts_template! {
            /* Block comment */
            const value = 1;
        };
        let doc_line = ts_template! {
            /// Generated field
            const value = 1;
        };
        let doc_tag = ts_template! {
            {>> "Generated field" <<}
            const value = 1;
        };

        let output = format!(
            "doc block output\n{doc_block:#?}\n\n block output\n{block:#?}\n\n doc line output\n{doc_line:#?}\n\ndoc-tag output:\n{doc_tag:#?}"
        );
        fs::write("logs/comment_output.log", output).expect("Failed to write comment_output.log");
    }
    // =============================================================================
    // ts_quote! - Interpolation Tests
    // =============================================================================

    mod ts_quote_interpolation {
        use super::*;

        #[test]
        fn multiple_interpolations() {
            let class_name = ident("Service");
            let method_name = ident("process");
            let stmt: Stmt = ts_quote!(
                class $(class_name) {
                    $(method_name)(): void {}
                }
            as Stmt);
            assert!(matches!(stmt, Stmt::Decl(Decl::Class(_))));
        }

        #[test]
        fn interpolation_with_type_annotation() {
            let value = expr_ident("myValue");
            let expr: Expr = ts_quote!($(value: Expr) + 1 as Expr);
            assert!(
                matches!(expr, Expr::Bin(_)),
                "Expected binary expression"
            );
        }

        #[test]
        fn formatted_identifier_getter() {
            let field = "userName";
            let expr: Expr = ts_quote!(
                this.$(ident!("get{}", field.to_pascal_case()))()
            as Expr);
            assert!(matches!(expr, Expr::Call(_)));
        }

        #[test]
        fn formatted_identifier_setter() {
            let field = "email";
            let expr: Expr = ts_quote!(
                this.$(ident!("set{}", field.to_pascal_case()))(value)
            as Expr);
            assert!(matches!(expr, Expr::Call(_)));
        }

        #[test]
        fn nested_property_access() {
            let obj = ident("data");
            let prop = ident("value");
            let expr: Expr = ts_quote!($(obj).nested.$(prop) as Expr);
            assert!(matches!(expr, Expr::Member(_)));
        }
    }

    // =============================================================================
    // ts_template! - Conditional Tests
    // =============================================================================

    mod ts_template_conditionals {
        use super::*;

        #[test]
        fn simple_if_block() {
            let show_field = true;
            let stream = ts_template! {
                {#if show_field}
                    const visible = true;
                {/if}
            };
            let source = stream.source();
            assert!(
                source.contains("const visible = true"),
                "Expected conditional block to be included when condition is true"
            );
        }

        #[test]
        fn if_block_false_condition() {
            let show_field = false;
            let stream = ts_template! {
                const always = 1;
                {#if show_field}
                    const never = 2;
                {/if}
            };
            let source = stream.source();
            assert!(source.contains("const always = 1"));
            assert!(
                !source.contains("const never = 2"),
                "Expected conditional block to be excluded when condition is false"
            );
        }

        #[test]
        fn if_else_block_true() {
            let condition = true;
            let stream = ts_template! {
                {#if condition}
                    const result = "yes";
                {:else}
                    const result = "no";
                {/if}
            };
            let source = stream.source();
            assert!(source.contains(r#"const result = "yes""#));
            assert!(!source.contains(r#"const result = "no""#));
        }

        #[test]
        fn if_else_block_false() {
            let condition = false;
            let stream = ts_template! {
                {#if condition}
                    const result = "yes";
                {:else}
                    const result = "no";
                {/if}
            };
            let source = stream.source();
            assert!(!source.contains(r#"const result = "yes""#));
            assert!(source.contains(r#"const result = "no""#));
        }

        #[test]
        fn if_else_if_chain() {
            let value = 2;
            let stream = ts_template! {
                {#if value == 1}
                    const status = "one";
                {:else if value == 2}
                    const status = "two";
                {:else}
                    const status = "other";
                {/if}
            };
            let source = stream.source();
            assert!(source.contains(r#"const status = "two""#));
            assert!(!source.contains(r#"const status = "one""#));
            assert!(!source.contains(r#"const status = "other""#));
        }

        #[test]
        fn nested_if_blocks() {
            let outer = true;
            let inner = true;
            let stream = ts_template! {
                {#if outer}
                    const outer_visible = true;
                    {#if inner}
                        const inner_visible = true;
                    {/if}
                {/if}
            };
            let source = stream.source();
            assert!(source.contains("const outer_visible = true"));
            assert!(source.contains("const inner_visible = true"));
        }

        #[test]
        fn conditional_with_interpolation() {
            let enabled = true;
            let feature_name = "logging";
            let stream = ts_template! {
                {#if enabled}
                    console.log(@{expr_ident(feature_name)});
                {/if}
            };
            let source = stream.source();
            assert!(source.contains("console.log(logging)"));
        }
    }

    // =============================================================================
    // ts_template! - For Loop Tests
    // =============================================================================

    mod ts_template_for_loops {
        use super::*;

        #[test]
        fn for_loop_with_idents() {
            let items = vec!["a", "b", "c"];
            let stream = ts_template! {
                {#for item in &items}
                    const @{ident(item)} = 1;
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("const a = 1"));
            assert!(source.contains("const b = 1"));
            assert!(source.contains("const c = 1"));
        }

        #[test]
        fn for_loop_empty_collection() {
            let items: Vec<&str> = vec![];
            let stream = ts_template! {
                const start = 1;
                {#for item in &items}
                    const @{ident(item)} = true;
                {/for}
                const end = 2;
            };
            let source = stream.source();
            assert!(source.contains("const start = 1"));
            assert!(source.contains("const end = 2"));
        }

        #[test]
        fn nested_for_loops() {
            let rows = vec!["row1", "row2"];
            let cols = vec!["a", "b"];
            let stream = ts_template! {
                {#for row in &rows}
                    {#for col in &cols}
                        this.@{ident(row)}_@{ident(col)} = null;
                    {/for}
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("this.row1_a = null"));
            assert!(source.contains("this.row1_b = null"));
            assert!(source.contains("this.row2_a = null"));
            assert!(source.contains("this.row2_b = null"));
        }

        #[test]
        fn for_loop_with_index_simulation() {
            let items = vec!["first", "second", "third"];
            let stream = ts_template! {
                {#for (idx, item) in items.iter().enumerate()}
                    this.items[@{expr_num(idx as f64)}] = @{expr_ident(item)};
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("this.items[0] = first"));
            assert!(source.contains("this.items[1] = second"));
            assert!(source.contains("this.items[2] = third"));
        }

        #[test]
        fn for_loop_with_conditional() {
            let fields = vec![("name", true), ("age", false), ("email", true)];
            let stream = ts_template! {
                {#for (field, required) in &fields}
                    {#if *required}
                        this.@{ident(field)} = @{expr_ident(field)};
                    {:else}
                        this.@{ident(field)} = @{expr_ident(field)};
                    {/if}
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("this.name = name"));
            assert!(source.contains("this.age = age"));
            assert!(source.contains("this.email = email"));
        }
    }

    // =============================================================================
    // ts_template! - Let Bindings Tests
    // =============================================================================

    mod ts_template_let_bindings {
        use super::*;

        #[test]
        fn simple_let_binding() {
            let base = "User";
            let stream = ts_template! {
                {$let class_name = format!("{}Service", base)}
                const serviceName = @{expr_ident(&class_name)};
            };
            let source = stream.source();
            assert!(source.contains("const serviceName = UserService"));
        }

        #[test]
        fn let_binding_with_computation() {
            let values = vec![1, 2, 3];
            let stream = ts_template! {
                {$let total = values.iter().sum::<i32>()}
                const sum = @{expr_num(total as f64)};
            };
            let source = stream.source();
            assert!(source.contains("const sum = 6"));
        }

        #[test]
        fn multiple_let_bindings() {
            let prefix = "get";
            let field = "Name";
            let stream = ts_template! {
                {$let getter = format!("{}{}", prefix, field)}
                {$let setter = format!("set{}", field)}
                this.@{ident(&getter)}();
                this.@{ident(&setter)}(value);
            };
            let source = stream.source();
            assert!(source.contains("this.getName()"));
            assert!(source.contains("this.setName(value)"));
        }

        #[test]
        fn let_binding_in_loop() {
            let items = vec!["apple", "banana"];
            let stream = ts_template! {
                {#for item in &items}
                    {$let upper = item.to_uppercase()}
                    console.log(@{expr_ident(&upper)});
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("console.log(APPLE)"));
            assert!(source.contains("console.log(BANANA)"));
        }
    }

    // =============================================================================
    // ts_template! - Template Literal Tests
    // =============================================================================

    mod ts_template_literals {
        use super::*;

        #[test]
        fn simple_template_literal() {
            let tag = "span";
            let stream = ts_template! {
                const html = "'^<@{tag}>content</@{tag}>^'";
            };
            let source = stream.source();
            assert!(source.contains("`<"));
            assert!(source.contains("span"));
        }

        #[test]
        fn template_literal_with_content() {
            let prefix = "user";
            let stream = ts_template! {
                const path = "'^/@{prefix}/profile^'";
            };
            let source = stream.source();
            assert!(source.contains("`/"));
            assert!(source.contains("user"));
        }
    }

    // =============================================================================
    // Scoped Macro Tests
    // =============================================================================

    mod scoped_macros {
        use super::*;

        #[test]
        fn above_with_import() {
            let module_name = "lodash";
            let stream = above! {
                const _ = require(@{expr_ident(module_name)});
            };
            let source = stream.source();
            assert!(source.starts_with("/* @macroforge:above */"));
            assert!(source.contains("require(lodash)"));
        }

        #[test]
        fn above_with_multiple_statements() {
            let stream = above! {
                const fs = require("fs");
                const path = require("path");
            };
            let source = stream.source();
            assert!(source.contains(r#"require("fs")"#));
            assert!(source.contains(r#"require("path")"#));
        }

        #[test]
        fn below_with_prototype_extension() {
            let class_name = "Array";
            let method_name = "first";
            let stream = below! {
                @{expr_ident(class_name)}.prototype.@{ident(method_name)} = function() {
                    return this[0];
                };
            };
            let source = stream.source();
            assert!(source.starts_with("/* @macroforge:below */"));
            assert!(source.contains("Array.prototype.first"));
        }

        #[test]
        fn below_with_for_loop() {
            let methods = vec!["serialize", "deserialize"];
            let stream = below! {
                {#for method in &methods}
                    MyClass.prototype.@{ident(method)} = function() {};
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("MyClass.prototype.serialize"));
            assert!(source.contains("MyClass.prototype.deserialize"));
        }

        #[test]
        fn body_constructor_initialization() {
            let fields = vec!["id", "name", "email"];
            let stream = body! {
                {#for field in &fields}
                    this.@{ident(field)} = @{expr_ident(field)};
                {/for}
            };
            let source = stream.source();
            assert!(source.starts_with("/* @macroforge:body */"));
            assert!(source.contains("this.id = id"));
            assert!(source.contains("this.name = name"));
            assert!(source.contains("this.email = email"));
        }

        #[test]
        fn body_with_validation() {
            let field = "age";
            let stream = body! {
                if (@{expr_ident(field)} < 0) {
                    throw new Error("invalid");
                }
            };
            let source = stream.source();
            assert!(source.contains("if (age < 0)"));
            assert!(source.contains("invalid"));
        }

        #[test]
        fn signature_single_param() {
            let param = "userId";
            let param_type = "string";
            let stream = signature! {
                @{ident(param)}: @{expr_ident(param_type)}
            };
            let source = stream.source();
            assert!(source.starts_with("/* @macroforge:signature */"));
            assert!(source.contains("userId: string"));
        }

        #[test]
        fn signature_multiple_params() {
            let params = vec![("id", "number"), ("name", "string")];
            let stream = signature! {
                {#for (name, ty) in &params}
                    @{ident(name)}: @{expr_ident(ty)},
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("id: number"));
            assert!(source.contains("name: string"));
        }
    }

    // =============================================================================
    // Comment Tests
    // =============================================================================

    mod comments {
        use super::*;

        #[test]
        fn doc_comment_on_variable() {
            let stream = ts_template! {
                /// This is a documented constant
                const PI = 3.14159;
            };
            let source = stream.source();
            assert!(source.contains("const PI = 3.14159"));
        }

        #[test]
        fn doc_tag_comment() {
            let stream = ts_template! {
                {>> "Auto-generated getter" <<}
                getValue(): number { return this.value; }
            };
            let source = stream.source();
            assert!(source.contains("/* Auto-generated getter */"));
        }

        #[test]
        fn multiple_doc_comments() {
            let stream = ts_template! {
                /// First field
                const first = 1;
                /// Second field
                const second = 2;
            };
            let source = stream.source();
            assert!(source.contains("const first = 1"));
            assert!(source.contains("const second = 2"));
        }
    }

    // =============================================================================
    // Edge Case Tests
    // =============================================================================

    mod edge_cases {
        use super::*;

        #[test]
        fn empty_class() {
            let name = ident("Empty");
            let stmt: Stmt = ts_quote!(class $(name) {} as Stmt);
            assert!(matches!(stmt, Stmt::Decl(Decl::Class(_))));
        }

        #[test]
        fn special_characters_in_strings() {
            let stream = ts_template! {
                const special = "hello world";
            };
            let source = stream.source();
            assert!(source.contains("const special"));
        }

        #[test]
        fn deeply_nested_objects() {
            let stream = ts_template! {
                const nested = { a: { b: { c: { d: 1 } } } };
            };
            let source = stream.source();
            assert!(source.contains("const nested"));
            assert!(source.contains("d: 1"));
        }

        #[test]
        fn array_with_mixed_types() {
            let stream = ts_template! {
                const mixed = [1, "two", true, null];
            };
            let source = stream.source();
            assert!(source.contains("const mixed"));
            assert!(source.contains("1"));
            assert!(source.contains("\"two\""));
        }

        #[test]
        fn chained_method_calls() {
            let stream = ts_template! {
                const result = obj.method1().method2().method3();
            };
            let source = stream.source();
            assert!(source.contains("obj.method1().method2().method3()"));
        }

        #[test]
        fn spread_operator() {
            let stream = ts_template! {
                const combined = { ...obj1, ...obj2 };
            };
            let source = stream.source();
            assert!(source.contains("...obj1"));
            assert!(source.contains("...obj2"));
        }

        #[test]
        fn destructuring_assignment() {
            let stream = ts_template! {
                const { a, b, c } = source;
            };
            let source = stream.source();
            assert!(source.contains("a") && source.contains("b") && source.contains("c"));
        }

        #[test]
        fn async_await_syntax() {
            let stream = ts_template! {
                const data = await fetch(url);
            };
            let source = stream.source();
            assert!(source.contains("await fetch"));
        }

        #[test]
        fn optional_chaining() {
            let stream = ts_template! {
                const value = obj?.nested?.property;
            };
            let source = stream.source();
            assert!(source.contains("?."));
        }

        #[test]
        fn nullish_coalescing() {
            let stream = ts_template! {
                const result = value ?? defaultValue;
            };
            let source = stream.source();
            assert!(source.contains("??"));
        }
    }

    // =============================================================================
    // TypeScript-Specific Construct Tests
    // =============================================================================

    mod typescript_constructs {
        use super::*;

        #[test]
        fn generic_interface() {
            let name = ident("Container");
            let stmt: Stmt = ts_quote!(interface $(name)<T> { value: T; } as Stmt);
            assert!(matches!(stmt, Stmt::Decl(Decl::TsInterface(_))));
        }

        #[test]
        fn generic_class() {
            let name = ident("Box");
            let stmt: Stmt = ts_quote!(class $(name)<T> { constructor(public value: T) {} } as Stmt);
            assert!(matches!(stmt, Stmt::Decl(Decl::Class(_))));
        }

        #[test]
        fn type_with_union() {
            let name = ident("StringOrNumber");
            let stmt: Stmt = ts_quote!(type $(name) = string | number as Stmt);
            assert!(matches!(stmt, Stmt::Decl(Decl::TsTypeAlias(_))));
        }

        #[test]
        fn type_with_intersection() {
            let name = ident("Combined");
            let stmt: Stmt = ts_quote!(type $(name) = A & B as Stmt);
            assert!(matches!(stmt, Stmt::Decl(Decl::TsTypeAlias(_))));
        }

        #[test]
        fn readonly_property() {
            let stream = ts_template! {
                interface Config {
                    readonly apiKey: string;
                }
            };
            let source = stream.source();
            assert!(source.contains("readonly"));
            assert!(source.contains("apiKey"));
        }

        #[test]
        fn optional_property() {
            let stream = ts_template! {
                interface User {
                    name: string;
                    age?: number;
                }
            };
            let source = stream.source();
            assert!(source.contains("name: string"));
            assert!(source.contains("age?"));
        }

        #[test]
        fn index_signature() {
            let stream = ts_template! {
                interface StringMap {
                    [key: string]: string;
                }
            };
            let source = stream.source();
            assert!(source.contains("[key: string]"));
        }

        #[test]
        fn mapped_type_template() {
            let fields = vec!["name", "email"];
            let stream = ts_template! {
                interface Partial {
                    {#for field in &fields}
                        @{ident(field)}?: string;
                    {/for}
                }
            };
            let source = stream.source();
            assert!(source.contains("name?"));
            assert!(source.contains("email?"));
        }

        #[test]
        fn type_assertion() {
            let stream = ts_template! {
                const x = value as string;
            };
            let source = stream.source();
            assert!(source.contains("as string"));
        }

        #[test]
        fn non_null_assertion() {
            let stream = ts_template! {
                const definite = maybeNull!;
            };
            let source = stream.source();
            assert!(source.contains("maybeNull!"));
        }
    }

    // =============================================================================
    // Integration Tests - Real-World Scenarios
    // =============================================================================

    mod integration {
        use super::*;

        #[test]
        fn generate_dto_class() {
            let class_name = "UserDTO";
            let fields = vec![("id", "number"), ("name", "string"), ("email", "string")];

            let stream = ts_template! {
                class @{expr_ident(class_name)} {
                    {#for (field, ty) in &fields}
                        @{ident(field)}: @{expr_ident(ty)};
                    {/for}

                    constructor(
                        {#for (field, ty) in &fields}
                            @{ident(field)}: @{expr_ident(ty)},
                        {/for}
                    ) {
                        {#for (field, _) in &fields}
                            this.@{ident(field)} = @{expr_ident(field)};
                        {/for}
                    }
                }
            };

            let source = stream.source();
            assert!(source.contains("class UserDTO"));
            assert!(source.contains("id: number"));
            assert!(source.contains("name: string"));
            assert!(source.contains("email: string"));
            assert!(source.contains("this.id = id"));
        }

        #[test]
        fn generate_factory_function() {
            let type_name = "Config";

            let stream = ts_template! {
                function create@{expr_ident(type_name)}(options?: Partial<@{expr_ident(type_name)}>): @{expr_ident(type_name)} {
                    return {
                        timeout: options?.timeout ?? 5000,
                        retries: options?.retries ?? 3,
                    };
                }
            };

            let source = stream.source();
            assert!(source.contains("function createConfig"));
            assert!(source.contains("Partial<Config>"));
            assert!(source.contains("timeout:"));
            assert!(source.contains("retries:"));
        }

        #[test]
        fn generate_event_emitter_methods() {
            let events = vec!["click", "hover", "focus"];

            let stream = below! {
                {#for event in &events}
                    {$let handler_name = format!("on{}", event.to_pascal_case())}
                    Component.prototype.@{ident(&handler_name)} = function(callback) {
                        this.addEventListener(@{expr_ident(event)}, callback);
                    };
                {/for}
            };

            let source = stream.source();
            assert!(source.contains("onClick"));
            assert!(source.contains("onHover"));
            assert!(source.contains("onFocus"));
            assert!(source.contains("addEventListener(click"));
        }

        #[test]
        fn generate_validation_methods() {
            let validators = vec!["required", "minLength", "maxLength"];

            let stream = ts_template! {
                const validators = {
                    {#for name in &validators}
                        @{ident(name)}: (value: any) => true,
                    {/for}
                };
            };

            let source = stream.source();
            assert!(source.contains("const validators"));
            assert!(source.contains("required:"));
            assert!(source.contains("minLength:"));
            assert!(source.contains("maxLength:"));
        }

        #[test]
        fn combined_scoped_macros() {
            let class_name = "Entity";
            let fields = vec!["createdAt", "updatedAt"];

            let above_stream = above! {
                const BaseEntity = require("./base");
            };

            let body_stream = body! {
                {#for field in &fields}
                    this.@{ident(field)} = new Date();
                {/for}
            };

            let below_stream = below! {
                @{expr_ident(class_name)}.tableName = "entity";
            };

            let above_source = above_stream.source();
            let body_source = body_stream.source();
            let below_source = below_stream.source();

            assert!(above_source.starts_with("/* @macroforge:above */"));
            assert!(body_source.starts_with("/* @macroforge:body */"));
            assert!(below_source.starts_with("/* @macroforge:below */"));

            assert!(above_source.contains("BaseEntity"));
            assert!(body_source.contains("this.createdAt"));
            assert!(body_source.contains("this.updatedAt"));
            assert!(below_source.contains("Entity.tableName"));
        }
    }

    // =============================================================================
    // While Loop Tests
    // =============================================================================

    mod ts_template_while_loops {
        use super::*;

        #[test]
        fn simple_while_loop() {
            let limit = 3;
            let stream = ts_template! {
                {$let mut count = 0}
                {#while count < limit}
                    const item@{expr_num(count as f64)} = true;
                    {$do count += 1}
                {/while}
            };
            let source = stream.source();
            assert!(source.contains("const item0 = true"));
            assert!(source.contains("const item1 = true"));
            assert!(source.contains("const item2 = true"));
        }
    }

    // =============================================================================
    // Match Expression Tests
    // =============================================================================

    mod ts_template_match {
        use super::*;

        #[test]
        fn simple_match() {
            let value: Option<&str> = Some("hello");
            let stream = ts_template! {
                {#match &value}
                    {:case Some(v)}
                        const result = @{expr_ident(v)};
                    {:case None}
                        const result = null;
                {/match}
            };
            let source = stream.source();
            assert!(source.contains("const result = hello"));
        }

        #[test]
        fn match_with_enum_variant() {
            #[derive(Debug)]
            enum Status {
                Active,
                Inactive,
            }
            let status = Status::Active;
            let stream = ts_template! {
                {#match status}
                    {:case Status::Active}
                        const isActive = true;
                    {:case Status::Inactive}
                        const isActive = false;
                {/match}
            };
            let source = stream.source();
            assert!(source.contains("const isActive = true"));
        }

        /// Tests match with pattern binding used directly as type
        /// This reproduces the issue from derive_deserialize.rs
        #[test]
        fn match_with_pattern_binding_as_type() {
            #[derive(Clone)]
            enum TypeCategory {
                Primitive,
                Array(String),
            }
            let type_cat = TypeCategory::Array("number".to_string());
            let field_ident = ident("myField");

            let stream = ts_template! {
                {#match type_cat}
                    {:case TypeCategory::Primitive}
                        instance.@{field_ident} = raw_value;
                    {:case TypeCategory::Array(inner)}
                        instance.@{field_ident} = raw_value as @{inner}[];
                {/match}
            };
            let source = stream.source();
            assert!(source.contains("instance.myField = raw_value as number[]"));
        }
    }

    // =============================================================================
    // If Let Tests
    // =============================================================================

    mod ts_template_if_let {
        use super::*;

        #[test]
        fn if_let_some() {
            let maybe_value: Option<&str> = Some("found");
            let stream = ts_template! {
                {#if let Some(v) = maybe_value}
                    const value = @{expr_ident(v)};
                {/if}
            };
            let source = stream.source();
            assert!(source.contains("const value = found"));
        }

        #[test]
        fn if_let_none() {
            let maybe_value: Option<&str> = None;
            let stream = ts_template! {
                const prefix = true;
                {#if let Some(v) = maybe_value}
                    const value = @{expr_ident(v)};
                {/if}
                const suffix = true;
            };
            let source = stream.source();
            assert!(source.contains("const prefix = true"));
            assert!(source.contains("const suffix = true"));
            assert!(!source.contains("const value"));
        }

        #[test]
        fn if_let_with_else() {
            let maybe_value: Option<&str> = None;
            let stream = ts_template! {
                {#if let Some(v) = maybe_value}
                    const found = @{expr_ident(v)};
                {:else}
                    const found = null;
                {/if}
            };
            let source = stream.source();
            assert!(source.contains("const found = null"));
        }
    }

    // =============================================================================
    // Do Expression Tests
    // =============================================================================

    mod ts_template_do {
        use super::*;

        #[test]
        fn do_side_effect() {
            let stream = ts_template! {
                {$let mut counter = 0}
                const before = @{expr_num(counter as f64)};
                {$do counter += 1}
                const after = @{expr_num(counter as f64)};
            };
            let source = stream.source();
            assert!(source.contains("const before = 0"));
            assert!(source.contains("const after = 1"));
        }

        #[test]
        fn do_with_function_call() {
            let mut log: Vec<String> = Vec::new();
            let stream = ts_template! {
                {$do log.push("started".to_string())}
                const value = 1;
                {$do log.push("finished".to_string())}
            };
            let _source = stream.source();
            assert_eq!(log.len(), 2);
            assert_eq!(log[0], "started");
            assert_eq!(log[1], "finished");
        }
    }

    // =============================================================================
    // Ident Block Tests
    // =============================================================================

    mod ts_template_ident_blocks {
        use super::*;

        #[test]
        fn ident_block_concatenation() {
            let prefix = "get";
            let field = "Name";
            let stream = ts_template! {
                this.{|@{prefix}@{field}|}();
            };
            let source = stream.source();
            assert!(source.contains("this.getName()"));
        }

        #[test]
        fn ident_block_in_loop() {
            let fields = vec!["user", "email"];
            let stream = ts_template! {
                {#for field in &fields}
                    this.{|get@{field.to_pascal_case()}|}();
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("this.getUser()"));
            assert!(source.contains("this.getEmail()"));
        }
    }

    // =============================================================================
    // Escape Sequence Tests
    // =============================================================================

    mod ts_template_escapes {
        use super::*;

        #[test]
        fn escape_at_symbol() {
            let stream = ts_template! {
                const email = "user@@{domain.com}";
            };
            let source = stream.source();
            // @@ should produce literal @{
            assert!(source.contains("@{domain.com}") || source.contains("user@"));
        }
    }

    // =============================================================================
    // Complex Nested Structure Tests
    // =============================================================================

    mod ts_template_complex {
        use super::*;

        #[test]
        fn deeply_nested_control_flow() {
            let outer_items = vec!["a", "b"];
            let inner_enabled = true;
            let stream = ts_template! {
                {#for outer in &outer_items}
                    {#if inner_enabled}
                        {$let name = format!("item_{}", outer)}
                        const @{ident(&name)} = @{expr_ident(outer)};
                    {/if}
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("const item_a = a"));
            assert!(source.contains("const item_b = b"));
        }

        #[test]
        fn conditional_in_loop_with_else() {
            let items = vec![("a", true), ("b", false), ("c", true)];
            let stream = ts_template! {
                {#for (name, enabled) in &items}
                    {#if *enabled}
                        const @{ident(name)}_enabled = true;
                    {:else}
                        const @{ident(name)}_disabled = true;
                    {/if}
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("const a_enabled = true"));
            assert!(source.contains("const b_disabled = true"));
            assert!(source.contains("const c_enabled = true"));
        }

        #[test]
        fn multiple_sequential_loops() {
            let fields = vec!["x", "y"];
            let methods = vec!["get", "set"];
            let stream = ts_template! {
                {#for field in &fields}
                    @{ident(field)}: number;
                {/for}
                {#for method in &methods}
                    @{ident(method)}Value(): void {}
                {/for}
            };
            let source = stream.source();
            assert!(source.contains("x: number"));
            assert!(source.contains("y: number"));
            assert!(source.contains("getValue()"));
            assert!(source.contains("setValue()"));
        }
    }

    // =============================================================================
    // ts_template! - Complex Deserialize Template Tests (derive_deserialize pattern)
    // =============================================================================

    mod ts_template_deserialize {
        use super::*;

        /// Simulates the structure used in derive_deserialize.rs for serialization/deserialization
        #[test]
        fn deserialize_function_with_doc_comments() {
            let fn_deserialize_ident = ident("deserialize");
            let return_type_ident = ident("Result");
            let success_result_expr = expr_ident("value");
            let error_generic_message_expr = expr_ident("err");

            let stream = ts_template! {
                /** Deserializes input to this type. @param input - JSON string or object */
                export function @{fn_deserialize_ident}(input: unknown): @{return_type_ident} {
                    try {
                        const data = typeof input === "string" ? JSON.parse(input) : input;
                        return @{success_result_expr};
                    } catch (e) {
                        const message = e instanceof Error ? e.message : String(e);
                        return @{error_generic_message_expr};
                    }
                }
            };
            let source = stream.source();
            assert!(source.contains("export function deserialize(input: unknown): Result"));
            assert!(source.contains("JSON.parse(input)"));
            assert!(source.contains("return value"));
            assert!(source.contains("return err"));
        }

        /// Tests control flow inside function bodies
        #[test]
        fn deserialize_with_control_flow() {
            let fn_ident = ident("deserializeWithContext");
            let type_ident = ident("User");
            let deny_unknown = true;
            let known_keys_str = r#""name", "age""#;

            let stream = ts_template! {
                export function @{fn_ident}(value: any): @{type_ident} {
                    const obj = value as Record<string, unknown>;
                    const errors: Array<{ field: string; message: string }> = [];

                    {#if deny_unknown}
                        const knownKeys = new Set([@{known_keys_str}]);
                        for (const key of Object.keys(obj)) {
                            if (!knownKeys.has(key)) {
                                errors.push({ field: key, message: "unknown field" });
                            }
                        }
                    {/if}

                    const instance: any = {};
                    return instance as @{type_ident};
                }
            };
            let source = stream.source();
            assert!(source.contains("export function deserializeWithContext(value: any): User"));
            assert!(source.contains("knownKeys"));
            assert!(source.contains(r#""name", "age""#));
        }

        /// Tests for loop inside function body
        #[test]
        fn deserialize_with_for_loop() {
            let fn_ident = ident("validateFields");

            // Simulate field info
            struct FieldInfo {
                json_key: &'static str,
            }
            let required_fields = vec![
                FieldInfo { json_key: "name" },
                FieldInfo { json_key: "email" },
            ];
            let has_required = true;

            let stream = ts_template! {
                export function @{fn_ident}(obj: Record<string, unknown>): string[] {
                    const errors: string[] = [];

                    {#if has_required}
                        {#for field in &required_fields}
                            if (!("@{field.json_key}" in obj)) {
                                errors.push("missing: @{field.json_key}");
                            }
                        {/for}
                    {/if}

                    return errors;
                }
            };
            let source = stream.source();
            assert!(source.contains("export function validateFields"));
            assert!(source.contains(r#""name" in obj"#));
            assert!(source.contains(r#""email" in obj"#));
            assert!(source.contains(r#""missing: name""#));
            assert!(source.contains(r#""missing: email""#));
        }

        /// Tests $let bindings inside loops
        #[test]
        fn deserialize_with_let_in_loop() {
            struct FieldInfo {
                field_name: &'static str,
                json_key: &'static str,
                field_ident: TsIdent,
            }
            let all_fields = vec![
                FieldInfo {
                    field_name: "userName",
                    json_key: "user_name",
                    field_ident: ident("userName"),
                },
                FieldInfo {
                    field_name: "userAge",
                    json_key: "user_age",
                    field_ident: ident("userAge"),
                },
            ];
            let has_fields = true;

            let stream = ts_template! {
                {#if has_fields}
                    {#for field in &all_fields}
                        {$let raw_var = format!("__raw_{}", field.field_name)}
                        const @{ident(&raw_var)} = obj["@{field.json_key}"];
                        instance.@{field.field_ident} = @{ident(&raw_var)};
                    {/for}
                {/if}
            };
            let source = stream.source();
            assert!(source.contains(r#"const __raw_userName = obj["user_name"]"#));
            assert!(source.contains(r#"const __raw_userAge = obj["user_age"]"#));
            assert!(source.contains("instance.userName = __raw_userName"));
            assert!(source.contains("instance.userAge = __raw_userAge"));
        }

        /// Tests match statement for type categorization
        #[test]
        fn deserialize_with_match() {
            #[derive(Clone)]
            enum TypeCategory {
                Primitive,
                Date,
                Other,
            }
            let type_cat = TypeCategory::Primitive;
            let field_ident = ident("createdAt");
            let raw_var = "__raw_createdAt";

            let stream = ts_template! {
                {#match type_cat}
                    {:case TypeCategory::Primitive}
                        instance.@{field_ident} = @{ident(raw_var)};
                    {:case TypeCategory::Date}
                        instance.@{field_ident} = new Date(@{ident(raw_var)});
                    {:case _}
                        instance.@{field_ident} = @{ident(raw_var)};
                {/match}
            };
            let source = stream.source();
            assert!(source.contains("instance.createdAt = __raw_createdAt"));
        }

        /// Tests if-let for Option handling
        #[test]
        fn deserialize_with_if_let() {
            let field_ident = ident("customField");
            let deserialize_with: Option<&str> = Some("customDeserializer");
            let json_key = "custom_field";

            let stream = ts_template! {
                {#if let Some(fn_name) = deserialize_with}
                    instance.@{field_ident} = @{ident(fn_name)}(obj["@{json_key}"]);
                {:else}
                    instance.@{field_ident} = obj["@{json_key}"];
                {/if}
            };
            let source = stream.source();
            assert!(source.contains(r#"instance.customField = customDeserializer(obj["custom_field"])"#));
        }

        /// Tests multiple exported functions (like the actual derive_deserialize)
        #[test]
        fn multiple_exported_functions() {
            let type_name = "User";
            let fn_deserialize = ident("deserialize");
            let fn_internal = ident("deserializeWithContext");
            let type_ident = ident(type_name);

            let stream = ts_template! {
                /** Public deserialization entry point */
                export function @{fn_deserialize}(input: unknown): @{type_ident} {
                    return @{fn_internal}(input);
                }

                /** Internal deserialization with context */
                export function @{fn_internal}(value: any): @{type_ident} {
                    const instance: any = {};
                    return instance as @{type_ident};
                }
            };
            let source = stream.source();
            assert!(source.contains("export function deserialize(input: unknown): User"));
            assert!(source.contains("export function deserializeWithContext(value: any): User"));
            assert!(source.contains("return deserializeWithContext(input)"));
        }

        /// Tests nested control structures like the actual derive_deserialize
        #[test]
        fn deeply_nested_field_processing() {
            struct FieldInfo {
                field_name: &'static str,
                json_key: &'static str,
                field_ident: TsIdent,
                ts_type: &'static str,
                optional: bool,
            }

            let all_fields = vec![
                FieldInfo {
                    field_name: "name",
                    json_key: "name",
                    field_ident: ident("name"),
                    ts_type: "string",
                    optional: false,
                },
                FieldInfo {
                    field_name: "age",
                    json_key: "age",
                    field_ident: ident("age"),
                    ts_type: "number",
                    optional: true,
                },
            ];
            let has_fields = true;

            let stream = ts_template! {
                const instance: any = {};
                {#if has_fields}
                    {#for field in &all_fields}
                        {$let raw_var = format!("__raw_{}", field.field_name)}
                        {#if field.optional}
                            if ("@{field.json_key}" in obj) {
                                const @{ident(&raw_var)} = obj["@{field.json_key}"] as @{field.ts_type};
                                instance.@{field.field_ident} = @{ident(&raw_var)};
                            }
                        {:else}
                            const @{ident(&raw_var)} = obj["@{field.json_key}"] as @{field.ts_type};
                            instance.@{field.field_ident} = @{ident(&raw_var)};
                        {/if}
                    {/for}
                {/if}
            };
            let source = stream.source();

            // Required field (name) - should have no if check
            assert!(source.contains(r#"const __raw_name = obj["name"] as string"#));
            assert!(source.contains("instance.name = __raw_name"));

            // Optional field (age) - should have if check
            assert!(source.contains(r#"if ("age" in obj)"#));
            assert!(source.contains(r#"const __raw_age = obj["age"] as number"#));
            assert!(source.contains("instance.age = __raw_age"));
        }
    }

    // =============================================================================
    // Builtin Macro Template Tests
    // =============================================================================
    // These tests mirror the actual templates used in macroforge_ts builtin derive macros

    mod builtin_templates {
        use super::*;

        // -------------------------------------------------------------------------
        // derive(Clone) Template Tests
        // -------------------------------------------------------------------------

        /// Clone for class - mirrors derive_clone.rs class implementation
        #[test]
        fn derive_clone_class_template() {
            let class_name = "User";
            let class_ident = ident(class_name);
            let fn_name_ident = ident("userClone");
            let fn_name_expr: Expr = fn_name_ident.clone().into();

            // Field names (simulating class.field_names())
            let field_names = vec!["name", "age", "email"];

            let standalone = ts_template! {
                export function @{fn_name_ident}(value: @{class_ident}): @{class_ident} {
                    const cloned = Object.create(Object.getPrototypeOf(value));
                    {#for field in field_names.iter().map(|f| ident(f))}
                        cloned.@{field.clone()} = value.@{field};
                    {/for}
                    return cloned;
                }
            };

            let class_body = body! {
                static clone(value: @{class_ident}): @{class_ident} {
                    return @{fn_name_expr}(value);
                }
            };

            let source = standalone.source();
            assert!(source.contains("export function userClone(value: User): User"));
            assert!(source.contains("const cloned = Object.create(Object.getPrototypeOf(value))"));
            assert!(source.contains("cloned.name = value.name"));
            assert!(source.contains("cloned.age = value.age"));
            assert!(source.contains("cloned.email = value.email"));
            assert!(source.contains("return cloned"));

            let body_source = class_body.source();
            assert!(body_source.contains("static clone(value: User): User"));
            assert!(body_source.contains("return userClone(value)"));
        }

        /// Clone for enum - mirrors derive_clone.rs enum implementation
        #[test]
        fn derive_clone_enum_template() {
            let enum_name = "Status";
            let fn_name_ident = ident("statusClone");

            let stream = ts_template! {
                export function @{fn_name_ident}(value: @{ident(enum_name)}): @{ident(enum_name)} {
                    return value;
                }
            };

            let source = stream.source();
            assert!(source.contains("export function statusClone(value: Status): Status"));
            assert!(source.contains("return value"));
        }

        /// Clone for interface - mirrors derive_clone.rs interface implementation
        #[test]
        fn derive_clone_interface_template() {
            let interface_name = "UserInterface";
            let interface_ident = ident(interface_name);
            let fn_name_ident = ident("userInterfaceClone");

            let field_names = vec!["id", "username", "active"];

            let stream = ts_template! {
                export function @{fn_name_ident}(value: @{interface_ident}): @{interface_ident} {
                    const result = {} as any;
                    {#for field in field_names.iter().map(|f| ident(f))}
                        result.@{field.clone()} = value.@{field};
                    {/for}
                    return result as @{interface_ident};
                }
            };

            let source = stream.source();
            assert!(source.contains("export function userInterfaceClone(value: UserInterface): UserInterface"));
            assert!(source.contains("const result = {} as any"));
            assert!(source.contains("result.id = value.id"));
            assert!(source.contains("result.username = value.username"));
            assert!(source.contains("result.active = value.active"));
            assert!(source.contains("return result as UserInterface"));
        }

        // -------------------------------------------------------------------------
        // derive(Hash) Template Tests
        // -------------------------------------------------------------------------

        /// Hash for class - mirrors derive_hash.rs class implementation
        #[test]
        fn derive_hash_class_template() {
            let class_name = "User";
            let class_ident = ident(class_name);
            let fn_name_ident = ident("userHashCode");
            let fn_name_expr: Expr = fn_name_ident.clone().into();
            let _has_fields = true;

            // Simulated hash expressions (in real code these come from generate_field_hash)
            let hash_exprs: Vec<Expr> = vec![
                parse_ts_expr("(value.name ? value.name.length : 0)").unwrap(),
                parse_ts_expr("(value.age | 0)").unwrap(),
            ];

            let standalone = ts_template! {
                export function @{fn_name_ident}(value: @{class_ident}): number {
                    let hash = 17;
                    {#if _has_fields}
                        {#for hash_expr in hash_exprs}
                            hash = (hash * 31 + @{hash_expr}) | 0;
                        {/for}
                    {/if}
                    return hash;
                }
            };

            let class_body = body! {
                static hashCode(value: @{class_ident}): number {
                    return @{fn_name_expr}(value);
                }
            };

            let source = standalone.source();
            assert!(source.contains("export function userHashCode(value: User): number"));
            assert!(source.contains("let hash = 17"));
            assert!(source.contains("hash = (hash * 31 + (value.name ? value.name.length : 0)) | 0"));
            assert!(source.contains("hash = (hash * 31 + (value.age | 0)) | 0"));
            assert!(source.contains("return hash"));

            let body_source = class_body.source();
            assert!(body_source.contains("static hashCode(value: User): number"));
            assert!(body_source.contains("return userHashCode(value)"));
        }

        /// Hash for string enum - mirrors derive_hash.rs string enum implementation
        #[test]
        fn derive_hash_string_enum_template() {
            let enum_name = "Color";
            let fn_name_ident = ident("colorHashCode");

            let stream = ts_template! {
                export function @{fn_name_ident}(value: @{ident(enum_name)}): number {
                    let hash = 0;
                    for (let i = 0; i < value.length; i++) {
                        hash = (hash * 31 + value.charCodeAt(i)) | 0;
                    }
                    return hash;
                }
            };

            let source = stream.source();
            assert!(source.contains("export function colorHashCode(value: Color): number"));
            assert!(source.contains("let hash = 0"));
            assert!(source.contains("value.charCodeAt(i)"));
        }

        // -------------------------------------------------------------------------
        // derive(Debug) Template Tests
        // -------------------------------------------------------------------------

        /// Debug for class - mirrors derive_debug.rs class implementation
        #[test]
        fn derive_debug_class_template() {
            let class_name = "Person";
            let class_ident = ident(class_name);
            let fn_name_ident = ident("personToString");
            let fn_name_expr: Expr = fn_name_ident.clone().into();
            let has_fields = true;

            struct FieldDebug {
                name: TsIdent,
                quoted_name: String,
            }

            let fields: Vec<FieldDebug> = vec![
                FieldDebug { name: ident("firstName"), quoted_name: "firstName".to_string() },
                FieldDebug { name: ident("lastName"), quoted_name: "lastName".to_string() },
            ];

            let standalone = ts_template! {
                export function @{fn_name_ident}(value: @{class_ident}): string {
                    {#if has_fields}
                        const parts: string[] = [];
                        {#for field in &fields}
                            parts.push("@{field.quoted_name}: " + value.@{field.name});
                        {/for}
                        return "@{class_name} { " + parts.join(", ") + " }";
                    {:else}
                        return "@{class_name} {}";
                    {/if}
                }
            };

            let class_body = body! {
                static toString(value: @{class_ident}): string {
                    return @{fn_name_expr}(value);
                }
            };

            let source = standalone.source();
            assert!(source.contains("export function personToString(value: Person): string"));
            assert!(source.contains("const parts: string[] = []"));
            assert!(source.contains(r#"parts.push("firstName: " + value.firstName)"#));
            assert!(source.contains(r#"parts.push("lastName: " + value.lastName)"#));

            let body_source = class_body.source();
            assert!(body_source.contains("static toString(value: Person): string"));
            assert!(body_source.contains("return personToString(value)"));
        }

        // -------------------------------------------------------------------------
        // derive(Default) Template Tests
        // -------------------------------------------------------------------------

        /// Default for class - mirrors derive_default.rs class implementation
        #[test]
        fn derive_default_class_template() {
            let class_name = "Config";
            let class_ident = ident(class_name);
            let class_expr: Expr = class_ident.clone().into();
            let fn_name_ident = ident("configDefault");
            let has_fields = true;

            struct DefaultField {
                name_ident: TsIdent,
                value_expr: Expr,
            }

            let fields: Vec<DefaultField> = vec![
                DefaultField {
                    name_ident: ident("timeout"),
                    value_expr: *parse_ts_expr("5000").unwrap(),
                },
                DefaultField {
                    name_ident: ident("retries"),
                    value_expr: *parse_ts_expr("3").unwrap(),
                },
                DefaultField {
                    name_ident: ident("enabled"),
                    value_expr: *parse_ts_expr("true").unwrap(),
                },
            ];

            let class_body = body! {
                static defaultValue(): @{class_ident} {
                    const instance = new @{class_expr}();
                    {#if has_fields}
                        {#for field in &fields}
                            instance.@{field.name_ident} = @{field.value_expr};
                        {/for}
                    {/if}
                    return instance;
                }
            };

            let standalone = ts_template! {
                export function @{fn_name_ident}(): @{class_ident} {
                    return @{class_expr}.defaultValue();
                }
            };

            let body_source = class_body.source();
            assert!(body_source.contains("static defaultValue(): Config"));
            assert!(body_source.contains("const instance = new Config()"));
            assert!(body_source.contains("instance.timeout = 5000"));
            assert!(body_source.contains("instance.retries = 3"));
            assert!(body_source.contains("instance.enabled = true"));
            assert!(body_source.contains("return instance"));

            let source = standalone.source();
            assert!(source.contains("export function configDefault(): Config"));
            assert!(source.contains("return Config.defaultValue()"));
        }

        /// Default for interface - mirrors derive_default.rs interface implementation
        #[test]
        fn derive_default_interface_template() {
            let interface_name = "Options";
            let interface_ident = ident(interface_name);
            let fn_name_ident = ident("optionsDefault");
            let has_fields = true;

            struct DefaultField {
                name_expr: Expr,
                value_expr: Expr,
            }

            let fields: Vec<DefaultField> = vec![
                DefaultField {
                    name_expr: *parse_ts_expr("\"debug\"").unwrap(),
                    value_expr: *parse_ts_expr("false").unwrap(),
                },
                DefaultField {
                    name_expr: *parse_ts_expr("\"level\"").unwrap(),
                    value_expr: *parse_ts_expr("1").unwrap(),
                },
            ];

            let stream = ts_template! {
                export function @{fn_name_ident}(): @{interface_ident} {
                    return {
                        {#if has_fields}
                            {#for field in &fields}
                                @{field.name_expr}: @{field.value_expr},
                            {/for}
                        {/if}
                    } as @{interface_ident};
                }
            };

            let source = stream.source();
            assert!(source.contains("export function optionsDefault(): Options"));
            assert!(source.contains("\"debug\": false"));
            assert!(source.contains("\"level\": 1"));
            assert!(source.contains("} as Options"));
        }

        // -------------------------------------------------------------------------
        // derive(Serialize) Template Tests
        // -------------------------------------------------------------------------

        /// Serialize for class - mirrors derive_serialize.rs class implementation
        #[test]
        fn derive_serialize_class_template() {
            let class_name = "User";
            let class_ident = ident(class_name);
            let fn_serialize_ident = ident("serializeUser");
            let fn_serialize_internal_ident = ident("serializeUserInternal");
            let fn_serialize_internal_expr: Expr = fn_serialize_internal_ident.clone().into();
            let serialize_context_ident = ident("SerializeContext");
            let serialize_context_expr: Expr = serialize_context_ident.clone().into();
            let has_regular = true;

            struct RegularField {
                field_ident: TsIdent,
                json_key: String,
                serialize_expr: Expr,
            }

            let regular_fields: Vec<RegularField> = vec![
                RegularField {
                    field_ident: ident("name"),
                    json_key: "name".to_string(),
                    serialize_expr: *parse_ts_expr("value.name").unwrap(),
                },
                RegularField {
                    field_ident: ident("age"),
                    json_key: "age".to_string(),
                    serialize_expr: *parse_ts_expr("value.age").unwrap(),
                },
            ];

            let stream = ts_template! {
                /** Serializes a User to JSON string */
                export function @{fn_serialize_ident}(value: @{class_ident}): string {
                    const ctx = @{serialize_context_expr}.create();
                    return JSON.stringify(@{fn_serialize_internal_expr}(value, ctx));
                }

                /** Internal serializer with context for cycle detection */
                export function @{fn_serialize_internal_ident}(value: @{class_ident}, ctx: @{serialize_context_ident}): Record<string, unknown> {
                    const existingId = ctx.getId(value);
                    if (existingId !== undefined) {
                        return { __ref: existingId };
                    }
                    const __id = ctx.register(value);
                    const result: Record<string, unknown> = {
                        __type: "@{class_name}",
                        __id,
                    };
                    {#if has_regular}
                        {#for field in &regular_fields}
                            result["@{field.json_key}"] = @{field.serialize_expr};
                        {/for}
                    {/if}
                    return result;
                }
            };

            let source = stream.source();
            assert!(source.contains("export function serializeUser(value: User): string"));
            assert!(source.contains("const ctx = SerializeContext.create()"));
            assert!(source.contains("return JSON.stringify(serializeUserInternal(value, ctx))"));
            assert!(source.contains("export function serializeUserInternal(value: User, ctx: SerializeContext)"));
            assert!(source.contains("const existingId = ctx.getId(value)"));
            assert!(source.contains("return { __ref: existingId }"));
            assert!(source.contains(r#"__type: "User""#));
            assert!(source.contains(r#"result["name"] = value.name"#));
            assert!(source.contains(r#"result["age"] = value.age"#));
        }

        // -------------------------------------------------------------------------
        // derive(Deserialize) Template Tests
        // -------------------------------------------------------------------------

        /// Deserialize for class - mirrors derive_deserialize.rs implementation
        #[test]
        fn derive_deserialize_class_template() {
            let class_name = "User";
            let class_ident = ident(class_name);
            let class_expr: Expr = class_ident.clone().into();
            let fn_deserialize_ident = ident("deserializeUser");
            let deserialize_options_ident = ident("DeserializeOptions");
            let return_type_ident = ident(class_name);
            let has_required = true;
            let has_optional = true;

            struct RequiredField {
                field_ident: TsIdent,
                json_key: String,
                raw_var: String,
            }

            struct OptionalField {
                field_ident: TsIdent,
                json_key: String,
                raw_var: String,
            }

            let required_fields: Vec<RequiredField> = vec![
                RequiredField {
                    field_ident: ident("name"),
                    json_key: "name".to_string(),
                    raw_var: "__raw_name".to_string(),
                },
            ];

            let optional_fields: Vec<OptionalField> = vec![
                OptionalField {
                    field_ident: ident("age"),
                    json_key: "age".to_string(),
                    raw_var: "__raw_age".to_string(),
                },
            ];

            let stream = ts_template! {
                /** Deserializes input to User */
                export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
                    try {
                        const obj = typeof input === "string" ? JSON.parse(input) : input;
                        const instance = new @{class_expr}();
                        {#if has_required}
                            {#for field in &required_fields}
                                const @{ident(&field.raw_var)} = obj["@{field.json_key}"];
                                instance.@{field.field_ident} = @{ident(&field.raw_var)};
                            {/for}
                        {/if}
                        {#if has_optional}
                            {#for field in &optional_fields}
                                if ("@{field.json_key}" in obj) {
                                    const @{ident(&field.raw_var)} = obj["@{field.json_key}"];
                                    instance.@{field.field_ident} = @{ident(&field.raw_var)};
                                }
                            {/for}
                        {/if}
                        return instance;
                    } catch (e) {
                        throw new Error("Deserialization failed");
                    }
                }
            };

            let source = stream.source();
            assert!(source.contains("export function deserializeUser(input: unknown, opts?: DeserializeOptions): User"));
            assert!(source.contains("const obj = typeof input === \"string\" ? JSON.parse(input) : input"));
            assert!(source.contains("const instance = new User()"));
            assert!(source.contains(r#"const __raw_name = obj["name"]"#));
            assert!(source.contains("instance.name = __raw_name"));
            assert!(source.contains(r#"if ("age" in obj)"#));
            assert!(source.contains(r#"const __raw_age = obj["age"]"#));
            assert!(source.contains("instance.age = __raw_age"));
            assert!(source.contains("return instance"));
        }

        // -------------------------------------------------------------------------
        // Match with Pattern Binding Tests
        // -------------------------------------------------------------------------

        /// Match with pattern binding - tests variable scoping in match arms
        #[test]
        fn derive_deserialize_match_type_category() {
            #[derive(Clone)]
            enum TypeCategory {
                Primitive,
                Array(String),
                Optional(String),
            }

            let type_cat = TypeCategory::Array("number".to_string());
            let field_ident = ident("scores");
            let raw_value = ident("rawValue");

            let stream = ts_template! {
                {#match type_cat}
                    {:case TypeCategory::Primitive}
                        instance.@{field_ident} = @{raw_value};
                    {:case TypeCategory::Array(inner)}
                        instance.@{field_ident} = @{raw_value} as @{ident(&inner)}[];
                    {:case TypeCategory::Optional(inner)}
                        if (@{raw_value} !== null && @{raw_value} !== undefined) {
                            instance.@{field_ident} = @{raw_value} as @{ident(&inner)};
                        }
                {/match}
            };

            let source = stream.source();
            // Should contain the Array case since type_cat is Array("number")
            assert!(source.contains("instance.scores = rawValue as number[]"));
        }

        // -------------------------------------------------------------------------
        // Body Macro Tests (Class Member Syntax)
        // -------------------------------------------------------------------------

        /// Body macro for static method - mirrors the pattern in all derive macros
        #[test]
        fn body_static_method_template() {
            let class_ident = ident("Calculator");
            let fn_external_ident = ident("add");
            let fn_external_expr: Expr = fn_external_ident.clone().into();

            let class_body = body! {
                static add(a: number, b: number): number {
                    return @{fn_external_expr}(a, b);
                }
            };

            let source = class_body.source();
            assert!(source.contains("static add(a: number, b: number): number"));
            assert!(source.contains("return add(a, b)"));
        }

        /// Body macro for constructor - simulates derive(Deserialize) constructor pattern
        #[test]
        fn body_constructor_template() {
            let has_required = true;
            let has_optional = true;

            struct FieldInit {
                field_ident: TsIdent,
                json_key: String,
            }

            let required_fields = vec![
                FieldInit { field_ident: ident("id"), json_key: "id".to_string() },
            ];
            let optional_fields = vec![
                FieldInit { field_ident: ident("name"), json_key: "name".to_string() },
            ];

            let class_body = body! {
                constructor(props: Record<string, unknown>) {
                    {#if has_required}
                        {#for field in &required_fields}
                            this.@{field.field_ident} = props["@{field.json_key}"] as any;
                        {/for}
                    {/if}
                    {#if has_optional}
                        {#for field in &optional_fields}
                            if ("@{field.json_key}" in props) {
                                this.@{field.field_ident} = props["@{field.json_key}"] as any;
                            }
                        {/for}
                    {/if}
                }
            };

            let source = class_body.source();
            assert!(source.contains("constructor(props: Record<string, unknown>)"));
            assert!(source.contains(r#"this.id = props["id"] as any"#));
            assert!(source.contains(r#"if ("name" in props)"#));
            assert!(source.contains(r#"this.name = props["name"] as any"#));
        }

        // -------------------------------------------------------------------------
        // $typescript Directive Tests
        // -------------------------------------------------------------------------

        /// Combining standalone function with class body using $typescript
        #[test]
        fn typescript_directive_compose_template() {
            let class_ident = ident("User");
            let fn_name_ident = ident("cloneUser");
            let fn_name_expr: Expr = fn_name_ident.clone().into();

            let standalone = ts_template! {
                export function @{fn_name_ident}(value: @{class_ident}): @{class_ident} {
                    return { ...value };
                }
            };

            let class_body = body! {
                static clone(value: @{class_ident}): @{class_ident} {
                    return @{fn_name_expr}(value);
                }
            };

            let combined = ts_template! {
                {$typescript standalone}
                {$typescript class_body}
            };

            let source = combined.source();
            assert!(source.contains("export function cloneUser(value: User): User"));
            assert!(source.contains("static clone(value: User): User"));
            assert!(source.contains("return cloneUser(value)"));
        }

        // -------------------------------------------------------------------------
        // ident! Macro Tests
        // -------------------------------------------------------------------------

        /// Formatted identifier using ident! macro - common pattern in derive macros
        #[test]
        fn ident_macro_formatted_template() {
            let class_name = "User";
            let fn_clone_ident = ident!("{}Clone", class_name.to_lowercase());
            let fn_hash_ident = ident!("{}HashCode", class_name.to_lowercase());
            let fn_default_ident = ident!("{}Default", class_name.to_lowercase());

            let stream = ts_template! {
                export function @{fn_clone_ident}(value: @{ident(class_name)}): @{ident(class_name)} {
                    return { ...value };
                }
                export function @{fn_hash_ident}(value: @{ident(class_name)}): number {
                    return 0;
                }
                export function @{fn_default_ident}(): @{ident(class_name)} {
                    return {} as @{ident(class_name)};
                }
            };

            let source = stream.source();
            assert!(source.contains("export function userClone(value: User): User"));
            assert!(source.contains("export function userHashCode(value: User): number"));
            assert!(source.contains("export function userDefault(): User"));
        }
    }

}
