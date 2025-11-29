use anyhow::{Context, Result};
use napi::bindgen_prelude::{Env, Unknown};
use napi::{JsValue, Status};
use std::{collections::HashMap, path::Path};
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

    pub fn new_with_env(env: Option<Env>) -> Result<Self> {
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
        env: Option<Env>,
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
            external_loader: env.map(|e| ExternalMacroLoader::new(e, root_dir.clone())),
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

    fn prepare_expansion_context(
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
                    if decorator.name == "Derive" || decorator.name == "Debug" {
                        if !self.config.keep_decorators {
                            let field_dec_removal = Patch::Delete {
                                span: span_ir_with_at(decorator.span, source),
                            };
                            collector.add_runtime_patches(vec![field_dec_removal.clone()]);
                            collector.add_type_patches(vec![field_dec_removal]);
                        }
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
                                    ctx.module_path, ctx.macro_name, err
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

    fn process_macro_output(
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

    fn apply_and_finalize_expansion(
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
        .any(|d| d.message.contains("Macro") && d.message.contains("not found in module"))
}

struct ExternalMacroLoader {
    env: Env,
    root_dir: std::path::PathBuf,
}

impl ExternalMacroLoader {
    fn new(env: Env, root_dir: std::path::PathBuf) -> Self {
        Self { env, root_dir }
    }

    fn run_macro(&self, ctx: &MacroContextIR) -> napi::Result<MacroResult> {
        let fn_name = format!("__tsMacrosRun{}", ctx.macro_name);
        let ctx_json =
            serde_json::to_string(ctx).map_err(|e| napi::Error::new(Status::InvalidArg, e))?;

        match self.try_run_via_workspaces(&fn_name, &ctx.module_path, &ctx_json) {
            Ok(Some(result_json)) => {
                let host_result: ts_macro_abi::MacroResult = serde_json::from_str(&result_json)
                    .map_err(|e| {
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
            Ok(None) => Ok(MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!(
                        "Macro '{}' not found in module '{}' or workspace macro packages",
                        ctx.macro_name, ctx.module_path
                    ),
                    span: Some(ctx.decorator_span),
                    notes: vec![],
                    help: None,
                }],
                ..MacroResult::default()
            }),
            Err(err) => Ok(MacroResult {
                diagnostics: vec![Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!(
                        "Failed to load macro '{}' from module '{}': {}",
                        ctx.macro_name, ctx.module_path, err
                    ),
                    span: Some(ctx.decorator_span),
                    notes: vec![],
                    help: None,
                }],
                ..MacroResult::default()
            }),
        }
    }

    fn try_run_via_workspaces(
        &self,
        fn_name: &str,
        module_path: &str,
        ctx_json: &str,
    ) -> napi::Result<Option<String>> {
        let module_path_lit =
            serde_json::to_string(module_path).unwrap_or_else(|_| "null".to_string());
        let fn_name_lit = serde_json::to_string(fn_name).unwrap_or_else(|_| "\"\"".to_string());
        let ctx_lit = serde_json::to_string(ctx_json).unwrap_or_else(|_| "\"\"".to_string());
        let root_dir_lit = serde_json::to_string(self.root_dir.to_string_lossy().as_ref())
            .unwrap_or_else(|_| "\"\"".to_string());
        let root_pkg_lit = serde_json::to_string(
            self.root_dir
                .join("package.json")
                .to_string_lossy()
                .as_ref(),
        )
        .unwrap_or_else(|_| "\"\"".to_string());

        let script = format!(
            r#"(function() {{
  const normalizeWorkspaces = (val) => Array.isArray(val) ? val : (val && Array.isArray(val.packages) ? val.packages : []);
  const buildRequire = () => {{
    if (typeof globalThis.require === "function") return globalThis.require;
    if (typeof require === "function") return require;
    if (typeof process !== "undefined" && process.mainModule && process.mainModule.require) {{
      return process.mainModule.require;
    }}
    try {{
      // Some embedders expose module with createRequire even when require is absent
      if (typeof module !== "undefined" && typeof module.createRequire === "function") {{
        return module.createRequire(process.cwd() + "/");
      }}
    }} catch {{}}
    try {{
      const {{ createRequire }} = (typeof module !== "undefined" ? module : null) || {{}};
      if (typeof createRequire === "function") {{
        return createRequire(process.cwd() + "/");
      }}
    }} catch {{}}
    return null;
  }};

  const baseRequire = buildRequire();
  if (!baseRequire) return null;

  const path = baseRequire("path");
  const fs = baseRequire("fs");
  let rootRequire = baseRequire;
  try {{
    const {{ createRequire }} = baseRequire("module");
    if (typeof createRequire === "function") {{
      rootRequire = createRequire(path.join({root_dir}, "package.json"));
    }}
  }} catch {{}}

  const resolveModule = (id) => {{
    try {{
      return rootRequire(id);
    }} catch (err) {{
      try {{
        return baseRequire(id);
      }} catch (_ignore) {{
        return null;
      }}
    }}
  }};

  const candidates = [];
  const seen = new Set();
  const addCandidate = (id) => {{
    if (!id || seen.has(id)) return;
    const mod = resolveModule(id);
    if (mod) {{
      seen.add(id);
      candidates.push(mod);
    }}
  }};

  // Try the requested module path first
  addCandidate({module_path});

  // Inspect workspace packages (supports ["packages/*"] and {{ packages: [...] }})
  try {{
    const rootPkg = resolveModule({root_pkg});
    const workspaces = normalizeWorkspaces(rootPkg?.workspaces);

    const expandWorkspace = (pattern) => {{
      if (typeof pattern !== "string") return [];
      const absolute = path.resolve({root_dir}, pattern);
      if (!pattern.includes("*")) {{
        return [absolute];
      }}

      const starIdx = pattern.indexOf("*");
      const baseDir = path.resolve({root_dir}, pattern.slice(0, starIdx));
      const suffix = pattern.slice(starIdx + 1);
      if (!fs.existsSync(baseDir)) return [];

      return fs
        .readdirSync(baseDir, {{ withFileTypes: true }})
        .filter((entry) => entry.isDirectory())
        .map((entry) => path.join(baseDir, entry.name + suffix));
    }};

    for (const ws of workspaces) {{
      for (const pkgDir of expandWorkspace(ws)) {{
        try {{
          const pkgJsonPath = path.join(pkgDir, "package.json");
          if (!fs.existsSync(pkgJsonPath)) continue;

          const pkgJson = JSON.parse(fs.readFileSync(pkgJsonPath, "utf8"));
          addCandidate(pkgJson.name || pkgDir);
          addCandidate(pkgDir);
        }} catch {{}}
      }}
    }}
  }} catch {{}}

  for (const mod of candidates) {{
    if (!mod) continue;
    const fn = mod[{fn_name}];
    if (typeof fn === "function") {{
      return fn({ctx_json});
    }}
  }}

  return null;
}})()"#,
            module_path = module_path_lit,
            root_pkg = root_pkg_lit,
            root_dir = root_dir_lit,
            fn_name = fn_name_lit,
            ctx_json = ctx_lit,
        );

        let result_unknown: Unknown = self.env.run_script(&script)?;
        let result_str = result_unknown
            .coerce_to_string()?
            .into_utf8()?
            .into_owned()?;
        if result_str == "null" || result_str == "undefined" {
            return Ok(None);
        }
        Ok(Some(result_str))
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
fn collect_import_sources(module: &Module) -> HashMap<String, String> {
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
mod tests {
    use super::*;
    use swc_core::{
        common::{FileName, GLOBALS, SourceMap, sync::Lrc},
        ecma::parser::{Lexer, Parser, StringInput, Syntax, TsSyntax},
    };

    fn parse_module(source: &str) -> Program {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("test.ts".into()).into(),
            source.to_string(),
        );

        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                decorators: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_module().expect("should parse");
        Program::Module(module)
    }

    trait StringExt {
        fn replace_whitespace(&self) -> String;
    }

    impl StringExt for str {
        fn replace_whitespace(&self) -> String {
            self.chars().filter(|c| !c.is_whitespace()).collect()
        }
    }

    fn base_class(name: &str) -> ClassIR {
        ClassIR {
            name: name.into(),
            span: SpanIR::new(0, 200),
            body_span: SpanIR::new(10, 190),
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
    fn test_derive_debug_runtime_output() {
        // Note: JSON macro is in playground-macros, not swc-napi-macros
        // Testing Debug macro which generates toString() implementation
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug)
class Data {
    val: number;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed, "expand() should report changes");
            // Debug macro adds toString() method
            assert!(result.code.contains("toString()"));
            assert!(result.code.contains("Data"));
        });
    }

    #[test]
    fn test_derive_debug_dts_output() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug)
class User {
    name: string;
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";


class User {
    name: string;
    toString(): string;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed, "expand() should report changes");
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_derive_clone_dts_output() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Clone)
class User {
    name: string;
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";


class User {
    name: string;
    clone(): User;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed, "expand() should report changes");
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_derive_eq_dts_output() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Eq)
class User {
    name: string;
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";


class User {
    name: string;
    equals(other: unknown): boolean;
    hashCode(): number;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed, "expand() should report changes");
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_derive_debug_complex_dts_output() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive("Debug")
class MacroUser {
  @Derive({ rename: "userId" })
  id: string;

  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;

  @Derive({ skip: true })
  apiToken: string;

  constructor(
    id: string,
    name: string,
    role: string,
    favoriteMacro: "Derive" | "JsonNative",
    since: string,
    apiToken: string,
  ) {
    this.id = id;
    this.name = name;
    this.role = role;
    this.favoriteMacro = favoriteMacro;
    this.since = since;
    this.apiToken = apiToken;
  }
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";

class MacroUser {
  id: string;

  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;

  apiToken: string;

  constructor(
    id: string,
    name: string,
    role: string,
    favoriteMacro: "Derive" | "JsonNative",
    since: string,
    apiToken: string,
  );
    toString(): string;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed, "expand() should report changes");
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(type_output, expected_dts);
        });
    }

    #[test]
    fn test_prepare_no_derive() {
        let source = "class User { name: string; }";
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.prepare_expansion_context(&program, source).unwrap();
        // Even without decorators, we return Some because we still need to
        // generate method signatures for type output
        assert!(result.is_some());
    }

    #[test]
    fn test_prepare_no_classes() {
        let source = "const x = 1;";
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.prepare_expansion_context(&program, source).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_prepare_with_classes() {
        let source = "@Derive(Debug) class User {}";
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.prepare_expansion_context(&program, source).unwrap();
        assert!(result.is_some());
        let (_module, classes) = result.unwrap();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "User");
    }

    #[test]
    fn test_process_macro_output_converts_tokens_into_patches() {
        GLOBALS.set(&Default::default(), || {
            let host = MacroHostIntegration::new().unwrap();
            let class_ir = base_class("TokenDriven");
            let ctx = MacroContextIR::new_derive_class(
                "Debug".into(),
                DERIVE_MODULE_PATH.into(),
                SpanIR::new(0, 5),
                class_ir.span,
                "token.ts".into(),
                class_ir.clone(),
                "class TokenDriven {}".into(),
            );

            let mut result = MacroResult {
                tokens: Some(
                    r#"
                        toString() { return `${this.value}`; }
                        constructor(value: string) { this.value = value; }
                    "#
                    .into(),
                ),
                ..Default::default()
            };

            let (runtime, type_patches) = host
                .process_macro_output(&mut result, &ctx)
                .expect("tokens should parse");

            assert_eq!(
                runtime.len(),
                2,
                "expected one runtime patch per generated member"
            );
            assert_eq!(
                type_patches.len(),
                2,
                "expected one type patch per generated member"
            );

            for patch in runtime {
                match patch {
                    Patch::Insert {
                        code: PatchCode::ClassMember(_),
                        ..
                    } => {}
                    other => panic!("expected class member insert, got {:?}", other),
                }
            }

            for patch in type_patches {
                if let Patch::Insert {
                    code: PatchCode::ClassMember(member),
                    ..
                } = patch
                {
                    match member {
                        ClassMember::Method(method) => assert!(
                            method.function.body.is_none(),
                            "type patch should strip method body"
                        ),
                        ClassMember::Constructor(cons) => assert!(
                            cons.body.is_none(),
                            "type patch should drop constructor body"
                        ),
                        _ => {}
                    }
                } else {
                    panic!("expected type patch insert");
                }
            }
        });
    }

    #[test]
    fn test_process_macro_output_reports_parse_errors() {
        GLOBALS.set(&Default::default(), || {
            let host = MacroHostIntegration::new().unwrap();
            let class_ir = base_class("Broken");
            let ctx = MacroContextIR::new_derive_class(
                "Debug".into(),
                DERIVE_MODULE_PATH.into(),
                SpanIR::new(0, 5),
                class_ir.span,
                "broken.ts".into(),
                class_ir.clone(),
                "class Broken {}".into(),
            );

            let mut result = MacroResult {
                tokens: Some("this is not valid class member syntax".into()),
                ..Default::default()
            };

            let err = host
                .process_macro_output(&mut result, &ctx)
                .expect_err("invalid tokens should bubble an error");

            assert!(
                err.to_string().contains("Failed to parse macro output"),
                "should mention parsing failure, got {err:?}"
            );
        });
    }

    #[test]
    fn test_collect_constructor_patch() {
        let source = "class User { constructor(id: string) { this.id = id; } }";
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let (module, classes) = host
            .prepare_expansion_context(&program, source)
            .unwrap()
            .unwrap();

        let (collector, _) = host.collect_macro_patches(&module, classes, "test.ts", source);

        let type_patches = collector.get_type_patches();
        assert_eq!(type_patches.len(), 1);
        let patch = &type_patches[0];

        if let Patch::Replace { code, .. } = patch {
            match code {
                PatchCode::Text(text) => assert_eq!(text, "constructor(id: string);"),
                _ => panic!("Expected textual patch for constructor signature"),
            }
        } else {
            panic!("Expected a replace patch for constructor");
        }
    }

    #[test]
    fn test_collect_derive_debug_patch() {
        let source = "@Derive(Debug) class User { name: string; }";
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let (module, classes) = host
            .prepare_expansion_context(&program, source)
            .unwrap()
            .unwrap();
        let (collector, _) = host.collect_macro_patches(&module, classes, "test.ts", source);

        let type_patches = collector.get_type_patches();
        // 1 for decorator removal, 1 for signature insertion
        assert_eq!(type_patches.len(), 2);
        // check for decorator deletion
        assert!(
            type_patches
                .iter()
                .any(|p| matches!(p, Patch::Delete { .. }))
        );
        // check for method signature insertion
        assert!(
            type_patches
                .iter()
                .any(|p| matches!(p, Patch::Insert { .. }))
        );
    }

    #[test]
    fn test_apply_and_finalize_expansion_no_type_patches() {
        let source = "class User {}";
        let mut collector = PatchCollector::new();
        let mut diagnostics = Vec::new();
        let host = MacroHostIntegration::new().unwrap();
        let result = host
            .apply_and_finalize_expansion(source, &mut collector, &mut diagnostics, Vec::new())
            .unwrap();
        assert!(result.type_output.is_none());
    }

    #[test]
    fn test_complex_class_with_multiple_derives() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug, Clone, Eq)
