//! @Derive(Eq) macro implementation

use crate::traits::TsMacro;
use ts_macro_abi::{MacroKind, MacroResult, Patch, SpanIR};
use ts_syn::TsStream;

pub struct DeriveEqMacro;

impl TsMacro for DeriveEqMacro {
    fn name(&self) -> &str {
        "Eq"
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
                    debug: Some("@Derive(Eq) can only be applied to classes".into()),
                };
            }
        };

        // Generate the equality implementation
        let eq_impl = generate_eq_implementation(class);

        // The body_span includes the enclosing braces. We want to insert inside, before the '}'.
        let insert_point = SpanIR {
            start: class.body_span.end.saturating_sub(1),
            end: class.body_span.end.saturating_sub(1),
        };

        MacroResult {
            runtime_patches: vec![Patch::Insert {
                at: insert_point,
                code: eq_impl,
            }],
            type_patches: vec![Patch::Insert {
                at: insert_point,
                code: generate_eq_signature(),
            }],
            diagnostics: vec![],
            debug: None,
        }
    }

    fn description(&self) -> &str {
        "Generates equals() and hashCode() methods"
    }
}

fn generate_eq_implementation(class: &ts_macro_abi::ClassIR) -> String {
    let field_comparisons: Vec<String> = class
        .fields
        .iter()
        .map(|f| format!("this.{} === other.{}", f.name, f.name))
        .collect();

    let comparison = if field_comparisons.is_empty() {
        "true".to_string()
    } else {
        field_comparisons.join(" && ")
    };

    let hash_components: Vec<String> = class
        .fields
        .iter()
        .map(|f| {
            format!(
                "hash = (hash * 31 + (this.{} ? this.{}.toString().charCodeAt(0) : 0)) | 0;",
                f.name, f.name
            )
        })
        .collect();

    let hash_calc = if hash_components.is_empty() {
        String::new()
    } else {
        format!("\n        {}", hash_components.join("\n        "))
    };

    format!(
        r#"
    equals(other: unknown): boolean {{
        if (this === other) return true;
        if (!(other instanceof {})) return false;
        const typedOther = other as {};
        return {};
    }}

    hashCode(): number {{
        let hash = 0;{}
        return hash;
    }}"#,
        class.name, class.name, comparison, hash_calc
    )
}

fn generate_eq_signature() -> String {
    "    equals(other: unknown): boolean;\n    hashCode(): number;\n".to_string()
}
