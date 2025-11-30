#[cfg(test)]
mod tests {
    use ts_quote::ts_template;
    use ts_syn::{Data, DeriveInput, ParseTs, TsStream};

    fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }

    #[test]
    pub fn derive_json_macro() {
        let raw = &mut TsStream::from_string(include_str!("./fixtures/macro-user.ts").to_owned());

        let input = DeriveInput::parse(raw).unwrap();

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

                // Assertions
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
        let raw = &mut TsStream::from_string(
            include_str!("./fixtures/field-controller-test.ts").to_owned(),
        );

        let input = DeriveInput::parse(raw).unwrap();

        match &input.data {
            Data::Class(class) => {
                // Collect decorated fields
                let decorated_fields: Vec<_> = class
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
                        {%let controller_type = format!("{}FieldController", label_text)}

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

                // Assertions
                assert!(source.contains("makeFormModelBaseProps"));
                assert!(source.contains("memoFieldController(superForm: SuperForm<FormModel>)"));
                assert!(
                    source.contains("descriptionFieldController(superForm: SuperForm<FormModel>)")
                );
                // Check for correct type generation in controller method
                assert!(source.contains("MemoFieldController<FormModel, string | null, 1>"));
                // Check static block generation
                assert!(source.contains("this.prototype.memoFieldPath = [\"memo\"];"));
            }
            _ => panic!("Expected class data in field-controller-test.ts"),
        }
    }

    #[test]
    fn test_json_macro_pattern() {
        let field_name_str = "id";
        // Use Ident to simulate what comes from class.field_names() which returns &str or String usually?
        // Actually, checking native.rs: class.field_names() returns Vec<String> or similar?
        // Let's assume strings for now as that simplifies testing spacing without hygiene noise.
        let field_name = field_name_str.to_string();
        // Simulate the iterator
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

        // Verify that result.id = this.id; is generated correctly
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
        let class_name = class_name_str.to_string(); // Use String, not Ident

        let field_name_str = "memo";
        let field_type = "string | null";

        // `field_data` is a Vec of tuples where each element will be interpolated
        let field_data_tuple = (
            format!("\"{}\"", capitalize(field_name_str)), // label_text
            format!("\"{}\"", field_name_str),             // field_path_literal
            format!("{}FieldPath", field_name_str),        // field_path_prop (String)
            format!("{}FieldController", field_name_str),  // field_controller_prop (String)
            field_type,                                    // field_type
        );
        let field_data: Vec<(_, _, _, _, _)> = vec![field_data_tuple];

        let base_props_method = format!("make{}BaseProps", class_name_str); // String

        // The entire template is passed as a TokenStream to ts_template!
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
                {%let controller_type_ident = format!("{}", label_text).replace("\"", "") + "FieldController"}

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
            s.contains(&format!(
                "this.prototype.{}",
                format!("{}FieldPath", field_name_str)
            )),
            "Expected 'this.prototype.' and field_path_prop to be concatenated, but found: {}",
            s
        );

        // Assert that 'this.' and 'base_props_method' are concatenated
        assert!(
            s.contains(&format!("this.{}(", base_props_method)),
            "Expected 'this.' and base_props_method to be concatenated, but found: {}",
            s
        );

        // Check a specific part of the generated code for the output without unwanted spaces.
        let expected_concat_ident_start = format!("make{}BaseProps<", class_name_str);
        assert!(
            s.contains(&expected_concat_ident_start),
            "Failed to find expected concatenated identifier at start: {}",
            s
        );

        // Example: const P extends DeepPath<FormModel, D>
        let expected_deep_path = format!("DeepPath<{}, D>", class_name_str);
        assert!(
            s.contains(&expected_deep_path),
            "Failed to find expected DeepPath part: {}",
            s
        );

        // Example: const fieldPath = this.memoFieldPath;
        let expected_field_path_assignment = format!(
            "const fieldPath = this.{};",
            format!("{}FieldPath", field_name_str)
        );
        assert!(
            s.contains(&expected_field_path_assignment),
            "Failed to find expected fieldPath assignment: {}",
            s
        );

        // Example: baseProps: this.makeFormModelBaseProps(
        let expected_base_props_call = format!("baseProps : this.{}(", base_props_method);
        assert!(
            s.contains(&expected_base_props_call),
            "Failed to find expected baseProps method call: {}",
            s
        );
    }
}
