use crate::api::CoreEngine;
use crate::api_types::{ExpandOptions, ScanOptions, SyntaxCheckResult};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn check_syntax(code: String, filepath: String) -> Result<JsValue, JsValue> {
    let result = CoreEngine::check_syntax(&code, &filepath).unwrap_or_else(|e| SyntaxCheckResult {
        ok: false,
        error: Some(e),
    });
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

#[wasm_bindgen]
pub fn parse_import_sources(code: String, filepath: String) -> Result<JsValue, JsValue> {
    let result =
        CoreEngine::parse_import_sources(&code, &filepath).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

#[wasm_bindgen(js_name = "Derive")]
pub fn derive_decorator() {}

#[wasm_bindgen]
pub fn load_config(content: String, filepath: String) -> Result<JsValue, JsValue> {
    let result = CoreEngine::load_config(&content, &filepath).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

#[wasm_bindgen]
pub fn clear_config_cache() {
    CoreEngine::clear_config_cache();
}

#[wasm_bindgen]
pub fn transform_sync(code: String, filepath: String) -> Result<JsValue, JsValue> {
    let result = CoreEngine::transform_sync(code, filepath).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.into())
}

#[wasm_bindgen]
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
