//! @Derive(Clone) macro implementation

use crate::traits::TsMacro;
use ts_macro_abi::{MacroKind, MacroResult, Patch, SpanIR};
use ts_syn::TsStream;

pub struct DeriveCloneMacro;

impl TsMacro for DeriveCloneMacro {
    fn name(&self) -> &str {
        "Clone"
    }

    fn kind(&self) -> MacroKind {
        MacroKind::Derive
    }

    fn run(&self, input: TsStream) -> MacroResult {
        let ctx = match input.context() {
            Some(c) => c,
            None => {
                return MacroResult {
                    runtime_patches: vec![],
                    type_patches: vec![],
                    diagnostics: vec![],
                    debug: Some("No macro context available".into()),
                };
            }
        };

        let class = match ctx.as_class() {
            Some(class) => class,
            None => {
                return MacroResult {
                    runtime_patches: vec![],
                    type_patches: vec![],
                    diagnostics: vec![],
                    debug: Some("@Derive(Clone) can only be applied to classes".into()),
                };
            }
        };

        // Generate the clone implementation
        let clone_impl = generate_clone_implementation(class);

        // The body_span includes the enclosing braces. We want to insert inside, before the '}'.
        let insert_point = SpanIR {
            start: class.body_span.end.saturating_sub(1),
            end: class.body_span.end.saturating_sub(1),
        };

        MacroResult {
            runtime_patches: vec![Patch::Insert {
                at: insert_point,
                code: clone_impl,
            }],
            type_patches: vec![Patch::Insert {
                at: insert_point,
                code: generate_clone_signature(class),
            }],
            diagnostics: vec![],
            debug: None,
        }
    }

    fn description(&self) -> &str {
        "Generates a clone() method for deep cloning"
    }
}

fn generate_clone_implementation(class: &ts_macro_abi::ClassIR) -> String {
    let field_assignments: Vec<String> = class
        .fields
        .iter()
        .map(|f| {
            // TODO: Handle arrays and objects properly for deep clone
            format!("        cloned.{} = this.{};", f.name, f.name)
        })
        .collect();

    let assignments = if field_assignments.is_empty() {
        String::new()
    } else {
        format!("\n{}", field_assignments.join("\n"))
    };

    format!(
        r#"
    clone(): {} {{
        const cloned = Object.create(Object.getPrototypeOf(this));{}
        return cloned;
    }}"#,
        class.name, assignments
    )
}

fn generate_clone_signature(class: &ts_macro_abi::ClassIR) -> String {
    format!("    clone(): {};\n", class.name)
}