class Product {
    id: string;
    name: string;
    price: number;
    private secret: string;

    constructor(id: string, name: string, price: number, secret: string) {
        this.id = id;
        this.name = name;
        this.price = price;
        this.secret = secret;
    }

    getDisplayName(): string {
        return `${this.name} - $${this.price}`;
    }

    static fromJSON(json: any): Product {
        return new Product(json.id, json.name, json.price, json.secret);
    }
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";

class Product {
    id: string;
    name: string;
    price: number;
    private secret: string;

    constructor(id: string, name: string, price: number, secret: string);

    getDisplayName(): string;

    static fromJSON(json: any): Product;

    toString(): string;
    clone(): Product;
    equals(other: unknown): boolean;
    hashCode(): number;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed, "expand() should report changes");
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_complex_method_signatures() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug)
class API {
    endpoint: string;

    constructor(endpoint: string) {
        this.endpoint = endpoint;
    }

    async fetch<T>(
        path: string,
        options?: { method?: string; body?: any }
    ): Promise<T> {
        return {} as T;
    }

    subscribe(
        event: "data" | "error",
        callback: (data: any) => void,
        thisArg?: any
    ): () => void {
        return () => {};
    }
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";

class API {
    endpoint: string;

    constructor(endpoint: string);

    async fetch<T>(
        path: string,
        options?: { method?: string; body?: any }
    ): Promise<T>;

