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

        let insertion = SpanIR {
            start: ctx.target_span.end.saturating_sub(1),
            end: ctx.target_span.end.saturating_sub(1),
        };

        MacroResult {
            runtime_patches: vec![Patch::Insert {
                at: insertion,
                code: generate_to_json(class),
            }],
            type_patches: vec![Patch::Insert {
                at: insertion,
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
    toJSON(): Record<string, unknown> {{
{body}
    }}
"#
    )
}
