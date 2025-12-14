#[cfg(test)]
mod tests {
    use macroforge_ts_quote::ts_template;
    use macroforge_ts_syn::abi::{MacroContextIR, SpanIR};
    use macroforge_ts_syn::{
        lower_classes, lower_interfaces, Data, DeriveInput, ParseTs, TsStream,
    };
    use swc_core::common::{sync::Lrc, FileName, Globals, SourceMap, GLOBALS};
    use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

    fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }

    // Helper to create a TsStream with valid context for testing derive macros
    fn create_test_stream(source: &str) -> TsStream {
        GLOBALS.set(&Globals::new(), || {
            let cm: Lrc<SourceMap> = Default::default();
            let fm = cm.new_source_file(
                FileName::Custom("test.ts".into()).into(),
                source.to_string(),
            );

            let lexer = Lexer::new(
                Syntax::Typescript(TsSyntax {
                    tsx: false,
                    decorators: true,
                    ..Default::default()
                }),
                Default::default(),
                StringInput::from(&*fm),
                None,
            );

            let mut parser = Parser::new_from(lexer);
            let module = parser.parse_module().expect("Failed to parse test source");

            let classes = lower_classes(&module, source).expect("Failed to lower classes");
            let class = classes
                .first()
                .expect("Expected at least one class in test source")
                .clone();

            let ctx = MacroContextIR::new_derive_class(
                "TestMacro".to_string(),
                "test-macro".to_string(),
                SpanIR::new(0, 0), // Dummy spans
                class.span,
                "test.ts".to_string(),
                class,
                source.to_string(),
            );

            TsStream::with_context(source, "test.ts", ctx).unwrap()
        })
    }

    // Helper to create a TsStream with valid context for testing derive macros on interfaces
    fn create_test_stream_interface(source: &str) -> TsStream {
        GLOBALS.set(&Globals::new(), || {
            let cm: Lrc<SourceMap> = Default::default();
            let fm = cm.new_source_file(
                FileName::Custom("test.ts".into()).into(),
                source.to_string(),
            );

            let lexer = Lexer::new(
                Syntax::Typescript(TsSyntax {
                    tsx: false,
                    decorators: true,
                    ..Default::default()
                }),
                Default::default(),
                StringInput::from(&*fm),
                None,
            );

            let mut parser = Parser::new_from(lexer);
            let module = parser.parse_module().expect("Failed to parse test source");

            let interfaces = lower_interfaces(&module, source).expect("Failed to lower interfaces");
            let interface = interfaces
                .first()
                .expect("Expected at least one interface in test source")
                .clone();

            let ctx = MacroContextIR::new_derive_interface(
                "TestMacro".to_string(),
                "test-macro".to_string(),
                SpanIR::new(0, 0), // Dummy spans
                interface.span,
                "test.ts".to_string(),
                interface,
                source.to_string(),
            );

            TsStream::with_context(source, "test.ts", ctx).unwrap()
        })
    }

    #[test]
    pub fn derive_json_macro() {
        let raw = include_str!("./fixtures/macro-user.ts");
        let mut stream = create_test_stream(raw);

        let input = DeriveInput::parse(&mut stream).unwrap();

        match &input.data {
            Data::Class(class) => {
                // Use Rust-style templating for clean code generation!
                let stream = ts_template! {
                    toJSON(): Record<string, unknown> {

                        const result: Record<string, unknown> = {};

                        {#for field in class.field_names()}
                            result.@{field} = this.@{field};
                        {/for}

                        return result;
                    }
                };

                let source = stream.source();
                println!("Generated JSON Source:\n{}", source);

                // Assertions - expect clean formatting now!
                assert!(source.contains("toJSON(): Record<string, unknown>"));
                assert!(source.contains("result.id = this.id"));
                assert!(source.contains("result.name = this.name"));
                assert!(source.contains("result.role = this.role"));
            }
            _ => panic!("Expected class data in macro-user.ts"),
        }
    }

    #[test]
    pub fn field_controller_macro() {
        let raw = include_str!("./fixtures/field-controller.fixture.ts");
        let mut stream = create_test_stream_interface(raw);

        let input = DeriveInput::parse(&mut stream).unwrap();

        match &input.data {
            Data::Interface(interface) => {
                // Collect decorated fields
                let decorated_fields: Vec<_> = interface
                    .fields()
                    .iter()
                    .filter(|field| {
                        field
                            .decorators
                            .iter()
                            .any(|d| d.name == "fieldController" || d.name == "textAreaController")
                    })
                    .collect();

                let class_name = input.name();
                let base_props_method = format!("make{}BaseProps", class_name);

                // Prepare field data for template
                let field_data: Vec<_> = decorated_fields
                    .iter()
                    .map(|field| {
                        let field_name = &field.name;
                        (
                            format!("\"{}\"", capitalize(field_name)), // label_text
                            format!("\"{}\"", field_name),             // field_path_literal
                            format!("{}FieldPath", field_name),        // field_path_prop
                            format!("{}FieldController", field_name),  // field_controller_prop
                            &field.ts_type,
                        )
                    })
                    .collect();

                // ===== Generate All Runtime Code in Single Template =====

                let stream = ts_template! {
                    make@{class_name}BaseProps<D extends number, const P extends DeepPath<@{class_name}, D>, V = DeepValue<@{class_name}, P, never, D>>(
                        superForm: SuperForm<@{class_name}>,
                        path: P,
                        overrides?: BasePropsOverrides<@{class_name}, V, D>
                     ): BaseFieldProps<@{class_name}, V, D> {
                        const proxy = formFieldProxy(superForm, path);
                        const baseProps = {
                            fieldPath: path,
                            ...(overrides ?? {}),
                            value: proxy.value,
                            errors: proxy.errors,
                            superForm
                        };
                        return baseProps;
                    };

                    {#for (label_text, field_path_literal, field_path_prop, field_controller_prop, field_type) in field_data}
                        {$let controller_type = format!("{}FieldController", label_text.replace("\"", ""))}

                        static {
                            this.prototype.@{field_path_prop} = [@{field_path_literal}];
                        }

                        @{field_controller_prop}(superForm: SuperForm<@{class_name}>): @{controller_type}<@{class_name}, @{field_type}, 1> {
                            const fieldPath = this.@{field_path_prop};

                            return {
                                fieldPath,
                                baseProps: this.@{base_props_method}(
                                    superForm,
                                    fieldPath,
                                    {
                                        labelText: @{label_text}
                                    }
                                )
                            };
                        };
                    {/for}
                };

                let source = stream.source();
                println!("Generated FieldController Source:\n{}", source);

                // Assertions - matching SWC default formatting
                assert!(source.contains("makeFormModelBaseProps"));
                // SWC adds spaces around colons but NOT before generics
                assert!(source.contains("memoFieldController(superForm: SuperForm<FormModel>"));
                assert!(
                    source.contains("descriptionFieldController(superForm: SuperForm<FormModel>")
                );
                // Check correct type generation
                assert!(source.contains("MemoFieldController<FormModel, string | null, 1>"));
                // Check static block generation
                assert!(source.contains("this.prototype.memoFieldPath = ["));
                assert!(source.contains("\"memo\""));
            }
            _ => panic!("Expected interface data in field-controller.fixture.ts"),
        }
    }

    #[test]
    fn test_json_macro_pattern() {
        let field_name_str = "id";
        let field_name = field_name_str.to_string();
        let fields = vec![field_name];

        let stream: TsStream = ts_template! {
            toJSON(): Record<string, unknown> {

                const result: Record<string, unknown> = {};

                {#for field in fields}
                    result.@{field} = this.@{field};
                {/for}

                return result;
            }
        };

        let s = stream.source();
        println!("Generated JSON Source:\n{}", s);

        // Verify that result.id = this.id; is generated correctly with CLEAN formatting
        assert!(
            s.contains(&format!("result.{} =", field_name_str)),
            "Expected result.field to be concatenated. Found: {}",
            s
        );

        assert!(
            s.contains(&format!("this.{};", field_name_str)),
            "Expected this.field to be concatenated. Found: {}",
            s
        );
    }

    #[test]
    fn test_field_controller_template_spacing() {
        let class_name_str = "FormModel";
        let class_name = class_name_str.to_string();

        let field_name_str = "memo";
        let field_type = "string | null";

        let field_data_tuple = (
            format!("\"{}\"", capitalize(field_name_str)),
            format!("\"{}\"", field_name_str),
            format!("{}FieldPath", field_name_str),
            format!("{}FieldController", field_name_str),
            field_type,
        );
        let field_data: Vec<(_, _, _, _, _)> = vec![field_data_tuple];

        let base_props_method = format!("make{}BaseProps", class_name_str);

        let stream: TsStream = ts_template! {
            {|make@{class_name}BaseProps|}<D extends number, const P extends DeepPath<@{class_name}, D>, V = DeepValue<@{class_name}, P, never, D>>(
                superForm: SuperForm<@{class_name}>,
                path: P,
                overrides?: BasePropsOverrides<@{class_name}, V, D>
             ): BaseFieldProps<@{class_name}, V, D> {
                const proxy = formFieldProxy(superForm, path);
                const baseProps = {
                    fieldPath: path,
                    ...(overrides ?? {}),
                    value: proxy.value,
                    errors: proxy.errors,
                    superForm
                };
                return baseProps;
            };

            {#for (label_text, field_path_literal, field_path_prop, field_controller_prop, field_type) in field_data}
                {$let controller_type_ident = label_text.replace("\"", "") + "FieldController"}

                static {
                    this.prototype.@{field_path_prop} = [@{field_path_literal}];
                }

                @{field_controller_prop}(superForm: SuperForm<@{class_name}>): @{controller_type_ident}<@{class_name}, @{field_type}, 1> {
                    const fieldPath = this.@{field_path_prop};

                    return {
                        fieldPath,
                        baseProps: this.@{base_props_method}(
                            superForm,
                            fieldPath,
                            {
                                labelText: @{label_text}
                            }
                        )
                    };
                };
            {/for}
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        // Assert that 'make' and 'FormModelBaseProps' are concatenated
        assert!(
            s.contains(&format!("make{}BaseProps<", class_name_str)),
            "Expected 'make' and class name to be concatenated, but found: {}",
            s
        );

        // Assert that 'return' has a space
        assert!(
            s.contains("return baseProps"),
            "Expected 'return' to have a space before 'baseProps', but found: {}",
            s
        );

        // Assert that 'this.prototype.' and 'field_path_prop' are concatenated
        assert!(
            s.contains(&format!("this.prototype.{}FieldPath", field_name_str)),
            "Expected 'this.prototype.' and field_path_prop to be concatenated, but found: {}",
            s
        );

        // Assert that 'this.' and 'base_props_method' are concatenated
        assert!(
            s.contains(&format!("this.{}(", base_props_method)),
            "Expected 'this.' and base_props_method to be concatenated, but found: {}",
            s
        );
    }

    // ========== Ident Block {| |} Integration Tests ==========

    #[test]
    fn test_ident_block_basic_concatenation() {
        // Test that {| |} concatenates content without spaces
        let suffix = "Status";

        let stream: TsStream = ts_template! {
            const {|namespace@{suffix}|} = "value";
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        // Should produce "namespaceStatus" as a single identifier
        assert!(
            s.contains("namespaceStatus"),
            "Expected 'namespace' and 'Status' to be concatenated without space, but found: {}",
            s
        );
    }

    #[test]
    fn test_ident_block_function_name() {
        // Test real-world use case: generating function names
        let type_name = "User";

        let stream: TsStream = ts_template! {
            function {|get@{type_name}|}(): @{type_name} {
                return {} as @{type_name};
            }
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        // Function name should be concatenated
        assert!(
            s.contains("getUser()"),
            "Expected 'get' and 'User' to form 'getUser()', but found: {}",
            s
        );

        // Return type should have space before it
        assert!(
            s.contains("getUser(): User"),
            "Expected proper spacing around return type, but found: {}",
            s
        );
    }

    #[test]
    fn test_ident_block_multiple_interpolations() {
        // Test multiple @{} inside a single ident block
        let prefix = "get";
        let middle = "User";
        let suffix = "ById";

        let stream: TsStream = ts_template! {
            function {|@{prefix}@{middle}@{suffix}|}(id: string) { }
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        // All three parts should be concatenated
        assert!(
            s.contains("getUserById("),
            "Expected all parts to be concatenated into 'getUserById', but found: {}",
            s
        );
    }

    #[test]
    fn test_ident_block_preserves_external_spacing() {
        // Test that space before {| |} is preserved
        let name = "Handler";

        let stream: TsStream = ts_template! {
            export class {|Event@{name}|} { }
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        // Should have space between "class" and "EventHandler"
        assert!(
            s.contains("class EventHandler"),
            "Expected 'class EventHandler' with space, but found: {}",
            s
        );
    }

    #[test]
    fn test_ident_block_vs_regular_interpolation() {
        // Compare ident block to regular interpolation
        let type_name = "User";

        // With ident block - explicit no space
        let with_block: TsStream = ts_template! {
            {|create@{type_name}|}
        };

        // Regular interpolation - relies on heuristics
        let regular: TsStream = ts_template! {
            create@{type_name}
        };

        let block_s = with_block.source();
        let regular_s = regular.source();

        println!("With block: {}", block_s);
        println!("Regular: {}", regular_s);

        // Ident block should always produce concatenated result
        assert!(
            block_s.contains("createUser"),
            "Ident block should produce 'createUser', but found: {}",
            block_s
        );
    }

    #[test]
    fn test_ident_block_in_method_chain() {
        // Test ident blocks in method/property access patterns
        let prop = "Status";

        let stream: TsStream = ts_template! {
            const value = obj.{|get@{prop}|}();
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        // Should produce "obj.getStatus()"
        assert!(
            s.contains("obj.getStatus()"),
            "Expected 'obj.getStatus()', but found: {}",
            s
        );
    }

    #[test]
    fn test_ident_block_empty() {
        // Test empty ident block produces empty string
        let stream: TsStream = ts_template! {
            const prefix{||}suffix = 1;
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        // Should have prefix and suffix (empty block produces nothing)
        assert!(
            s.contains("prefix") && s.contains("suffix"),
            "Expected prefix and suffix in output, but found: {}",
            s
        );
    }

    #[test]
    fn test_ident_block_with_underscore_separator() {
        // Test ident blocks with underscores (snake_case generation)
        // All parts that need concatenation should be inside {| |}
        let entity = "user";
        let action = "create";

        let stream: TsStream = ts_template! {
            function {|@{entity}_@{action}|}() { }
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        // Should produce "user_create"
        assert!(
            s.contains("user_create()"),
            "Expected 'user_create()', but found: {}",
            s
        );
    }

    // ========== Union Type For Loop Tests ==========

    #[test]
    fn test_union_type_for_loop_basic() {
        // Test basic for loop with Vec<String> - simulating union type refs
        let type_refs: Vec<String> = vec!["User".to_string(), "Admin".to_string(), "Guest".to_string()];

        let stream: TsStream = ts_template! {
            function dispatch(value: any) {
                {#for type_ref in type_refs}
                    if (value.__type === "@{type_ref}") {
                        return @{type_ref}.__deserialize(value);
                    }
                {/for}
            }
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        assert!(s.contains("User.__deserialize"), "Expected User.__deserialize, found: {}", s);
        assert!(s.contains("Admin.__deserialize"), "Expected Admin.__deserialize, found: {}", s);
        assert!(s.contains("Guest.__deserialize"), "Expected Guest.__deserialize, found: {}", s);
    }

    #[test]
    fn test_union_type_for_loop_with_conditionals() {
        // Test for loop inside conditionals - like the union type pattern
        let type_refs: Vec<String> = vec!["Success".to_string(), "Failure".to_string()];
        let literals: Vec<String> = vec!["\"pending\"".to_string(), "\"active\"".to_string()];

        let is_literal_only = false;
        let is_type_ref_only = true;

        let stream: TsStream = ts_template! {
            function __deserialize(value: any) {
                {#if is_literal_only}
                    const allowedValues = [{#for lit in literals}@{lit}, {/for}] as const;
                    return value;
                {:else if is_type_ref_only}
                    const typeName = value.__type;
                    {#for type_ref in type_refs}
                        if (typeName === "@{type_ref}") {
                            return @{type_ref}.__deserialize(value);
                        }
                    {/for}
                    throw new Error("Unknown type");
                {:else}
                    return value;
                {/if}
            }
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        assert!(s.contains("Success.__deserialize"), "Expected Success.__deserialize, found: {}", s);
        assert!(s.contains("Failure.__deserialize"), "Expected Failure.__deserialize, found: {}", s);
    }

    #[test]
    fn test_union_type_for_loop_reuse() {
        // Test using the same Vec in multiple for loops - need to clone
        let type_refs: Vec<String> = vec!["TypeA".to_string(), "TypeB".to_string()];

        let stream: TsStream = ts_template! {
            function dispatch(value: any) {
                // First use
                {#for type_ref in type_refs.clone()}
                    console.log("@{type_ref}");
                {/for}

                // Second use
                {#for type_ref in type_refs}
                    return @{type_ref};
                {/for}
            }
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        // Count occurrences - should appear twice each
        assert!(s.matches("TypeA").count() >= 2, "Expected TypeA to appear at least twice, found: {}", s);
        assert!(s.matches("TypeB").count() >= 2, "Expected TypeB to appear at least twice, found: {}", s);
    }

    #[test]
    fn test_union_type_nested_if_for() {
        // Test nested if with for loop - matching the actual union type pattern
        let type_refs: Vec<String> = vec!["Option1".to_string(), "Option2".to_string()];
        let has_type_refs = !type_refs.is_empty();

        let stream: TsStream = ts_template! {
            function __deserialize(value: any) {
                {#if has_type_refs}
                    if (typeof value === "object" && value !== null) {
                        const __typeName = value.__type;
                        if (typeof __typeName === "string") {
                            {#for type_ref in type_refs}
                                if (__typeName === "@{type_ref}") {
                                    return @{type_ref}.__deserialize(value);
                                }
                            {/for}
                        }
                    }
                {/if}
                return value;
            }
        };

        let s = stream.source();
        println!("Generated Source:\n{}", s);

        assert!(s.contains("Option1.__deserialize"), "Expected Option1.__deserialize, found: {}", s);
        assert!(s.contains("Option2.__deserialize"), "Expected Option2.__deserialize, found: {}", s);
    }
}
