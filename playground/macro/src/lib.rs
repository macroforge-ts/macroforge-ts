use std::{path::PathBuf, sync::Arc};

use ts_macro_abi::{
    ClassIR, Diagnostic, DiagnosticLevel, MacroContextIR, MacroKind, MacroResult, Patch, SpanIR,
};
use ts_macro_host::{MacroManifest, MacroRegistry, Result, TsMacro};

const PACKAGE_NAME: &str = "@playground/macro";
const DERIVE_MODULE: &str = "@macro/derive";

pub fn register_playground_macros(registry: &MacroRegistry) -> Result<()> {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("macro.toml");
    let manifest = MacroManifest::from_toml_file(manifest_path)?;
    registry.register_manifest(PACKAGE_NAME, manifest)?;

    registry.register(DERIVE_MODULE, "JSON", Arc::new(DeriveJsonMacro))?;
    registry.register(DERIVE_MODULE, "FieldController", Arc::new(DeriveFieldControllerMacro))?;
    Ok(())
}

ts_macro_host::register_macro_package!(PACKAGE_NAME, register_playground_macros);

struct DeriveJsonMacro;

impl TsMacro for DeriveJsonMacro {
    fn name(&self) -> &str {
        "JSON"
    }

    fn kind(&self) -> MacroKind {
        MacroKind::Derive
    }

    fn run(&self, ctx: MacroContextIR) -> MacroResult {
        let class = match ctx.as_class() {
            Some(class) => class,
            None => {
                return MacroResult {
                    diagnostics: vec![Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: "@Derive(JSON) can only target classes".to_string(),
                        span: Some(ctx.decorator_span),
                        notes: vec![],
                        help: Some(
                            "Remove the decorator or apply it to a class declaration".into(),
                        ),
                    }],
                    ..Default::default()
                };
            }
        };

        let insertion = insertion_span(&ctx);
        let post_class_insertion = SpanIR {
            start: ctx.target_span.end,
            end: ctx.target_span.end,
        };
        MacroResult {
            runtime_patches: vec![Patch::Insert {
                at: post_class_insertion,
                code: generate_to_json(class),
            }],
            type_patches: vec![
                Patch::Delete {
                    span: ctx.decorator_span,
                },
                Patch::Insert {
                    at: insertion,
                    code: generate_to_json_signature(),
                },
            ],
            ..Default::default()
        }
    }

    fn description(&self) -> &str {
        "Generates a toJSON() implementation that returns a plain object with all fields"
    }
}

fn insertion_span(ctx: &MacroContextIR) -> SpanIR {
    let end = ctx.target_span.end.saturating_sub(1);
    SpanIR { start: end, end }
}

fn generate_to_json(class: &ClassIR) -> String {
    let mut entries = Vec::new();
    for field in &class.fields {
        entries.push(format!("            {}: this.{},", field.name, field.name));
    }

    let body = if entries.is_empty() {
        "        return {};".to_string()
    } else {
        format!("        return {{\n{}\n        }};", entries.join("\n"))
    };

    format!(
        r#"
{class_name}.prototype.toJSON = function () {{
{body}
}};
"#,
        class_name = class.name
    )
}

fn generate_to_json_signature() -> String {
    "    toJSON(): Record<string, unknown>;\n".to_string()
}

struct DeriveFieldControllerMacro;

impl TsMacro for DeriveFieldControllerMacro {
    fn name(&self) -> &str {
        "FieldController"
    }

    fn kind(&self) -> MacroKind {
        MacroKind::Derive
    }

    fn run(&self, ctx: MacroContextIR) -> MacroResult {
        let class = match ctx.as_class() {
            Some(class) => class,
            None => {
                return MacroResult {
                    diagnostics: vec![Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: "@Derive(FieldController) can only target classes".to_string(),
                        span: Some(ctx.decorator_span),
                        notes: vec![],
                        help: Some(
                            "Remove the decorator or apply it to a class declaration".into(),
                        ),
                    }],
                    ..Default::default()
                };
            }
        };

        // Find fields with Field decorator
        let decorated_fields: Vec<_> = class
            .fields
            .iter()
            .filter(|field| field.decorators.iter().any(|d| d.name == "Field"))
            .collect();

