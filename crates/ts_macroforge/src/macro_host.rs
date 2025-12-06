use anyhow::{Context, Result};
use napi::Status;
use napi::bindgen_prelude::Env;
use std::{collections::HashMap, path::Path, process::Command};
use swc_core::{common::Span, ecma::ast::{ClassMember, Module, Program}};
use ts_macro_abi::{
    ClassIR, Diagnostic, DiagnosticLevel, InterfaceIR, MacroContextIR, MacroResult, Patch,
    PatchCode, SourceMapping, SpanIR, TargetIR,
};
use ts_macro_host::{MacroConfig, MacroDispatcher, MacroRegistry, PatchCollector, derived};

use ts_syn::{lower_classes, lower_interfaces};

/// Default module path for built-in derive macros
const DERIVE_MODULE_PATH: &str = "@macro/derive";

/// Special marker for dynamic module resolution
const DYNAMIC_MODULE_MARKER: &str = "__DYNAMIC_MODULE__";

/// Connects the SWC parser to the macro host.
pub struct MacroHostIntegration {
    pub dispatcher: MacroDispatcher,
    config: MacroConfig,
    /// Whether to keep decorators in the expanded output (only set by TS plugin for mapping)
    keep_decorators: bool,
    external_loader: Option<ExternalMacroLoader>,
}

/// Result of attempting to expand macros in a source file.
pub struct MacroExpansion {
    pub code: String,
    pub diagnostics: Vec<Diagnostic>,
    pub changed: bool,
    pub type_output: Option<String>,
    pub classes: Vec<ClassIR>,
    pub interfaces: Vec<InterfaceIR>,
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

        let keep_decorators = config.keep_decorators;

