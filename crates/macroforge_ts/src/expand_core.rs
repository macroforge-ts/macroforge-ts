use anyhow::{Result, anyhow};
#[cfg(feature = "swc")]
use swc_core::{
    common::{FileName, SourceMap, errors::Handler, sync::Lrc},
    ecma::{
        ast::{EsVersion, Program},
        codegen::{Emitter, text_writer::JsWriter},
        parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
    },
};

#[cfg(feature = "oxc")]
use oxc::allocator::Allocator;
#[cfg(feature = "oxc")]
use oxc::codegen::Codegen as OxcCodegen;
#[cfg(feature = "oxc")]
use oxc::parser::Parser as OxcParser;
#[cfg(feature = "oxc")]
use oxc::span::SourceType;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

use crate::api_types::{
    ExpandOptions, ExpandResult, GeneratedRegionResult, MacroDiagnostic, MappingSegmentResult,
    SourceMappingResult, TransformResult,
};
use crate::host::CONFIG_CACHE;
use crate::host::MacroExpander;
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
use crate::host::expand::LoweredItems;
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;
#[cfg(feature = "swc")]
use crate::ts_syn::{Diagnostic, DiagnosticLevel};

// ============================================================================
// MacroExpander Cache
// ============================================================================

/// Cached config discovery result to avoid filesystem walks on every expand call.
/// On native, `MacroExpander::new()` calls `MacroConfig::find_with_root()` which
/// walks up directories looking for config files. Caching the result here means
/// the filesystem is only touched once.
#[cfg(not(target_arch = "wasm32"))]
static DISCOVERED_CONFIG: LazyLock<
    std::sync::Mutex<Option<(crate::host::config::MacroConfig, std::path::PathBuf)>>,
> = LazyLock::new(|| std::sync::Mutex::new(None));

/// Create a MacroExpander using cached config discovery.
fn create_expander() -> Result<MacroExpander> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::host::MacroConfig;
        let mut guard = DISCOVERED_CONFIG
            .lock()
            .map_err(|e| anyhow!("Lock poisoned: {e}"))?;
        let (config, root) = if let Some((c, r)) = guard.as_ref() {
            (c.clone(), r.clone())
        } else {
            let discovered = MacroConfig::find_with_root()
                .map_err(|e| anyhow!("Config discovery failed: {e}"))?
                .unwrap_or_else(|| {
                    (
                        MacroConfig::default(),
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                    )
                });
            *guard = Some(discovered.clone());
            discovered
        };
        MacroExpander::with_config(config, root)
            .map_err(|err| anyhow!("Failed to initialize macro host: {err:?}"))
    }
    #[cfg(target_arch = "wasm32")]
    {
        MacroExpander::new().map_err(|err| anyhow!("Failed to initialize macro host: {err:?}"))
    }
}

// ============================================================================
// Type Registry Cache
// ============================================================================

pub(crate) static REGISTRY_CACHE: LazyLock<dashmap::DashMap<u64, TypeRegistry>> =
    LazyLock::new(dashmap::DashMap::new);

pub(crate) fn get_or_parse_registry(json: &str) -> Option<TypeRegistry> {
    let mut hasher = DefaultHasher::new();
    json.hash(&mut hasher);
    let key = hasher.finish();

    if let Some(entry) = REGISTRY_CACHE.get(&key) {
        return Some(entry.clone());
    }

    match serde_json::from_str::<TypeRegistry>(json) {
        Ok(registry) => {
            REGISTRY_CACHE.insert(key, registry.clone());
            Some(registry)
        }
        Err(_) => None,
    }
}

// ============================================================================
// Backend Abstraction
// ============================================================================

pub(crate) trait CompilerBackend {
    fn expand(
        &self,
        code: &str,
        filepath: &str,
        options: &Option<ExpandOptions>,
    ) -> Result<ExpandResult>;
    fn transform(&self, code: &str, filepath: &str) -> Result<TransformResult>;
}

#[cfg(feature = "swc")]
pub(crate) struct SwcBackend;

