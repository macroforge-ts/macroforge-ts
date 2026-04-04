use crate::api::CoreEngine;
use crate::api_types::{
    ExpandOptions, ExpandResult, GeneratedRegionResult, MappingSegmentResult, ProcessFileOptions,
    ScanOptions, SyntaxCheckResult,
};
use crate::manifest::{
    debug_descriptors, debug_get_modules, debug_lookup, get_macro_manifest, get_macro_names,
    is_macro_package,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "checkSyntax")]
pub fn check_syntax(code: String, filepath: String) -> Result<JsValue, JsValue> {
    let result = CoreEngine::check_syntax(&code, &filepath).unwrap_or_else(|e| SyntaxCheckResult {
        ok: false,
        error: Some(e),
    });
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

#[wasm_bindgen(js_name = "parseImportSources")]
pub fn parse_import_sources(code: String, filepath: String) -> Result<JsValue, JsValue> {
    let result =
        CoreEngine::parse_import_sources(&code, &filepath).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

#[wasm_bindgen(js_name = "Derive")]
pub fn derive_decorator() {}

#[wasm_bindgen(js_name = "loadConfig")]
pub fn load_config(content: String, filepath: String) -> Result<JsValue, JsValue> {
    let result = CoreEngine::load_config(&content, &filepath).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

#[wasm_bindgen(js_name = "clearConfigCache")]
pub fn clear_config_cache() {
    CoreEngine::clear_config_cache();
}

#[wasm_bindgen(js_name = "transformSync")]
pub fn transform_sync(code: String, filepath: String) -> Result<JsValue, JsValue> {
    let result = CoreEngine::transform_sync(code, filepath).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

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
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cache: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    #[wasm_bindgen(js_name = "transformSync")]
    pub fn transform_sync(&self, code: String, filepath: String) -> Result<JsValue, JsValue> {
        transform_sync(code, filepath)
    }

    #[wasm_bindgen(js_name = "expandSync")]
    pub fn expand_sync(
        &self,
        code: String,
        filepath: String,
        options: JsValue,
    ) -> Result<JsValue, JsValue> {
        expand_sync(code, filepath, options)
    }

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

        if let (Some(ver), Ok(guard)) = (version.as_ref(), self.cache.lock()) {
            if let Some(cached) = guard.get(&filepath) {
                if cached.version.as_ref() == Some(ver) {
                    return serde_wasm_bindgen::to_value(&cached.result).map_err(|e| e.into());
                }
            }
        }

        let expand_opts = opts.map(|o| ExpandOptions {
            keep_decorators: o.keep_decorators,
            external_decorator_modules: o.external_decorator_modules,
            config_path: o.config_path,
            type_registry_json: o.type_registry_json,
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

    #[wasm_bindgen(js_name = "log")]
    pub fn log(&self, _message: String) {}

    #[wasm_bindgen(js_name = "getMapper")]
    pub fn get_mapper(&self, filepath: String) -> Option<WasmNativeMapper> {
        let mapping = match self.cache.lock() {
            Ok(guard) => guard
                .get(&filepath)
                .cloned()
                .and_then(|c| c.result.source_mapping),
            Err(_) => None,
        };

        mapping.map(|m| WasmNativeMapper {
            segments: m.segments,
            generated_regions: m.generated_regions,
        })
    }

    #[wasm_bindgen(js_name = "mapDiagnostics")]
    pub fn map_diagnostics(&self, filepath: String, diags: JsValue) -> Result<JsValue, JsValue> {
        let diags: Vec<crate::api_types::JsDiagnostic> = serde_wasm_bindgen::from_value(diags)?;

        let mapper = self.get_mapper(filepath);

        let mapped: Vec<crate::api_types::JsDiagnostic> = if let Some(m) = mapper {
            diags
                .into_iter()
                .map(|mut d| {
                    if let (Some(start), Some(length)) = (d.start, d.length) {
                        if let Some(mapped) = m.map_span_to_original_inner(start, length) {
                            d.start = Some(mapped.0);
                            d.length = Some(mapped.1);
                        }
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

#[wasm_bindgen(js_name = "NativeMapper")]
pub struct WasmNativeMapper {
    segments: Vec<MappingSegmentResult>,
    generated_regions: Vec<GeneratedRegionResult>,
}

#[wasm_bindgen(js_class = "NativeMapper")]
impl WasmNativeMapper {
    #[wasm_bindgen(js_name = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() && self.generated_regions.is_empty()
    }

    #[wasm_bindgen(js_name = "mapSpanToOriginal")]
    pub fn map_span_to_original(&self, start: u32, length: u32) -> JsValue {
        match self.map_span_to_original_inner(start, length) {
            Some((s, l)) => {
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"start".into(), &JsValue::from(s)).ok();
                js_sys::Reflect::set(&obj, &"length".into(), &JsValue::from(l)).ok();
                obj.into()
            }
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(js_name = "isInGenerated")]
    pub fn is_in_generated(&self, pos: u32) -> bool {
        self.generated_regions
            .iter()
            .any(|r| pos >= r.start && pos < r.end)
    }

    #[wasm_bindgen(js_name = "generatedBy")]
    pub fn generated_by(&self, pos: u32) -> Option<String> {
        self.generated_regions
            .iter()
            .find(|r| pos >= r.start && pos < r.end)
            .map(|r| r.source_macro.clone())
    }

    #[wasm_bindgen(js_name = "originalToExpanded")]
    pub fn original_to_expanded(&self, pos: u32) -> u32 {
        for seg in &self.segments {
            if pos >= seg.original_start && pos < seg.original_end {
                let offset = pos - seg.original_start;
                return seg.expanded_start + offset;
            }
        }
        pos
    }

    #[wasm_bindgen(js_name = "expandedToOriginal")]
    pub fn expanded_to_original(&self, pos: u32) -> Option<u32> {
        for seg in &self.segments {
            if pos >= seg.expanded_start && pos < seg.expanded_end {
                let offset = pos - seg.expanded_start;
                return Some(seg.original_start + offset);
            }
        }
        None
    }

    #[wasm_bindgen(js_name = "mapSpanToExpanded")]
    pub fn map_span_to_expanded(&self, start: u32, length: u32) -> JsValue {
        let end = start + length;
        for seg in &self.segments {
            if start >= seg.original_start && end <= seg.original_end {
                let offset = start - seg.original_start;
                let mapped_start = seg.expanded_start + offset;
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"start".into(), &JsValue::from(mapped_start)).ok();
                js_sys::Reflect::set(&obj, &"length".into(), &JsValue::from(length)).ok();
                return obj.into();
            }
        }
        // Fallback: return original span
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"start".into(), &JsValue::from(start)).ok();
        js_sys::Reflect::set(&obj, &"length".into(), &JsValue::from(length)).ok();
        obj.into()
    }
}

impl WasmNativeMapper {
    fn map_span_to_original_inner(&self, start: u32, length: u32) -> Option<(u32, u32)> {
        let end = start + length;
        for seg in &self.segments {
            if start >= seg.expanded_start && end <= seg.expanded_end {
                let offset = start - seg.expanded_start;
                let mapped_start = seg.original_start + offset;
                return Some((mapped_start, length));
            }
        }
        None
    }
}

#[wasm_bindgen]
pub struct NativePositionMapper {}

#[wasm_bindgen]
impl NativePositionMapper {
    #[wasm_bindgen(constructor)]
    pub fn new(_mapping: JsValue) -> Self {
        Self {}
    }

    #[wasm_bindgen(js_name = "mapToOriginal")]
    pub fn map_to_original(&self, _line: u32, _column: u32) -> JsValue {
        JsValue::NULL
    }

    #[wasm_bindgen(js_name = "mapToExpanded")]
    pub fn map_to_expanded(&self, _line: u32, _column: u32) -> JsValue {
        JsValue::NULL
    }
}

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

#[wasm_bindgen(js_name = "__macroforgeGetManifest")]
pub fn get_macro_manifest_wasm() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&get_macro_manifest()).map_err(|e| e.into())
}

#[wasm_bindgen(js_name = "__macroforgeIsMacroPackage")]
pub fn is_macro_package_wasm() -> bool {
    is_macro_package()
}

#[wasm_bindgen(js_name = "__macroforgeGetMacroNames")]
pub fn get_macro_names_wasm() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&get_macro_names()).map_err(|e| e.into())
}

#[wasm_bindgen(js_name = "__macroforgeDebugGetModules")]
pub fn debug_get_modules_wasm() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&debug_get_modules()).map_err(|e| e.into())
}

#[wasm_bindgen(js_name = "__macroforgeDebugLookup")]
pub fn debug_lookup_wasm(module: String, name: String) -> String {
    debug_lookup(module, name)
}

#[wasm_bindgen(js_name = "__macroforgeDebugDescriptors")]
pub fn debug_descriptors_wasm() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&debug_descriptors()).map_err(|e| e.into())
}
