use crate::NativePositionMapper;
use crate::api::CoreEngine;
use crate::api_types::{
    ExpandOptions, ExpandResult, ProcessFileOptions, ScanOptions, SourceMappingResult,
    SyntaxCheckResult,
};
use crate::manifest::{
    debug_descriptors, debug_get_modules, debug_lookup, get_macro_manifest, get_macro_names,
    is_macro_package,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(line: &str);
}

/// Runs `entry`, then prints to the JavaScript console what the engine and
/// the macros it ran logged meanwhile: wasm has no debug log file to write.
fn with_debug_flush<T>(entry: impl FnOnce() -> T) -> T {
    let value = entry();
    for line in crate::debug::take_pending() {
        console_error(&line);
    }
    value
}

/// Whether `code`, the source of `filepath`, may contain anything the engine
/// expands. Integrations use this to skip files without paying for a full
/// expansion; in a Svelte module, the compiler's runes do not count.
#[wasm_bindgen(js_name = "hasMacroAnnotations")]
pub fn has_macro_annotations(code: &str, filepath: &str) -> bool {
    crate::has_macro_annotations(code, filepath)
}

/// The macros `code` imports through `import macro` JSDoc comments, as macro
/// name to module.
#[wasm_bindgen(
    js_name = "macroImports",
    unchecked_return_type = "Record<string, string>"
)]
pub fn macro_imports(code: &str, filepath: &str) -> Result<JsValue, JsValue> {
    let imports = crate::macro_imports(code, filepath);
    imports
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(JsValue::from)
}

