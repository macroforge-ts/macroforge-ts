//! # Macro Expansion Engine
//!
//! This module provides the core expansion functionality for TypeScript macros.
//! It handles classes, interfaces, enums, and type aliases, supports external
//! macro loading via Node.js, and provides source mapping for IDE integration.
//!
//! ## Architecture Overview
//!
//! ```text
//! Source Code + AST
//!        │
//!        ▼
//! ┌──────────────────────────────────────────┐
//! │            MacroExpander                  │
//! │                                           │
//! │  1. prepare_expansion_context()           │
//! │     - Lower AST to IR (ClassIR, etc.)    │
//! │                                           │
//! │  2. collect_macro_patches()               │
//! │     - Find @derive decorators             │
//! │     - Dispatch to macro implementations   │
//! │     - Collect patches from each macro     │
//! │                                           │
//! │  3. apply_and_finalize_expansion()        │
//! │     - Apply patches to source code        │
//! │     - Generate source mapping             │
//! │     - Optionally strip decorators         │
//! └──────────────────────────────────────────┘
//!        │
//!        ▼
//!  MacroExpansion
//!  (expanded code + diagnostics + source mapping)
//! ```
//!
//! ## Key Types
//!
//! - [`MacroExpander`] - The main expansion engine
//! - [`MacroExpansion`] - Result of expansion with code, diagnostics, mapping
//! - `LoweredItems` - IR representations of TypeScript declarations
//!
//! ## External Macro Loading
//!
//! When a macro is not found in the built-in registry, the expander can
//! load external macros by spawning a Node.js process. This enables:
//!
//! - User-defined macros in JavaScript/TypeScript
//! - Workspace-local macros in monorepos
//! - npm-published macro packages
//!
//! The external loader searches for macros in:
//! 1. `node_modules` nearest to the file being processed
//! 2. The module path specified in the import
//! 3. Workspace packages defined in `package.json`
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use macroforge_ts::host::{MacroExpander, DiagnosticLevel, Result};
//!
//! fn example() -> Result<()> {
//!     let expander = MacroExpander::new()?;
//!
//!     let source = r#"
//!         /** @derive(Debug) */
//!         class User { name: string; }
//!     "#;
//!
//!     // Simple API - handles parsing internally
//!     let expansion = expander.expand_source(source, "file.ts")?;
//!
//!     // Check for errors
//!     for diag in &expansion.diagnostics {
//!         if diag.level == DiagnosticLevel::Error {
//!             eprintln!("Error: {}", diag.message);
//!         }
//!     }
//!
//!     // Use the expanded code
//!     println!("{}", expansion.code);
//!     Ok(())
//! }
//! ```

mod derive_targets;
mod external_loader;
mod helpers;
pub mod imports;
mod registration;
#[cfg(test)]
mod tests;

#[cfg(feature = "swc")]
pub use imports::{ImportCollectionResult, collect_import_sources};

use std::collections::HashMap;

use crate::ts_syn::abi::{
    ClassIR, Diagnostic, DiagnosticLevel, EnumIR, InterfaceIR, MacroContextIR, MacroResult, Patch,
    PatchCode, SourceMapping, SpanIR, TargetIR, TypeAliasIR,
};
#[cfg(feature = "swc")]
use crate::ts_syn::{lower_classes, lower_enums, lower_interfaces, lower_type_aliases};
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
use crate::ts_syn::{
    lower_classes_oxc, lower_enums_oxc, lower_interfaces_oxc, lower_type_aliases_oxc,
};
#[cfg(any(not(target_arch = "wasm32"), feature = "swc"))]
use anyhow::Context;
#[cfg(feature = "swc")]
use swc_core::ecma::ast::{ClassMember, Module, Program};

use super::{
    MacroConfig, MacroDispatcher, MacroError, MacroRegistry, PatchCollector, Result, derived,
};

pub(crate) use derive_targets::{
    DeriveTargetIR, SpanKey, collect_derive_targets, diagnostic_span_for_derive,
    find_macro_name_span, span_ir_with_at,
};
use external_loader::{ExternalMacroLoader, resolve_external_decorator_names};
#[cfg(feature = "swc")]
use helpers::{MemberWithComment, parse_members_from_tokens};
use helpers::{
    derive_insert_pos, extract_function_names_from_patches, find_macro_comment_span,
    generate_convenience_export, get_derive_target_end_span, get_derive_target_name,
    get_derive_target_start_span, has_existing_namespace_or_const, is_declaration_exported,
    split_by_markers,
};
#[cfg(feature = "swc")]
use imports::check_builtin_import_warnings;
use imports::external_type_function_import_patches;
use registration::register_packages;

/// Default module path for built-in derive macros
const DERIVE_MODULE_PATH: &str = "@macro/derive";

/// Special marker for dynamic module resolution
const DYNAMIC_MODULE_MARKER: &str = "__DYNAMIC_MODULE__";

/// Built-in macro names that don't need to be imported
#[cfg(feature = "swc")]
const BUILTIN_MACRO_NAMES: &[&str] = &[
    "Debug",
    "Clone",
    "Default",
    "Hash",
    "Ord",
    "PartialEq",
    "PartialOrd",
    "Serialize",
    "Deserialize",
];

/// Result of macro expansion.
///
/// Contains the expanded source code along with diagnostics, IR representations
/// of all discovered declarations, and an optional source mapping for IDE integration.
#[derive(Debug, Clone)]
pub struct MacroExpansion {
    /// The expanded source code after macro processing.
    pub code: String,
    /// Diagnostics (errors, warnings, info) generated during expansion.
    pub diagnostics: Vec<Diagnostic>,
    /// Whether any macros were expanded (i.e., source code was modified).
    pub changed: bool,
    /// Separate type-level output (`.d.ts` patches), if any macros generated type declarations.
    pub type_output: Option<String>,
    /// All classes found in the source, lowered to IR.
    pub classes: Vec<ClassIR>,
    /// All interfaces found in the source, lowered to IR.
    pub interfaces: Vec<InterfaceIR>,
    /// All enums found in the source, lowered to IR.
    pub enums: Vec<EnumIR>,
    /// All type aliases found in the source, lowered to IR.
    pub type_aliases: Vec<TypeAliasIR>,
    /// Bidirectional source mapping between original and expanded code positions.
    pub source_mapping: Option<SourceMapping>,
}

