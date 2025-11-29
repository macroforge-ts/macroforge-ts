//! Reusable macro expansion logic
//!
//! This module provides the core expansion functionality that can be used by any
//! macro package to implement its own `expandSync` NAPI function.

use std::collections::HashMap;

use swc_core::{
    common::Span,
    ecma::ast::{
        Class, Decl, Decorator, ExportDecl, Module, ModuleDecl, ModuleItem, Stmt,
    },
};
use ts_macro_abi::{
    ClassIR, Diagnostic, DiagnosticLevel, MacroContextIR, MacroResult, Patch, PatchCode,
    SourceMapping, SpanIR, TargetIR,
};
use ts_syn::{lower_classes, parse_ts_module};

use crate::{MacroConfig, MacroDispatcher, MacroRegistry, PatchCollector, derived};

/// Default module path for built-in derive macros
const DERIVE_MODULE_PATH: &str = "@macro/derive";

/// Result of macro expansion
#[derive(Debug, Clone)]
pub struct MacroExpansion {
    pub code: String,
    pub diagnostics: Vec<Diagnostic>,
    pub changed: bool,
    pub type_output: Option<String>,
    pub classes: Vec<ClassIR>,
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
}

impl MacroExpander {
    /// Create a new expander with the local registry populated from inventory
    pub fn new() -> crate::Result<Self> {
        let (config, root_dir) = MacroConfig::find_with_root()
            .map_err(|e| crate::MacroError::InvalidConfig(format!("{:?}", e)))?
            .unwrap_or_else(|| {
                (
                    MacroConfig::default(),
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                )
            });
        Self::with_config(config, root_dir)
    }

    /// Create an expander with a specific config
    pub fn with_config(config: MacroConfig, root_dir: std::path::PathBuf) -> crate::Result<Self> {
        let registry = MacroRegistry::new();
        register_packages(&registry, &config, &root_dir)?;

        Ok(Self {
            dispatcher: MacroDispatcher::new(registry),
            config,
        })
    }