/// Whether `code` parses as TypeScript, with the parse error when it does not.
#[wasm_bindgen(js_name = "checkSyntax")]
pub fn check_syntax(code: String, filepath: String) -> Result<JsValue, JsValue> {
    let result = CoreEngine::check_syntax(&code, &filepath).unwrap_or_else(|e| SyntaxCheckResult {
        ok: false,
        error: Some(e),
    });
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

/// The import declarations of `code`: each imported name with the module it comes from.
#[wasm_bindgen(js_name = "parseImportSources")]
pub fn parse_import_sources(code: String, filepath: String) -> Result<JsValue, JsValue> {
    let result =
        CoreEngine::parse_import_sources(&code, &filepath).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

/// Names `@derive` so it can be imported. It does nothing at runtime.
#[wasm_bindgen(js_name = "Derive")]
pub fn derive_decorator() {}

/// Parses a `macroforge.config.*` file's `content` and caches it under `filepath`.
#[wasm_bindgen(js_name = "loadConfig")]
pub fn load_config(content: String, filepath: String) -> Result<JsValue, JsValue> {
    let result = CoreEngine::load_config(&content, &filepath).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

/// Forgets every config `loadConfig` cached.
#[wasm_bindgen(js_name = "clearConfigCache")]
pub fn clear_config_cache() {
    CoreEngine::clear_config_cache();
}

/// Parses a type registry and keeps it for the process. Pass the returned id
/// as `typeRegistryId` to expand against it without sending its JSON again.
#[wasm_bindgen(js_name = "setTypeRegistry")]
pub fn set_type_registry(json: &str) -> Result<u32, JsValue> {
    CoreEngine::set_type_registry(json).map_err(|e| JsValue::from_str(&e))
}

/// Parses a declarative registry and keeps it for the process. Pass the
/// returned id as `declarativeRegistryId`.
#[wasm_bindgen(js_name = "setDeclarativeRegistry")]
pub fn set_declarative_registry(json: &str) -> Result<u32, JsValue> {
    CoreEngine::set_declarative_registry(json).map_err(|e| JsValue::from_str(&e))
}

/// Forgets a registry `setTypeRegistry` or `setDeclarativeRegistry` kept.
#[wasm_bindgen(js_name = "releaseRegistry")]
pub fn release_registry(id: u32) -> Result<(), JsValue> {
    CoreEngine::release_registry(id).map_err(|e| JsValue::from_str(&e))
}

/// Expands the macros in `code`, the source of `filepath`, and returns the
/// expanded code, its type declarations, diagnostics and source mapping.
#[wasm_bindgen(js_name = "expandSync")]
pub fn expand_sync(code: String, filepath: String, options: JsValue) -> Result<JsValue, JsValue> {
    with_debug_flush(|| {
        let opts: Option<ExpandOptions> = if options.is_null() || options.is_undefined() {
            None
        } else {
            Some(serde_wasm_bindgen::from_value(options)?)
        };

        let result =
            CoreEngine::expand_sync(code, filepath, opts).map_err(|e| JsValue::from_str(&e))?;
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
    })
}

/// A stateful expander for editor integrations: it caches each file's
/// expansion by version and maps positions and diagnostics back to the source.
#[derive(Default)]
#[wasm_bindgen]
pub struct NativePlugin {
    cache: std::sync::Mutex<ExpansionCache>,
}

/// How many files' expansions a plugin keeps. An editor works in a few files
/// at a time; the bound keeps a long session from holding every file it
/// ever opened.
const CACHED_FILES: usize = 1024;

/// Each file's last expansion, with its position mapper built once.
#[derive(Default)]
struct ExpansionCache {
    entries: std::collections::HashMap<String, CachedResult>,
    /// Advances on every use, so the least recently used entry is evicted.
    clock: u64,
}

struct CachedResult {
    version: String,
    result: ExpandResult,
    mapper: Option<std::sync::Arc<NativePositionMapper>>,
    last_used: u64,
}

impl ExpansionCache {
    fn tick(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }

    fn get(&mut self, filepath: &str) -> Option<&CachedResult> {
        let now = self.tick();
        let entry = self.entries.get_mut(filepath)?;
        entry.last_used = now;
        Some(entry)
    }

    fn insert(&mut self, filepath: String, version: String, result: ExpandResult) {
        if self.entries.len() >= CACHED_FILES
            && !self.entries.contains_key(&filepath)
            && let Some(oldest) = self
                .entries
                .iter()
                .min_by_key(|(_, entry)| entry.last_used)
                .map(|(path, _)| path.clone())
        {
            self.entries.remove(&oldest);
        }
        let mapper = result
            .source_mapping
            .clone()
            .map(|mapping| std::sync::Arc::new(NativePositionMapper::new(mapping)));
        let last_used = self.tick();
        self.entries.insert(
            filepath,
            CachedResult {
                version,
                result,
                mapper,
                last_used,
            },
        );
    }
}

#[wasm_bindgen]
impl NativePlugin {
    /// Creates a plugin with an empty cache.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    fn cache(&self) -> Result<std::sync::MutexGuard<'_, ExpansionCache>, JsValue> {
        self.cache
            .lock()
            .map_err(|err| JsValue::from_str(&format!("expansion cache lock poisoned: {err}")))
    }

    /// Expands the macros in `code`, uncached; see the module-level `expandSync`.
    #[wasm_bindgen(js_name = "expandSync")]
    pub fn expand_sync(
        &self,
        code: String,
        filepath: String,
        options: JsValue,
    ) -> Result<JsValue, JsValue> {
        expand_sync(code, filepath, options)
    }

    /// Expands `filepath`, reusing the cached result when `options.version`
    /// matches the version it was last expanded at.
    #[wasm_bindgen(js_name = "processFile")]
    pub fn process_file(
        &self,
        filepath: String,
        code: String,
        options: JsValue,
    ) -> Result<JsValue, JsValue> {
        with_debug_flush(|| {
            let opts: Option<ProcessFileOptions> = if options.is_null() || options.is_undefined() {
                None
            } else {
                Some(serde_wasm_bindgen::from_value(options)?)
            };

            let version = opts.as_ref().and_then(|o| o.version.clone());

            if let Some(version) = &version
                && let Some(cached) = self.cache()?.get(&filepath)
                && &cached.version == version
            {
                return serde_wasm_bindgen::to_value(&cached.result).map_err(|e| e.into());
            }

            let expand_opts = opts.map(|o| ExpandOptions {
                keep_decorators: o.keep_decorators,
                external_decorator_modules: o.external_decorator_modules,
                config_path: o.config_path,
                type_registry_json: o.type_registry_json,
                declarative_registry_json: o.declarative_registry_json,
                type_registry_id: o.type_registry_id,
                declarative_registry_id: o.declarative_registry_id,
                build_mode: o.build_mode,
                emit_metadata: o.emit_metadata,
            });

            let result = CoreEngine::expand_sync(code, filepath.clone(), expand_opts)
                .map_err(|e| JsValue::from_str(&e))?;

            let value = serde_wasm_bindgen::to_value(&result)?;
            if let Some(version) = version {
                self.cache()?.insert(filepath, version, result);
            }
            Ok(value)
        })
    }

    /// The position mapper for `filepath`'s last expansion, when it produced one.
    #[wasm_bindgen(js_name = "getMapper")]
    pub fn get_mapper(&self, filepath: String) -> Result<Option<PositionMapper>, JsValue> {
        Ok(self
            .cache()?
            .get(&filepath)
            .and_then(|cached| cached.mapper.clone())
            .map(|inner| PositionMapper { inner }))
    }

    /// Moves `diags`, positioned in `filepath`'s expanded code, back onto its source.
    #[wasm_bindgen(js_name = "mapDiagnostics")]
    pub fn map_diagnostics(&self, filepath: String, diags: JsValue) -> Result<JsValue, JsValue> {
        let diags: Vec<crate::api_types::JsDiagnostic> = serde_wasm_bindgen::from_value(diags)?;

        let mapper = self.get_mapper(filepath)?;

        let mapped: Vec<crate::api_types::JsDiagnostic> = if let Some(m) = mapper {
            diags
                .into_iter()
                .map(|mut d| {
                    if let (Some(start), Some(length)) = (d.start, d.length)
                        && let Some(mapped) = m.inner.map_span_to_original(start, length)
                    {
                        d.start = Some(mapped.start);
                        d.length = Some(mapped.length);
                    }
                    d
                })
                .collect()
        } else {
            diags
        };

        serde_wasm_bindgen::to_value(&mapped).map_err(|e| e.into())
    }
}

/// What the batch mapping methods return for a position with no original.
const UNMAPPED_POSITION: u32 = u32::MAX;

/// The value the batch mapping methods of [`PositionMapper`] return for a
/// position that lies in generated code.
#[wasm_bindgen(js_name = "unmappedPosition")]
pub fn unmapped_position() -> u32 {
    UNMAPPED_POSITION
}

/// Maps positions between a file's original source and its macro-expanded
/// code, and tells which macro generated a span.
#[wasm_bindgen]
pub struct PositionMapper {
    inner: std::sync::Arc<NativePositionMapper>,
}

#[wasm_bindgen]
impl PositionMapper {
    /// Creates a mapper from the `sourceMapping` of an expansion result.
    #[wasm_bindgen(constructor)]
    pub fn new(mapping: JsValue) -> Result<PositionMapper, JsValue> {
        let mapping: SourceMappingResult = serde_wasm_bindgen::from_value(mapping)?;
        Ok(Self {
            inner: std::sync::Arc::new(NativePositionMapper::new(mapping)),
        })
    }

    /// Whether the mapping has no segments and no generated regions.
    #[wasm_bindgen(js_name = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// The expanded position of the original position `pos`.
    #[wasm_bindgen(js_name = "originalToExpanded")]
    pub fn original_to_expanded(&self, pos: u32) -> u32 {
        self.inner.original_to_expanded(pos)
    }

    /// The original position of the expanded position `pos`, or `undefined`
    /// inside generated code.
    #[wasm_bindgen(js_name = "expandedToOriginal")]
    pub fn expanded_to_original(&self, pos: u32) -> Option<u32> {
        self.inner.expanded_to_original(pos)
    }

    /// The macro that generated the code at expanded position `pos`.
    #[wasm_bindgen(js_name = "generatedBy")]
    pub fn generated_by(&self, pos: u32) -> Option<String> {
        self.inner.generated_by(pos)
    }

    /// The original span of an expanded span, or `null` when it lies in
    /// generated code.
    #[wasm_bindgen(js_name = "mapSpanToOriginal")]
    pub fn map_span_to_original(&self, start: u32, length: u32) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner.map_span_to_original(start, length))
            .map_err(JsValue::from)
    }

    /// The expanded span of an original span.
    #[wasm_bindgen(js_name = "mapSpanToExpanded")]
    pub fn map_span_to_expanded(&self, start: u32, length: u32) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner.map_span_to_expanded(start, length))
            .map_err(JsValue::from)
    }

    /// Maps many expanded spans at once, given as flat `start, length` pairs.
    /// Each pair comes back as its original span, or as two
    /// [`unmapped_position`] values when it touches generated code.
    #[wasm_bindgen(js_name = "mapSpansToOriginal")]
    pub fn map_spans_to_original(&self, spans: Vec<u32>) -> Vec<u32> {
        spans
            .chunks(2)
            .flat_map(|pair| {
                match (pair.first(), pair.get(1)) {
                    (Some(&start), Some(&length)) => self.inner.map_span_to_original(start, length),
                    _ => None,
                }
                .map_or([UNMAPPED_POSITION; 2], |span| [span.start, span.length])
            })
            .collect()
    }

    /// Maps many expanded positions at once. A position in generated code
    /// comes back as [`unmapped_position`].
    #[wasm_bindgen(js_name = "expandedPositionsToOriginal")]
    pub fn expanded_positions_to_original(&self, positions: Vec<u32>) -> Vec<u32> {
        positions
            .into_iter()
            .map(|pos| {
                self.inner
                    .expanded_to_original(pos)
                    .unwrap_or(UNMAPPED_POSITION)
            })
            .collect()
    }

    /// Whether expanded position `pos` lies in macro-generated code.
    #[wasm_bindgen(js_name = "isInGenerated")]
    pub fn is_in_generated(&self, pos: u32) -> bool {
        self.inner.is_in_generated(pos)
    }
}

