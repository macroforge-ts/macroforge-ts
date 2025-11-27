use ts_macro_derive::ts_macro_derive;
use ts_quote::ts_template;
use ts_syn::{Data, DeriveInput, TsMacroError, TsStream, parse_ts_macro_input};

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
pub fn derive_json_macro(mut input: TsStream) -> Result<TsStream, TsMacroError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            // Use Svelte-style templating for clean code generation!
            let stream = ts_template! {
                toJSON(): Record<string, unknown> {

                    const result = {};

                    {#each class.field_names() as field}
                        result.${field} = this.${field};
                    {/each}

                    return result;
                };
            };
            Ok(stream)
        }
        Data::Enum(_) => Err(TsMacroError::new(
            input.decorator_span(),
            "@Derive(JSON) can only target classes",
        )),
    }
}

#[ts_macro_derive(
    FieldController,
    description = "Generates depth-aware field controller helpers for reactive forms",
    attributes(fieldController, textAreaController)
)]
pub fn field_controller_macro(mut input: TsStream) -> Result<TsStream, TsMacroError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

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
                        .any(|d| d.name == "FieldController" || d.name == "textAreaController")
                })
                .collect();

            if decorated_fields.is_empty() {
                // Return empty stream
                return Ok(ts_template! {});
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
                        &field.ts_type,
                    )
                })
                .collect();

            // ===== Generate All Runtime Code in Single Template =====

            let stream = ts_template! {
                make${class_name}BaseProps<D extends number, const P extends DeepPath<${class_name}, D>, V = DeepValue<${class_name}, P, never, D>>(
                    superForm: SuperForm<${class_name}>,
                    path: P,
                    overrides?: BasePropsOverrides<${class_name}, V, D>
                 ): BaseFieldProps<${class_name}, V, D> {
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

                {#each field_data as (label_text, field_path_literal, field_path_prop, field_controller_prop, field_type)}
                    {@const controller_type = format!("{}FieldController", label_text)}

                    static {
                        this.prototype.${field_path_prop} = [${field_path_literal}];
                    }

                    ${field_controller_prop}(superForm: SuperForm<${class_name}>): ${controller_type}<${class_name}, ${field_type}, 1> {
                        const fieldPath = this.${field_path_prop};

                        return {
                            fieldPath,
                            baseProps: this.${base_props_method}(
                                superForm,
                                fieldPath,
                                {
                                    labelText: ${label_text}
                                }
                            )
                        };
                    };
                {/each}
            };
            Ok(stream)
        }
        Data::Enum(_) => Err(TsMacroError::new(
            input.decorator_span(),
            "@Derive(FieldController) can only target classes",
        )),
    }
}
