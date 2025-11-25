use ts_macro_abi::{insert_into_class, Diagnostic, DiagnosticLevel, MacroResult, Patch, SpanIR};
use ts_macro_derive::ts_macro_derive;
use ts_syn::TsStream;
use swc_ecma_ast::{ClassDecl, ClassMember, Decl, ModuleItem, PropName, Stmt, TsType};

#[ts_macro_derive(JSON, description = "Generates a toJSON() implementation that returns a plain object with all fields")]
pub fn derive_json_macro(mut input: TsStream) -> MacroResult {
    // Parse the class using TsStream
    let parsed_class = match parse_class_from_stream(&mut input) {
        Ok(c) => c,
        Err(e) => {
            return MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("Failed to parse class: {:?}", e),
                    span: None,
                    notes: vec![],
                    help: None,
                }],
                ..Default::default()
            };
        }
    };

    // Get the context (after we're done parsing)
    let ctx = match input.context() {
        Some(c) => c,
        None => {
            return MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: "No macro context available".to_string(),
                    span: None,
                    notes: vec![],
                    help: None,
                }],
                ..Default::default()
            };
        }
    };

    let class_name = &parsed_class.name;
    let fields = &parsed_class.fields;

    // Generate toJSON implementation using format!()
    let mut field_entries = Vec::new();
    for field in fields {
        field_entries.push(format!("            {}: this.{},", field, field));
    }

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

    // Get class body span from context (for inserting into class body)
    let class_ir = match ctx.as_class() {
        Some(c) => c,
        None => {
            return MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: "@Derive(JSON) can only target classes".to_string(),
                    span: Some(ctx.decorator_span),
                    notes: vec![],
                    help: None,
                }],
                ..Default::default()
            };
        }
    };

    MacroResult {
        runtime_patches: vec![Patch::Insert {
            at: SpanIR {
                start: ctx.target_span.end,
                end: ctx.target_span.end,
            },
            code: runtime_code,
        }],
        type_patches: vec![
            Patch::Delete {
                span: ctx.decorator_span,
            },
            insert_into_class(class_ir.body_span, type_signature),
        ],
        ..Default::default()
    }
}

#[ts_macro_derive(
    FieldController,
    description = "Generates depth-aware field controller helpers for reactive forms",
    attributes(FieldController)
)]
pub fn field_controller_macro(mut input: TsStream) -> MacroResult {
    // Parse the class using TsStream
    let parsed_class = match parse_class_from_stream(&mut input) {
        Ok(c) => c,
        Err(e) => {
            return MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("Failed to parse class: {:?}", e),
                    span: None,
                    notes: vec![],
                    help: None,
                }],
                ..Default::default()
            };
        }
    };

    // Get the context (after we're done parsing)
    let ctx = match input.context() {
        Some(c) => c,
        None => {
            return MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: "No macro context available".to_string(),
                    span: None,
                    notes: vec![],
                    help: None,
                }],
                ..Default::default()
            };
        }
    };

    // Find fields with @FieldController decorator
    let decorated_fields: Vec<_> = parsed_class
        .members
        .iter()
        .filter_map(|member| {
            if let ClassMember::ClassProp(prop) = member {
                // Check if this property has @FieldController decorator
                if prop.decorators.iter().any(|d| {
                    if let swc_ecma_ast::Expr::Ident(ident) = d.expr.as_ref() {
                        return ident.sym.as_ref() == "FieldController";
                    }
                    false
                }) {
                    if let PropName::Ident(ident) = &prop.key {
                        return Some(FieldInfo {
                            name: ident.sym.to_string(),
                            ts_type: extract_type_annotation(&prop.type_ann),
                        });
                    }
                }
            }
            None
        })
        .collect();

    if decorated_fields.is_empty() {
        return MacroResult {
            diagnostics: vec![Diagnostic {
                level: DiagnosticLevel::Warning,
                message: "@Derive(FieldController) found no @FieldController decorators"
                    .to_string(),
                span: Some(ctx.decorator_span),
                notes: vec![],
                help: Some(
                    "Add @FieldController decorators to fields you want to generate controllers for"
                        .into(),
                ),
            }],
            ..Default::default()
        };
    }

    let class_name = &parsed_class.name;
    let runtime_code = generate_field_controller_runtime(class_name, &decorated_fields);
    let type_code = generate_field_controller_types(class_name, &decorated_fields);

    // Get class body span from context for patch locations
    let class_ir = match ctx.as_class() {
        Some(c) => c,
        None => {
            return MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: "@Derive(FieldController) can only target classes".to_string(),
                    span: Some(ctx.decorator_span),
                    notes: vec![],
                    help: None,
                }],
                ..Default::default()
            };
        }
    };

    // Delete the class decorator and all @FieldController field decorators
    let mut type_patches = vec![Patch::Delete {
        span: ctx.decorator_span,
    }];

    // Delete @FieldController decorators from fields
    for field in &class_ir.fields {
        for decorator in &field.decorators {
            if decorator.name == "FieldController" {
                type_patches.push(Patch::Delete {
                    span: decorator.span,
                });
            }
        }
    }

    type_patches.push(insert_into_class(class_ir.body_span, type_code));

    let mut runtime_patches = vec![];
    for field in &class_ir.fields {
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
            start: ctx.target_span.end,
            end: ctx.target_span.end,
        },
        code: runtime_code,
    });

    MacroResult {
        runtime_patches,
        type_patches,
        ..Default::default()
    }
}

