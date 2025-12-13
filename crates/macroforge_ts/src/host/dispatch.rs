//! Macro dispatch and execution

use crate::host::MacroRegistry;
use crate::ts_syn::TsStream;
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, MacroContextIR, MacroResult};

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
        // Look up the macro in the registry, with fallback to name-only lookup
        // This supports both exact module paths and dynamic module resolution
        match self
            .registry
            .lookup_with_fallback(&ctx.module_path, &ctx.macro_name)
        {
            Ok(macro_impl) => {
                // Check ABI version compatibility
                let impl_abi = macro_impl.abi_version();
                if impl_abi != ctx.abi_version {
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
                        tokens: None,
                        debug: None,
                    };
                }

                // Create TsStream from context
                let input =
                    match TsStream::with_context(&ctx.target_source, &ctx.file_name, ctx.clone()) {
                        Ok(stream) => stream,
                        Err(err) => {
                            return MacroResult {
                                runtime_patches: vec![],
                                type_patches: vec![],
                                diagnostics: vec![Diagnostic {
                                    level: DiagnosticLevel::Error,
                                    message: format!("Failed to create TsStream: {:?}", err),
                                    span: Some(ctx.decorator_span),
                                    notes: vec![],
                                    help: None,
                                }],
                                tokens: None,
                                debug: None,
                            };
                        }
                    };

                // Execute the macro
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    macro_impl.run(input)
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
                            tokens: None,
                            debug: None,
                        }
                    }
                }
            }
            Err(_err) => MacroResult {
                runtime_patches: vec![],
                type_patches: vec![],
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!(
                        "Macro '{}' is not a Macroforge built-in macro. Ensure you are using the 'import macro' syntax import statement.",
                        ctx.macro_name
                    ),
                    span: Some(ctx.error_span()),
                    notes: vec![],
                    help: None,
                }],
                tokens: None,
                debug: None,
            },
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
    use crate::host::traits::Macroforge;
    use std::sync::Arc;
    use crate::ts_syn::abi::{ClassIR, MacroKind, SpanIR, TargetIR};

    struct TestMacro {
        name: String,
    }

    impl Macroforge for TestMacro {
        fn name(&self) -> &str {
            &self.name
        }

        fn kind(&self) -> MacroKind {
            MacroKind::Derive
        }

        fn run(&self, _input: TsStream) -> MacroResult {
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
            macro_name_span: None,
            target_span: SpanIR {
                start: 10,
                end: 100,
            },
            file_name: "test.ts".to_string(),
            target: TargetIR::Class(ClassIR {
                name: "Test".to_string(),
                span: SpanIR {
                    start: 10,
                    end: 100,
                },
                body_span: SpanIR {
                    start: 20,
                    end: 100,
                },
                is_abstract: false,
                type_params: vec![],
                heritage: vec![],
                decorators: vec![],
                decorators_ast: vec![],
                fields: vec![],
                methods: vec![],
                members: vec![],
            }),
            target_source: "class Test {}".to_string(),
        };

        let result = dispatcher.dispatch(ctx);
        assert!(result.diagnostics.is_empty());
    }
}