    /// Expand all macros in the source code
    pub fn expand(&self, source: &str, file_name: &str) -> crate::Result<MacroExpansion> {
        // Quick check for @Derive
        if !source.contains("@Derive") {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
                type_output: None,
                classes: Vec::new(),
                source_mapping: None,
            });
        }

        // Parse the module
        let module = parse_ts_module(source)
            .map_err(|e| crate::MacroError::InvalidConfig(format!("Parse error: {:?}", e)))?;

        // Lower classes to IR
        let classes = lower_classes(&module, source)
            .map_err(|e| crate::MacroError::InvalidConfig(format!("Lower error: {:?}", e)))?;

        if classes.is_empty() {
            return Ok(MacroExpansion {
                code: source.to_string(),
                diagnostics: Vec::new(),
                changed: false,
                type_output: None,
                classes: Vec::new(),
                source_mapping: None,
            });
        }

        let classes_clone = classes.clone();

        let (mut collector, mut diagnostics) =
            self.collect_macro_patches(&module, classes, file_name, source);

        self.apply_and_finalize_expansion(source, &mut collector, &mut diagnostics, classes_clone)
    }

    fn collect_macro_patches(
        &self,
        module: &Module,
        classes: Vec<ClassIR>,
        file_name: &str,
        source: &str,
    ) -> (PatchCollector, Vec<Diagnostic>) {
        let mut collector = PatchCollector::new();
        let mut diagnostics = Vec::new();

        // Add patches to remove method bodies from type output
        for class_ir in classes.iter() {
            for method in &class_ir.methods {
                let method_signature = if method.name == "constructor" {
                    let visibility = match method.visibility {
                        ts_macro_abi::Visibility::Private => "private ",
                        ts_macro_abi::Visibility::Protected => "protected ",
                        ts_macro_abi::Visibility::Public => "",
                    };
                    format!(
                        "{visibility}constructor({params_src});",
                        visibility = visibility,
                        params_src = method.params_src
                    )
                } else {
                    let visibility = match method.visibility {
                        ts_macro_abi::Visibility::Private => "private ",
                        ts_macro_abi::Visibility::Protected => "protected ",
                        ts_macro_abi::Visibility::Public => "",
                    };
                    let static_kw = if method.is_static { "static " } else { "" };
                    let async_kw = if method.is_async { "async " } else { "" };

                    format!(
                        "{visibility}{static_kw}{async_kw}{method_name}{type_params}({params_src}): {return_type_src};",
                        visibility = visibility,
                        static_kw = static_kw,
                        async_kw = async_kw,
                        method_name = method.name,
                        type_params = method.type_params_src,
                        params_src = method.params_src,
                        return_type_src = method.return_type_src
                    )
                };

                collector.add_type_patches(vec![Patch::Replace {
                    span: method.span,
                    code: method_signature.into(),
                }]);
            }
        }

        let class_map: HashMap<SpanKey, ClassIR> = classes
            .into_iter()
            .map(|class| (SpanKey::from(class.span), class))
            .collect();

        let derive_targets = collect_derive_targets(module, &class_map, source);

        if derive_targets.is_empty() {
            return (collector, diagnostics);
        }

        for target in derive_targets {
            let decorator_removal = Patch::Delete {
                span: target.decorator_span,
            };
            collector.add_runtime_patches(vec![decorator_removal.clone()]);
            collector.add_type_patches(vec![decorator_removal]);

            // Remove field decorators
            for field in &target.class_ir.fields {
                for decorator in &field.decorators {
                    if decorator.name == "Derive" || decorator.name == "Debug" {
                        let field_dec_removal = Patch::Delete {
                            span: span_ir_with_at(decorator.span, source),
                        };
                        collector.add_runtime_patches(vec![field_dec_removal.clone()]);
                        collector.add_type_patches(vec![field_dec_removal]);
                    }
                }
            }

            let target_source = source
                .get(target.class_ir.span.start as usize..target.class_ir.span.end as usize)
                .unwrap_or("")
                .to_string();

            for (macro_name, module_path) in target.macro_names {
                let ctx = MacroContextIR::new_derive_class(
                    macro_name.clone(),
                    module_path.clone(),
                    target.decorator_span,
                    target.class_ir.span,
                    file_name.to_string(),
                    target.class_ir.clone(),
                    target_source.clone(),
                );

                let mut result = self.dispatcher.dispatch(ctx.clone());

                // Process potential token stream result
                if let Ok((runtime, type_def)) = self.process_macro_output(&mut result, &ctx) {
                    result.runtime_patches.extend(runtime);
                    result.type_patches.extend(type_def);
                }

                if !result.diagnostics.is_empty() {
                    diagnostics.extend(result.diagnostics.clone());
                }

                collector.add_runtime_patches(result.runtime_patches);
                collector.add_type_patches(result.type_patches);
            }
        }
        (collector, diagnostics)
    }

    fn process_macro_output(
        &self,
        result: &mut MacroResult,
        ctx: &MacroContextIR,
    ) -> crate::Result<(Vec<Patch>, Vec<Patch>)> {
        use swc_core::ecma::ast::{ClassMember, Stmt as SwcStmt, Decl as SwcDecl};

        let mut runtime_patches = Vec::new();
        let mut type_patches = Vec::new();

        if let Some(tokens) = &result.tokens
            && ctx.macro_kind == ts_macro_abi::MacroKind::Derive
            && let TargetIR::Class(class_ir) = &ctx.target
        {
            let wrapped_src = format!("class __Temp {{ {} }}", tokens);
            let stmt = ts_syn::parse_ts_stmt(&wrapped_src)
                .map_err(|e| crate::MacroError::InvalidConfig(format!("Failed to parse macro output: {:?}", e)))?;

            if let SwcStmt::Decl(SwcDecl::Class(class_decl)) = stmt {
                for member in class_decl.class.body {
                    let insert_pos = class_ir.body_span.end - 1;

                    runtime_patches.push(Patch::Insert {
                        at: SpanIR {
                            start: insert_pos,
                            end: insert_pos,
                        },
                        code: PatchCode::ClassMember(member.clone()),
                    });

                    let mut signature_member = member.clone();
                    match &mut signature_member {
                        ClassMember::Method(m) => m.function.body = None,
                        ClassMember::Constructor(c) => c.body = None,
                        ClassMember::PrivateMethod(m) => m.function.body = None,
                        _ => {}
                    }

                    type_patches.push(Patch::Insert {
                        at: SpanIR {
                            start: insert_pos,
                            end: insert_pos,
                        },
                        code: PatchCode::ClassMember(signature_member),
                    });
                }
            }
        }
        Ok((runtime_patches, type_patches))
    }

    fn apply_and_finalize_expansion(
        &self,
        source: &str,
        collector: &mut PatchCollector,
        diagnostics: &mut Vec<Diagnostic>,
        classes: Vec<ClassIR>,
    ) -> crate::Result<MacroExpansion> {
        let runtime_result = collector
            .apply_runtime_patches_with_mapping(source, None)
            .map_err(|e| crate::MacroError::InvalidConfig(format!("Patch error: {:?}", e)))?;

        let type_output = if collector.has_type_patches() {
            Some(
                collector
                    .apply_type_patches(source)
                    .map_err(|e| crate::MacroError::InvalidConfig(format!("Type patch error: {:?}", e)))?,
            )
        } else {
            None
        };

        let source_mapping = if runtime_result.mapping.is_empty() {
            None
        } else {
            Some(runtime_result.mapping)
        };

        let mut expansion = MacroExpansion {
            code: runtime_result.code,
            diagnostics: std::mem::take(diagnostics),
            changed: true,
            type_output,
            classes,
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
                    "Adjust `limits.maxDiagnostics` in ts-macros.json to see all diagnostics"
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

// ============================================================================
// Helper types and functions
// ============================================================================

#[derive(Hash, PartialEq, Eq)]
struct SpanKey(u32, u32);

impl From<SpanIR> for SpanKey {
    fn from(span: SpanIR) -> Self {
        SpanKey(span.start, span.end)
    }
}

impl From<Span> for SpanKey {
    fn from(span: Span) -> Self {
        SpanKey(span.lo.0, span.hi.0)
    }
}

#[derive(Clone)]
struct DeriveTarget {
    macro_names: Vec<(String, String)>,
    decorator_span: SpanIR,
    class_ir: ClassIR,
}

fn collect_import_sources(module: &Module) -> HashMap<String, String> {
    use swc_core::ecma::ast::{ImportDecl, ImportSpecifier};

    let mut import_map = HashMap::new();

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            specifiers,
            src,
            ..
        })) = item
        {
            let module_source = src.value.to_string_lossy().to_string();

            for specifier in specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        let local_name = named.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                    ImportSpecifier::Default(default) => {
                        let local_name = default.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                    ImportSpecifier::Namespace(ns) => {
                        let local_name = ns.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                }
            }
        }
    }

    import_map
}