// ============================================================================
// TsStream Parsing
// ============================================================================

struct ParsedClass {
    name: String,
    fields: Vec<String>,
    members: Vec<ClassMember>,
}

struct FieldInfo {
    name: String,
    ts_type: String,
}

/// Parse a class from TsStream using the syn-like API
fn parse_class_from_stream(stream: &mut TsStream) -> Result<ParsedClass, ts_syn::TsSynError> {
    // Parse the entire module
    let module = stream.parse_module()?;

    // Find the class declaration
    for item in &module.body {
        if let ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) = item {
            return Ok(parse_class_decl(class_decl));
        }
        if let ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::ExportDecl(
            swc_ecma_ast::ExportDecl {
                decl: Decl::Class(class_decl),
                ..
            },
        )) = item
        {
            return Ok(parse_class_decl(class_decl));
        }
    }

    Err(ts_syn::TsSynError::Parse("No class found in stream".into()))
}

fn parse_class_decl(class_decl: &ClassDecl) -> ParsedClass {
    let name = class_decl.ident.sym.to_string();
    let mut fields = Vec::new();
    let members = class_decl.class.body.clone();

    // Extract field names
    for member in &class_decl.class.body {
        if let ClassMember::ClassProp(prop) = member {
            if let PropName::Ident(ident) = &prop.key {
                fields.push(ident.sym.to_string());
            }
        }
    }

    ParsedClass {
        name,
        fields,
        members,
    }
}

fn extract_type_annotation(type_ann: &Option<Box<swc_ecma_ast::TsTypeAnn>>) -> String {
    if let Some(ann) = type_ann {
        return type_to_string(&ann.type_ann);
    }
    "any".to_string()
}

fn type_to_string(ts_type: &TsType) -> String {
    match ts_type {
        TsType::TsKeywordType(keyword) => match keyword.kind {
            swc_ecma_ast::TsKeywordTypeKind::TsStringKeyword => "string".to_string(),
            swc_ecma_ast::TsKeywordTypeKind::TsNumberKeyword => "number".to_string(),
            swc_ecma_ast::TsKeywordTypeKind::TsBooleanKeyword => "boolean".to_string(),
            swc_ecma_ast::TsKeywordTypeKind::TsAnyKeyword => "any".to_string(),
            _ => "unknown".to_string(),
        },
        TsType::TsTypeRef(type_ref) => {
            if let swc_ecma_ast::TsEntityName::Ident(ident) = &type_ref.type_name {
                ident.sym.to_string()
            } else {
                "unknown".to_string()
            }
        }
        TsType::TsArrayType(arr) => {
            format!("{}[]", type_to_string(&arr.elem_type))
        }
        _ => "unknown".to_string(),
    }
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
