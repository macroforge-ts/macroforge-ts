//! The Rust examples from the custom-macro documentation on the website,
//! compiled and run, so the documentation cannot drift from the API.

use std::sync::Arc;

use macroforge_ts::builtin::derive_common::{extract_named_string, has_flag};
use macroforge_ts::host::{MacroRegistry, Macroforge};
use macroforge_ts::macros::{ts_macro_derive, ts_quote, ts_template};
use macroforge_ts::ts_syn::abi::{MacroContextIR, SpanIR};
use macroforge_ts::ts_syn::oxc::allocator::Allocator;
use macroforge_ts::ts_syn::oxc::parser::Parser;
use macroforge_ts::ts_syn::oxc::span::SourceType;
use macroforge_ts::ts_syn::{
    Data, DeriveInput, Diagnostic, DiagnosticCollector, DiagnosticLevel, FieldIR, ImportConfig,
    InsertPos, MacroKind, MacroResult, MacroforgeError, MacroforgeErrors, Patch, TsStream,
    TypeDefinitionIR, expr_to_string, insert_into_class, lower_classes, parse_expr,
    parse_ts_macro_input, resolve_generic_aliases, ts_ident,
};

// ts-macro-derive: Complete Example
fn has_decorator(field: &FieldIR, name: &str) -> bool {
    field
        .decorators
        .iter()
        .any(|d| d.name.eq_ignore_ascii_case(name))
}

#[ts_macro_derive(
    DocValidate,
    description = "Generates a validate() method",
    attributes(validate, (redact, "Hides the field's value"))
)]
pub fn derive_validate(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let validations: Vec<_> = class
                .fields()
                .iter()
                .filter(|f| has_decorator(f, "validate"))
                .collect();

            Ok(ts_template!(Within {
                validate(): string[] {
                    const errors: string[] = [];
                    {#for field in validations}
                        if (!this.@{field.name}) {
                            errors.push("@{field.name} is required");
                        }
                    {/for}
                    return errors;
                }
            }))
        }
        _ => Err(MacroforgeError::new(
            input.error_span(),
            "@derive(Validate) only works on classes",
        )),
    }
}

// Errors and Diagnostics: reporting every problem at once
#[ts_macro_derive(DocTyped)]
pub fn derive_typed(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let Some(class) = input.as_class() else {
        return Err(MacroforgeError::new(
            input.error_span(),
            "@derive(DocTyped) needs a class",
        ));
    };

    let mut diagnostics = DiagnosticCollector::new();
    for field in class.fields() {
        if field.ts_type.is_empty() {
            diagnostics.error_with_help(
                field.span,
                format!("field `{}` has no type", field.name),
                "add a type annotation",
            );
        }
        if field.ts_type == "any" {
            diagnostics.warning(field.span, format!("field `{}` is `any`", field.name));
        }
    }

    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }

    let mut output = ts_template!(Within {
        typed(): boolean { return true; }
    });
    output.add_diagnostics(diagnostics.into_vec());
    Ok(output)
}

// Testing and Debugging: building a derive's input
fn class_input(source: &str) -> TsStream {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    let class = lower_classes(&parsed.program, source, None)
        .expect("the class lowers")
        .into_iter()
        .next()
        .expect("the source has a class");
    let context = MacroContextIR::new_derive_class(
        "DocValidate".to_string(),
        "my-macros".to_string(),
        SpanIR::new(0, 0),
        class.span,
        "test.ts".to_string(),
        class,
        source.to_string(),
    );
    TsStream::with_context(source, "test.ts", context).expect("the input stream builds")
}

#[test]
fn validate_checks_every_field() {
    let output = derive_validate(class_input(
        "class User { /** @validate */ name: string; /** @validate */ email: string; }",
    ))
    .expect("the macro succeeds");
    assert!(output.source().contains("this.name"), "{}", output.source());
    assert!(
        output.source().contains("this.email"),
        "{}",
        output.source()
    );
}

