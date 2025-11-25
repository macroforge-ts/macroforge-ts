use ts_macro_abi::{
    ClassIR, Diagnostic, DiagnosticLevel, MacroContextIR, MacroResult, Patch, SpanIR,
};
use ts_macro_derive::TsMacroDefinition;

#[derive(TsMacroDefinition)]
#[ts_macro(
    package = "@playground/macro",
    module = "@macro/derive",
    name = "JSON",
    description = "Generates a toJSON() implementation that returns a plain object with all fields",
    runtime = ["native"]
)]
pub struct DeriveJsonMacro;

impl DeriveJsonMacro {
    fn expand(&self, ctx: MacroContextIR) -> MacroResult {
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
}

#[derive(TsMacroDefinition)]
#[ts_macro(
    package = "@playground/macro",
    module = "@macro/derive",
    name = "FieldController",
    description = "Generates depth-aware field controller helpers for reactive forms",
    runtime = ["native"],
    decorator(
        module = "@playground/macro",
        name = "FieldController",
        kind = "property",
        docs = "Marks a form field for FieldController macro expansion"
    )
)]
pub struct DeriveFieldControllerMacro;

impl DeriveFieldControllerMacro {
    fn expand(&self, ctx: MacroContextIR) -> MacroResult {
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

        let decorated_fields: Vec<_> = class
            .fields
            .iter()
            .filter(|field| {
                field
                    .decorators
                    .iter()
                    .any(|d| is_field_controller_decorator(&d.name))
            })
            .collect();

        if decorated_fields.is_empty() {
            return MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Warning,
                    message: "@Derive(FieldController) found no @Field decorators".to_string(),
                    span: Some(ctx.decorator_span),
                    notes: vec![],
                    help: Some(
                        "Add @Field decorators to fields you want to generate controllers for"
                            .into(),
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

        let runtime_code = generate_field_controller_runtime(class, &decorated_fields);
        let type_code = generate_field_controller_types(class, &decorated_fields);

        let mut type_patches = vec![Patch::Delete {
            span: ctx.decorator_span,
        }];

        for field in &decorated_fields {
            for decorator in &field.decorators {
                if is_field_controller_decorator(&decorator.name) {
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
                if is_field_controller_decorator(&decorator.name) {
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

fn generate_field_controller_runtime(
    class: &ClassIR,
    decorated_fields: &[&ts_macro_abi::FieldIR],
) -> String {
    let class_name = &class.name;

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

    let mut field_methods = Vec::new();

    for field in decorated_fields {
        let field_name = &field.name;
        let options = ControllerOptions::from_field(field);
        let controller_name = format!("{field_name}FieldController");
        let controller_interface = controller_interface_name(field_name, &options);
        let overrides = controller_overrides(&options);
        let overrides_arg = overrides
            .map(|body| format!("{{\n{body}\n        }}"))
            .unwrap_or_else(|| "undefined".to_string());

        field_methods.push(format!(
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
            {overrides_arg}
        )
    }};
}};
"#,
            field_type = &field.ts_type
        ));
    }

    format!("{}{}", make_base_props, field_methods.join("\n"))
}

fn generate_field_controller_types(
    class: &ClassIR,
    decorated_fields: &[&ts_macro_abi::FieldIR],
) -> String {
    let class_name = &class.name;
    let mut code = Vec::new();

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

    for field in decorated_fields {
        let field_name = &field.name;
        let options = ControllerOptions::from_field(field);
        let controller_name = format!("{field_name}FieldController");
        let controller_interface = controller_interface_name(field_name, &options);

        code.push(format!(
            r#"
    private readonly {field_name}FieldPath: ["{field_name}"];
    {controller_name}(
        superForm: SuperForm<{class_name}>
    ): {controller_interface}<{class_name}, {field_type}, 1>;
"#,
            field_type = &field.ts_type
        ));
    }

    code.join("\n")
}

fn is_field_controller_decorator(name: &str) -> bool {
    matches!(name, "Field" | "FieldController")
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn controller_interface_name(field_name: &str, options: &ControllerOptions) -> String {
    if let Some(expr) = &options.controller_ref {
        expr.clone()
    } else if let Some(component) = &options.component_hint {
        if component.ends_with("Controller") {
            component.clone()
        } else if component.ends_with("FieldController") {
            component.clone()
        } else {
            format!("{component}FieldController")
        }
    } else {
        format!("{}FieldController", capitalize(field_name))
    }
}

struct ControllerOptions {
    controller_ref: Option<String>,
    component_hint: Option<String>,
    label_text: String,
    placeholder: Option<String>,
}

impl ControllerOptions {
    fn from_field(field: &ts_macro_abi::FieldIR) -> Self {
        let mut options = ControllerOptions {
            controller_ref: None,
            component_hint: None,
            label_text: capitalize(&field.name),
            placeholder: None,
        };

        for decorator in &field.decorators {
            if !is_field_controller_decorator(&decorator.name) {
                continue;
            }

            let args = decorator.args_src.trim();
            if args.is_empty() {
                continue;
            }

            if args.starts_with('{') {
                if options.component_hint.is_none() {
                    options.component_hint = extract_named_string_option(args, "component");
                }
                if let Some(label) = extract_label_text(args) {
                    options.label_text = label;
                }
                if let Some(placeholder) = extract_named_string_option(args, "placeholder") {
                    options.placeholder = Some(placeholder);
                }
                continue;
            }

            let (component_expr, overrides_expr) = split_component_and_overrides(args);

            if let Some(expr) = component_expr {
                let trimmed = expr.trim();
                if trimmed.starts_with('{') {
                    if options.component_hint.is_none() {
                        options.component_hint = extract_named_string_option(trimmed, "component");
                    }
                    if let Some(label) = extract_label_text(trimmed) {
                        options.label_text = label;
                    }
                    if let Some(placeholder) = extract_named_string_option(trimmed, "placeholder") {
                        options.placeholder = Some(placeholder);
                    }
                } else if let Some(literal) = parse_string_literal(trimmed) {
                    if options.component_hint.is_none() {
                        options.component_hint = Some(literal);
                    }
                } else {
                    options.controller_ref = Some(trimmed.to_string());
                }
            }

            if let Some(overrides) = overrides_expr {
                if options.component_hint.is_none() {
                    options.component_hint = extract_named_string_option(&overrides, "component");
                }
                if let Some(label) = extract_label_text(&overrides) {
                    options.label_text = label;
                }
                if let Some(placeholder) = extract_named_string_option(&overrides, "placeholder") {
                    options.placeholder = Some(placeholder);
                }
            }
        }

        options
    }
}

fn controller_overrides(options: &ControllerOptions) -> Option<String> {
    let mut entries = Vec::new();
    entries.push(format!(
        r#"            labelText: "{}""#,
        escape_ts_string(&options.label_text)
    ));

    if let Some(placeholder) = &options.placeholder {
        entries.push(format!(
            r#"            placeholder: "{}""#,
            escape_ts_string(placeholder)
        ));
    }

    if entries.is_empty() {
        None
    } else {
        Some(entries.join(",\n"))
    }
}

fn extract_named_string_option(args: &str, name: &str) -> Option<String> {
    let lower = args.to_ascii_lowercase();
    let name_lower = name.to_ascii_lowercase();
    let mut offset = 0usize;

    while let Some(pos) = lower[offset..].find(&name_lower) {
        let absolute = offset + pos;
        let before = lower[..absolute].chars().rev().find(|c| !c.is_whitespace());
        if before.is_some() && (before.unwrap().is_alphanumeric() || before.unwrap() == '_') {
            offset = absolute + name.len();
            continue;
        }

        let remainder = &args[absolute + name.len()..];
        let remainder = remainder.trim_start();

        if remainder.starts_with(':') || remainder.starts_with('=') {
            let value = remainder[1..].trim_start();
            return parse_string_literal(value);
        }

        offset = absolute + name.len();
    }

    None
}

fn extract_label_text(args: &str) -> Option<String> {
    extract_named_string_option(args, "labelText")
        .or_else(|| extract_named_string_option(args, "label"))
}

fn split_component_and_overrides(args: &str) -> (Option<String>, Option<String>) {
    let trimmed = args.trim();
    if trimmed.is_empty() {
        return (None, None);
    }

    if trimmed.starts_with('{') {
        return (None, Some(trimmed.to_string()));
    }

    let mut depth = 0usize;
    let mut in_string: Option<char> = None;
    let mut escape = false;

    for (idx, ch) in trimmed.char_indices() {
        if let Some(delim) = in_string {
            if escape {
                escape = false;
                continue;
            }
            if ch == '\\' {
                escape = true;
                continue;
            }
            if ch == delim {
                in_string = None;
            }
            continue;
        }

        match ch {
            '"' | '\'' | '`' => {
                in_string = Some(ch);
            }
            '(' | '[' | '{' => {
                depth += 1;
            }
            ')' | ']' | '}' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            ',' if depth == 0 => {
                let left = trimmed[..idx].trim().to_string();
                let right = trimmed[idx + 1..].trim().to_string();
                let component = if left.is_empty() { None } else { Some(left) };
                let overrides = if right.is_empty() { None } else { Some(right) };
                return (component, overrides);
            }
            _ => {}
        }
    }

    (Some(trimmed.to_string()), None)
}

fn parse_string_literal(input: &str) -> Option<String> {
    let trimmed = input.trim_start();
    let mut chars = trimmed.chars();
    let quote = chars.next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let mut escaped = false;
    let mut buf = String::new();
    for c in chars {
        if escaped {
            buf.push(c);
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == quote {
            return Some(buf);
        }
        buf.push(c);
    }
    None
}

fn escape_ts_string(value: &str) -> String {
    value
        .chars()
        .flat_map(|c| match c {
            '\\' => vec!['\\', '\\'],
            '"' => vec!['\\', '"'],
            '\n' => vec!['\\', 'n'],
            '\r' => vec!['\\', 'r'],
            '\t' => vec!['\\', 't'],
            _ => vec![c],
        })
        .collect()
}
