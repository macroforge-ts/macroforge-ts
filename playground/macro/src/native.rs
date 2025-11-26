use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Expr, Ident, Lit, ModuleItem, Stmt, Str};
use ts_macro_abi::{Diagnostic, DiagnosticLevel, MacroResult, Patch, SpanIR, insert_into_class};
use ts_macro_derive::ts_macro_derive;
use ts_quote::ts_quote;
use ts_syn::{Data, DeriveInput, TsStream, parse_ts_macro_input};

// Helper to create SWC String Literal Expressions
fn str_lit(s: &str) -> Expr {
    Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: s.into(),
        raw: None,
    }))
}

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

            // 1. Initialize result object
            // This is safer than constructing a massive object literal string manually.
            let mut body_stmts = vec![ts_quote!( const result = {}; as Stmt )];

            // 2. Loop through fields to generate assignments
            // Generates: result.fieldName = this.fieldName;
            for field_name in class.field_names() {
                body_stmts.push(ts_quote!(
                    result.$(ident!(field_name)) = this.$(ident!(field_name)); as Stmt
                ));
            }

            // 3. Return result
            body_stmts.push(ts_quote!( return result; as Stmt ));

            // 4. Wrap it all in the prototype function
            // SWC's quote! automatically expands the Vec<Stmt> into the function body
            let runtime_code = ts_quote!(
                $(ident!(class_name)).prototype.toJSON = function () {
                    $(body_stmts)
                }; as Stmt
            );

            // Generate type signature
            let type_signature = ts_quote!(
                toJSON(): Record<string, unknown>; as ModuleItem
            );

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
                    insert_into_class(class.body_span(), type_signature.into()),
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

            // Generate Runtime ASTs
            let runtime_nodes = generate_field_controller_runtime(class_name, &decorated_fields);

            // Generate Type ASTs
            let type_nodes = generate_field_controller_types(class_name, &decorated_fields);

            // --- Patch Logic ---
            let mut type_patches = vec![Patch::Delete {
                span: input.decorator_span(),
            }];

            for field in class.fields() {
                for decorator in &field.decorators {
                    if decorator.name == "FieldController" {
                        type_patches.push(Patch::Delete {
                            span: decorator.span,
                        });
                    }
                }
            }

            type_patches.push(insert_into_class(class.body_span(), type_nodes.into()));

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
                code: runtime_nodes.into(),
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

fn generate_field_controller_runtime(
    class_name: &str,
    decorated_fields: &[FieldInfo],
) -> Vec<Stmt> {
    // 1. Generate the base props maker function
    let make_base_props_stmt = ts_quote!(
        $(ident!(class_name)).prototype.$(ident!("make{}BaseProps", class_name)) = function (superForm, path, overrides) {
            const proxy = formFieldProxy(superForm, path);
            const baseProps = {
                fieldPath: path,
                ...(overrides ?? {}),
                value: proxy.value,
                errors: proxy.errors,
                superForm
            };

            return baseProps;
        }; as Stmt
    );

    let mut stmts = vec![make_base_props_stmt];

    // 2. Generate field controller methods
    for field in decorated_fields {
        stmts.extend(generate_field_controller_method(class_name, field));
    }

    stmts
}

fn generate_field_controller_method(class_name: &str, field: &FieldInfo) -> Vec<Stmt> {
    let field_name_str = &field.name;

    // Create string literals as Expr nodes
    let label_expr = str_lit(&capitalize(field_name_str));
    let path_literal = str_lit(field_name_str);

    let path_stmt = ts_quote!(
        $(ident!(class_name)).prototype.$(ident!("{}FieldPath", field_name_str)) = [$(expr!(path_literal))]; as Stmt
    );

    let controller_stmt = ts_quote!(
        $(ident!(class_name)).prototype.$(ident!("{}FieldController", field_name_str)) = function (superForm) {
            const fieldPath = this.$(ident!("{}FieldPath", field_name_str));

            return {
                fieldPath,
                baseProps: this.$(ident!("make{}BaseProps", class_name))(
                    superForm,
                    fieldPath,
                    {
                        labelText: $(expr!(label_expr))
                    }
                )
            };
        }; as Stmt
    );

    vec![path_stmt, controller_stmt]
}

fn generate_field_controller_types(
    class_name: &str,
    decorated_fields: &[FieldInfo],
) -> Vec<ModuleItem> {
    let mut items = vec![];

    // 1. Add base props maker signature
    items.push(ts_quote!(
        $(ident!("make{}BaseProps", class_name))<
            D extends number,
            const P extends DeepPath<$(ident!(class_name)), D>,
            V = DeepValue<$(ident!(class_name)), P, never, D>
        >(
            superForm: SuperForm<$(ident!(class_name))>,
            path: P,
            overrides?: BasePropsOverrides<$(ident!(class_name)), V, D>
        ): BaseFieldProps<$(ident!(class_name)), V, D>; as ModuleItem
    ));

    // 2. Add field controller signatures
    for field in decorated_fields {
        let field_name_str = &field.name;

        // Handling the type annotation via ts_quote interpolation.
        let type_ident = ident!(field.ts_type); // Using coercing ident! macro logic from ts_quote

        // Property: private readonly fieldNameFieldPath: ["fieldName"];
        let field_name_literal = str_lit(field_name_str);

        items.push(ts_quote!(
            private readonly $(ident!("{}FieldPath", field_name_str)): [$(expr!(field_name_literal))]; as ModuleItem
        ));

        // Method: fieldNameFieldController(superForm: SuperForm<Class>): Interface<Class, Type, 1>;
        items.push(ts_quote!(
            $(ident!("{}FieldController", field_name_str))(
                superForm: SuperForm<$(ident!(class_name))>
            ): $(ident!("{}FieldController", capitalize(field_name_str)))<$(ident!(class_name)), $(ident!(field.ts_type)), 1>; as ModuleItem
        ));
    }

    items
}
