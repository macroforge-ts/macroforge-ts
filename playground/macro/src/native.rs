use ts_macro_abi::{insert_into_class, Diagnostic, DiagnosticLevel, MacroResult, Patch, SpanIR};
use ts_macro_derive::ts_macro_derive;
use ts_syn::{parse_ts_macro_input, Data, DeriveInput, TsStream};

#[ts_macro_derive(JSON, description = "Generates a toJSON() implementation that returns a plain object with all fields")]
pub fn derive_json_macro(mut input: TsStream) -> MacroResult {
    // Parse using the syn-like API
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

            // Generate toJSON implementation using format!()
            let field_entries: Vec<String> = class
                .field_names()
                .map(|f| format!("            {}: this.{},", f, f))
                .collect();

            let body = if field_entries.is_empty() {
                "        return {};".to_string()
            } else {
                format!("        return {{\n{}\n        }};", field_entries.join("\n"))
            };

            let runtime_code = format!(
                r#"
{class_name}.prototype.toJSON = function () {{
{body}
}};
"#
            );

            let type_signature = "    toJSON(): Record<string, unknown>;\n".to_string();

            MacroResult {
                runtime_patches: vec![Patch::Insert {
                    at: SpanIR {
                        start: input.target_span().end,
                        end: input.target_span().end,
                    },
                    code: runtime_code,
                }],
                type_patches: vec![
                    Patch::Delete {
                        span: input.decorator_span(),
                    },
                    insert_into_class(class.body_span(), type_signature),
                ],
                ..Default::default()
            }
        }
        Data::Enum(_) => MacroResult {
            diagnostics: vec![Diagnostic {
                level: DiagnosticLevel::Error,
                message: "@Derive(JSON) can only target classes".to_string(),
                span: Some(input.decorator_span()),
                notes: vec![],
                help: None,
            }],
            ..Default::default()
        },
    }
}

#[ts_macro_derive(
    FieldController,
    description = "Generates depth-aware field controller helpers for reactive forms",
    attributes(FieldController)
)]
pub fn field_controller_macro(mut input: TsStream) -> MacroResult {
    // Parse using the syn-like API
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            // Find fields with @FieldController decorator
            let decorated_fields: Vec<_> = class
                .fields()
                .iter()
                .filter(|field| field.decorators.iter().any(|d| d.name == "FieldController"))
                .map(|field| FieldInfo {
                    name: field.name.clone(),
                    ts_type: field.ts_type.clone(),
                })
                .collect();

            if decorated_fields.is_empty() {
                return MacroResult {
                    diagnostics: vec![Diagnostic {
                        level: DiagnosticLevel::Warning,
                        message: "@Derive(FieldController) found no @FieldController decorators"
                            .to_string(),
                        span: Some(input.decorator_span()),
                        notes: vec![],
                        help: Some(
                            "Add @FieldController decorators to fields you want to generate controllers for"
                                .into(),
                        ),
                    }],
                    ..Default::default()
                };
            }

            let class_name = input.name();
            let runtime_code = generate_field_controller_runtime(class_name, &decorated_fields);
            let type_code = generate_field_controller_types(class_name, &decorated_fields);

            // Delete the class decorator and all @FieldController field decorators
            let mut type_patches = vec![Patch::Delete {
                span: input.decorator_span(),
            }];

            // Delete @FieldController decorators from fields
            for field in class.fields() {
                for decorator in &field.decorators {
                    if decorator.name == "FieldController" {
                        type_patches.push(Patch::Delete {
                            span: decorator.span,
                        });
                    }
                }
            }

            type_patches.push(insert_into_class(class.body_span(), type_code));

            let mut runtime_patches = vec![];
            for field in class.fields() {
                for decorator in &field.decorators {
                    if decorator.name == "FieldController" {
                        runtime_patches.push(Patch::Delete {
                            span: decorator.span,
                        });
                    }
                }
            }

            runtime_patches.push(Patch::Insert {
                at: SpanIR {
                    start: input.target_span().end,
                    end: input.target_span().end,
                },
                code: runtime_code,
            });

            MacroResult {
                runtime_patches,
                type_patches,
                ..Default::default()
            }
        }
        Data::Enum(_) => MacroResult {
            diagnostics: vec![Diagnostic {
                level: DiagnosticLevel::Error,
                message: "@Derive(FieldController) can only target classes".to_string(),
                span: Some(input.decorator_span()),
                notes: vec![],
                help: None,
            }],
            ..Default::default()
        },
    }
}

