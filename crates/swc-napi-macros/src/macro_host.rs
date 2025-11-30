use anyhow::{Context, Result};
use napi::Status;
use napi::bindgen_prelude::Env;
use std::{collections::HashMap, path::Path, process::Command};
use swc_core::{
    common::Span,
    ecma::ast::{
        Class, ClassMember, Decl, Decorator, ExportDecl, Module, ModuleDecl, ModuleItem, Program,
        Stmt,
    },
};
use ts_macro_abi::{
    ClassIR, Diagnostic, DiagnosticLevel, MacroContextIR, MacroResult, Patch, PatchCode,
    SourceMapping, SpanIR, TargetIR,
};
use ts_macro_host::{MacroConfig, MacroDispatcher, MacroRegistry, PatchCollector, derived};

use ts_syn::lower_classes;

/// Default module path for built-in derive macros
const DERIVE_MODULE_PATH: &str = "@macro/derive";

/// Special marker for dynamic module resolution
const DYNAMIC_MODULE_MARKER: &str = "__DYNAMIC_MODULE__";

/// Connects the SWC parser to the macro host.
pub struct MacroHostIntegration {
    pub dispatcher: MacroDispatcher,
    config: MacroConfig,
    external_loader: Option<ExternalMacroLoader>,
}

/// Result of attempting to expand macros in a source file.
pub struct MacroExpansion {
    pub code: String,
    pub diagnostics: Vec<Diagnostic>,
    pub changed: bool,
    pub type_output: Option<String>,
    pub classes: Vec<ClassIR>,
    /// Source mapping between original and expanded code positions
    pub source_mapping: Option<SourceMapping>,
}

impl MacroHostIntegration {
    /// Build a macro host with the built-in macro registry populated.
    pub fn new() -> Result<Self> {
        Self::new_with_env(None)
    }

    pub fn new_with_env(env: Option<&Env>) -> Result<Self> {
        let (config, root_dir) = MacroConfig::find_with_root()
            .context("failed to discover macro configuration")?
            .unwrap_or_else(|| {
                (
                    MacroConfig::default(),
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                )
            });
        Self::with_config_and_env(config, root_dir, env)
    }

    #[allow(dead_code)]
    pub fn with_config(config: MacroConfig, root_dir: std::path::PathBuf) -> Result<Self> {
        Self::with_config_and_env(config, root_dir, None)
    }

    pub fn with_config_and_env(
        config: MacroConfig,
        root_dir: std::path::PathBuf,
        _env: Option<&Env>,
    ) -> Result<Self> {
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
            registry.contains("@macro/derive", "Eq"),
            "Built-in @macro/derive::Eq macro should be registered"
        );