/// Scans the project under `root_dir` and returns its type registry and
/// declarative macro registry as JSON.
#[wasm_bindgen(js_name = "scanProjectSync")]
pub fn scan_project_sync(root_dir: String, options: JsValue) -> Result<JsValue, JsValue> {
    with_debug_flush(|| {
        let opts: Option<ScanOptions> = if options.is_null() || options.is_undefined() {
            None
        } else {
            Some(serde_wasm_bindgen::from_value(options)?)
        };

        let result =
            CoreEngine::scan_project_sync(root_dir, opts).map_err(|e| JsValue::from_str(&e))?;
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
    })
}

/// Drops `path` from the project scan cache and returns whether it was
/// cached. A wasm32 build rescans on every call, so it has nothing to drop.
#[wasm_bindgen(js_name = "invalidateScanCacheEntry")]
pub fn invalidate_scan_cache_entry_wasm(path: String) -> bool {
    CoreEngine::invalidate_scan_cache_entry(&path)
}

/// Empties the project scan cache; a wasm32 build keeps none.
#[wasm_bindgen(js_name = "clearScanCache")]
pub fn clear_scan_cache_wasm() {
    CoreEngine::clear_scan_cache();
}

/// The built-in macros and decorators this engine provides.
#[wasm_bindgen(js_name = "__macroforgeGetManifest")]
pub fn get_macro_manifest_wasm() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&get_macro_manifest()).map_err(|e| e.into())
}

