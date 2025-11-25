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