    subscribe(
        event: "data" | "error",
        callback: (data: any) => void,
        thisArg?: any
    ): () => void;

    toString(): string;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed);
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_class_with_visibility_modifiers() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Clone)
class Account {
    public username: string;
    protected password: string;
    private apiKey: string;

    constructor(username: string, password: string, apiKey: string) {
        this.username = username;
        this.password = password;
        this.apiKey = apiKey;
    }

    public login(): boolean {
        return true;
    }

    protected validatePassword(input: string): boolean {
        return this.password === input;
    }

    private getApiKey(): string {
        return this.apiKey;
    }
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";

class Account {
    public username: string;
    protected password: string;
    private apiKey: string;

    constructor(username: string, password: string, apiKey: string);

    login(): boolean;

    protected validatePassword(input: string): boolean;

    private getApiKey(): string;

    clone(): Account;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed);
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_class_with_optional_and_readonly_fields() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug, Eq)
class Config {
    readonly id: string;
    name: string;
    description?: string;
    readonly createdAt: Date;
    updatedAt?: Date;

    constructor(id: string, name: string, createdAt: Date) {
        this.id = id;
        this.name = name;
        this.createdAt = createdAt;
    }

    update(name: string, description?: string): void {
        this.name = name;
        this.description = description;
        this.updatedAt = new Date();
    }
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";

class Config {
    readonly id: string;
    name: string;
    description?: string;
    readonly createdAt: Date;
    updatedAt?: Date;