/// Core macro expansion engine
///
/// This struct provides the expansion logic that can be reused by any macro package.
/// Each macro package creates its own instance, which will use that package's
/// local inventory of macros.
pub struct MacroExpander {
    pub dispatcher: MacroDispatcher,
    config: MacroConfig,
    /// Whether to keep decorators in emitted output (used only by host integrations that need mapping)
    keep_decorators: bool,
    /// Additional decorator module names from external macros
    external_decorator_modules: Vec<String>,
    external_loader: Option<ExternalMacroLoader>,
    /// Project-wide type registry for compile-time type awareness.
    /// When set, macros receive type information about all types in the project.
    type_registry: Option<crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
}

type ContextFactory = Box<dyn Fn(String, String) -> MacroContextIR>;

/// Lowered IR representations of TypeScript declarations
#[derive(Clone)]
pub(crate) struct LoweredItems {
    pub classes: Vec<ClassIR>,
    pub interfaces: Vec<InterfaceIR>,
    pub enums: Vec<EnumIR>,
    pub type_aliases: Vec<TypeAliasIR>,
    pub imports: crate::host::import_registry::ImportRegistry,
}

impl LoweredItems {
    pub(crate) fn is_empty(&self) -> bool {
        self.classes.is_empty()
            && self.interfaces.is_empty()
            && self.enums.is_empty()
            && self.type_aliases.is_empty()
    }
}

impl MacroExpander {
    /// Create a new expander with the local registry populated from inventory.
    ///
    /// Discovers the project configuration by searching upward from the current
    /// directory for a `macroforge.config.*` file, then registers all built-in
    /// macros via the inventory.
    ///
    /// # Errors
    ///
    /// Returns an error if configuration discovery or macro registration fails.
    pub fn new() -> anyhow::Result<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        let (config, root_dir) = MacroConfig::find_with_root()
            .context("failed to discover macro configuration")?
            .unwrap_or_else(|| {
                (
                    MacroConfig::default(),
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                )
            });

        #[cfg(target_arch = "wasm32")]
        let (config, root_dir) = (MacroConfig::default(), std::path::PathBuf::from("."));

