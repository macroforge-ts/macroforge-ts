use ts_macro_abi::{Diagnostic, DiagnosticLevel, MacroResult, Patch, SpanIR, insert_into_class};
use ts_macro_derive::ts_macro_derive;
use ts_quote::ts_template;
use ts_syn::{Data, DeriveInput, TsStream, parse_ts_macro_input};

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[ts_macro_derive(
    JSON,
    description = "Generates a toJSON() implementation that returns a plain object with all fields"
)]
pub fn derive_json_macro(mut input: TsStream) -> MacroResult {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.field_names();

            // Use Svelte-style templating for clean code generation!
            let runtime_code = ts_template! {
                #{class_name}.prototype.toJSON = function() {

                    const result = {};

                    {#each fields as field}
                        result.#{field} = this.#{field};
                    {/each}

                    return result;
                };
            };

            // Generate type signature (use string for TypeScript syntax)
            let type_signature = "    toJSON(): Record<string, unknown>;\n";

            MacroResult {
                runtime_patches: vec![Patch::Insert {
                    at: SpanIR {
                        start: input.target_span().end,
                        end: input.target_span().end,
                    },
                    code: runtime_code.into(),
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
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            // Collect decorated fields
            let decorated_fields: Vec<_> = class
                .fields()
                .iter()
                .filter(|field| field.decorators.iter().any(|d| d.name == "FieldController"))
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
                    )
                })
                .collect();

            // ===== Generate All Runtime Code in Single Template =====

            let runtime_code = ts_template! {
                #{class_name}.prototype.#{base_props_method} = function (superForm, path, overrides) {
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

                {#each field_data as (label_text, field_path_literal, field_path_prop, field_controller_prop)}
                    #{class_name}.prototype.#{field_path_prop} = [#{field_path_literal}];

                    #{class_name}.prototype.#{field_controller_prop} = function (superForm) {
                        const fieldPath = this.#{field_path_prop};

                        return {
                            fieldPath,
                            baseProps: this.#{base_props_method}(
                                superForm,
                                fieldPath,
                                {
                                    labelText: #{label_text}
                                }
                            )
                        };
                    };
                {/each}
            };

            // ===== Generate Type Code =====

            let mut type_code = format!(
                "    make{class_name}BaseProps<\n\
                 D extends number,\n        \
                 const P extends DeepPath<{class_name}, D>,\n        \
                 V = DeepValue<{class_name}, P, never, D>\n    \
                 >(\n        \
                 superForm: SuperForm<{class_name}>,\n        \
                 path: P,\n        \
                 overrides?: BasePropsOverrides<{class_name}, V, D>\n    \
                 ): BaseFieldProps<{class_name}, V, D>;\n"
            );

            for field in &decorated_fields {
                let field_name = &field.name;
                let field_type = &field.ts_type;
                let field_path_name = format!("{}FieldPath", field_name);
                let controller_name = format!("{}FieldController", field_name);
                let controller_type = format!("{}FieldController", capitalize(field_name));

                type_code.push_str(&format!(
                    "    private readonly {}: [\"{}\"];\n",
                    field_path_name, field_name
                ));

                type_code.push_str(&format!(
                    "    {}(superForm: SuperForm<{}>): {}<{}, {}, 1>;\n",
                    controller_name, class_name, controller_type, class_name, field_type
                ));
            }

            // ===== Create Patches =====

            let mut type_patches = vec![Patch::Delete {
                span: input.decorator_span(),
            }];

            let mut runtime_patches = vec![];

            // Delete all @FieldController decorators from both runtime and type patches
            for field in class.fields() {
                for decorator in &field.decorators {
                    if decorator.name == "FieldController" {
                        type_patches.push(Patch::Delete {
                            span: decorator.span,
                        });
                        runtime_patches.push(Patch::Delete {
                            span: decorator.span,
                        });
                    }
                }
            }

            // Insert generated code
            type_patches.push(insert_into_class(class.body_span(), type_code));

            runtime_patches.push(Patch::Insert {
                at: SpanIR {
                    start: input.target_span().end,
                    end: input.target_span().end,
                },
                code: runtime_code.into(),
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
