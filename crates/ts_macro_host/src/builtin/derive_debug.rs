//! @Derive(Debug) macro implementation

use crate::traits::TsMacro;
use ts_macro_abi::{MacroContextIR, MacroResult, Patch, SpanIR, MacroKind};

pub struct DeriveDebugMacro;

impl TsMacro for DeriveDebugMacro {
    fn name(&self) -> &str {
        "Debug"
    }

    fn kind(&self) -> MacroKind {
        MacroKind::Derive
    }

    fn run(&self, ctx: MacroContextIR) -> MacroResult {
        let class = match ctx.as_class() {
            Some(class) => class,
            None => {
                return MacroResult {
                    runtime_patches: vec![],
                    type_patches: vec![],
                    diagnostics: vec![],
                    debug: Some("@Derive(Debug) can only be applied to classes".into()),
                };
            }
        };

        // Generate the debug implementation
        let debug_impl = generate_debug_implementation(class);

        // Calculate where to insert the debug implementation
        // For now, we'll insert at the end of the class body
        let insert_point = SpanIR {
            start: ctx.target_span.end - 1,  // Just before the closing brace
            end: ctx.target_span.end - 1,
        };

        MacroResult {
            runtime_patches: vec![Patch::Insert {
                at: insert_point,
                code: debug_impl,
            }],
            type_patches: vec![],
            diagnostics: vec![],
            debug: None,
        }
    }

    fn description(&self) -> &str {
        "Generates a toString() method for debugging"
    }
}

fn generate_debug_implementation(class: &ts_macro_abi::ClassIR) -> String {
    let fields_debug: Vec<String> = class
        .fields
        .iter()
        .map(|f| format!("{}: ${{this.{}}}", f.name, f.name))
        .collect();

    let fields_str = if fields_debug.is_empty() {
        String::new()
    } else {
        format!(" {{ {} }}", fields_debug.join(", "))
    };

    format!(
        r#"
    static DEBUG_SYM = Symbol.for("ts-macro.debug");

    [{}["DEBUG_SYM"]](): string {{
        return `{}{fields_str}`;
    }}

    toString(): string {{
        return this[{}["DEBUG_SYM"]]();
    }}"#,
        class.name, class.name, class.name
    )
}