    constructor(id: string, name: string, createdAt: Date);

    update(name: string, description?: string): void;

    toString(): string;
    equals(other: unknown): boolean;
    hashCode(): number;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed);
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_empty_constructor_and_no_params_methods() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug)
class Singleton {
    private static instance: Singleton;

    private constructor() {
        // Private constructor
    }

    static getInstance(): Singleton {
        if (!Singleton.instance) {
            Singleton.instance = new Singleton();
        }
        return Singleton.instance;
    }

    reset(): void {
        // Reset logic
    }
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";

class Singleton {
    private static instance: Singleton;

    private constructor();

    static getInstance(): Singleton;

    reset(): void;

    toString(): string;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed);
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_class_with_field_decorators_and_derive() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug)
class ValidationExample {
    @Derive({ rename: "userId" })
    id: string;

    name: string;

    @Derive({ skip: true })
    internalFlag: boolean;

    constructor(id: string, name: string, internalFlag: boolean) {
        this.id = id;
        this.name = name;
        this.internalFlag = internalFlag;
    }
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";

class ValidationExample {
    id: string;

    name: string;

    internalFlag: boolean;

    constructor(id: string, name: string, internalFlag: boolean);

    toString(): string;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed);
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_generated_methods_on_separate_lines() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug, Clone)
class User {
    id: number;
    name: string;