#[cfg(feature = "swc")]
impl CompilerBackend for SwcBackend {
    fn expand(
        &self,
        code: &str,
        filepath: &str,
        options: &Option<ExpandOptions>,
    ) -> Result<ExpandResult> {
        let mut macro_host = create_expander()?;

        apply_options(&mut macro_host, options);

        let (program, _) = match parse_program(code, filepath) {
            Ok(p) => p,
            Err(e) => return Ok(make_syntax_error_result(code, &e.to_string())),
        };

        let expansion_result = macro_host.expand(code, &program, filepath);
        crate::host::import_registry::clear_registry();

        let expansion =
            expansion_result.map_err(|err| anyhow!("Macro expansion failed: {err:?}"))?;

        Ok(finalize_expansion(expansion))
    }

    fn transform(&self, code: &str, filepath: &str) -> Result<TransformResult> {
        let macro_host = create_expander()?;
        let (program, cm) = parse_program(code, filepath)?;

        let expansion = macro_host
            .expand(code, &program, filepath)
            .map_err(|err| anyhow!("Expansion failed: {err:?}"))?;

        handle_macro_diagnostics(&expansion.diagnostics, filepath).map_err(|e| anyhow!(e))?;

        let generated = if expansion.changed {
            expansion.code
        } else {
            emit_program(&program, &cm)?
        };

        Ok(TransformResult {
            code: generated,
            map: None,
            types: expansion.type_output,
            metadata: serialize_metadata(&expansion.classes),
        })
    }
}

#[cfg(feature = "oxc")]
pub(crate) struct OxcBackend;

/// Execute the `@buildtime` pre-pass against `code`.
///
/// Returns:
/// - `Some(rewritten)` if any `@buildtime` declaration produced a
///   successful patch — the downstream declarative/derive passes will
///   see the rewritten source.
/// - `None` otherwise (no markers, all failed, or no patches).
/// - The dependency list (absolute paths read during evaluation).
/// - Diagnostics for any evaluation failures (throw/timeout/unauthorized/etc.).
///
/// Fast-paths the common case where the source contains no `@buildtime`
/// marker: a single text-contains check, and the function returns
/// without parsing or constructing a sandbox.
/// Resolve the `MacroforgeConfig` the attribute pre-pass should use.
///
/// If `options.config_path` points at a config already parsed into
/// [`CONFIG_CACHE`], return that. Otherwise fall back to defaults — the
/// attribute pre-pass is still invoked so annotations get validated, but no
/// project-level customisation applies.
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn resolve_config_for_attributes(
    options: &Option<ExpandOptions>,
) -> macroforge_ts_syn::config::MacroforgeConfig {
    if let Some(opts) = options.as_ref()
        && let Some(path) = opts.config_path.as_ref()
        && let Some(entry) = CONFIG_CACHE.get(path)
    {
        return entry.clone();
    }
    macroforge_ts_syn::config::MacroforgeConfig::default()
}