// ============================================================================
// Helper Types
// ============================================================================

struct FieldInfo {
    name: String,
    ts_type: String,
}

// ============================================================================
// FieldController Code Generation
// ============================================================================

fn generate_field_controller_runtime(class_name: &str, decorated_fields: &[FieldInfo]) -> String {
    // Generate the base props maker function
    let make_base_props = format!(
        r#"
/**
 * Creates BaseFieldProps for {class_name} type with depth metadata preserved.
 * This function ensures the depth parameter D is properly propagated through the type system.
 */
{class_name}.prototype.make{class_name}BaseProps = function <
    D extends number,
    const P extends DeepPath<{class_name}, D>,
    V = DeepValue<{class_name}, P, never, D>
>(
    superForm: SuperForm<{class_name}>,
    path: P,
    overrides?: BasePropsOverrides<{class_name}, V, D>
): BaseFieldProps<{class_name}, V, D> {{
    const proxy = formFieldProxy(superForm, path);
    const baseProps: BaseFieldProps<{class_name}, V, D> = {{
        fieldPath: path,
        ...(overrides ?? {{}}),
        value: proxy.value as unknown as BaseFieldProps<{class_name}, V, D>['value'],
        errors: proxy.errors,
        superForm
    }};

    return baseProps;
}};
"#
    );

    // Generate field controller methods
    let field_methods: Vec<String> = decorated_fields
        .iter()
        .map(|field| generate_field_controller_method(class_name, field))
        .collect();

    format!("{}{}", make_base_props, field_methods.join("\n"))
}

fn generate_field_controller_method(class_name: &str, field: &FieldInfo) -> String {
    let field_name = &field.name;
    let field_type = &field.ts_type;
    let controller_name = format!("{field_name}FieldController");
    let controller_interface = format!("{}FieldController", capitalize(field_name));
    let label_text = capitalize(field_name);

    format!(
        r#"
{class_name}.prototype.{field_name}FieldPath = ["{field_name}"] as const;

{class_name}.prototype.{controller_name} = function (
    superForm: SuperForm<{class_name}>
): {controller_interface}<{class_name}, {field_type}, 1> {{
    const fieldPath = this.{field_name}FieldPath;

    return {{
        fieldPath,
        baseProps: this.make{class_name}BaseProps<1, typeof fieldPath>(
            superForm,
            fieldPath,
            {{
                labelText: "{label_text}"
            }}
        )
    }};
}};
"#
    )
}

fn generate_field_controller_types(class_name: &str, decorated_fields: &[FieldInfo]) -> String {
    let mut code = Vec::new();

    // Add base props maker signature
    code.push(format!(
        r#"
    /**
     * Creates BaseFieldProps for {class_name} type with depth metadata preserved.
     * This function ensures the depth parameter D is properly propagated through the type system.
     */
    make{class_name}BaseProps<
        D extends number,
        const P extends DeepPath<{class_name}, D>,
        V = DeepValue<{class_name}, P, never, D>
    >(
        superForm: SuperForm<{class_name}>,
        path: P,
        overrides?: BasePropsOverrides<{class_name}, V, D>
    ): BaseFieldProps<{class_name}, V, D>;
"#
    ));

    // Add field controller signatures
    for field in decorated_fields {
        let field_name = &field.name;
        let field_type = &field.ts_type;
        let controller_name = format!("{field_name}FieldController");
        let controller_interface = format!("{}FieldController", capitalize(field_name));

        code.push(format!(
            r#"
    private readonly {field_name}FieldPath: ["{field_name}"];
    {controller_name}(
        superForm: SuperForm<{class_name}>
    ): {controller_interface}<{class_name}, {field_type}, 1>;
"#
        ));
    }

    code.join("\n")
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