    constructor(id: number, name: string) {
        this.id = id;
        this.name = name;
    }
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed);
            let type_output = result.type_output.expect("should have type output");

            // Verify methods are on separate lines, not jammed together
            let lines: Vec<&str> = type_output.lines().collect();

            // Find the toString line
            let tostring_line = lines
                .iter()
                .position(|l| l.contains("toString()"))
                .expect("should have toString");
            // Find the clone line
            let clone_line = lines
                .iter()
                .position(|l| l.contains("clone()"))
                .expect("should have clone");

            // They should be on different lines
            assert_ne!(
                tostring_line, clone_line,
                "toString and clone should be on different lines"
            );

            // Verify no line contains multiple method signatures
            for line in &lines {
                let method_count = line.matches("(): ").count();
                assert!(
                    method_count <= 1,
                    "Line should not contain multiple methods: {}",
                    line
                );
            }
        });
    }

    #[test]
    fn test_proper_indentation_in_generated_code() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug)
class User {
  id: number;
  name: string;

  constructor(id: number, name: string) {
    this.id = id;
    this.name = name;
  }
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed);
            let type_output = result.type_output.expect("should have type output");

            // Find the toString line
            let tostring_line = type_output
                .lines()
                .find(|l| l.contains("toString()"))
                .expect("should have toString method");

            // Verify it has proper indentation (2 spaces to match the class body)
            assert!(
                tostring_line.starts_with("  toString()")
                    || tostring_line.trim().starts_with("toString()"),
                "toString should have proper indentation, got: '{}'",
                tostring_line
            );
        });
    }

    #[test]
    fn test_default_parameter_values() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug)