/// Run the attribute-macro pre-pass (`@cfg`, `@deprecated`, `@mustUse`,
/// `@nonExhaustive`) against `code`. Fast-paths when none of the tag heads
/// appears in the source.
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn run_attributes_prepass_oxc(
    code: &str,
    filepath: &str,
    source_type: oxc::span::SourceType,
    config: &macroforge_ts_syn::config::MacroforgeConfig,
) -> Result<(Option<String>, Vec<crate::ts_syn::Diagnostic>)> {
    let has_any_tag = ["@cfg", "@deprecated", "@mustUse", "@nonExhaustive"]
        .iter()
        .any(|t| code.contains(t));
    if !has_any_tag {
        return Ok((None, Vec::new()));
    }
    let allocator = Allocator::default();
    let ret = OxcParser::new(&allocator, code, source_type).parse();
    if !ret.errors.is_empty() {
        return Err(anyhow!(
            "Oxc parse errors (attributes pre-pass): {:?}",
            ret.errors
        ));
    }
    let origin_path = std::path::PathBuf::from(filepath);
    let out = crate::host::attributes::run_prepass(&ret.program, code, &origin_path, config);
    Ok((out.rewritten, out.diagnostics))
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn run_buildtime_prepass_oxc(
    code: &str,
    filepath: &str,
    source_type: oxc::span::SourceType,
) -> Result<(
    Option<String>,
    Vec<std::path::PathBuf>,
    Vec<crate::ts_syn::Diagnostic>,
)> {
    if !code.contains("@buildtime") {
        return Ok((None, Vec::new(), Vec::new()));
    }
    let Some(sandbox) = crate::host::buildtime::default_backend() else {
        return Ok((None, Vec::new(), Vec::new()));
    };
    let allocator = Allocator::default();
    let ret = OxcParser::new(&allocator, code, source_type).parse();
    if !ret.errors.is_empty() {
        return Err(anyhow!(
            "Oxc parse errors (buildtime pre-pass): {:?}",
            ret.errors
        ));
    }

    let origin_path = std::path::PathBuf::from(filepath);
    let mut options = crate::host::buildtime::SandboxOptions::new(origin_path.clone());
    // PR 4 ships with a permissive default capability set so built-in
    // tests and simple user code (arithmetic, crypto, string ops) work
    // out of the box. PR 5 will thread per-project config from
    // `macroforge.config.js` through `ExpandOptions.config_path`.
    options.capabilities = crate::host::buildtime::CapabilitySet {
        fs_read: vec![crate::host::buildtime::PathPattern::new("**").expect("valid glob")],
        fs_write: vec![],
        env_allow: vec![],
        network: false,
    };

    let out = crate::host::buildtime::run_prepass(
        &ret.program,
        code,
        &origin_path,
        sandbox.as_ref(),
        &options,
    );
    Ok((out.rewritten, out.dependencies, out.diagnostics))
}

#[cfg(feature = "oxc")]
impl CompilerBackend for OxcBackend {
    fn expand(
        &self,
        code: &str,
        filepath: &str,
        options: &Option<ExpandOptions>,
    ) -> Result<ExpandResult> {
        let source_type = SourceType::ts().with_jsx(filepath.ends_with(".tsx"));

        // --- Attribute pre-pass (OXC-only path) ---
        //
        // Runs BEFORE the buildtime pre-pass so `@cfg`-stripped declarations
        // don't get evaluated (and `@buildtime` expressions that would be
        // dead under the current build flags are simply never seen). The
        // pre-pass also rewrites `@deprecated(...)` to the tsc-readable
        // JSDoc form, brands `@nonExhaustive` type aliases, and collects
        // `@mustUse` diagnostics against discarded call sites.
        //
        // Fast-paths when none of `@cfg / @deprecated / @mustUse /
        // @nonExhaustive` appears in the source.
        #[cfg(not(feature = "swc"))]
        let attribute_config = resolve_config_for_attributes(options);
        #[cfg(not(feature = "swc"))]
        let (attribute_rewritten, attribute_diagnostics) =
            run_attributes_prepass_oxc(code, filepath, source_type, &attribute_config)?;
        #[cfg(not(feature = "swc"))]
        let code_after_attributes: &str = attribute_rewritten.as_deref().unwrap_or(code);

        // --- Buildtime pre-pass (OXC-only path) ---
        //
        // Runs BEFORE the declarative macro pre-pass so @buildtime
        // declarations are fully evaluated and spliced as literals by
        // the time declarative macros + derives see the source. This
        // matches the Zig-comptime semantics from plans/buildtime.md:
        // one-shot evaluation, no fixpoint, no reverse influence.
        //
        // If the source has no `@buildtime` markers, this block is a
        // text-contains check plus the initial parse — cheap.
        #[cfg(not(feature = "swc"))]
        let (buildtime_rewritten, buildtime_deps, buildtime_diagnostics) =
            run_buildtime_prepass_oxc(code_after_attributes, filepath, source_type)?;
        #[cfg(not(feature = "swc"))]
        let code_after_buildtime: &str = buildtime_rewritten
            .as_deref()
            .unwrap_or(code_after_attributes);

        // --- Declarative macro pre-pass (OXC-only path) ---
        //
        // Run an early parse + discover + rewrite pass in its own allocator
        // scope. If any declarative macros produced patches, `rewritten`
        // holds the new source; otherwise it's `None` and we use the input
        // unchanged. Diagnostics from the pre-pass are merged into the
        // final result below.
        //
        // Input to this block is `code_after_buildtime` (the output of
        // the buildtime pre-pass), so declarative macros see buildtime
        // constants as plain TS literals.
        #[cfg(not(feature = "swc"))]
        let (rewritten, decl_diagnostics) = {
            use std::path::PathBuf;

            let code = code_after_buildtime;
            let allocator = Allocator::default();
            let ret = OxcParser::new(&allocator, code, source_type).parse();
            if !ret.errors.is_empty() {
                return Err(anyhow!("Oxc parse errors: {:?}", ret.errors));
            }
            let discovered = crate::host::declarative::discover(&ret.program, code)
                .map_err(|e| anyhow!("Declarative macro error: {}", e))?;

            // Load the project-wide declarative registry (if provided via
            // ExpandOptions) and resolve cross-file imports against it.
            let project_registry = options
                .as_ref()
                .and_then(|o| o.declarative_registry_json.as_ref())
                .and_then(|json| {
                    crate::host::declarative::ProjectDeclarativeRegistry::from_json(json).ok()
                });
            let file_path = PathBuf::from(filepath);
            let resolved_imports = if let Some(ref pr) = project_registry {
                crate::host::declarative::resolve_cross_file_imports(code, &file_path, pr)
            } else {
                crate::host::declarative::ResolvedImports::default()
            };

            // Check for `$identifier` patterns that might be proc macro calls.
            let has_dollar_calls = code.contains('$') && {
                let bytes = code.as_bytes();
                bytes
                    .windows(2)
                    .any(|w| w[0] == b'$' && w[1].is_ascii_alphabetic())
            };
            if discovered.is_empty() && resolved_imports.imported.is_empty() && !has_dollar_calls {
                (None::<String>, resolved_imports.diagnostics)
            } else {
                let mut registry = crate::host::declarative::DeclarativeMacroRegistry::new();
                for dm in &discovered {
                    registry
                        .register(dm.def.clone())
                        .map_err(|e| anyhow!("Declarative macro error: {}", e))?;
                }
                for imported in &resolved_imports.imported {
                    registry.register(imported.def.clone()).map_err(|e| {
                        anyhow!(
                            "Declarative macro error (imported from {}): {}",
                            imported.source_file.display(),
                            e
                        )
                    })?;
                }
                let build_mode = crate::host::declarative::BuildMode::from_option(
                    options.as_ref().and_then(|o| o.build_mode.as_deref()),
                );
                // Phase 14: parse and thread the project-wide type
                // registry into the declarative rewriter so the
                // megamorphism analyzer can attach structural
                // fingerprints to each call site's argument shape.
                let type_registry = options
                    .as_ref()
                    .and_then(|o| o.type_registry_json.as_ref())
                    .and_then(|json| get_or_parse_registry(json));
                // Create an expander early so its dispatcher and external
                // loader are available for proc macro call fallback during
                // the declarative rewrite pass.
                let early_expander = create_expander()?;

                let macro_import_sources =
                    crate::ts_syn::import_registry::collect_macro_import_comments_pub(code);

                let proc_fallback = crate::host::declarative::ProcMacroFallback {
                    dispatcher: &early_expander.dispatcher,
                    import_sources: &macro_import_sources,
                    external_loader: early_expander.external_loader_ref(),
                };

                let rewrite_out = crate::host::declarative::rewrite(
                    &ret.program,
                    code,
                    &registry,
                    &discovered,
                    build_mode,
                    type_registry.as_ref(),
                    Some(proc_fallback),
                );
                let mut diagnostics = resolved_imports.diagnostics;
                diagnostics.extend(rewrite_out.diagnostics);
                let source = if rewrite_out.patches.is_empty() {
                    None
                } else {
                    let applicator = crate::host::patch_applicator::PatchApplicator::new(
                        code,
                        rewrite_out.patches,
                    );
                    Some(
                        applicator
                            .apply()
                            .map_err(|e| anyhow!("Patch apply failed: {}", e))?,
                    )
                };
                (source, diagnostics)
            }
        };

        // After the pre-pass block, the first allocator + ret are dropped.
        // `code` for the rest of the function layers both pre-pass
        // rewrites: the buildtime result (if any) is the fallback when
        // the declarative pass produced no further changes.
        #[cfg(not(feature = "swc"))]
        let code_owned = rewritten;
        #[cfg(not(feature = "swc"))]
        let code: &str = code_owned.as_deref().unwrap_or(code_after_buildtime);
        #[cfg(not(feature = "swc"))]
        let changed_by_decl = code_owned.is_some();

        let allocator = Allocator::default();
        let ret = OxcParser::new(&allocator, code, source_type).parse();

        if !ret.errors.is_empty() {
            #[cfg(not(feature = "swc"))]
            let ctx = if changed_by_decl {
                "Oxc parse errors after declarative macro expansion"
            } else {
                "Oxc parse errors"
            };
            #[cfg(feature = "swc")]
            let ctx = "Oxc parse errors";
            return Err(anyhow!("{}: {:?}", ctx, ret.errors));
        }

        #[cfg(feature = "swc")]
        {
            SwcBackend.expand(code, filepath, options)
        }
        #[cfg(not(feature = "swc"))]
        {
            let mut macro_host = create_expander()?;
            apply_options(&mut macro_host, options);

            let classes = crate::ts_syn::lower_classes_oxc(&ret.program, code, None)?;
            let interfaces = crate::ts_syn::lower_interfaces_oxc(&ret.program, code, None)?;
            let enums = crate::ts_syn::lower_enums_oxc(&ret.program, code, None)?;
            let type_aliases = crate::ts_syn::lower_type_aliases_oxc(&ret.program, code, None)?;
            let imports = crate::ts_syn::ImportRegistry::from_oxc_program(&ret.program, code);

            let functions = crate::ts_syn::lower_functions_oxc(&ret.program, code, None)?;

            let items = LoweredItems {
                classes,
                interfaces,
                enums,
                type_aliases,
                functions,
                imports,
            };

            if items.is_empty() {
                // No derive targets, but we might still have attribute,
                // buildtime, or declarative rewrites / diagnostics to surface.
                let changed_by_buildtime = buildtime_rewritten.is_some();
                let changed_by_attributes = attribute_rewritten.is_some();
                let has_work = changed_by_buildtime
                    || changed_by_attributes
                    || !buildtime_diagnostics.is_empty()
                    || !attribute_diagnostics.is_empty();
                if !changed_by_decl && decl_diagnostics.is_empty() && !has_work {
                    return Ok(ExpandResult::unchanged(code));
                }
                let mut diagnostics: Vec<MacroDiagnostic> = attribute_diagnostics
                    .iter()
                    .map(|d| MacroDiagnostic {
                        level: format!("{:?}", d.level).to_lowercase(),
                        message: d.message.clone(),
                        start: d.span.map(|s| s.start),
                        end: d.span.map(|s| s.end),
                    })
                    .collect();
                diagnostics.extend(buildtime_diagnostics.iter().map(|d| MacroDiagnostic {
                    level: format!("{:?}", d.level).to_lowercase(),
                    message: d.message.clone(),
                    start: d.span.map(|s| s.start),
                    end: d.span.map(|s| s.end),
                }));
                diagnostics.extend(decl_diagnostics.iter().map(|d| MacroDiagnostic {
                    level: format!("{:?}", d.level).to_lowercase(),
                    message: d.message.clone(),
                    start: d.span.map(|s| s.start),
                    end: d.span.map(|s| s.end),
                }));
                // The returned `code` is the last successful rewrite: buildtime
                // beats attributes beats the original. (Derive work would
                // further layer on top, but we're inside the no-derive branch.)
                let code_for_result = if let Some(ref c) = buildtime_rewritten {
                    c.clone()
                } else if let Some(ref c) = attribute_rewritten {
                    c.clone()
                } else {
                    code.to_string()
                };
                let mut result = ExpandResult {
                    code: code_for_result,
                    types: None,
                    metadata: None,
                    diagnostics,
                    source_mapping: None,
                    buildtime_dependencies: buildtime_deps
                        .iter()
                        .map(|p| p.to_string_lossy().into_owned())
                        .collect(),
                };
                inject_log_comments(&mut result);
                return Ok(result);
            }

            let items_clone = items.clone();
            let (mut collector, mut diagnostics) =
                macro_host.collect_macro_patches_oxc(items, filepath, code);

            let expansion = macro_host
                .apply_and_finalize_expansion(code, &mut collector, &mut diagnostics, items_clone)
                .map_err(anyhow::Error::from)?;

            let mut result_diagnostics: Vec<MacroDiagnostic> = attribute_diagnostics
                .iter()
                .map(|d| MacroDiagnostic {
                    level: format!("{:?}", d.level).to_lowercase(),
                    message: d.message.clone(),
                    start: d.span.map(|s| s.start),
                    end: d.span.map(|s| s.end),
                })
                .collect();
            result_diagnostics.extend(buildtime_diagnostics.iter().map(|d| MacroDiagnostic {
                level: format!("{:?}", d.level).to_lowercase(),
                message: d.message.clone(),
                start: d.span.map(|s| s.start),
                end: d.span.map(|s| s.end),
            }));
            result_diagnostics.extend(expansion.diagnostics.into_iter().map(|d| MacroDiagnostic {
                level: format!("{:?}", d.level).to_lowercase(),
                message: d.message,
                start: d.span.map(|s| s.start),
                end: d.span.map(|s| s.end),
            }));
            // Merge declarative-pass diagnostics.
            for d in &decl_diagnostics {
                result_diagnostics.push(MacroDiagnostic {
                    level: format!("{:?}", d.level).to_lowercase(),
                    message: d.message.clone(),
                    start: d.span.map(|s| s.start),
                    end: d.span.map(|s| s.end),
                });
            }

            let mut result = ExpandResult {
                code: expansion.code,
                types: expansion.type_output,
                metadata: serialize_metadata(&expansion.classes),
                diagnostics: result_diagnostics,
                source_mapping: expansion.source_mapping.map(|mapping| SourceMappingResult {
                    segments: mapping
                        .segments
                        .into_iter()
                        .map(|segment| MappingSegmentResult {
                            original_start: segment.original_start,
                            original_end: segment.original_end,
                            expanded_start: segment.expanded_start,
                            expanded_end: segment.expanded_end,
                        })
                        .collect(),
                    generated_regions: mapping
                        .generated_regions
                        .into_iter()
                        .map(|region| GeneratedRegionResult {
                            start: region.start,
                            end: region.end,
                            source_macro: region.source_macro,
                        })
                        .collect(),
                }),
                buildtime_dependencies: buildtime_deps
                    .iter()
                    .map(|p| p.to_string_lossy().into_owned())
                    .collect(),
            };
            inject_log_comments(&mut result);
            Ok(result)
        }
    }

    fn transform(&self, code: &str, filepath: &str) -> Result<TransformResult> {
        let allocator = Allocator::default();
        let source_type = SourceType::ts().with_jsx(filepath.ends_with(".tsx"));

        let ret = OxcParser::new(&allocator, code, source_type).parse();

        if !ret.errors.is_empty() {
            return Err(anyhow!("Oxc parse errors: {:?}", ret.errors));
        }

        let generated = OxcCodegen::new().build(&ret.program).code;

        Ok(TransformResult {
            code: generated,
            map: None,
            types: None,
            metadata: None,
        })
    }
}

fn apply_options(macro_host: &mut MacroExpander, options: &Option<ExpandOptions>) {
    if let Some(opts) = options {
        if let Some(keep) = opts.keep_decorators {
            macro_host.set_keep_decorators(keep);
        }
        if let Some(modules) = &opts.external_decorator_modules {
            macro_host.set_external_decorator_modules(modules.clone());
        }
        if let Some(json) = &opts.type_registry_json {
            let registry = get_or_parse_registry(json);
            if let Some(ref reg) = registry {
                eprintln!(
                    "[macroforge:expand] Found type registry with {} types",
                    reg.len()
                );
            } else {
                eprintln!("[macroforge:expand] Failed to parse type registry JSON");
            }
            macro_host.set_type_registry(registry);
        }
        // Note: `declarative_registry_json` is read directly by the
        // declarative pre-pass in OxcBackend::expand without going through
        // macro_host, because that pre-pass runs before macro_host is
        // constructed. The CLI path (`MacroExpander::expand_source`) reads
        // the registry from `self.declarative_registry` instead and its
        // caller must call `set_declarative_registry` explicitly.
        if let Some(path) = opts.config_path.as_ref() {
            if let Some(config) = CONFIG_CACHE.get(path) {
                eprintln!("[macroforge:expand] Applying config from cache: {}", path);
                crate::host::import_registry::set_foreign_types(config.foreign_types.clone());
                crate::host::import_registry::with_registry_mut(|r| {
                    r.config_imports = config
                        .config_imports
                        .iter()
                        .map(|(name, info)| (name.clone(), info.source.clone()))
                        .collect();
                });
            } else {
                eprintln!(
                    "[macroforge:expand] Config path provided but NOT FOUND in cache: {}",
                    path
                );
            }
        }
    }
}

#[cfg(feature = "swc")]
fn make_syntax_error_result(code: &str, error_msg: &str) -> ExpandResult {
    crate::host::import_registry::clear_registry();
    crate::host::import_registry::clear_foreign_types();
    ExpandResult {
        code: code.to_string(),
        types: None,
        metadata: None,
        diagnostics: vec![MacroDiagnostic {
            level: "info".to_string(),
            message: format!("Macro expansion skipped due to syntax error: {}", error_msg),
            start: None,
            end: None,
        }],
        source_mapping: None,
        buildtime_dependencies: vec![],
    }
}

fn serialize_metadata(classes: &Vec<crate::ts_syn::abi::ir::ClassIR>) -> Option<String> {
    if classes.is_empty() {
        None
    } else {
        serde_json::to_string(classes).ok()
    }
}

#[cfg(feature = "swc")]
fn finalize_expansion(expansion: crate::host::expand::MacroExpansion) -> ExpandResult {
    let diagnostics = expansion
        .diagnostics
        .into_iter()
        .map(|d| MacroDiagnostic {
            level: format!("{:?}", d.level).to_lowercase(),
            message: d.message,
            start: d.span.map(|s| s.start),
            end: d.span.map(|s| s.end),
        })
        .collect();

    let source_mapping = expansion.source_mapping.map(|mapping| SourceMappingResult {
        segments: mapping
            .segments
            .into_iter()
            .map(|seg| MappingSegmentResult {
                original_start: seg.original_start,
                original_end: seg.original_end,
                expanded_start: seg.expanded_start,
                expanded_end: seg.expanded_end,
            })
            .collect(),
        generated_regions: mapping
            .generated_regions
            .into_iter()
            .map(|region| GeneratedRegionResult {
                start: region.start,
                end: region.end,
                source_macro: region.source_macro,
            })
            .collect(),
    });

    let mut types_output = expansion.type_output;
    if let Some(types) = &mut types_output
        && expansion.code.contains("toJSON(")
        && !types.contains("toJSON(")
        && let Some(insert_at) = types.rfind('}')
    {
        let types: &mut String = types;
        types.insert_str(insert_at, "  toJSON(): Record<string, unknown>;\n");
    }

    let mut result = ExpandResult {
        code: expansion.code,
        types: types_output,
        metadata: serialize_metadata(&expansion.classes),
        diagnostics,
        source_mapping,
        buildtime_dependencies: vec![],
    };
    inject_log_comments(&mut result);
    result
}

// ============================================================================
// Public Interface
// ============================================================================

pub(crate) fn get_backend() -> Box<dyn CompilerBackend> {
    #[cfg(feature = "oxc")]
    {
        Box::new(OxcBackend)
    }
    #[cfg(all(not(feature = "oxc"), feature = "swc"))]
    {
        Box::new(SwcBackend)
    }
}

pub(crate) fn expand_inner(
    code: &str,
    filepath: &str,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    if !has_macro_annotations(code) {
        return Ok(ExpandResult::unchanged(code));
    }

    get_backend().expand(code, filepath, &options)
}

pub(crate) fn transform_inner(code: &str, filepath: &str) -> Result<TransformResult> {
    get_backend().transform(code, filepath)
}

// ============================================================================
// Log Level Support
// ============================================================================

/// Log levels for MF_LOG env var, ordered by verbosity.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

fn parse_log_level() -> LogLevel {
    match std::env::var("MF_LOG").ok().as_deref() {
        Some("error") => LogLevel::Error,
        Some("warn") => LogLevel::Warn,
        Some("info") => LogLevel::Info,
        Some("debug") => LogLevel::Debug,
        Some("trace") => LogLevel::Trace,
        Some("1" | "true") => LogLevel::Info,
        _ => LogLevel::Off,
    }
}

/// Inject trace/debug diagnostics as comments into expanded code.
/// Diagnostics with spans are inserted above the relevant line;
/// those without spans go into a block comment at the top.
fn inject_log_comments(result: &mut ExpandResult) {
    let level = parse_log_level();
    if level == LogLevel::Off {
        return;
    }

    let trace_diags: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| {
            d.message.starts_with("[trace]") && level >= LogLevel::Trace
                || d.level == "error" && level >= LogLevel::Error
                || d.level == "warning" && level >= LogLevel::Warn
                || d.level == "info" && level >= LogLevel::Info
        })
        .collect();

    if trace_diags.is_empty() {
        return;
    }

    // Separate positioned vs unpositioned
    let mut positioned: Vec<(u32, &str)> = Vec::new();
    let mut top_lines: Vec<String> = Vec::new();

    for d in &trace_diags {
        if let Some(start) = d.start {
            positioned.push((start, &d.message));
        } else {
            top_lines.push(format!("// {}", d.message));
        }
    }

    // Build the top block
    let mut header = String::new();
    if !top_lines.is_empty() {
        header.push_str("/*\n * MF_LOG output\n");
        for line in &top_lines {
            header.push_str(" * ");
            header.push_str(line.trim_start_matches("// "));
            header.push('\n');
        }
        header.push_str(" */\n");
    }

    // Insert positioned comments (process in reverse order to preserve offsets)
    let mut code = result.code.clone();
    positioned.sort_by(|a, b| b.0.cmp(&a.0));
    for (offset, msg) in &positioned {
        let offset = *offset as usize;
        if offset <= code.len() {
            // Find the start of the line containing this offset
            let line_start = code[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let indent = &code[line_start..offset]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let comment = format!("{}// {}\n", indent, msg);
            code.insert_str(line_start, &comment);
        }
    }

    if !header.is_empty() {
        code.insert_str(0, &header);
    }

    result.code = code;
}

// ============================================================================
// Inner Logic (Optimized)
// ============================================================================

/// Check if source code contains `@derive(` as a standalone JSDoc directive,
/// or imports the declarative macro module (`"macroforge/rules"`), or uses
/// a `/** import macro { $name } from "..." */` JSDoc comment for
/// cross-file declarative macro imports.
pub(crate) fn has_macro_annotations(source: &str) -> bool {
    // Buildtime pre-pass: fires on `/** @buildtime */` JSDoc markers.
    // Must be checked before the @derive fast-path below — a file can
    // have @buildtime without any @derive.
    if source.contains("@buildtime") {
        return true;
    }
    // Declarative macros: the defining file imports `macroRules` from the
    // rules module; consuming files use a JSDoc `/** import macro */`
    // comment. Either signal means the pre-pass must run.
    if source.contains("macroforge/rules") {
        return true;
    }
    if source.contains("import macro") {
        return true;
    }
    // Attribute-macro pre-pass tags. Cheap text check first; full
    // line-start parsing only fires when one of these substrings is present.
    let has_attribute_tag = source.contains("@cfg")
        || source.contains("@deprecated")
        || source.contains("@mustUse")
        || source.contains("@nonExhaustive");
    if !source.contains("@derive") && !has_attribute_tag {
        return false;
    }
    let mut in_code_block = false;
    for line in source.lines() {
        let trimmed = line
            .trim()
            .trim_start_matches('/')
            .trim_start_matches('*')
            .trim_end_matches('/')
            .trim_end_matches('*')
            .trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }
        if trimmed.starts_with("@derive(")
            || trimmed.starts_with("@cfg(")
            || trimmed.starts_with("@deprecated")
            || trimmed.starts_with("@mustUse")
            || trimmed.starts_with("@nonExhaustive")
        {
            return true;
        }
    }
    false
}