        if decorated_fields.is_empty() {
            return MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Warning,
                    message: "@Derive(FieldController) found no @Field decorators".to_string(),
                    span: Some(ctx.decorator_span),
                    notes: vec![],
                    help: Some(
                        "Add @Field decorators to fields you want to generate controllers for".into(),
                    ),
                }],
                ..Default::default()
            };
        }

        let insertion = SpanIR {
            start: class.body_span.end.saturating_sub(1),
            end: class.body_span.end.saturating_sub(1),
        };

        let post_class_insertion = SpanIR {
            start: ctx.target_span.end,
            end: ctx.target_span.end,
        };

        // Generate runtime code
        let runtime_code = generate_field_controller_runtime(class, &decorated_fields);

        // Generate type signatures
        let type_code = generate_field_controller_types(class, &decorated_fields);

        // Collect field decorator deletions
        let mut type_patches = vec![
            Patch::Delete {
                span: ctx.decorator_span,
            },
        ];

        for field in &decorated_fields {
            for decorator in &field.decorators {
                if decorator.name == "Field" {
                    type_patches.push(Patch::Delete {
                        span: decorator.span,
                    });
                }
            }
        }

        type_patches.push(Patch::Insert {
            at: insertion,
            code: type_code,
        });

        let mut runtime_patches = vec![];
        for field in &decorated_fields {
            for decorator in &field.decorators {
                if decorator.name == "Field" {
                    runtime_patches.push(Patch::Delete {
                        span: decorator.span,
                    });
                }
            }
        }

        runtime_patches.push(Patch::Insert {
            at: post_class_insertion,
            code: runtime_code,
        });

        MacroResult {
            runtime_patches,
            type_patches,
            ..Default::default()
        }
    }

    fn description(&self) -> &str {
        "Generates field controller helpers for form fields with @Field decorators"
    }
}

fn generate_field_controller_runtime(class: &ClassIR, decorated_fields: &[&ts_macro_abi::FieldIR]) -> String {
    let class_name = &class.name;

    // Generate the makeBaseProps function
    let make_base_props = format!(
        r#"
/**
 * Creates BaseFieldProps for {class_name} type with depth metadata preserved.
 * This function ensures the depth parameter D is properly propagated through the type system.
 */
function make{class_name}BaseProps(superForm, path, overrides) {{
    const proxy = formFieldProxy(superForm, path);
    const baseProps = {{
        fieldPath: path,
        ...(overrides ?? {{}}),
        value: proxy.value,
        errors: proxy.errors,
        superForm
    }};

    return baseProps;
}}
"#
    );

    let mut field_methods = Vec::new();

    for field in decorated_fields {
        let field_name = &field.name;
        let capitalized = capitalize(field_name);

        // Extract component type from decorator args
        let component = extract_component_type(field);

        field_methods.push(format!(
            r#"
{class_name}.prototype.make{capitalized}{component}Controller = function(superForm) {{
    this.{field_name}BaseProps = make{class_name}BaseProps(
        superForm,
        this.{field_name}FieldPath,
        {{
            labelText: "{capitalized}",
            placeholder: "Enter {field_name}..."
        }}
    );
}};
"#
        ));
    }

    format!("{}{}", make_base_props, field_methods.join("\n"))
}

fn generate_field_controller_types(class: &ClassIR, decorated_fields: &[&ts_macro_abi::FieldIR]) -> String {
    let class_name = &class.name;
    let mut code = Vec::new();

    // Generate the makeBaseProps function signature
    code.push(format!(
        r#"
    /**
     * Creates BaseFieldProps for {class_name} type with depth metadata preserved.
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

    for field in decorated_fields {
        let field_name = &field.name;
        let capitalized = capitalize(field_name);
        let component = extract_component_type(field);

        code.push(format!(
            r#"
    private readonly {field_name}FieldPath: ["{field_name}"];
    readonly {field_name}BaseProps: BaseFieldProps<{class_name}, {field_type}>;
    make{capitalized}{component}Controller(superForm: SuperForm<{class_name}>): void;
"#,
            field_type = &field.ts_type
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

fn extract_component_type(field: &ts_macro_abi::FieldIR) -> String {
    for decorator in &field.decorators {
        if decorator.name == "Field" {
            let args = &decorator.args_src;
            // Simple parsing: look for component: "TextArea" pattern
            if let Some(start) = args.find("component") {
                let remainder = &args[start..];
                if let Some(quote_start) = remainder.find('"') {
                    let after_quote = &remainder[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find('"') {
                        return after_quote[..quote_end].to_string();
                    }
                }
                // Try single quotes
                if let Some(quote_start) = remainder.find('\'') {
                    let after_quote = &remainder[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find('\'') {
                        return after_quote[..quote_end].to_string();
                    }
                }
            }
        }
    }
    "TextArea".to_string() // Default
}