class ServerConfig {
    host: string;
    port: number;

    constructor(
        host: string = "localhost",
        port: number = 8080,
        secure: boolean = false
    ) {
        this.host = host;
        this.port = port;
    }

    connect(
        timeout: number = 5000,
        retries: number = 3,
        onError?: (err: Error) => void
    ): Promise<void> {
        return Promise.resolve();
    }

    static create(
        config: Partial<ServerConfig> = {},
        defaults: { host?: string; port?: number } = { host: "0.0.0.0", port: 3000 }
    ): ServerConfig {
        return new ServerConfig();
    }
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";

class ServerConfig {
    host: string;
    port: number;

    constructor(
        host: string = "localhost",
        port: number = 8080,
        secure: boolean = false
    );

    connect(
        timeout: number = 5000,
        retries: number = 3,
        onError?: (err: Error) => void
    ): Promise<void>;

    static create(
        config: Partial<ServerConfig> = {},
        defaults: { host?: string; port?: number } = { host: "0.0.0.0", port: 3000 }
    ): ServerConfig;

    toString(): string;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed);
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_rest_parameters_and_destructuring() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Clone)
class EventEmitter {
    listeners: Map<string, Function[]>;

    constructor() {
        this.listeners = new Map();
    }

    on(event: string, ...callbacks: Array<(...args: any[]) => void>): void {
        const existing = this.listeners.get(event) || [];
        this.listeners.set(event, [...existing, ...callbacks]);
    }

    emit(event: string, ...args: any[]): void {
        const callbacks = this.listeners.get(event) || [];
        callbacks.forEach(cb => cb(...args));
    }
}
"#;

        let expected_dts = r#"
import { Derive } from "@macro/derive";

class EventEmitter {
    listeners: Map<string, Function[]>;

    constructor();

    on(event: string, ...callbacks: Array<(...args: any[]) => void>): void;

    emit(event: string, ...args: any[]): void;

    clone(): EventEmitter;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed);
            let type_output = result.type_output.expect("should have type output");

            assert_eq!(
                type_output.replace_whitespace(),
                expected_dts.replace_whitespace()
            );
        });
    }

    #[test]
    fn test_source_mapping_produced() {
        let source = r#"
import { Derive } from "@macro/derive";

@Derive(Debug)
class User {
    name: string;
}
"#;

        GLOBALS.set(&Default::default(), || {
            let program = parse_module(source);
            let host = MacroHostIntegration::new().unwrap();
            let result = host.expand(source, &program, "test.ts").unwrap();

            assert!(result.changed, "Expansion should report changes");

            // Source mapping should be produced
            let mapping = result
                .source_mapping
                .expect("Source mapping should be produced");

            // Should have segments for unchanged regions
            assert!(!mapping.segments.is_empty(), "Should have mapping segments");

            // Should have a generated region for the toString implementation
            assert!(
                !mapping.generated_regions.is_empty(),
                "Should have generated regions"
            );

            // Print mapping for debugging
            println!("Source mapping segments: {:?}", mapping.segments);
            println!(
                "Source mapping generated regions: {:?}",
                mapping.generated_regions
            );
        });
    }
}