#[test]
fn an_any_field_is_a_warning_and_the_macro_succeeds() {
    let output = derive_typed(class_input("class User { name: any; }")).expect("only a warning");
    let result = output.into_result();
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(result.diagnostics[0].level, DiagnosticLevel::Warning);
}

// Output and Imports
#[test]
fn output_examples() {
    let members = ts_template!(Within {
        toString(): string { return "User"; }
    });
    let standalone = ts_template! {
        export function describeUser(): string { return "a user"; }
    };
    let by_hand = TsStream::with_insert_pos(
        "export const userVersion = 1;".to_string(),
        InsertPos::Below,
    );
    let mut output = ts_template! {
        {$typescript standalone}
        {$typescript members}
    };
    output = output.merge(by_hand);

    output.add_import("validate", "my-validation-lib");
    output.add_type_import("Result", "my-validation-lib");
    output.add_import("validate as runValidate", "my-validation-lib");
    output.add_aliased_import("resultOk", "@my/runtime");
    output.add_aliased_type_import("Options", "@my/runtime");
    output.add_import_as("resultOk", "myResultOk", "@my/runtime");
    output.add_type_import_as("Options", "MyOptions", "@my/runtime");

    const RUNTIME_IMPORTS: &[ImportConfig] = &[
        ImportConfig::value("resultOk", "__mf_resultOk", "@my/runtime"),
        ImportConfig::type_only("Options", "__mf_Options", "@my/runtime"),
    ];
    output.add_imports(RUNTIME_IMPORTS);

    let module: Option<String> = output.module_specifier_for("User");
    assert_eq!(module, None, "no registry, so no module");
    let added: bool = output.add_import_for("userValidate", "User");
    assert!(!added);
    output.add_type_import_for("UserErrors", "User");
    output.add_helpers_for("User", &[("userValidate", false), ("UserErrors", true)]);
    output.add_cross_module_suffix("GetFields");
    output.add_cross_module_type_suffix("Errors");

    let class_span = SpanIR::new(10, 30);
    output.runtime_patches.push(Patch::Replace {
        span: class_span,
        code: "rewritten".to_string(),
        source_macro: Some("traced".to_string()),
    });
    output
        .runtime_patches
        .push(insert_into_class(class_span, "static version = 1;").with_source_macro("Versioned"));
    output
        .runtime_patches
        .push(Patch::Delete { span: class_span });

    output.add_diagnostic(Diagnostic {
        level: DiagnosticLevel::Warning,
        message: "field `id` has no type; it is serialized as unknown".to_string(),
        span: Some(class_span),
        notes: vec![],
        help: Some("give `id` a type annotation".to_string()),
    });

    let result = output.into_result();
    let imported: Vec<&str> = result
        .imports
        .iter()
        .map(|import| import.local_name.as_str())
        .collect();
    assert!(imported.contains(&"validate"));
    assert!(imported.contains(&"__mf_resultOk"));
    assert!(imported.contains(&"myResultOk"));
    assert_eq!(result.diagnostics.len(), 1);
}

#[test]
fn reading_a_span_inside_the_target() {
    let source = "class User { name: string; }";
    let input = class_input(source);
    let ctx = input.context().expect("the input has a context");
    let class = ctx.as_class().expect("a class");
    let field = &class.fields[0];
    let offset = ctx.target_span.start;
    let field_text = &ctx.target_source
        [(field.span.start - offset) as usize..(field.span.end - offset) as usize];
    assert_eq!(field_text, "name: string;");
    assert_eq!(&source[field.span.source_range()], "name: string;");
}