#[cfg(feature = "swc")]
pub(crate) fn parse_program(code: &str, filepath: &str) -> Result<(Program, Lrc<SourceMap>)> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom(filepath.to_string()).into(),
        code.to_string(),
    );
    let handler =
        Handler::with_emitter_writer(Box::new(std::io::Cursor::new(Vec::new())), Some(cm.clone()));

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: filepath.ends_with(".tsx"),
            decorators: true,
            no_early_errors: true,
            ..Default::default()
        }),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    match parser.parse_program() {
        Ok(program) => Ok((program, cm)),
        Err(error) => {
            let msg = format!("Failed to parse TypeScript: {:?}", error);
            error.into_diagnostic(&handler).emit();
            Err(anyhow!(msg))
        }
    }
}

#[cfg(feature = "swc")]
pub(crate) fn emit_program(program: &Program, cm: &Lrc<SourceMap>) -> Result<String> {
    let mut buf = vec![];
    let mut emitter = Emitter {
        cfg: swc_core::ecma::codegen::Config::default(),
        cm: cm.clone(),
        comments: None,
        wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
    };
    emitter
        .emit_program(program)
        .map_err(|e| anyhow!("{:?}", e))?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

#[cfg(feature = "swc")]
pub(crate) fn handle_macro_diagnostics(diags: &[Diagnostic], file: &str) -> Result<(), String> {
    for diag in diags {
        if matches!(diag.level, DiagnosticLevel::Error) {
            let loc = diag
                .span
                .map(|s| format!("{}:{}-{}", file, s.start, s.end))
                .unwrap_or_else(|| file.to_string());
            return Err(format!("Macro error at {}: {}", loc, diag.message));
        }
    }
    Ok(())
}
