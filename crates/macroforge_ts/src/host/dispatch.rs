//! # Macro Dispatch and Execution
//!
//! The dispatcher is responsible for routing macro calls to their implementations
//! and handling the execution lifecycle, including:
//!
//! - Looking up macros in the registry (with fallback resolution)
//! - ABI version compatibility checking
//! - Creating the `TsStream` input for macros
//! - Catching panics and converting them to diagnostics
//!
//! ## Dispatch Flow
//!
//! ```text
//! MacroContextIR
//!       │
//!       ▼
//! ┌─────────────────┐
//! │  Registry Lookup │  (with module fallback)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ ABI Version Check│  (reject if mismatch)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ Create TsStream  │  (parse context)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ Execute Macro    │  (with panic catching)
//! └────────┬────────┘
//!          │
//!          ▼
//!    MacroResult
//! ```
//!
//! ## Error Handling
//!
//! The dispatcher never panics. All errors are converted to diagnostics:
//! - Macro not found → Error diagnostic with helpful message
//! - ABI mismatch → Error diagnostic with version info
//! - TsStream creation failure → Error diagnostic with details
//! - Macro panic → Error diagnostic with panic message

use crate::host::MacroRegistry;
use crate::ts_syn::TsStream;
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, MacroContextIR, MacroResult};

/// Routes macro calls to registered implementations.
///
/// The dispatcher is the bridge between the expansion engine and individual
/// macro implementations. It handles lookup, validation, and execution.
///
/// # Safety
///
/// The dispatcher catches all panics from macro execution, converting them
/// to error diagnostics. This ensures a single misbehaving macro doesn't
/// crash the entire expansion process.
///
/// # Example
///
/// ```ignore
/// let registry = MacroRegistry::new();
/// // ... register macros ...
///
/// let dispatcher = MacroDispatcher::new(registry);
///
/// // Dispatch a macro call from expansion context
/// let result = dispatcher.dispatch(macro_context);
///
/// // Handle diagnostics
/// for diag in &result.diagnostics {
///     if diag.level == DiagnosticLevel::Error {
///         eprintln!("Error: {}", diag.message);
///     }
/// }
/// ```
pub struct MacroDispatcher {
    /// The registry to look up macros in.
    registry: MacroRegistry,
}

impl MacroDispatcher {
    /// Creates a new dispatcher with the given registry.
    ///
    /// # Arguments
    ///
    /// * `registry` - The macro registry to use for lookups
    pub fn new(registry: MacroRegistry) -> Self {
        Self { registry }
    }

    /// Dispatches a macro call to its registered implementation.
    ///
    /// This is the main entry point for macro execution. It performs:
    /// 1. Registry lookup with fallback resolution
    /// 2. ABI version compatibility checking
    /// 3. TsStream creation from context
    /// 4. Macro execution with panic catching
    ///
    /// # Arguments
    ///
    /// * `ctx` - The macro invocation context containing all information
    ///   needed for execution (macro name, target code, file info, etc.)
    ///
    /// # Returns
    ///
    /// A [`MacroResult`] containing:
    /// - `runtime_patches` - Code patches to apply to the source
    /// - `type_patches` - Patches for type declarations
    /// - `diagnostics` - Errors, warnings, and info messages
    ///
    /// # Error Handling
    ///
    /// All errors are returned as diagnostics, never as panics or Results:
    /// - Unknown macro → Error diagnostic
    /// - ABI mismatch → Error diagnostic with versions
    /// - Execution panic → Error diagnostic with panic message
    pub fn dispatch(&self, ctx: MacroContextIR) -> MacroResult {
        // Look up the macro in the registry, with fallback to name-only lookup.
        // This supports both exact module paths and dynamic module resolution
        // where the import path might not exactly match the registration path.
        match self
            .registry
            .lookup_with_fallback(&ctx.module_path, &ctx.macro_name)
        {
            Ok(macro_impl) => {
                // Safety check: Verify ABI version compatibility.
                // Mismatched versions can cause memory corruption or crashes
                // due to incompatible data structure layouts.
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

                // Create the TsStream input from the macro context.
                // This parses the target code and provides the macro with
                // structured access to the decorated item.
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

                // Execute the macro with panic catching.
                // This ensures a buggy macro doesn't crash the entire process.
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    macro_impl.run(input)
                })) {
                    Ok(result) => result,
                    Err(panic_err) => {
                        // Extract the panic message from the boxed error
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
            Err(_err) => {
                // Macro not found - provide a helpful error message
                MacroResult {
                    runtime_patches: vec![],
                    type_patches: vec![],
                    diagnostics: vec![Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!(
                            "Macro '{}' is not a Macroforge built-in macro. Ensure you are using the 'import macro' syntax import statement.",
                            ctx.macro_name
                        ),
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

    /// Returns a reference to the underlying registry.
    ///
    /// Useful for debugging and introspection.
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