        Ok(Self {
            dispatcher: MacroDispatcher::new(registry),
            config,
            keep_decorators,
            external_loader: Some(ExternalMacroLoader::new(root_dir.clone())),
        })
    }

    pub fn set_keep_decorators(&mut self, keep: bool) {
        self.keep_decorators = keep;
    }

    /// Expand all macros found in the parsed program and return the updated source code.
    pub fn expand(
        &self,
        source: &str,
        program: &Program,
        file_name: &str,
    ) -> Result<MacroExpansion> {
        let (module, classes, interfaces) =
            match self.prepare_expansion_context(program, source)? {
                Some(context) => context,
                None => {
                    return Ok(MacroExpansion {
                        code: source.to_string(),
                        diagnostics: Vec::new(),
                        changed: false,
                        type_output: None,
                        classes: Vec::new(),
                        interfaces: Vec::new(),
                        source_mapping: None,
                    });
                }
            };

        let classes_clone = classes.clone();
        let interfaces_clone = interfaces.clone();

        let (mut collector, mut diagnostics) =
            self.collect_macro_patches(&module, classes, interfaces, file_name, source);
        self.apply_and_finalize_expansion(
            source,
            &mut collector,
            &mut diagnostics,
            classes_clone,
            interfaces_clone,
        )
    }

    pub(crate) fn prepare_expansion_context(
        &self,
        program: &Program,
        source: &str,
    ) -> Result<Option<(Module, Vec<ClassIR>, Vec<InterfaceIR>)>> {
        let module = match program {
            Program::Module(module) => module.clone(),
            Program::Script(_) => return Ok(None),
        };

        let classes = lower_classes(&module, source)
            .context("failed to lower classes for macro processing")?;

        let interfaces = lower_interfaces(&module, source)
            .context("failed to lower interfaces for macro processing")?;

        if classes.is_empty() && interfaces.is_empty() {
            return Ok(None);
        }

        Ok(Some((module, classes, interfaces)))
    }

    pub(crate) fn collect_macro_patches(
        &self,
        module: &Module,
        classes: Vec<ClassIR>,
        interfaces: Vec<InterfaceIR>,
        file_name: &str,
        source: &str,
    ) -> (PatchCollector, Vec<Diagnostic>) {
        let mut collector = PatchCollector::new();
        let mut diagnostics = Vec::new();

        // Add patches to remove method bodies from type output (classes only)
        for class_ir in classes.iter() {
            for method in &class_ir.methods {
                let return_type = method
                    .return_type_src
                    .trim_start()
                    .trim_start_matches(':')
                    .trim_start();
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
                    source_macro: None, // Method body stripping is internal, not macro-generated
                }]);
            }
        }

        let class_map: HashMap<SpanKey, ClassIR> = classes
            .into_iter()
            .map(|class| (SpanKey::from(class.span), class))
            .collect();

        let interface_map: HashMap<SpanKey, InterfaceIR> = interfaces
            .into_iter()
            .map(|iface| (SpanKey::from(iface.span), iface))
            .collect();

        let derive_targets = collect_derive_targets(module, &class_map, &interface_map, source);

        if derive_targets.is_empty() {
            return (collector, diagnostics);
        }

        for target in derive_targets {
            if !self.keep_decorators {
                let decorator_removal = Patch::Delete {
                    span: target.decorator_span,
                };
                collector.add_runtime_patches(vec![decorator_removal.clone()]);
                collector.add_type_patches(vec![decorator_removal]);
            }

            // Remove all field decorators when not keeping decorators
            if !self.keep_decorators {
                match &target.target_ir {
                    DeriveTargetIR::Class(class_ir) => {
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
                    DeriveTargetIR::Interface(interface_ir) => {
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
                }
            }

            // Extract the source code for this target
            let (_target_span, _target_source, ctx_factory): (
                SpanIR,
                String,
                Box<dyn Fn(String, String) -> MacroContextIR>,
            ) = match &target.target_ir {
                DeriveTargetIR::Class(class_ir) => {
                    let span = class_ir.span;
                    let src = source
                        .get(span.start as usize..span.end as usize)
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
                        .get(span.start as usize..span.end as usize)
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
            };

            for (macro_name, module_path) in target.macro_names {
                let mut ctx = ctx_factory(macro_name.clone(), module_path.clone());

                // Calculate macro_name_span by finding the macro name within the decorator source
                if let Some(macro_name_span) =
                    find_macro_name_span(source, target.decorator_span, &macro_name)
                {
                    ctx = ctx.with_macro_name_span(macro_name_span);
                }

                let mut result = self.dispatcher.dispatch(ctx.clone());

                if is_macro_not_found(&result)
                    && ctx.module_path != DERIVE_MODULE_PATH
                    && ctx.module_path.starts_with('.')
                {
                    let fallback_ctx = ctx_factory(macro_name.clone(), DERIVE_MODULE_PATH.to_string());
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
                    result.runtime_patches.extend(runtime);
                    result.type_patches.extend(type_def);
                }

                if !result.diagnostics.is_empty() {
                    // Adjust diagnostic spans to point to @derive(...) for better error display
                    for diag in &mut result.diagnostics {
                        if let Some(span) = diag.span {
                            diag.span = Some(diagnostic_span_for_derive(span, source));
                        }
                    }
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
        source: &str,
    ) -> Result<(Vec<Patch>, Vec<Patch>)> {
        let mut runtime_patches = Vec::new();
        let mut type_patches = Vec::new();

        if let Some(tokens) = &result.tokens
            && ctx.macro_kind == ts_macro_abi::MacroKind::Derive
        {
            // Track which macro generated these patches
            let macro_name = Some(ctx.macro_name.clone());

            match &ctx.target {
                TargetIR::Class(class_ir) => {
                    // It's a derive on a class.
                    // Split tokens by location markers (default is "below")
                    let chunks = split_by_markers(tokens);

                    for (location, code) in chunks {
                        match location {
                            "above" => {
                                // Insert before class declaration
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
                                // Insert after class declaration (new default)
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
                                // Insert after opening brace
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
                                // Parse as class members and insert before closing brace
                                let insert_pos = derive_insert_pos(class_ir, source);
                                match parse_members_from_tokens(&code) {
                                    Ok(members) => {
                                        for member in members {
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
                                        // Fallback: inject raw tokens to avoid losing macro output
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
                            }
                            _ => {} // Unknown location, ignore
                        }
                    }
                }
                TargetIR::Interface(interface_ir) => {
                    // It's a derive on an interface.
                    // Use the same marker-based system as classes (default is "below").
                    // Macro authors must explicitly structure their output.
                    let chunks = split_by_markers(tokens);

                    for (location, code) in chunks {
                        match location {
                            "above" => {
                                // Insert before interface declaration
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
                                // For interfaces, body/signature also insert below since
                                // interfaces don't have a body to insert into
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
                            _ => {} // Unknown location, ignore
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
        classes: Vec<ClassIR>,
        interfaces: Vec<InterfaceIR>,
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

        let mut code = runtime_result.code;
        if !self.keep_decorators {
            code = strip_decorators(&code);
        }

        let mut expansion = MacroExpansion {
            code,
            diagnostics: std::mem::take(diagnostics),
            changed: true,
            type_output,
            classes,
            interfaces,
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

fn is_macro_not_found(result: &MacroResult) -> bool {
    result
        .diagnostics
        .iter()
        .any(|d| d.message.contains("Macro") && d.message.contains("not found"))
}

fn strip_decorators(code: &str) -> String {
    code.lines()
        .filter(|line| !line.trim_start().starts_with('@'))
        .collect::<Vec<_>>()
        .join("\n")
}

struct ExternalMacroLoader {
    root_dir: std::path::PathBuf,
}

impl ExternalMacroLoader {
    fn new(root_dir: std::path::PathBuf) -> Self {
        Self { root_dir }
    }

    fn run_macro(&self, ctx: &MacroContextIR) -> napi::Result<MacroResult> {
        let fn_name = format!("__macroforgeRun{}", ctx.macro_name);
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

/// The IR for a derive target - either a class or interface
#[derive(Clone)]
enum DeriveTargetIR {
    Class(ClassIR),
    Interface(InterfaceIR),
}

#[derive(Clone)]
struct DeriveTarget {
    /// List of (macro_name, module_path) pairs
    /// module_path is the import source (e.g., "@playground/macro")
    macro_names: Vec<(String, String)>,
    decorator_span: SpanIR,
    target_ir: DeriveTargetIR,
}

/// Collect a map of identifier name -> module source from import statements
pub(crate) fn collect_import_sources(module: &Module, source: &str) -> HashMap<String, String> {
    use swc_core::ecma::ast::{ImportDecl, ImportSpecifier, ModuleDecl, ModuleItem};

    let mut import_map = HashMap::new();

    import_map.extend(collect_macro_import_comments(source));

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

fn collect_macro_import_comments(source: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let mut search_start = 0usize;

    while let Some(idx) = source[search_start..].find("/** import macro") {
        let abs_idx = search_start + idx;
        let remaining = &source[abs_idx..];
        let Some(end_idx) = remaining.find("*/") else {
            break;
        };
        let block = &remaining[..end_idx];

        if let Some(open_brace) = block.find('{')
            && let Some(close_brace) = block.find('}')
            && close_brace > open_brace
            && let Some(from_idx) = block[close_brace..].find("from")
        {
            let names_src = &block[open_brace + 1..close_brace];
            let from_section = &block[close_brace + from_idx + "from".len()..];
            let module_src = from_section
                .split(['"', '\''])
                .nth(1)
                .map(str::trim)
                .unwrap_or("");

            if !module_src.is_empty() {
                for name in names_src.split(',') {
                    let trimmed = name.trim();
                    if !trimmed.is_empty() {
                        out.insert(trimmed.to_string(), module_src.to_string());
                    }
                }
            }
        }

        search_start = abs_idx + end_idx + 2;
    }

    out
}

fn collect_derive_targets(
    module: &Module,
    class_map: &HashMap<SpanKey, ClassIR>,
    interface_map: &HashMap<SpanKey, InterfaceIR>,
    source: &str,
) -> Vec<DeriveTarget> {
    let mut targets = Vec::new();

    // Build import source map
    let import_sources = collect_import_sources(module, source);

    for class_ir in class_map.values() {
        collect_from_class(class_ir, source, &import_sources, &mut targets);
    }

    for interface_ir in interface_map.values() {
        collect_from_interface(interface_ir, source, &import_sources, &mut targets);
    }

    targets
}

fn collect_from_class(
    class_ir: &ClassIR,
    source: &str,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<DeriveTarget>,
) {
    // Prefer decorators (legacy and comment-lowered)
    for decorator in &class_ir.decorators {
        if let Some(macro_names) = parse_derive_decorator(&decorator.args_src, import_sources) {
            if macro_names.is_empty() {
                continue;
            }

            out.push(DeriveTarget {
                macro_names,
                decorator_span: span_ir_with_at(decorator.span, source),
                target_ir: DeriveTargetIR::Class(class_ir.clone()),
            });
            return;
        }
    }

    // Fallback: detect leading /** @derive(...) */ comment directly
    if let Some((span, args_src)) = find_leading_derive_comment(source, class_ir.span.start)
        && let Some(macro_names) = parse_derive_decorator(&args_src, import_sources)
        && !macro_names.is_empty()
    {
        out.push(DeriveTarget {
            macro_names,
            decorator_span: span,
            target_ir: DeriveTargetIR::Class(class_ir.clone()),
        });
    }
}

fn collect_from_interface(
    interface_ir: &InterfaceIR,
    source: &str,
    import_sources: &HashMap<String, String>,
    out: &mut Vec<DeriveTarget>,
) {
    // Check decorators from JSDoc comments
    for decorator in &interface_ir.decorators {
        if let Some(macro_names) = parse_derive_decorator(&decorator.args_src, import_sources) {
            if macro_names.is_empty() {
                continue;
            }

            out.push(DeriveTarget {
                macro_names,
                decorator_span: span_ir_with_at(decorator.span, source),
                target_ir: DeriveTargetIR::Interface(interface_ir.clone()),
            });
            return;
        }
    }

    // Fallback: detect leading /** @derive(...) */ comment directly
    if let Some((span, args_src)) = find_leading_derive_comment(source, interface_ir.span.start)
        && let Some(macro_names) = parse_derive_decorator(&args_src, import_sources)
        && !macro_names.is_empty()
    {
        out.push(DeriveTarget {
            macro_names,
            decorator_span: span,
            target_ir: DeriveTargetIR::Interface(interface_ir.clone()),
        });
    }
}

/// Adjusts span to include the @ prefix if present (for actual decorators).
/// Used for deletion spans - keeps the whole JSDoc comment intact.
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

/// Find the span of a specific macro name within the decorator source.
/// For example, given `@derive(FieldController, Debug)` and macro_name "Debug",
/// returns the span of "Debug" within the source.
/// Input decorator_span is 1-based (SWC convention), output is also 1-based.
fn find_macro_name_span(source: &str, decorator_span: SpanIR, macro_name: &str) -> Option<SpanIR> {
    let start = decorator_span.start.saturating_sub(1) as usize;
    let end = decorator_span.end.saturating_sub(1) as usize;

    if start >= source.len() || end > source.len() {
        return None;
    }

    let decorator_source = &source[start..end];

    // Find the opening paren after @derive
    let paren_start = decorator_source.find('(')?;
    let args_slice = &decorator_source[paren_start + 1..];

    // Find the macro name within the arguments
    // We need to find the exact macro name, not a substring
    let mut search_start = 0;
    while let Some(pos) = args_slice[search_start..].find(macro_name) {
        let abs_pos = search_start + pos;

        // Check that this is a whole word match (not part of another identifier)
        let before_ok = abs_pos == 0
            || !args_slice
                .chars()
                .nth(abs_pos - 1)
                .map(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or(false);
        let after_ok = abs_pos + macro_name.len() >= args_slice.len()
            || !args_slice
                .chars()
                .nth(abs_pos + macro_name.len())
                .map(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or(false);

        if before_ok && after_ok {
            // Found the macro name - calculate the absolute position
            // Position in source: start + paren_start + 1 + abs_pos
            let macro_start = start + paren_start + 1 + abs_pos;
            let macro_end = macro_start + macro_name.len();
            // Return 1-based span
            return Some(SpanIR::new(macro_start as u32 + 1, macro_end as u32 + 1));
        }

        search_start = abs_pos + 1;
    }

    None
}

/// Returns a span pointing to @derive(...) for better diagnostic display.
/// If the span is a JSDoc comment, finds @derive inside it.
/// Note: Input is 1-based (SWC convention), but output is 0-based for JS API compatibility.
fn diagnostic_span_for_derive(span: SpanIR, source: &str) -> SpanIR {
    let start = span.start.saturating_sub(1) as usize; // Convert to 0-based for indexing
    let end = span.end.saturating_sub(1) as usize;

    if start >= source.len() {
        // Return 0-based span
        return SpanIR::new(span.start.saturating_sub(1), span.end.saturating_sub(1));
    }

    // Check if span starts with JSDoc comment "/**"
    if source[start..].starts_with("/**") {
        // Find @derive or @Derive within the comment
        let comment_slice = &source[start..end.min(source.len())];
        if let Some(at_pos) = comment_slice
            .find("@derive")
            .or_else(|| comment_slice.find("@Derive"))
        {
            // Find the closing paren
            if let Some(close_pos) = comment_slice[at_pos..].find(')') {
                // Return span pointing to @derive(...) in 0-based coordinates for JS API
                let derive_start = start + at_pos;
                let derive_end = derive_start + close_pos + 1;
                return SpanIR::new(derive_start as u32, derive_end as u32);
            }
        }
    }

    // For actual @ decorators, return 0-based span
    SpanIR::new(span.start.saturating_sub(1), span.end.saturating_sub(1))
}

fn parse_derive_decorator(
    args_src: &str,
    import_sources: &HashMap<String, String>,
) -> Option<Vec<(String, String)>> {
    let args = args_src.split(',');
    let mut macros = Vec::new();
    for arg in args {
        let name = arg.trim();
        if name.is_empty() {
            continue;
        }
        let name = name.trim_matches(|c| c == '"' || c == '\'').to_string();
        if name.is_empty() {
            continue;
        }
        // Look up the import source for this macro identifier
        // Default to @macro/derive if not found in imports
        let module_path = import_sources
            .get(&name)
            .cloned()
            .unwrap_or_else(|| DERIVE_MODULE_PATH.to_string());
        macros.push((name, module_path));
    }

    Some(macros)
}

fn find_leading_derive_comment(source: &str, class_start: u32) -> Option<(SpanIR, String)> {
    // SWC spans are 1-based, so class_start of N means byte index N-1 (0-based)
    let start = class_start.saturating_sub(1) as usize;
    if start == 0 || start > source.len() {
        return None;
    }

    let search_area = &source[..start];
    let comment_start_idx = search_area.rfind("/**")?;
    let rest = &search_area[comment_start_idx..];
    let end_rel = rest.find("*/")?;
    let comment_end_idx = comment_start_idx + end_rel + 2;

    // Find @derive within the comment for diagnostic span
    let at_derive_rel = rest.find("@derive").or_else(|| rest.find("@Derive"))?;
    let derive_start_idx = comment_start_idx + at_derive_rel;

    // Find the closing paren for the derive span end
    let derive_close_rel = rest[at_derive_rel..].find(')')?;
    let derive_end_idx = derive_start_idx + derive_close_rel + 1;

    let comment_body = &search_area[comment_start_idx + 3..comment_end_idx - 2];

    let content = comment_body.trim().trim_start_matches('*').trim();
    let content = content.strip_prefix('@')?;

    let open = content.find('(')?;
    let close = content.rfind(')')?;
    if close <= open {
        return None;
    }

    let name = content[..open].trim();
    if !name.eq_ignore_ascii_case("derive") {
        return None;
    }

    let args_src = content[open + 1..close].trim().to_string();
    // Return 1-based span pointing to @derive(...) for better diagnostics
    Some((
        SpanIR::new(derive_start_idx as u32 + 1, derive_end_idx as u32 + 1),
        args_src,
    ))
}

fn derive_insert_pos(class_ir: &ClassIR, source: &str) -> u32 {
    // class_ir.span.end is 1-based, so we can use it directly for exclusive slice end
    let end = class_ir.span.end as usize;
    let search = &source[..end.min(source.len())];
    // rfind returns 0-based index, add 1 to convert to 1-based for SpanIR
    search
        .rfind('}')
        .map(|idx| idx as u32 + 1)
        .unwrap_or_else(|| {
            // Fallback: use body_span.end (already 1-based)
            class_ir.body_span.end.max(class_ir.span.start)
        })
}

/// Splits macro output by location markers.
/// Default is "below" when no markers are present.
fn split_by_markers(source: &str) -> Vec<(&str, String)> {
    let markers = [
        ("above", "/* @macroforge:above */"),
        ("below", "/* @macroforge:below */"),
        ("body", "/* @macroforge:body */"),
        ("signature", "/* @macroforge:signature */"),
    ];

    // Find all occurrences
    let mut occurrences = Vec::new();
    for (name, pattern) in markers {
        for (idx, _) in source.match_indices(pattern) {
            occurrences.push((idx, pattern.len(), name));
        }
    }
    occurrences.sort_by_key(|k| k.0);

    if occurrences.is_empty() {
        return vec![("below", source.to_string())];
    }

    let mut chunks = Vec::new();

    // Handle text before first marker (default to below)
    if occurrences[0].0 > 0 {
        let text = &source[0..occurrences[0].0];
        if !text.trim().is_empty() {
            chunks.push(("below", text.to_string()));
        }
    }

    for i in 0..occurrences.len() {
        let (start, len, name) = occurrences[i];
        let content_start = start + len;
        let content_end = if i + 1 < occurrences.len() {
            occurrences[i + 1].0
        } else {
            source.len()
        };

        let content = &source[content_start..content_end];
        chunks.push((name, content.to_string()));
    }

    chunks
}

fn find_macro_comment_span(source: &str, target_start: u32) -> Option<SpanIR> {
    // SWC spans are 1-based, so target_start of 17 means byte index 16 (0-based)
    // We want to search in the source up to but NOT including the target
    let start = target_start.saturating_sub(1) as usize;
    if start == 0 || start > source.len() {
        return None;
    }
    let search_area = &source[..start];
    let start_idx = search_area.rfind("/**")?;
    let rest = &search_area[start_idx..];
    let end_rel = rest.find("*/")?;
    let end_idx = start_idx + end_rel + 2;

    // Only return the comment if it's directly adjacent to the target
    // (only whitespace between comment end and target start)
    let between = &search_area[end_idx..];
    if !between.trim().is_empty() {
        // There's non-whitespace content between the comment and target
        return None;
    }

    // Return 1-based span to match SWC conventions
    Some(SpanIR::new(start_idx as u32 + 1, end_idx as u32 + 1))
}

fn parse_members_from_tokens(tokens: &str) -> Result<Vec<swc_core::ecma::ast::ClassMember>> {
    let wrapped_stmt = format!("class __Temp {{ {} }}", tokens);
    if let Ok(swc_core::ecma::ast::Stmt::Decl(swc_core::ecma::ast::Decl::Class(class_decl))) =
        ts_syn::parse_ts_stmt(&wrapped_stmt)
    {
        return Ok(class_decl.class.body);
    }

    if let Ok(module) = ts_syn::parse_ts_module(&wrapped_stmt) {
        for item in module.body {
            if let swc_core::ecma::ast::ModuleItem::Stmt(swc_core::ecma::ast::Stmt::Decl(
                swc_core::ecma::ast::Decl::Class(class_decl),
            )) = item
            {
                return Ok(class_decl.class.body);
            }
        }
    }

    Err(anyhow::anyhow!(
        "Failed to parse macro output into class members"
    ))
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
export function __macroforgeRunDebug(ctxJson) {
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
