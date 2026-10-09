use std::sync::Arc;

use crate::host::{DiagnosticLevel, MacroExpander, Macroforge};
use crate::ts_syn::TsStream;
use crate::ts_syn::abi::{Diagnostic, MacroKind, MacroResult};

/// A derive that refuses every target.
struct Refuses;

impl Macroforge for Refuses {
    fn name(&self) -> &str {
        "Refuses"
    }

    fn kind(&self) -> MacroKind {
        MacroKind::Derive
    }

    fn run(&self, _input: TsStream) -> MacroResult {
        MacroResult {
            diagnostics: vec![Diagnostic {
                level: DiagnosticLevel::Error,
                message: "Refuses refused the target".to_string(),
                span: None,
                notes: vec![],
                help: None,
            }],
            ..MacroResult::default()
        }
    }
}

#[test]
fn a_derive_that_reports_an_error_keeps_it() -> anyhow::Result<()> {
    let host = MacroExpander::new()?;
    host.dispatcher
        .registry()
        .register("builtin", "Refuses", Arc::new(Refuses))?;
    let source = r#"/** import macro {Refuses} from "@app/macros"; */

/** @derive(Refuses) */
export interface Form {
    name: string;
}
"#;

    let expansion = host.expand_source(source, "form.ts")?;

    assert!(
        expansion.diagnostics.iter().any(|diagnostic| {
            diagnostic.level == DiagnosticLevel::Error
                && diagnostic.message == "Refuses refused the target"
        }),
        "{:?}",
        expansion.diagnostics
    );
    Ok(())
}
