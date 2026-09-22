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
#[cfg(feature = "oxc")]
use serde::Serialize;
use wasm_bindgen::prelude::*;

/// Whether `code` may contain anything the engine expands. Integrations use
/// this to skip files without paying for a full expansion.
#[wasm_bindgen(js_name = "hasMacroAnnotations")]
pub fn has_macro_annotations(code: &str) -> bool {
    crate::has_macro_annotations(code)
}

/// The macros `code` imports through `import macro` JSDoc comments, as macro
/// name to module.
#[cfg(feature = "oxc")]
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

/// Expands the macros in `code` and returns the result with its metadata.
#[wasm_bindgen(js_name = "transformSync")]
pub fn transform_sync(code: String, filepath: String) -> Result<JsValue, JsValue> {
    let result = CoreEngine::transform_sync(code, filepath).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

/// Expands the macros in `code`, the source of `filepath`, and returns the
/// expanded code, its type declarations, diagnostics and source mapping.
#[wasm_bindgen(js_name = "expandSync")]
pub fn expand_sync(code: String, filepath: String, options: JsValue) -> Result<JsValue, JsValue> {
    let opts: Option<ExpandOptions> = if options.is_null() || options.is_undefined() {
        None
    } else {
        Some(serde_wasm_bindgen::from_value(options)?)
    };

    let result =
        CoreEngine::expand_sync(code, filepath, opts).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

/// A stateful expander for editor integrations: it caches each file's
/// expansion by version and maps positions and diagnostics back to the source.
#[derive(Default)]
#[wasm_bindgen]
pub struct NativePlugin {
    cache: std::sync::Mutex<std::collections::HashMap<String, CachedResult>>,
}

#[derive(Clone)]
struct CachedResult {
    version: Option<String>,
    result: ExpandResult,
}

#[wasm_bindgen]
impl NativePlugin {
    /// Creates a plugin with an empty cache.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cache: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// Expands the macros in `code` and returns the result with its metadata.
    #[wasm_bindgen(js_name = "transformSync")]
    pub fn transform_sync(&self, code: String, filepath: String) -> Result<JsValue, JsValue> {
        transform_sync(code, filepath)
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
        let opts: Option<ProcessFileOptions> = if options.is_null() || options.is_undefined() {
            None
        } else {
            Some(serde_wasm_bindgen::from_value(options)?)
        };

        let version = opts.as_ref().and_then(|o| o.version.clone());

        if let (Some(ver), Ok(guard)) = (version.as_ref(), self.cache.lock())
            && let Some(cached) = guard.get(&filepath)
            && cached.version.as_ref() == Some(ver)
        {
            return serde_wasm_bindgen::to_value(&cached.result).map_err(|e| e.into());
        }

        let expand_opts = opts.map(|o| ExpandOptions {
            keep_decorators: o.keep_decorators,
            external_decorator_modules: o.external_decorator_modules,
            config_path: o.config_path,
            type_registry_json: o.type_registry_json,
            declarative_registry_json: o.declarative_registry_json,
            build_mode: o.build_mode,
        });

        let result = CoreEngine::expand_sync(code, filepath.clone(), expand_opts)
            .map_err(|e| JsValue::from_str(&e))?;

        if let (Some(ver), Ok(mut guard)) = (version, self.cache.lock()) {
            guard.insert(
                filepath,
                CachedResult {
                    version: Some(ver),
                    result: result.clone(),
                },
            );
        }

        serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
    }

    /// The position mapper for `filepath`'s last expansion, when it produced one.
    #[wasm_bindgen(js_name = "getMapper")]
    pub fn get_mapper(&self, filepath: String) -> Result<Option<PositionMapper>, JsValue> {
        let cache = self
            .cache
            .lock()
            .map_err(|err| JsValue::from_str(&format!("expansion cache lock poisoned: {err}")))?;
        Ok(cache
            .get(&filepath)
            .and_then(|cached| cached.result.source_mapping.clone())
            .map(|mapping| PositionMapper {
                inner: NativePositionMapper::new(mapping),
            }))
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

/// Maps positions between a file's original source and its macro-expanded
/// code, and tells which macro generated a span.
#[wasm_bindgen]
pub struct PositionMapper {
    inner: NativePositionMapper,
}

#[wasm_bindgen]
impl PositionMapper {
    /// Creates a mapper from the `sourceMapping` of an expansion result.
    #[wasm_bindgen(constructor)]
    pub fn new(mapping: JsValue) -> Result<PositionMapper, JsValue> {
        let mapping: SourceMappingResult = serde_wasm_bindgen::from_value(mapping)?;
        Ok(Self {
            inner: NativePositionMapper::new(mapping),
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
    let opts: Option<ScanOptions> = if options.is_null() || options.is_undefined() {
        None
    } else {
        Some(serde_wasm_bindgen::from_value(options)?)
    };

    let result =
        CoreEngine::scan_project_sync(root_dir, opts).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
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