fn collect_derive_targets(
    module: &Module,
    class_map: &HashMap<SpanKey, ClassIR>,
    source: &str,
) -> Vec<DeriveTarget> {
    let mut targets = Vec::new();
    let import_sources = collect_import_sources(module);

    for item in &module.body {
        match item {
            ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) => {
                collect_from_class(&class_decl.class, class_map, source, &import_sources, &mut targets);
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                decl: Decl::Class(class_decl),
                ..
            })) => {
                collect_from_class(&class_decl.class, class_map, source, &import_sources, &mut targets);
            }
            _ => {}
        }
    }

    targets
}

fn collect_from_class(
    class: &Class,
    class_map: &HashMap<SpanKey, ClassIR>,
    source: &str,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<DeriveTarget>,
) {
    if class.decorators.is_empty() {
        return;
    }

    let key = SpanKey::from(class.span);
    let Some(class_ir) = class_map.get(&key) else {
        return;
    };

    for decorator in &class.decorators {
        if let Some(macro_names) = parse_derive_decorator(decorator, import_sources) {
            if macro_names.is_empty() {
                continue;
            }

            out.push(DeriveTarget {
                macro_names,
                decorator_span: decorator_span_with_at(decorator.span, source),
                class_ir: class_ir.clone(),
            });
        }
    }
}