        Self::with_config(config, root_dir)
    }

    /// Create an expander with a specific configuration and project root.
    ///
    /// Use this when you already have a parsed [`MacroConfig`] (e.g., from tests
    /// or when the config was loaded externally).
    ///
    /// # Errors
    ///
    /// Returns an error if macro registration fails.
    pub fn with_config(config: MacroConfig, root_dir: std::path::PathBuf) -> anyhow::Result<Self> {
        let registry = MacroRegistry::new();
        register_packages(&registry, &config, &root_dir)?;

        debug_assert!(
            registry.contains("@macro/derive", "Debug"),
            "Built-in @macro/derive::Debug macro should be registered"
        );
        debug_assert!(
            registry.contains("@macro/derive", "Clone"),
            "Built-in @macro/derive::Clone macro should be registered"
        );
        debug_assert!(
            registry.contains("@macro/derive", "PartialEq"),
            "Built-in @macro/derive::PartialEq macro should be registered"
        );
        debug_assert!(
            registry.contains("@macro/derive", "Hash"),
            "Built-in @macro/derive::Hash macro should be registered"
        );
        debug_assert!(
            registry.contains("@macro/derive", "PartialOrd"),
            "Built-in @macro/derive::PartialOrd macro should be registered"
        );
        debug_assert!(
            registry.contains("@macro/derive", "Ord"),
            "Built-in @macro/derive::Ord macro should be registered"
        );
        debug_assert!(
            registry.contains("@macro/derive", "Default"),
            "Built-in @macro/derive::Default macro should be registered"
        );
        debug_assert!(
            registry.contains("@macro/derive", "Serialize"),
            "Built-in @macro/derive::Serialize macro should be registered"
        );
        debug_assert!(
            registry.contains("@macro/derive", "Deserialize"),
            "Built-in @macro/derive::Deserialize macro should be registered"
        );

        let keep_decorators = config.keep_decorators;

        Ok(Self {
            dispatcher: MacroDispatcher::new(registry),
            config,
            keep_decorators,
            external_decorator_modules: Vec::new(),
            external_loader: Some(ExternalMacroLoader::new(root_dir)),
            type_registry: None,
        })
    }

    /// Control whether decorators are preserved in the expanded output.
    pub fn set_keep_decorators(&mut self, keep: bool) {
        self.keep_decorators = keep;
    }

    /// Set additional decorator module names from external macros.
    ///
    /// These are used during decorator stripping to identify Macroforge-specific
    /// decorators that should be removed from the output.
    pub fn set_external_decorator_modules(&mut self, modules: Vec<String>) {
        self.external_decorator_modules = modules;
    }

    /// Set the project-wide type registry for compile-time type awareness.
    ///
    /// When set, each macro invocation receives the full registry and
    /// resolved field types in its [`MacroContextIR`].
    pub fn set_type_registry(
        &mut self,
        registry: Option<crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
    ) {
        self.type_registry = registry;
    }

    /// Build the complete set of valid annotation names for lowering.
    ///
    /// Includes `"derive"`, all builtin decorator annotation names, any explicitly
    /// configured external decorator modules, and — if the source imports external
    /// macros — decorator names resolved from those packages' manifests.
    fn valid_annotation_names(&self, source: &str) -> std::collections::HashSet<String> {
        let mut set = std::collections::HashSet::new();
        set.insert("derive".to_string());

        // Add all builtin decorator annotation names
        for name in derived::decorator_annotation_names() {
            set.insert(name.to_ascii_lowercase());
        }

        // Add explicitly configured external decorator modules
        for module in &self.external_decorator_modules {
            set.insert(module.to_ascii_lowercase());
        }

        // Resolve decorator names from external macro packages imported in the source
        if self.external_decorator_modules.is_empty() {
            for name in resolve_external_decorator_names(source, self.external_loader.as_ref()) {
                set.insert(name.to_ascii_lowercase());
            }
        }

        set
    }

    /// Expand all macros in the source code (simple API for CLI usage)
    #[cfg(all(not(feature = "swc"), feature = "oxc"))]
    pub fn expand_source(&self, source: &str, file_name: &str) -> Result<MacroExpansion> {
        use oxc_allocator::Allocator;
        use oxc_parser::Parser;
        use oxc_span::SourceType;

        let allocator = Allocator::default();
        let source_type = SourceType::ts().with_jsx(file_name.ends_with(".tsx"));
        let parsed = Parser::new(&allocator, source, source_type).parse();

        if !parsed.errors.is_empty() {
            return Err(MacroError::InvalidConfig(format!(
                "Parse error: {}",
                parsed
                    .errors
                    .into_iter()
                    .map(|diagnostic| diagnostic.to_string())
                    .collect::<Vec<_>>()
                    .join("; ")
            )));
        }

        let valid_annotations = self.valid_annotation_names(source);
        let filter = Some(&valid_annotations);

        let items = LoweredItems {
            classes: lower_classes_oxc(&parsed.program, source, filter)
                .map_err(|e| MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?,
            interfaces: lower_interfaces_oxc(&parsed.program, source, filter)
                .map_err(|e| MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?,
            enums: lower_enums_oxc(&parsed.program, source, filter)
                .map_err(|e| MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?,
            type_aliases: lower_type_aliases_oxc(&parsed.program, source, filter)
                .map_err(|e| MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?,
            imports: crate::host::import_registry::ImportRegistry::from_oxc_program(
                &parsed.program,
                source,
            ),
        };

        if items.is_empty() {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
                type_output: None,
                classes: Vec::new(),
                interfaces: Vec::new(),
                enums: Vec::new(),
                type_aliases: Vec::new(),
                source_mapping: None,
            });
        }

        let items_clone = LoweredItems {
            classes: items.classes.clone(),
            interfaces: items.interfaces.clone(),
            enums: items.enums.clone(),
            type_aliases: items.type_aliases.clone(),
            imports: crate::host::import_registry::ImportRegistry::new(),
        };

        let (mut collector, mut diagnostics) =
            self.collect_macro_patches_oxc(items, file_name, source);

        self.apply_and_finalize_expansion(source, &mut collector, &mut diagnostics, items_clone)
    }

    /// Expand all macros in the source code (simple API for CLI usage)
    #[cfg(feature = "swc")]
    pub fn expand_source(&self, source: &str, file_name: &str) -> Result<MacroExpansion> {
        use crate::ts_syn::parse_ts_module;

        let module = parse_ts_module(source)
            .map_err(|e| MacroError::InvalidConfig(format!("Parse error: {:?}", e)))?;

        let valid_annotations = self.valid_annotation_names(source);
        let filter = Some(&valid_annotations);

        let classes = lower_classes(&module, source, filter)
            .map_err(|e| MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?;

        let interfaces = lower_interfaces(&module, source, filter)
            .map_err(|e| MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?;

        let enums = lower_enums(&module, source, filter)
            .map_err(|e| MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?;

        let type_aliases = lower_type_aliases(&module, source, filter)
            .map_err(|e| MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?;

        let imports = crate::host::import_registry::ImportRegistry::from_module(&module, source);

        let items = LoweredItems {
            classes,
            interfaces,
            enums,
            type_aliases,
            imports,
        };
        if items.is_empty() {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
                type_output: None,
                classes: Vec::new(),
                interfaces: Vec::new(),
                enums: Vec::new(),
                type_aliases: Vec::new(),
                source_mapping: None,
            });
        }

        let items_clone = LoweredItems {
            classes: items.classes.clone(),
            interfaces: items.interfaces.clone(),
            enums: items.enums.clone(),
            type_aliases: items.type_aliases.clone(),
            imports: crate::host::import_registry::ImportRegistry::new(),
        };

        let (mut collector, mut diagnostics) =
            self.collect_macro_patches(&module, items, file_name, source);

        self.apply_and_finalize_expansion(source, &mut collector, &mut diagnostics, items_clone)
    }

    /// Expand all macros found in the parsed program and return the updated source code.
    #[cfg(feature = "swc")]
    pub fn expand(
        &self,
        source: &str,
        program: &Program,
        file_name: &str,
    ) -> anyhow::Result<MacroExpansion> {
        let (module, items) = match self.prepare_expansion_context(program, source)? {
            Some(context) => context,
            None => {
                return Ok(MacroExpansion {
                    code: source.to_string(),
                    diagnostics: Vec::new(),
                    changed: false,
                    type_output: None,
                    classes: Vec::new(),
                    interfaces: Vec::new(),
                    enums: Vec::new(),
                    type_aliases: Vec::new(),
                    source_mapping: None,
                });
            }
        };

        let items_clone = LoweredItems {
            classes: items.classes.clone(),
            interfaces: items.interfaces.clone(),
            enums: items.enums.clone(),
            type_aliases: items.type_aliases.clone(),
            imports: crate::host::import_registry::ImportRegistry::new(),
        };

        let (mut collector, mut diagnostics) =
            self.collect_macro_patches(&module, items, file_name, source);
        self.apply_and_finalize_expansion(source, &mut collector, &mut diagnostics, items_clone)
            .map_err(anyhow::Error::from)
    }

    #[cfg(feature = "swc")]
    pub(crate) fn prepare_expansion_context(
        &self,
        program: &Program,
        source: &str,
    ) -> anyhow::Result<Option<(Module, LoweredItems)>> {
        let module = match program {
            Program::Module(module) => module.clone(),
            Program::Script(script) => {
                use swc_core::ecma::ast::{Module as SwcModule, ModuleItem};
                SwcModule {
                    span: script.span,
                    body: script
                        .body
                        .iter()
                        .map(|stmt| ModuleItem::Stmt(stmt.clone()))
                        .collect(),
                    shebang: script.shebang.clone(),
                }
            }
        };

        let valid_annotations = self.valid_annotation_names(source);
        let filter = Some(&valid_annotations);

        let classes = lower_classes(&module, source, filter)
            .context("failed to lower classes for macro processing")?;

        let interfaces = lower_interfaces(&module, source, filter)
            .context("failed to lower interfaces for macro processing")?;

        let enums = lower_enums(&module, source, filter)
            .context("failed to lower enums for macro processing")?;

        let type_aliases = lower_type_aliases(&module, source, filter)
            .context("failed to lower type aliases for macro processing")?;

        let imports = crate::host::import_registry::ImportRegistry::from_module(&module, source);

        let items = LoweredItems {
            classes,
            interfaces,
            enums,
            type_aliases,
            imports,
        };
        if items.is_empty() {
            return Ok(None);
        }

        Ok(Some((module, items)))
    }

    #[cfg(feature = "swc")]
    pub(crate) fn collect_macro_patches(
        &self,
        module: &Module,
        items: LoweredItems,
        file_name: &str,
        source: &str,
    ) -> (PatchCollector, Vec<Diagnostic>) {
        let mut diagnostics = check_builtin_import_warnings(module, source);
        self.collect_macro_patches_inner(items, file_name, source, &mut diagnostics)
    }

    #[cfg(all(not(feature = "swc"), feature = "oxc"))]
    pub(crate) fn collect_macro_patches_oxc(
        &self,
        items: LoweredItems,
        file_name: &str,
        source: &str,
    ) -> (PatchCollector, Vec<Diagnostic>) {
        let mut diagnostics = Vec::new();
        self.collect_macro_patches_inner(items, file_name, source, &mut diagnostics)
    }

    fn collect_macro_patches_inner(
        &self,
        items: LoweredItems,
        file_name: &str,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> (PatchCollector, Vec<Diagnostic>) {
        let LoweredItems {
            classes,
            interfaces,
            enums,
            type_aliases,
            imports,
        } = items;

        // Install registry into thread-local (replaces 5 separate set_* calls).
        // Merge in any config_imports already set by the entry point.
        // (foreign_types are stored in a separate thread-local in host/import_registry.rs)
        let existing_config_imports =
            crate::host::import_registry::with_registry(|r| r.config_imports.clone());
        let mut registry = imports;
        if !existing_config_imports.is_empty() {
            registry.config_imports = existing_config_imports;
        }
        let entries = registry.source_import_entries();
        let mut trace_logs: Vec<String> = Vec::new();

        trace_logs.push(format!(
            "install_registry: {} source_imports for {}",
            entries.len(),
            file_name
        ));
        for e in &entries {
            trace_logs.push(format!(
                "  import '{}' from '{}'",
                e.local_name, e.source_module
            ));
        }
        crate::host::import_registry::install_registry(registry);

        // Get import sources from the registry (no redundant collect_import_sources call)
        let import_sources = crate::host::import_registry::with_registry(|r| r.source_modules());

        trace_logs.push(format!(
            "import_sources for {}: {:?}",
            file_name, import_sources
        ));

        let mut collector = PatchCollector::new();

        let class_map: HashMap<SpanKey, ClassIR> = classes
            .into_iter()
            .map(|class| (SpanKey::from(class.span), class))
            .collect();

        let interface_map: HashMap<SpanKey, InterfaceIR> = interfaces
            .into_iter()
            .map(|iface| (SpanKey::from(iface.span), iface))
            .collect();

        let enum_map: HashMap<SpanKey, EnumIR> = enums
            .into_iter()
            .map(|e| (SpanKey::from(e.span), e))
            .collect();

        let type_alias_map: HashMap<SpanKey, TypeAliasIR> = type_aliases
            .into_iter()
            .map(|ta| (SpanKey::from(ta.span), ta))
            .collect();

        trace_logs.push(format!(
            "lowered items: classes={}, interfaces={}, enums={}, type_aliases={}",
            class_map.len(),
            interface_map.len(),
            enum_map.len(),
            type_alias_map.len()
        ));

        let derive_targets = collect_derive_targets(
            &class_map,
            &interface_map,
            &enum_map,
            &type_alias_map,
            source,
        );

        trace_logs.push(format!("derive_targets: {}", derive_targets.len()));
        for t in &derive_targets {
            trace_logs.push(format!(
                "  target: macros={:?}, decorator_span={:?}",
                t.macro_names, t.decorator_span
            ));
        }

        if derive_targets.is_empty() {
            trace_logs.push("no derive targets found, returning early".to_string());
            flush_trace(&trace_logs, diagnostics);
            return (collector, std::mem::take(diagnostics));
        }

        for target in derive_targets {
            if !self.keep_decorators {
                let decorator_removal = Patch::Delete {
                    span: target.decorator_span,
                };
                collector.add_runtime_patches(vec![decorator_removal.clone()]);
                collector.add_type_patches(vec![decorator_removal]);
            }

            // Process class-specific patches (field decorators and method body stripping)
            if let DeriveTargetIR::Class(class_ir) = &target.target_ir {
                // Remove field decorators when not keeping decorators
                if !self.keep_decorators {
                    for field in &class_ir.fields {
                        for decorator in &field.decorators {
                            let field_dec_removal = Patch::Delete {
                                span: span_ir_with_at(decorator.span, source),
                            };
                            collector.add_runtime_patches(vec![field_dec_removal.clone()]);
                            collector.add_type_patches(vec![field_dec_removal]);
                        }

                        if let Some(span) = find_macro_comment_span(source, field.span.start) {
                            let removal = Patch::Delete { span };
                            collector.add_runtime_patches(vec![removal.clone()]);
                            collector.add_type_patches(vec![removal]);
                        }
                    }
                }

                // Add patches to strip method bodies for type output (only for classes with @derive)
                for method in &class_ir.methods {
                    let return_type = method
                        .return_type_src
                        .trim_start()
                        .trim_start_matches(':')
                        .trim_start();
                    let method_signature = if method.name == "constructor" {
                        let visibility = match method.visibility {
                            crate::ts_syn::abi::Visibility::Private => "private ",
                            crate::ts_syn::abi::Visibility::Protected => "protected ",
                            crate::ts_syn::abi::Visibility::Public => "",
                        };
                        format!(
                            "{visibility}constructor({params_src});",
                            visibility = visibility,
                            params_src = method.params_src
                        )
                    } else {
                        let visibility = match method.visibility {
                            crate::ts_syn::abi::Visibility::Private => "private ",
                            crate::ts_syn::abi::Visibility::Protected => "protected ",
                            crate::ts_syn::abi::Visibility::Public => "",
                        };
                        let static_kw = if method.is_static { "static " } else { "" };
                        let async_kw = if method.is_async { "async " } else { "" };

                        format!(
                            "{visibility}{static_kw}{async_kw}{method_name}{type_params}({params_src}): {return_type};",
                            visibility = visibility,
                            static_kw = static_kw,
                            async_kw = async_kw,
                            method_name = method.name,
                            type_params = method.type_params_src,
                            params_src = method.params_src,
                            return_type = return_type
                        )
                    };

                    collector.add_type_patches(vec![Patch::Replace {
                        span: method.span,
                        code: method_signature.into(),
                        source_macro: None,
                    }]);
                }
            }

            // Remove interface field decorators when not keeping decorators
            if !self.keep_decorators
                && let DeriveTargetIR::Interface(interface_ir) = &target.target_ir
            {
                for field in &interface_ir.fields {
                    for decorator in &field.decorators {
                        let field_dec_removal = Patch::Delete {
                            span: span_ir_with_at(decorator.span, source),
                        };
                        collector.add_runtime_patches(vec![field_dec_removal.clone()]);
                        collector.add_type_patches(vec![field_dec_removal]);
                    }

                    if let Some(span) = find_macro_comment_span(source, field.span.start) {
                        let removal = Patch::Delete { span };
                        collector.add_runtime_patches(vec![removal.clone()]);
                        collector.add_type_patches(vec![removal]);
                    }
                }
            }

            // Extract the source code for this target
            let (_target_span, _target_source, ctx_factory): (SpanIR, String, ContextFactory) =
                match &target.target_ir {
                    DeriveTargetIR::Class(class_ir) => {
                        let span = class_ir.span;
                        let src = source
                            .get(
                                span.start.saturating_sub(1) as usize
                                    ..span.end.saturating_sub(1) as usize,
                            )
                            .unwrap_or("")
                            .to_string();
                        let class_ir_clone = class_ir.clone();
                        let decorator_span = target.decorator_span;
                        let file = file_name.to_string();
                        let src_clone = src.clone();
                        (
                            span,
                            src,
                            Box::new(move |macro_name, module_path| {
                                MacroContextIR::new_derive_class(
                                    macro_name,
                                    module_path,
                                    decorator_span,
                                    span,
                                    file.clone(),
                                    class_ir_clone.clone(),
                                    src_clone.clone(),
                                )
                            }),
                        )
                    }
                    DeriveTargetIR::Interface(interface_ir) => {
                        let span = interface_ir.span;
                        let src = source
                            .get(
                                span.start.saturating_sub(1) as usize
                                    ..span.end.saturating_sub(1) as usize,
                            )
                            .unwrap_or("")
                            .to_string();
                        let interface_ir_clone = interface_ir.clone();
                        let decorator_span = target.decorator_span;
                        let file = file_name.to_string();
                        let src_clone = src.clone();
                        (
                            span,
                            src,
                            Box::new(move |macro_name, module_path| {
                                MacroContextIR::new_derive_interface(
                                    macro_name,
                                    module_path,
                                    decorator_span,
                                    span,
                                    file.clone(),
                                    interface_ir_clone.clone(),
                                    src_clone.clone(),
                                )
                            }),
                        )
                    }
                    DeriveTargetIR::Enum(enum_ir) => {
                        let span = enum_ir.span;
                        let src = source
                            .get(
                                span.start.saturating_sub(1) as usize
                                    ..span.end.saturating_sub(1) as usize,
                            )
                            .unwrap_or("")
                            .to_string();
                        let enum_ir_clone = enum_ir.clone();
                        let decorator_span = target.decorator_span;
                        let file = file_name.to_string();
                        let src_clone = src.clone();
                        (
                            span,
                            src,
                            Box::new(move |macro_name, module_path| {
                                MacroContextIR::new_derive_enum(
                                    macro_name,
                                    module_path,
                                    decorator_span,
                                    span,
                                    file.clone(),
                                    enum_ir_clone.clone(),
                                    src_clone.clone(),
                                )
                            }),
                        )
                    }
                    DeriveTargetIR::TypeAlias(type_alias_ir) => {
                        let span = type_alias_ir.span;
                        let src = source
                            .get(
                                span.start.saturating_sub(1) as usize
                                    ..span.end.saturating_sub(1) as usize,
                            )
                            .unwrap_or("")
                            .to_string();
                        let type_alias_ir_clone = type_alias_ir.clone();
                        let decorator_span = target.decorator_span;
                        let file = file_name.to_string();
                        let src_clone = src.clone();
                        (
                            span,
                            src,
                            Box::new(move |macro_name, module_path| {
                                MacroContextIR::new_derive_type_alias(
                                    macro_name,
                                    module_path,
                                    decorator_span,
                                    span,
                                    file.clone(),
                                    type_alias_ir_clone.clone(),
                                    src_clone.clone(),
                                )
                            }),
                        )
                    }
                };

            // Capture patch count before macro processing for convenience const generation
            let patches_start = collector.runtime_patches_count();

            for (macro_name, module_path) in target.macro_names {
                trace_logs.push(format!(
                    "dispatching macro '{}' from module '{}'",
                    macro_name, module_path
                ));
                let mut ctx = ctx_factory(macro_name.clone(), module_path.clone());

                // Calculate macro_name_span
                if let Some(macro_name_span) =
                    find_macro_name_span(source, target.decorator_span, &macro_name)
                {
                    ctx = ctx.with_macro_name_span(macro_name_span);
                }

                // Enrich context with project-wide type awareness
                if let Some(ref registry) = self.type_registry {
                    ctx.type_registry = Some(registry.clone());

                    let resolver = crate::host::type_resolver::TypeResolver::new(registry);
                    ctx.resolved_fields = Some(crate::host::type_resolver::resolve_target_fields(
                        &ctx.target,
                        &resolver,
                    ));
                }

                trace_logs.push(format!(
                    "registered macros: {:?}",
                    self.dispatcher
                        .registry()
                        .all_macros()
                        .iter()
                        .map(|(k, _)| format!("{}::{}", k.module, k.name))
                        .collect::<Vec<_>>()
                ));
                let mut result = self.dispatcher.dispatch(ctx.clone());
                trace_logs.push(format!(
                    "dispatch result: runtime={}, type={}, tokens={:?}, diags={}",
                    result.runtime_patches.len(),
                    result.type_patches.len(),
                    result.tokens.as_ref().map(|t| t.len()),
                    result.diagnostics.len()
                ));
                for d in &result.diagnostics {
                    trace_logs.push(format!("  diag: {:?} - {}", d.level, d.message));
                }

                if is_macro_not_found(&result)
                    && ctx.module_path != DERIVE_MODULE_PATH
                    && ctx.module_path.starts_with('.')
                {
                    let fallback_ctx =
                        ctx_factory(macro_name.clone(), DERIVE_MODULE_PATH.to_string());
                    result = self.dispatcher.dispatch(fallback_ctx);
                }

                if std::env::var("MF_DEBUG_EXPAND").is_ok() {
                    eprintln!("[DEBUG] Macro '{}' result:", ctx.macro_name);
                    eprintln!(
                        "[DEBUG]   runtime_patches: {}",
                        result.runtime_patches.len()
                    );
                    eprintln!("[DEBUG]   type_patches: {}", result.type_patches.len());
                    eprintln!(
                        "[DEBUG]   tokens: {:?}",
                        result.tokens.as_ref().map(|t: &String| t.len())
                    );
                    if let Some(tokens) = &result.tokens {
                        eprintln!(
                            "[DEBUG]   tokens content (first 500 chars): {:?}",
                            &tokens[..tokens.len().min(500)]
                        );
                        #[cfg(debug_assertions)]
                        if std::env::var("MF_DEBUG_TOKENS").is_ok() {
                            eprintln!(
                                "[MF_DEBUG_TOKENS] has_validation={}",
                                tokens.contains("valid email")
                            );
                        }
                    }
                }

                let no_output = result.runtime_patches.is_empty()
                    && result.type_patches.is_empty()
                    && result.tokens.is_none();

                if std::env::var("MF_DEBUG_EXPAND").is_ok() {
                    eprintln!(
                        "[DEBUG] External loader check for '{}': module_path='{}', no_output={}, is_not_found={}, has_loader={}",
                        ctx.macro_name,
                        ctx.module_path,
                        no_output,
                        is_macro_not_found(&result),
                        self.external_loader.is_some(),
                    );
                }

                trace_logs.push(format!("external loader check: module_path='{}', DERIVE_MODULE_PATH='{}', is_not_found={}, no_output={}, has_loader={}", ctx.module_path, DERIVE_MODULE_PATH, is_macro_not_found(&result), no_output, self.external_loader.is_some()));

                if ctx.module_path != DERIVE_MODULE_PATH
                    && (is_macro_not_found(&result) || no_output)
                    && let Some(loader) = &self.external_loader
                {
                    if std::env::var("MF_DEBUG_EXPAND").is_ok() {
                        eprintln!(
                            "[DEBUG] Invoking external loader for '{}' from '{}'",
                            ctx.macro_name, ctx.module_path
                        );
                    }

                    // Pass the full import registry so external macros have
                    // access to source imports, config imports, and generated imports.
                    ctx.import_registry =
                        crate::host::import_registry::with_registry(|r| r.clone());

                    // Pass the full macroforge config so external macros have
                    // access to foreign type configs, etc.
                    ctx.config = Some(crate::host::import_registry::with_foreign_types(|ft| {
                        crate::ts_syn::config::MacroforgeConfig {
                            foreign_types: ft.to_vec(),
                            ..Default::default()
                        }
                    }));

                    match loader.run_macro(&ctx) {
                        Ok(external_result) => {
                            if std::env::var("MF_DEBUG_EXPAND").is_ok() {
                                eprintln!(
                                    "[DEBUG] External loader success for '{}': runtime={}, type={}, tokens={:?}",
                                    ctx.macro_name,
                                    external_result.runtime_patches.len(),
                                    external_result.type_patches.len(),
                                    external_result.tokens.as_ref().map(|t: &String| t.len()),
                                );
                            }
                            result = external_result;
                        }
                        Err(err) => {
                            if std::env::var("MF_DEBUG_EXPAND").is_ok() {
                                eprintln!(
                                    "[DEBUG] External loader FAILED for '{}': {}",
                                    ctx.macro_name, err
                                );
                            }
                            result.diagnostics.push(Diagnostic {
                                level: DiagnosticLevel::Error,
                                message: format!(
                                    "Failed to load external macro '{}::{}': {}",
                                    ctx.macro_name, ctx.module_path, err
                                ),
                                span: Some(diagnostic_span_for_derive(ctx.decorator_span, source)),
                                notes: vec![],
                                help: None,
                            });
                        }
                    }
                }

                // Process potential token stream result
                if let Ok((runtime, type_def)) =
                    self.process_macro_output(&mut result, &ctx, source)
                {
                    let mut runtime = runtime;
                    let mut type_def = type_def;

                    if let Some(tokens) = &result.tokens {
                        let external_imports = external_type_function_import_patches(
                            tokens,
                            &import_sources,
                            &result.cross_module_suffixes,
                            &result.cross_module_type_suffixes,
                        );
                        runtime.extend(external_imports.clone());
                        type_def.extend(external_imports);
                    }

                    result.runtime_patches.extend(runtime);
                    result.type_patches.extend(type_def);
                }

                if !result.diagnostics.is_empty() {
                    for diag in &mut result.diagnostics {
                        if let Some(span) = diag.span {
                            diag.span = Some(diagnostic_span_for_derive(span, source));
                        }
                    }
                    diagnostics.extend(result.diagnostics.clone());
                }

                // Merge imports from the MacroResult back into the registry.
                // This is essential for external macros that run in a child process —
                // their TsStream::add_import() calls write to that process's registry,
                // and into_result() captures them into MacroResult.imports.
                if !result.imports.is_empty() {
                    crate::host::import_registry::with_registry_mut(|r| {
                        r.merge_imports(result.imports);
                    });
                }

                collector.add_runtime_patches(result.runtime_patches);
                collector.add_type_patches(result.type_patches);
            }

            // Generate convenience const for non-class types (Prefix style)
            // Skip if disabled in config or there's already a namespace or const with the same name
            if let Some(type_name) = get_derive_target_name(&target.target_ir)
                && self.config.generate_convenience_const
                && !has_existing_namespace_or_const(source, type_name)
            {
                let new_patches = collector.runtime_patches_slice(patches_start);
                let functions = extract_function_names_from_patches(new_patches, type_name);

                if !functions.is_empty() {
                    let start_pos = get_derive_target_start_span(&target.target_ir);
                    let is_exported = is_declaration_exported(source, start_pos);
                    let const_code = generate_convenience_export(
                        &target.target_ir,
                        type_name,
                        &functions,
                        is_exported,
                    );
                    let end_pos = get_derive_target_end_span(&target.target_ir);

                    let patch = Patch::Insert {
                        at: SpanIR {
                            start: end_pos,
                            end: end_pos,
                        },
                        code: PatchCode::Text(format!("\n\n{}", const_code)),
                        source_macro: Some("__convenience_const".to_string()),
                    };
                    collector.add_runtime_patches(vec![patch.clone()]);
                    collector.add_type_patches(vec![patch]);
                }
            }
        }
        flush_trace(&trace_logs, diagnostics);
        (collector, std::mem::take(diagnostics))
    }

    pub(crate) fn process_macro_output(
        &self,
        result: &mut MacroResult,
        ctx: &MacroContextIR,
        source: &str,
    ) -> anyhow::Result<(Vec<Patch>, Vec<Patch>)> {
        let mut runtime_patches = Vec::new();
        let mut type_patches = Vec::new();

        if let Some(tokens) = &result.tokens
            && ctx.macro_kind == crate::ts_syn::abi::MacroKind::Derive
        {
            let macro_name = Some(ctx.macro_name.clone());

            match &ctx.target {
                TargetIR::Class(class_ir) => {
                    let chunks = split_by_markers(tokens, result.insert_pos);

                    for (location, code) in chunks {
                        match location {
                            "above" => {
                                let patch = Patch::Insert {
                                    at: SpanIR {
                                        start: class_ir.span.start,
                                        end: class_ir.span.start,
                                    },
                                    code: PatchCode::Text(code.clone()),
                                    source_macro: macro_name.clone(),
                                };
                                runtime_patches.push(patch.clone());
                                type_patches.push(patch);
                            }
                            "below" => {
                                let patch = Patch::Insert {
                                    at: SpanIR {
                                        start: class_ir.span.end,
                                        end: class_ir.span.end,
                                    },
                                    code: PatchCode::Text(format!("\n\n{}", code.trim())),
                                    source_macro: macro_name.clone(),
                                };
                                runtime_patches.push(patch.clone());
                                type_patches.push(patch);
                            }
                            "signature" => {
                                let patch = Patch::Insert {
                                    at: SpanIR {
                                        start: class_ir.body_span.start,
                                        end: class_ir.body_span.start,
                                    },
                                    code: PatchCode::Text(code.clone()),
                                    source_macro: macro_name.clone(),
                                };
                                runtime_patches.push(patch.clone());
                                type_patches.push(patch);
                            }
                            "body" => {
                                let insert_pos = derive_insert_pos(class_ir, source);
                                #[cfg(feature = "swc")]
                                match parse_members_from_tokens(&code) {
                                    Ok(members_with_comments) => {
                                        for MemberWithComment {
                                            leading_comment,
                                            member,
                                        } in members_with_comments
                                        {
                                            // Insert leading JSDoc comment if present
                                            if let Some(comment_text) = &leading_comment {
                                                let jsdoc = format!("/**{} */\n", comment_text);
                                                runtime_patches.push(Patch::InsertRaw {
                                                    at: SpanIR {
                                                        start: insert_pos,
                                                        end: insert_pos,
                                                    },
                                                    code: jsdoc.clone(),
                                                    context: Some("JSDoc comment".into()),
                                                    source_macro: macro_name.clone(),
                                                });
                                                type_patches.push(Patch::InsertRaw {
                                                    at: SpanIR {
                                                        start: insert_pos,
                                                        end: insert_pos,
                                                    },
                                                    code: jsdoc,
                                                    context: Some("JSDoc comment".into()),
                                                    source_macro: macro_name.clone(),
                                                });
                                            }

                                            runtime_patches.push(Patch::Insert {
                                                at: SpanIR {
                                                    start: insert_pos,
                                                    end: insert_pos,
                                                },
                                                code: PatchCode::ClassMember(member.clone()),
                                                source_macro: macro_name.clone(),
                                            });

                                            let mut signature_member = member.clone();
                                            match &mut signature_member {
                                                ClassMember::Method(m) => m.function.body = None,
                                                ClassMember::Constructor(c) => c.body = None,
                                                ClassMember::PrivateMethod(m) => {
                                                    m.function.body = None
                                                }
                                                _ => {}
                                            }

                                            type_patches.push(Patch::Insert {
                                                at: SpanIR {
                                                    start: insert_pos,
                                                    end: insert_pos,
                                                },
                                                code: PatchCode::ClassMember(signature_member),
                                                source_macro: macro_name.clone(),
                                            });
                                        }
                                    }
                                    Err(err) => {
                                        let warning = format!(
                                            "/** macroforge warning: Failed to parse macro output for {}::{}: {:?} */\n",
                                            ctx.module_path, ctx.macro_name, err
                                        );
                                        let payload = format!("{warning}{code}");

                                        runtime_patches.push(Patch::InsertRaw {
                                            at: SpanIR {
                                                start: insert_pos,
                                                end: insert_pos,
                                            },
                                            code: payload.clone(),
                                            context: Some(format!(
                                                "Macro {}::{} output (unparsed)",
                                                ctx.module_path, ctx.macro_name
                                            )),
                                            source_macro: macro_name.clone(),
                                        });
                                        type_patches.push(Patch::ReplaceRaw {
                                            span: SpanIR {
                                                start: insert_pos,
                                                end: insert_pos,
                                            },
                                            code: payload,
                                            context: Some(format!(
                                                "Macro {}::{} output (unparsed)",
                                                ctx.module_path, ctx.macro_name
                                            )),
                                            source_macro: macro_name.clone(),
                                        });

                                        result.diagnostics.push(Diagnostic {
                                            level: DiagnosticLevel::Warning,
                                            message: format!(
                                                "Failed to parse macro output, inserted raw tokens: {err:?}"
                                            ),
                                            span: Some(diagnostic_span_for_derive(
                                                ctx.decorator_span,
                                                source,
                                            )),
                                            notes: vec![],
                                            help: None,
                                        });
                                    }
                                }
                                #[cfg(all(not(feature = "swc"), feature = "oxc"))]
                                {
                                    let payload = if code.starts_with('\n') {
                                        code.clone()
                                    } else {
                                        format!("\n{}", code)
                                    };

                                    runtime_patches.push(Patch::InsertRaw {
                                        at: SpanIR {
                                            start: insert_pos,
                                            end: insert_pos,
                                        },
                                        code: payload.clone(),
                                        context: Some("class body".to_string()),
                                        source_macro: macro_name.clone(),
                                    });
                                    type_patches.push(Patch::InsertRaw {
                                        at: SpanIR {
                                            start: insert_pos,
                                            end: insert_pos,
                                        },
                                        code: payload,
                                        context: Some("class body".to_string()),
                                        source_macro: macro_name.clone(),
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                }
                TargetIR::Interface(interface_ir) => {
                    let chunks = split_by_markers(tokens, result.insert_pos);

                    for (location, code) in chunks {
                        match location {
                            "above" => {
                                let patch = Patch::Insert {
                                    at: SpanIR {
                                        start: interface_ir.span.start,
                                        end: interface_ir.span.start,
                                    },
                                    code: PatchCode::Text(code.clone()),
                                    source_macro: macro_name.clone(),
                                };
                                runtime_patches.push(patch.clone());
                                type_patches.push(patch);
                            }
                            "below" | "body" | "signature" => {
                                let patch = Patch::Insert {
                                    at: SpanIR {
                                        start: interface_ir.span.end,
                                        end: interface_ir.span.end,
                                    },
                                    code: PatchCode::Text(format!("\n\n{}", code.trim())),
                                    source_macro: macro_name.clone(),
                                };
                                runtime_patches.push(patch.clone());
                                type_patches.push(patch);
                            }
                            _ => {}
                        }
                    }
                }
                TargetIR::Enum(enum_ir) => {
                    let chunks = split_by_markers(tokens, result.insert_pos);

                    for (location, code) in chunks {
                        match location {
                            "above" => {
                                let patch = Patch::Insert {
                                    at: SpanIR {
                                        start: enum_ir.span.start,
                                        end: enum_ir.span.start,
                                    },
                                    code: PatchCode::Text(code.clone()),
                                    source_macro: macro_name.clone(),
                                };
                                runtime_patches.push(patch.clone());
                                type_patches.push(patch);
                            }
                            _ => {
                                // Enums get namespace code inserted after the enum declaration
                                let patch = Patch::Insert {
                                    at: SpanIR {
                                        start: enum_ir.span.end,
                                        end: enum_ir.span.end,
                                    },
                                    code: PatchCode::Text(format!("\n\n{}", code.trim())),
                                    source_macro: macro_name.clone(),
                                };
                                runtime_patches.push(patch.clone());
                                type_patches.push(patch);
                            }
                        }
                    }
                }
                TargetIR::TypeAlias(type_alias_ir) => {
                    let chunks = split_by_markers(tokens, result.insert_pos);

                    for (location, code) in chunks {
                        match location {
                            "above" => {
                                let patch = Patch::Insert {
                                    at: SpanIR {
                                        start: type_alias_ir.span.start,
                                        end: type_alias_ir.span.start,
                                    },
                                    code: PatchCode::Text(code.clone()),
                                    source_macro: macro_name.clone(),
                                };
                                runtime_patches.push(patch.clone());
                                type_patches.push(patch);
                            }
                            _ => {
                                // Type aliases get namespace code inserted after the type declaration
                                let patch = Patch::Insert {
                                    at: SpanIR {
                                        start: type_alias_ir.span.end,
                                        end: type_alias_ir.span.end,
                                    },
                                    code: PatchCode::Text(format!("\n\n{}", code.trim())),
                                    source_macro: macro_name.clone(),
                                };
                                runtime_patches.push(patch.clone());
                                type_patches.push(patch);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok((runtime_patches, type_patches))
    }

    pub(crate) fn apply_and_finalize_expansion(
        &self,
        source: &str,
        collector: &mut PatchCollector,
        diagnostics: &mut Vec<Diagnostic>,
        items: LoweredItems,
    ) -> Result<MacroExpansion> {
        let LoweredItems {
            classes,
            interfaces,
            enums,
            type_aliases,
            ..
        } = items;
        let has_patches = collector.has_patches();
        let runtime_result = collector
            .apply_runtime_patches_with_mapping(source, None)
            .map_err(|e| MacroError::InvalidConfig(format!("Patch error: {:?}", e)))?;

        let type_output = if collector.has_type_patches() {
            Some(
                collector
                    .apply_type_patches(source)
                    .map_err(|e| MacroError::InvalidConfig(format!("Type patch error: {:?}", e)))?,
            )
        } else {
            None
        };

        let source_mapping = if runtime_result.mapping.is_empty() {
            None
        } else {
            Some(runtime_result.mapping)
        };

        let mut code = runtime_result.code;
        if !self.keep_decorators {
            // Convert Vec<String> to Vec<&str> for strip_decorators
            let external_modules: Vec<&str> = self
                .external_decorator_modules
                .iter()
                .map(|s| s.as_str())
                .collect();
            code = strip_decorators(&code, &external_modules);
        }

        // Emit all generated imports from the registry (namespace + type-only)
        let import_block =
            crate::host::import_registry::with_registry(|r| r.emit_generated_imports());
        if !import_block.is_empty() {
            code = format!("{}{}", import_block, code);
        }

        let mut expansion = MacroExpansion {
            code,
            diagnostics: std::mem::take(diagnostics),
            changed: has_patches,
            type_output,
            classes,
            interfaces,
            enums,
            type_aliases,
            source_mapping,
        };

        self.enforce_diagnostic_limit(&mut expansion.diagnostics);

        Ok(expansion)
    }

    fn enforce_diagnostic_limit(&self, diagnostics: &mut Vec<Diagnostic>) {
        let max = self.config.limits.max_diagnostics;
        if max == 0 {
            diagnostics.clear();
            return;
        }

        if diagnostics.len() > max {
            diagnostics.truncate(max.saturating_sub(1));
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                message: format!(
                    "Diagnostic output truncated to {} entries per macro host configuration",
                    max
                ),
                span: None,
                notes: vec![],
                help: Some(
                    "Adjust `limits.maxDiagnostics` in macroforge.json to see all diagnostics"
                        .to_string(),
                ),
            });
        }
    }
}

impl Default for MacroExpander {
    fn default() -> Self {
        Self::new().expect("Failed to create default MacroExpander")
    }
}

fn flush_trace(logs: &[String], diagnostics: &mut Vec<Diagnostic>) {
    for msg in logs {
        crate::debug::log("expand", msg);
        diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: format!("[trace] {}", msg),
            span: None,
            notes: vec![],
            help: None,
        });
    }
}

fn is_macro_not_found(result: &MacroResult) -> bool {
    result
        .diagnostics
        .iter()
        .any(|d| d.message.contains("Macro") && d.message.contains("not found"))
}

/// Strips Macroforge decorator lines from expanded code.
///
/// Only strips lines that contain Macroforge-specific decorators, preserving
/// standard JSDoc annotations like @returns, @param, @internal, etc.
///
/// # Arguments
///
/// * `code` - The expanded source code
/// * `external_decorator_modules` - Additional decorator module names from external macros
///
/// # Decorator patterns stripped
///
/// - `@derive(...)` - the main macro invocation keyword
/// - `@<decorator_module>({ ... })` - field-level decorators (e.g., @serde, @debug)
fn strip_decorators(code: &str, external_decorator_modules: &[&str]) -> String {
    // Get built-in decorator modules from the macro registry
    let builtin_modules = derived::decorator_modules();

    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());

    for line in &lines {
        let trimmed = line.trim_start();
        if let Some(after_at) = trimmed.strip_prefix('@') {
            // Extract the keyword after @
            let keyword_end = after_at
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(after_at.len());
            let keyword = &after_at[..keyword_end];

            // Check if this is:
            // 1. The main macro keyword "derive" (case-insensitive)
            // 2. A built-in decorator module name (case-insensitive)
            // 3. An external decorator module name (case-insensitive)
            let is_derive = keyword.eq_ignore_ascii_case("derive");
            let is_builtin_module = builtin_modules
                .iter()
                .any(|m| m.eq_ignore_ascii_case(keyword));
            let is_external_module = external_decorator_modules
                .iter()
                .any(|m| m.eq_ignore_ascii_case(keyword));

            if is_derive || is_builtin_module || is_external_module {
                // This is a Macroforge decorator, skip it
                continue;
            }
        }
        result.push(*line);
    }

    result.join("\n")
}
