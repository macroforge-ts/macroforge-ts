use ts_macro_derive::ts_macro_derive;
use ts_quote::{body, ts_template};
use ts_syn::{Data, DataInterface, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

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
pub fn derive_json_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            // Use Rust-style templating for clean code generation!
            // body! wrapper ensures this gets inserted as class members
            Ok(body! {
                toJSON(): Record<string, unknown> {

                    const result: Record<string, unknown> = {};

                    {#for field in class.field_names()}
                        result.@{field} = this.@{field};
                    {/for}

                    return result;
                }
            })
        }
        Data::Enum(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(JSON) */ can only target classes",
        )),
        Data::Interface(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(JSON) */ can only target classes, not interfaces",
        )),
    }
}

#[ts_macro_derive(
    FieldController,
    description = "Generates depth-aware field controller helpers for reactive forms",
    attributes(fieldController, textAreaController)
)]
pub fn field_controller_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
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
                        capitalize(field_name),                   // label_text
                        format!("\"{}\"", field_name),            // field_path_literal
                        format!("{}FieldPath", field_name),       // field_path_prop
                        format!("{}FieldController", field_name), // field_controller_prop
                        field.ts_type.trim_end_matches(';').trim(),
                    )
                })
                .collect();

            // ===== Generate All Runtime Code in Single Template =====
            // body! wrapper ensures this gets inserted as class members

            let stream = body! {
                @{base_props_method}<D extends number, const P extends DeepPath<@{class_name}, D>, V = DeepValue<@{class_name}, P, never, D>>(
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

                    @{field_path_prop} = [@{field_path_literal}];


                    @{field_controller_prop}(superForm: SuperForm<@{class_name}>): @{controller_type}<@{class_name}, @{field_type}, 1> {
                        const fieldPath = this.@{field_path_prop};

                        return {
                            fieldPath,
                            baseProps: this.@{base_props_method}(
                                superForm,
                                fieldPath,
                                {
                                    labelText: "@{label_text}"
                                }
                            )
                        };
                    };
                {/for}
            };
            Ok(stream)
        }
        Data::Enum(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(FieldController) */ can only target classes or interfaces",
        )),
        Data::Interface(interface) => {
            // For interfaces, we generate a companion namespace with functions
            generate_field_controller_for_interface(interface, &input)
        }
    }
}

fn generate_field_controller_for_interface(
    interface: &DataInterface,
    input: &DeriveInput,
) -> Result<TsStream, MacroforgeError> {
    // Collect decorated fields from interface
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

    let interface_name = input.name();
    let base_props_fn = format!("make{}BaseProps", interface_name);

    // Prepare field data for template
    let field_data: Vec<_> = decorated_fields
        .iter()
        .map(|field| {
            let field_name = &field.name;
            (
                capitalize(field_name),                   // label_text
                format!("\"{}\"", field_name),            // field_path_literal
                format!("{}FieldPath", field_name),       // field_path_const
                format!("{}FieldController", field_name), // field_controller_fn
                field.ts_type.trim_end_matches(';').trim(),
            )
        })
        .collect();

    // Generate code that will be inserted after the interface (default "below" location).
    // Macro authors have full control over the output structure.
    let stream = ts_template! {
        export function @{base_props_fn}<D extends number, const P extends DeepPath<@{interface_name}, D>, V = DeepValue<@{interface_name}, P, never, D>>(
            superForm: SuperForm<@{interface_name}>,
            path: P,
            overrides?: BasePropsOverrides<@{interface_name}, V, D>
        ): BaseFieldProps<@{interface_name}, V, D> {
            const proxy = formFieldProxy(superForm, path);
            const baseProps = {
                fieldPath: path,
                ...(overrides ?? {}),
                value: proxy.value,
                errors: proxy.errors,
                superForm
            };
            return baseProps;
        }

        {#for (label_text, field_path_literal, field_path_const, field_controller_fn, field_type) in field_data}
            {%let controller_type = format!("{}FieldController", label_text)}

            export const @{field_path_const} = [@{field_path_literal}] as const;

            export function @{field_controller_fn}(superForm: SuperForm<@{interface_name}>): @{controller_type}<@{interface_name}, @{field_type}, 1> {
                const fieldPath = @{field_path_const};

                return {
                    fieldPath,
                    baseProps: @{base_props_fn}(
                        superForm,
                        fieldPath,
                        {
                            labelText: "@{label_text}"
                        }
                    )
                };
            }
        {/for}
    };
    Ok(stream)
}
