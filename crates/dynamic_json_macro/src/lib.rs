use std::sync::Arc;

use ts_macro_abi::{ClassIR, MacroContextIR, MacroKind, MacroResult, Patch, SpanIR};
use ts_macro_host::{MacroRegistry, Result, TsMacro};

const DERIVE_MODULE: &str = "@macro/derive";
const MACRO_NAME: &str = "JsonNative";

struct JsonNativeMacro;

impl TsMacro for JsonNativeMacro {
    fn name(&self) -> &str {
        MACRO_NAME
    }

    fn kind(&self) -> MacroKind {
        MacroKind::Derive
    }

    fn run(&self, ctx: MacroContextIR) -> MacroResult {
        let class = match ctx.as_class() {
            Some(class) => class,
            None => {
                return MacroResult {
                    debug: Some("JsonNative can only be applied to classes".into()),
                    ..Default::default()
                };
            }
        };

        // Insert type signature inside the class body (before closing brace)
        let class_insert = SpanIR {
            start: class.body_span.end.saturating_sub(1),
            end: class.body_span.end.saturating_sub(1),
        };
        // Insert runtime implementation after the class
        let post_class_insert = SpanIR {
            start: ctx.target_span.end,
            end: ctx.target_span.end,
        };

        MacroResult {
            runtime_patches: vec![Patch::Insert {
                at: post_class_insert,
                code: generate_to_json(class),
            }],
            type_patches: vec![Patch::Insert {
                at: class_insert,
                code: "    toJSON(): Record<string, unknown>;\n".into(),
            }],
            ..Default::default()
        }
    }

    fn description(&self) -> &str {
        "Generates a toJSON() implementation from a dynamic native macro crate"
    }
}

/// # Safety
/// Caller must ensure `registry` points to a valid `MacroRegistry` allocated by the host.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_macro_register(registry: *const MacroRegistry) -> bool {
    if registry.is_null() {
        return false;
    }

    let registry = unsafe { &*registry };
    register_impl(registry).is_ok()
}

fn register_impl(registry: &MacroRegistry) -> Result<()> {
    registry.register(DERIVE_MODULE, MACRO_NAME, Arc::new(JsonNativeMacro))?;
    Ok(())
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