fn decorator_span_with_at(span: Span, source: &str) -> SpanIR {
    let mut ir = span_to_ir(span);
    let start = ir.start as usize;
    if start > 0 && start <= source.len() {
        let bytes = source.as_bytes();
        if bytes[start - 1] == b'@' {
            ir.start -= 1;
        }
    }
    ir
}

fn span_ir_with_at(span: SpanIR, source: &str) -> SpanIR {
    let mut ir = span;
    let start = ir.start as usize;
    if start > 0 && start <= source.len() {
        let bytes = source.as_bytes();
        if bytes[start - 1] == b'@' {
            ir.start -= 1;
        }
    }
    ir
}

fn parse_derive_decorator(
    decorator: &Decorator,
    import_sources: &HashMap<String, String>,
) -> Option<Vec<(String, String)>> {
    let call = match decorator.expr.as_ref() {
        swc_core::ecma::ast::Expr::Call(call) => call,
        _ => return None,
    };

    let callee = match &call.callee {
        swc_core::ecma::ast::Callee::Expr(expr) => match expr.as_ref() {
            swc_core::ecma::ast::Expr::Ident(ident) => ident.sym.to_string(),
            _ => return None,
        },
        _ => return None,
    };

    if callee != "Derive" {
        return None;
    }

    let mut macros = Vec::new();
    for arg in &call.args {
        if arg.spread.is_some() {
            continue;
        }

        if let Some(name) = derive_name_from_expr(arg.expr.as_ref()) {
            let module_path = import_sources
                .get(&name)
                .cloned()
                .unwrap_or_else(|| DERIVE_MODULE_PATH.to_string());
            macros.push((name, module_path));
        }
    }

    Some(macros)
}

fn derive_name_from_expr(expr: &swc_core::ecma::ast::Expr) -> Option<String> {
    use swc_core::ecma::ast::{Expr, Lit};

    match expr {
        Expr::Ident(ident) => Some(ident.sym.to_string()),
        Expr::Lit(Lit::Str(str_lit)) => Some(str_lit.value.to_string_lossy().to_string()),
        _ => None,
    }
}

fn span_to_ir(span: Span) -> SpanIR {
    SpanIR::new(span.lo.0, span.hi.0)
}

// ============================================================================
// Package registration
// ============================================================================

/// Special marker for dynamic module resolution
const DYNAMIC_MODULE_MARKER: &str = "__DYNAMIC_MODULE__";

fn register_packages(
    registry: &MacroRegistry,
    config: &MacroConfig,
    _config_root: &std::path::Path,
) -> crate::Result<()> {
    use crate::package_registry;

    let mut embedded_map: HashMap<&'static str, fn(&MacroRegistry) -> crate::Result<()>> =
        HashMap::new();

    for pkg in package_registry::registrars() {
        embedded_map.entry(pkg.module).or_insert(pkg.registrar);
    }

    let derived_modules = derived::modules();
    let derived_set: std::collections::HashSet<&'static str> =
        derived_modules.iter().copied().collect();

    let mut requested = if config.macro_packages.is_empty() {
        embedded_map.keys().cloned().collect::<Vec<_>>()
    } else {
        config
            .macro_packages
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
    };

    if config.macro_packages.is_empty() {
        requested.extend(derived_modules.iter().copied());
    }

    requested.sort();
    requested.dedup();

    for module in requested {
        let mut found = false;

        if let Some(registrar) = embedded_map.get(module) {
            registrar(registry)?;
            found = true;
        }

        if derived_set.contains(module) {
            derived::register_module(module, registry)?;
            found = true;
        }

        if !found {
            // Module not found - this is a warning, not an error
        }
    }

    // Register dynamic-module macros under the default derive path
    if derived_set.contains(DYNAMIC_MODULE_MARKER) {
        let _ = derived::register_module(DYNAMIC_MODULE_MARKER, registry);
    }

    for entry in inventory::iter::<derived::DerivedMacroRegistration> {
        let descriptor = entry.descriptor;
        if descriptor.module == DYNAMIC_MODULE_MARKER {
            let _ = registry.register(
                DERIVE_MODULE_PATH,
                descriptor.name,
                (descriptor.constructor)(),
            );
        }
    }

    Ok(())
}