// Context and IR: decorator options
#[test]
fn decorator_options() {
    let input =
        class_input(r#"class User { /** @validate(required, rename: "id") */ name: string; }"#);
    let ctx = input.context().expect("the input has a context");
    let field = &ctx.as_class().expect("a class").fields[0];
    for decorator in &field.decorators {
        if decorator.name.eq_ignore_ascii_case("validate") {
            let required = has_flag(&decorator.args_src, "required");
            let rename = extract_named_string(&decorator.args_src, "rename");
            assert!(required);
            assert_eq!(rename.as_deref(), Some("id"));
        }
    }
}

// Type-Aware Macros
/// The enums the class's fields hold, and each field's type with its generic
/// aliases expanded.
fn type_aware(input: &DeriveInput) -> (Vec<String>, Vec<String>) {
    let ctx = &input.context;
    let imports = ctx.import_registry.file_import_entries();
    let resolved_fields = ctx.resolved_fields.as_ref();
    let mut enums = Vec::new();
    let mut shapes = Vec::new();

    let Some(class) = input.as_class() else {
        return (enums, shapes);
    };
    for field in class.fields() {
        shapes.push(resolve_generic_aliases(
            &field.ts_type,
            &ctx.type_registry,
            &ctx.file_name,
            &imports,
        ));
        let Some(resolved) = resolved_fields.and_then(|fields| fields.get(&field.name)) else {
            continue;
        };
        let Some(entry) =
            ctx.type_registry
                .resolve_in_file(&resolved.base_type_name, &ctx.file_name, &imports)
        else {
            continue;
        };
        if let TypeDefinitionIR::Enum(enum_ir) = &entry.definition {
            enums.push(enum_ir.name.clone());
        }
    }
    (enums, shapes)
}

#[test]
fn type_aware_example_runs() {
    let mut stream = class_input("class User { status: Status; }");
    let input = parse_input(&mut stream);
    let (enums, shapes) = type_aware(&input);
    assert!(enums.is_empty(), "an empty registry finds nothing");
    assert_eq!(shapes, ["Status"]);
}

fn parse_input(stream: &mut TsStream) -> DeriveInput {
    use macroforge_ts::ts_syn::ParseTs;
    DeriveInput::parse(stream).expect("the input parses")
}

// Template Syntax: ts_ident! and ts_quote!
#[test]
fn template_identifiers_and_quotes() {
    let type_name = "User";
    let serialize_fn = ts_ident!("{}Serialize", type_name.to_lowercase());
    let code = ts_template! {
        export function @{serialize_fn}(value: @{type_name}): string { return ""; }
    };
    assert!(
        code.source()
            .contains("export function userSerialize(value: User): string")
    );

    let arena = Allocator::default();
    let rhs = parse_expr(&arena, "1 + 2").expect("a valid expression");
    let assignment = ts_quote!("$name = $rhs" as Expr, name = "count", rhs: Expr = rhs);
    assert_eq!(expr_to_string(&assignment), "count = 1 + 2");
}

// Testing and Debugging: a macro implemented by hand
struct Stamp;

impl Macroforge for Stamp {
    fn name(&self) -> &str {
        "Stamp"
    }
    fn kind(&self) -> MacroKind {
        MacroKind::Derive
    }
    fn run(&self, input: TsStream) -> MacroResult {
        let name = input
            .context()
            .map_or("Unknown", |ctx| ctx.macro_name.as_str());
        TsStream::from_string(format!("export const stampedBy = \"{name}\";")).into_result()
    }
}

fn register(registry: &MacroRegistry) -> macroforge_ts::host::Result<()> {
    registry.register("my-macros", "Stamp", Arc::new(Stamp))
}

macroforge_ts::register_macro_package!("my-macros", register);

#[test]
fn a_hand_written_macro_registers_and_runs() {
    let registry = MacroRegistry::new();
    register(&registry).expect("registers");
    let stamp = registry.lookup("my-macros", "Stamp").expect("found");
    let result = stamp.run(class_input("class User {}"));
    assert_eq!(
        result.tokens.as_deref(),
        Some("export const stampedBy = \"DocValidate\";")
    );
}
