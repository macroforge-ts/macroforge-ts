#[cfg(test)]
mod tests {
    use swc_core::common::{FileName, GLOBALS, Globals, SourceMap, sync::Lrc};
    use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
    use ts_syn::abi::{MacroContextIR, SpanIR};
    use ts_quote::ts_template;
    use ts_syn::{Data, DeriveInput, ParseTs, TsStream, lower_classes, lower_interfaces};

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
                        {%let controller_type = format!("{}FieldController", label_text.replace("\"", ""))}

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
                {%let controller_type_ident = label_text.replace("\"", "") + "FieldController"}

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
}