/// Installs the type registry a host sends once, so the contexts it sends
/// afterwards refer to it instead of carrying it on every call.
#[wasm_bindgen(js_name = "__macroforgeSetRegistry")]
pub fn set_resident_registry_wasm(payload_json: &str) -> Result<(), JsValue> {
    crate::ts_syn::abi::ir::type_registry::install_resident_registry(payload_json)
        .map_err(|message| JsValue::from_str(&message))
}

/// Marks this module as a macroforge macro package.
#[wasm_bindgen(js_name = "__macroforgeIsMacroPackage")]
pub fn is_macro_package_wasm() -> bool {
    is_macro_package()
}

/// The names of the built-in macros.
#[wasm_bindgen(js_name = "__macroforgeGetMacroNames")]
pub fn get_macro_names_wasm() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&get_macro_names()).map_err(|e| e.into())
}

/// Debugging aid: the modules that registered macros.
#[wasm_bindgen(js_name = "__macroforgeDebugGetModules")]
pub fn debug_get_modules_wasm() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&debug_get_modules()).map_err(|e| e.into())
}

/// Debugging aid: describes how the macro `name` in `module` resolves.
#[wasm_bindgen(js_name = "__macroforgeDebugLookup")]
pub fn debug_lookup_wasm(module: String, name: String) -> String {
    debug_lookup(module, name)
}

/// Debugging aid: every registered macro descriptor.
#[wasm_bindgen(js_name = "__macroforgeDebugDescriptors")]
pub fn debug_descriptors_wasm() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&debug_descriptors()).map_err(|e| e.into())
}
