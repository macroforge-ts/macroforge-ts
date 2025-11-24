//! Macro dispatch and execution

use crate::MacroRegistry;
use ts_macro_abi::{MacroContextIR, MacroResult, Diagnostic, DiagnosticLevel};
use tracing::{debug, error, warn};

/// Dispatches macro calls to registered macro implementations
pub struct MacroDispatcher {
    registry: MacroRegistry,
}

impl MacroDispatcher {
    /// Create a new dispatcher with the given registry
    pub fn new(registry: MacroRegistry) -> Self {
        Self { registry }
    }

    /// Dispatch a macro call
    pub fn dispatch(&self, ctx: MacroContextIR) -> MacroResult {
        debug!(
            "Dispatching macro: {}::{} (kind: {:?})",
            ctx.module_path, ctx.macro_name, ctx.macro_kind
        );

        // Look up the macro in the registry
        match self.registry.lookup(&ctx.module_path, &ctx.macro_name) {
            Ok(macro_impl) => {
                // Check ABI version compatibility
                let impl_abi = macro_impl.abi_version();
                if impl_abi != ctx.abi_version {
                    warn!(
                        "ABI version mismatch for macro {}::{}: expected {}, got {}",
                        ctx.module_path, ctx.macro_name, ctx.abi_version, impl_abi
                    );

                    return MacroResult {
                        runtime_patches: vec![],
                        type_patches: vec![],
                        diagnostics: vec![Diagnostic {
                            level: DiagnosticLevel::Error,
                            message: format!(
                                "ABI version mismatch: expected {}, got {}",
                                ctx.abi_version, impl_abi
                            ),
                            span: Some(ctx.decorator_span),
                            notes: vec![],
                            help: Some(
                                "The macro may need to be rebuilt with the current ABI version"
                                    .to_string(),
                            ),
                        }],
                        debug: None,
                    };
                }

                // Execute the macro
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    macro_impl.run(ctx.clone())
                })) {
                    Ok(result) => result,
                    Err(panic_err) => {
                        let panic_msg = if let Some(s) = panic_err.downcast_ref::<String>() {
                            s.clone()
                        } else if let Some(s) = panic_err.downcast_ref::<&str>() {
                            s.to_string()
                        } else {
                            "Unknown panic in macro execution".to_string()
                        };

                        error!(
                            "Macro {}::{} panicked: {}",
                            ctx.module_path, ctx.macro_name, panic_msg
                        );

                        MacroResult {
                            runtime_patches: vec![],
                            type_patches: vec![],
                            diagnostics: vec![Diagnostic {
                                level: DiagnosticLevel::Error,
                                message: format!("Macro execution panicked: {}", panic_msg),
                                span: Some(ctx.decorator_span),
                                notes: vec![],
                                help: None,
                            }],
                            debug: None,
                        }
                    }
                }
            }
            Err(err) => {
                error!("Macro not found: {}", err);

                MacroResult {
                    runtime_patches: vec![],
                    type_patches: vec![],
                    diagnostics: vec![Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!(
                            "Macro '{}' not found in module '{}'",
                            ctx.macro_name, ctx.module_path
                        ),
                        span: Some(ctx.decorator_span),
                        notes: vec![],
                        help: Some(
                            "Make sure the macro package is installed and configured".to_string(),
                        ),
                    }],
                    debug: None,
                }
            }
        }
    }

    /// Get a reference to the registry
    pub fn registry(&self) -> &MacroRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::TsMacro;
    use std::sync::Arc;
    use ts_macro_abi::{SpanIR, TargetIR, ClassIR, MacroKind};

    struct TestMacro {
        name: String,
    }

    impl TsMacro for TestMacro {
        fn name(&self) -> &str {
            &self.name
        }

        fn kind(&self) -> MacroKind {
            MacroKind::Derive
        }

        fn run(&self, _ctx: MacroContextIR) -> MacroResult {
            MacroResult::default()
        }
    }

    #[test]
    fn test_dispatch() {
        let registry = MacroRegistry::new();
        let test_macro = Arc::new(TestMacro {
            name: "Debug".to_string(),
        });

        registry
            .register("@macro/derive", "Debug", test_macro)
            .unwrap();

        let dispatcher = MacroDispatcher::new(registry);

        let ctx = MacroContextIR {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name: "Debug".to_string(),
            module_path: "@macro/derive".to_string(),
            decorator_span: SpanIR { start: 0, end: 10 },
            target_span: SpanIR { start: 10, end: 100 },
            file_name: "test.ts".to_string(),
            target: TargetIR::Class(ClassIR {
                name: "Test".to_string(),
                span: SpanIR { start: 10, end: 100 },
                is_abstract: false,
                type_params: vec![],
                heritage: vec![],
                decorators: vec![],
                fields: vec![],
                methods: vec![],
            }),
        };

        let result = dispatcher.dispatch(ctx);
        assert!(result.diagnostics.is_empty());
    }
}