        Ok(Self {
            dispatcher: MacroDispatcher::new(registry),
            config,
            external_loader: Some(ExternalMacroLoader::new(root_dir.clone())),
        })
    }

    pub fn set_keep_decorators(&mut self, keep: bool) {
        self.config.keep_decorators = keep;
    }

    /// Expand all macros found in the parsed program and return the updated source code.
    pub fn expand(
        &self,
        source: &str,
        program: &Program,
        file_name: &str,
    ) -> Result<MacroExpansion> {
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

        let (module, classes) = match self.prepare_expansion_context(program, source)? {
            Some(context) => context,
            None => {
                return Ok(MacroExpansion {
                    code: source.to_string(),
                    diagnostics: Vec::new(),
                    changed: false,
                    type_output: None,
                    classes: Vec::new(),
                    source_mapping: None,
                });
            }
        };

        let classes_clone = classes.clone();

        let (mut collector, mut diagnostics) =
            self.collect_macro_patches(&module, classes, file_name, source);
        self.apply_and_finalize_expansion(source, &mut collector, &mut diagnostics, classes_clone)
    }

    pub(crate) fn prepare_expansion_context(
        &self,
        program: &Program,
        source: &str,
    ) -> Result<Option<(Module, Vec<ClassIR>)>> {
        let module = match program {
            Program::Module(module) => module.clone(),
            Program::Script(_) => return Ok(None),
        };

        let classes = lower_classes(&module, source)
            .context("failed to lower classes for macro processing")?;

        if classes.is_empty() {
            return Ok(None);
        }

        Ok(Some((module, classes)))
    }

    pub(crate) fn collect_macro_patches(
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
            if !self.config.keep_decorators {
                let decorator_removal = Patch::Delete {
                    span: target.decorator_span,
                };
                collector.add_runtime_patches(vec![decorator_removal.clone()]);
                collector.add_type_patches(vec![decorator_removal]);
            }

            // Remove field decorators (e.g., @Derive({skip: true}) on fields)
            // These are macro configuration decorators that should not appear in output
            for field in &target.class_ir.fields {
                for decorator in &field.decorators {
                    // Strip Derive decorators and any declared attribute decorators
                    if (decorator.name == "Derive" || decorator.name == "Debug")
                        && !self.config.keep_decorators
                    {
                        let field_dec_removal = Patch::Delete {
                            span: span_ir_with_at(decorator.span, source),
                        };
                        collector.add_runtime_patches(vec![field_dec_removal.clone()]);
                        collector.add_type_patches(vec![field_dec_removal]);
                    }
                }
            }

            // Extract the source code for this class
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

                if is_macro_not_found(&result)
                    && ctx.module_path != DERIVE_MODULE_PATH
                    && ctx.module_path.starts_with('.')
                {
                    let fallback_ctx = MacroContextIR::new_derive_class(
                        macro_name.clone(),
                        DERIVE_MODULE_PATH.to_string(),
                        target.decorator_span,
                        target.class_ir.span,
                        file_name.to_string(),
                        target.class_ir.clone(),
                        target_source.clone(),
                    );
                    result = self.dispatcher.dispatch(fallback_ctx);
                }

                let no_output = result.runtime_patches.is_empty()
                    && result.type_patches.is_empty()
                    && result.tokens.is_none();

                if ctx.module_path != DERIVE_MODULE_PATH
                    && (is_macro_not_found(&result) || no_output)
                    && let Some(loader) = &self.external_loader
                {
                    match loader.run_macro(&ctx) {
                        Ok(external_result) => {
                            result = external_result;
                        }
                        Err(err) => {
                            result.diagnostics.push(Diagnostic {
                                level: DiagnosticLevel::Error,
                                message: format!(
                                    "Failed to load external macro '{}::{}': {}",
                                    ctx.macro_name, ctx.module_path, err
                                ),
                                span: Some(ctx.decorator_span),
                                notes: vec![],
                                help: None,
                            });
                        }
                    }
                }

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

    pub(crate) fn process_macro_output(
        &self,
        result: &mut MacroResult,
        ctx: &MacroContextIR,
    ) -> Result<(Vec<Patch>, Vec<Patch>)> {
        let mut runtime_patches = Vec::new();
        let mut type_patches = Vec::new();

        if let Some(tokens) = &result.tokens
            && ctx.macro_kind == ts_macro_abi::MacroKind::Derive
            && let TargetIR::Class(class_ir) = &ctx.target
        {
            // It's a derive on a class.
            // Wrap tokens in a class to parse members
            let wrapped_src = format!("class __Temp {{ {} }}", tokens);
            let stmt = ts_syn::parse_ts_stmt(&wrapped_src)
                .map_err(|e| anyhow::anyhow!("Failed to parse macro output: {:?}", e))?;

            if let Stmt::Decl(Decl::Class(class_decl)) = stmt {
                for member in class_decl.class.body {
                    // Intent: AppendClassMember
                    // Insert at end of body (before closing brace)
                    let insert_pos = class_ir.body_span.end - 1;

                    runtime_patches.push(Patch::Insert {
                        at: SpanIR {
                            start: insert_pos,
                            end: insert_pos,
                        },
                        code: PatchCode::ClassMember(member.clone()),
                    });

                    // Generate type signature
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

    pub(crate) fn apply_and_finalize_expansion(
        &self,
        source: &str,
        collector: &mut PatchCollector,
        diagnostics: &mut Vec<Diagnostic>,
        classes: Vec<ClassIR>,
    ) -> Result<MacroExpansion> {
        // Apply runtime patches with source mapping
        // Note: macro_name is passed as None here since patches come from multiple macros
        // The individual generated regions track which macro generated them
        let runtime_result = collector
            .apply_runtime_patches_with_mapping(source, None)
            .context("failed to apply macro-generated patches")?;

        let type_output = if collector.has_type_patches() {
            // Type patches don't need mapping - they're for .d.ts output only
            Some(
                collector
                    .apply_type_patches(source)
                    .context("failed to apply macro-generated type patches")?,
            )
        } else {
            None
        };

        // Only include mapping if code actually changed
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

fn is_macro_not_found(result: &MacroResult) -> bool {
    result
        .diagnostics
        .iter()
        .any(|d| d.message.contains("Macro") && d.message.contains("not found"))
}

struct ExternalMacroLoader {
    root_dir: std::path::PathBuf,
}

impl ExternalMacroLoader {
    fn new(root_dir: std::path::PathBuf) -> Self {
        Self { root_dir }
    }

    fn run_macro(&self, ctx: &MacroContextIR) -> napi::Result<MacroResult> {
        let fn_name = format!("__tsMacrosRun{}", ctx.macro_name);
        let ctx_json =
            serde_json::to_string(ctx).map_err(|e| napi::Error::new(Status::InvalidArg, e))?;

        let script = r#"
const [modulePath, fnName, ctxJson, rootDir] = process.argv.slice(1);
const path = require('path');
const fs = require('fs');
const { pathToFileURL } = require('url');

const normalizeWorkspaces = (val) =>
  Array.isArray(val) ? val : (val && Array.isArray(val.packages) ? val.packages : []);

const toImportSpecifier = (id) => {
  if (id.startsWith('.') || id.startsWith('/')) {
    return pathToFileURL(path.resolve(rootDir, id)).href;
  }
  return id;
};

const expandWorkspace = (pattern) => {
  if (typeof pattern !== 'string') return [];
  const absolute = path.resolve(rootDir, pattern);
  if (!pattern.includes('*')) {
    return [absolute];
  }

  const starIdx = pattern.indexOf('*');
  const baseDir = path.resolve(rootDir, pattern.slice(0, starIdx));
  const suffix = pattern.slice(starIdx + 1);
  if (!fs.existsSync(baseDir)) return [];

  return fs
    .readdirSync(baseDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(baseDir, entry.name + suffix));
};

const candidates = [];
const seen = new Set();
const addCandidate = (id) => {
  if (!id) return;
  const key = id.startsWith('.') || id.startsWith('/') ? path.resolve(rootDir, id) : id;
  if (seen.has(key)) return;
  seen.add(key);
  candidates.push(id);
};

addCandidate(modulePath);

try {
  const rootPkg = JSON.parse(fs.readFileSync(path.join(rootDir, 'package.json'), 'utf8'));
  const workspaces = normalizeWorkspaces(rootPkg.workspaces);

  for (const ws of workspaces) {
    for (const pkgDir of expandWorkspace(ws)) {
      try {
        const pkgJsonPath = path.join(pkgDir, 'package.json');
        if (!fs.existsSync(pkgJsonPath)) continue;

        const pkgJson = JSON.parse(fs.readFileSync(pkgJsonPath, 'utf8'));
        addCandidate(pkgJson.name || pkgDir);
        addCandidate(pkgDir);
      } catch {}
    }
  }
} catch {}

const tryRequire = (id) => {
  try {
    return { module: require(id), loader: 'require' };
  } catch (error) {
    return { error };
  }
};

const tryImport = async (id) => {
  try {
    return { module: await import(toImportSpecifier(id)), loader: 'import' };
  } catch (error) {
    return { error };
  }
};

(async () => {
  const errors = [];

  for (const id of candidates) {
    let loaded = tryRequire(id);

    if (!loaded.module) {
      const imported = await tryImport(id);
      if (imported.module) {
        loaded = imported;
      } else {
        errors.push(
          `Failed to load '${id}' via require/import: ${
            imported.error?.message || loaded.error?.message || 'unknown error'
          }`
        );
        continue;
      }
    }

    const mod = loaded.module;
    const fn =
      mod?.[fnName] ||
      mod?.default?.[fnName] ||
      (typeof mod?.default === 'object' ? mod.default[fnName] : undefined);

    if (typeof fn !== 'function') {
      errors.push(`Module '${id}' loaded via ${loaded.loader} but missing export '${fnName}'`);
      continue;
    }

    const out = await fn(ctxJson);
    if (typeof out === 'string') {
      process.stdout.write(out);
      process.exit(0);
    }

    errors.push(`Macro '${fnName}' in '${id}' returned ${typeof out}, expected string`);
  }

  if (errors.length === 0) {
    errors.push('Macro not found in any workspace candidate');
  }

  console.error(errors.join('\n'));
  process.exit(2);
})().catch((err) => {
  console.error(err?.stack || String(err));
  process.exit(1);
});
"#;

        let child = Command::new("node")
            .current_dir(&self.root_dir)
            .arg("-e")
            .arg(script)
            .arg(&ctx.module_path)
            .arg(&fn_name)
            .arg(&ctx_json)
            .arg(self.root_dir.to_string_lossy().as_ref())
            .output()
            .map_err(|e| {
                napi::Error::new(
                    Status::GenericFailure,
                    format!("Failed to spawn node for external macro: {e}"),
                )
            })?;

        if !child.status.success() {
            let stderr = String::from_utf8_lossy(&child.stderr);
            return Err(napi::Error::new(
                Status::GenericFailure,
                format!("External macro runner failed: {stderr}"),
            ));
        }

        let result_json = String::from_utf8(child.stdout).map_err(|e| {
            napi::Error::new(
                Status::InvalidArg,
                format!("Macro runner returned non-UTF8 output: {e}"),
            )
        })?;

        let host_result: ts_macro_abi::MacroResult =
            serde_json::from_str(&result_json).map_err(|e| {
                napi::Error::new(
                    Status::InvalidArg,
                    format!("Failed to parse macro result: {e}"),
                )
            })?;

        Ok(MacroResult {
            runtime_patches: host_result.runtime_patches,
            type_patches: host_result.type_patches,
            diagnostics: host_result.diagnostics,
            tokens: host_result.tokens,
            debug: host_result.debug,
        })
    }
}

/// Internal span key for matching lowered IR to SWC nodes.
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
    /// List of (macro_name, module_path) pairs
    /// module_path is the import source (e.g., "@playground/macro")
    macro_names: Vec<(String, String)>,
    decorator_span: SpanIR,
    class_ir: ClassIR,
}

/// Collect a map of identifier name -> module source from import statements
pub(crate) fn collect_import_sources(module: &Module) -> HashMap<String, String> {
    use swc_core::ecma::ast::{ImportDecl, ImportSpecifier};

    let mut import_map = HashMap::new();

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            specifiers, src, ..
        })) = item
        {
            let module_source = src.value.to_string_lossy().to_string();

            for specifier in specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        // import { Foo } from "module" or import { Foo as Bar } from "module"
                        let local_name = named.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                    ImportSpecifier::Default(default) => {
                        // import Foo from "module"
                        let local_name = default.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                    ImportSpecifier::Namespace(ns) => {
                        // import * as Foo from "module"
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

    // Build import source map
    let import_sources = collect_import_sources(module);

    for item in &module.body {
        match item {
            ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) => {
                collect_from_class(
                    &class_decl.class,
                    class_map,
                    source,
                    &import_sources,
                    &mut targets,
                );
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                decl: Decl::Class(class_decl),
                ..
            })) => {
                collect_from_class(
                    &class_decl.class,
                    class_map,
                    source,
                    &import_sources,
                    &mut targets,
                );
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
            // Look up the import source for this macro identifier
            // Default to @macro/derive if not found in imports
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

type PackageRegistrar = fn(&MacroRegistry) -> ts_macro_host::Result<()>;

fn available_package_registrars() -> Vec<(&'static str, PackageRegistrar)> {
    // Built-in macros are now registered via inventory (ts_macro_derive)
    vec![]
}

fn register_packages(
    registry: &MacroRegistry,
    config: &MacroConfig,
    _config_root: &Path,
) -> Result<()> {
    let mut embedded_map: HashMap<&'static str, PackageRegistrar> =
        available_package_registrars().into_iter().collect();
    for pkg in ts_macro_host::package_registry::registrars() {
        embedded_map.entry(pkg.module).or_insert(pkg.registrar);
    }

    let derived_modules = ts_macro_host::derived::modules();
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
            registrar(registry)
                .map_err(anyhow::Error::from)
                .with_context(|| format!("failed to register macro package {module}"))?;
            found = true;
        }

        if derived_set.contains(module) {
            ts_macro_host::derived::register_module(module, registry)?;
            found = true;
        }

        if !found {
            // Module not found - this is a warning, not an error
            // The user may have configured a module that isn't compiled in
        }
    }

    // Also register macros with dynamic module marker under the default path
    // This ensures backward compatibility
    if derived_set.contains(DYNAMIC_MODULE_MARKER) {
        // Register dynamic macros under the default derive path for fallback
        let _ = derived::register_module(DYNAMIC_MODULE_MARKER, registry);
    }

    // Additionally, register all dynamic-module macros under the default derive path
    // This handles macros that use __DYNAMIC_MODULE__ marker
    for entry in inventory::iter::<derived::DerivedMacroRegistration> {
        let descriptor = entry.descriptor;
        if descriptor.module == DYNAMIC_MODULE_MARKER {
            // Register under the default derive path for backward compatibility
            let _ = registry.register(
                DERIVE_MODULE_PATH,
                descriptor.name,
                (descriptor.constructor)(),
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod external_macro_loader_tests {
    use super::ExternalMacroLoader;
    use std::{fs, path::Path};
    use tempfile::tempdir;
    use ts_macro_abi::{ClassIR, MacroContextIR, SpanIR};

    fn write(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    fn test_class() -> ClassIR {
        ClassIR {
            name: "Temp".into(),
            span: SpanIR::new(0, 10),
            body_span: SpanIR::new(1, 9),
            is_abstract: false,
            type_params: vec![],
            heritage: vec![],
            decorators: vec![],
            decorators_ast: vec![],
            fields: vec![],
            methods: vec![],
            members: vec![],
        }
    }

    #[test]
    fn loads_esm_workspace_macro_via_dynamic_import() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        write(
            &root.join("package.json"),
            r#"{"name":"root","type":"module","workspaces":["packages/*"]}"#,
        );
        write(
            &root.join("packages/macro/package.json"),
            r#"{"name":"@ext/macro","type":"module","main":"index.js"}"#,
        );
        write(
            &root.join("packages/macro/index.js"),
            r#"
export function __tsMacrosRunDebug(ctxJson) {
  return JSON.stringify({
    runtime_patches: [],
    type_patches: [],
    diagnostics: [],
    tokens: null,
    debug: null
  });
}
"#,
        );

        let loader = ExternalMacroLoader::new(root.to_path_buf());
        let ctx = MacroContextIR::new_derive_class(
            "Debug".into(),
            "@ext/macro".into(),
            SpanIR::new(0, 1),
            SpanIR::new(0, 1),
            "file.ts".into(),
            test_class(),
            "class Temp {}".into(),
        );

        let result = loader
            .run_macro(&ctx)
            .expect("should load ESM macro via dynamic import");

        assert!(result.diagnostics.is_empty());
    }
}
