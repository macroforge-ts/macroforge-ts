use anyhow::{Context, anyhow, bail};

use crate::ts_syn::abi::{MacroContextIR, MacroResult};

// ============================================================================
// External Macro Loader
// ============================================================================

/// Type signature for FFI macro functions exported by macro packages.
/// See `#[ts_macro_derive]` for the generated `__macroforge_ffi_run_*` symbols.
#[cfg(not(target_arch = "wasm32"))]
type FfiRunFn = unsafe extern "C" fn(
    ctx_ptr: *const u8,
    ctx_len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32;

/// Type signature for `__macroforge_ffi_get_manifest` — returns the full manifest as JSON.
#[cfg(not(target_arch = "wasm32"))]
type FfiManifestFn = unsafe extern "C" fn(out_ptr: *mut *mut u8, out_len: *mut usize) -> i32;

/// Type signature for the FFI free function.
#[cfg(not(target_arch = "wasm32"))]
type FfiFreeFn = unsafe extern "C" fn(ptr: *mut u8, len: usize);

pub(crate) struct ExternalMacroLoader {
    #[cfg(not(target_arch = "wasm32"))]
    root_dir: std::path::PathBuf,
    /// Cache of loaded dynamic libraries, keyed by module path.
    /// Libraries must be kept alive for the symbols to remain valid.
    #[cfg(not(target_arch = "wasm32"))]
    loaded_libs: std::sync::Mutex<std::collections::HashMap<String, libloading::Library>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ExternalMacroLoader {
    pub(crate) fn new(root_dir: std::path::PathBuf) -> Self {
        Self {
            root_dir,
            loaded_libs: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// Resolve decorator export names from an external macro package via FFI.
    ///
    /// Loads the `.node`/`.dylib` shared library and calls the
    /// `__macroforge_ffi_get_manifest` symbol to get the full manifest,
    /// then extracts decorator names from it.
    pub(crate) fn resolve_decorator_names(&self, package_path: &str) -> Vec<String> {
        let lib_path = match self.find_native_lib(package_path) {
            Some(p) => p,
            None => return Vec::new(),
        };

        let mut libs = match self.loaded_libs.lock() {
            Ok(l) => l,
            Err(_) => return Vec::new(),
        };

        let lib = if let Some(lib) = libs.get(package_path) {
            lib
        } else {
            match unsafe { libloading::Library::new(&lib_path) } {
                Ok(loaded) => {
                    libs.insert(package_path.to_string(), loaded);
                    libs.get(package_path).unwrap()
                }
                Err(_) => return Vec::new(),
            }
        };

        // Call __macroforge_ffi_get_manifest to get the full manifest as JSON
        let manifest_fn: libloading::Symbol<FfiManifestFn> =
            match unsafe { lib.get(b"__macroforge_ffi_get_manifest") } {
                Ok(f) => f,
                Err(_) => return Vec::new(),
            };

        let free_fn: libloading::Symbol<FfiFreeFn> =
            match unsafe { lib.get(b"__macroforge_ffi_free") } {
                Ok(f) => f,
                Err(_) => return Vec::new(),
            };

        let mut out_ptr: *mut u8 = std::ptr::null_mut();
        let mut out_len: usize = 0;

        let status = unsafe { manifest_fn(&mut out_ptr, &mut out_len) };

        if status != 0 || out_ptr.is_null() || out_len == 0 {
            return Vec::new();
        }

        let json = unsafe {
            let slice = std::slice::from_raw_parts(out_ptr, out_len);
            let s = String::from_utf8_lossy(slice).into_owned();
            free_fn(out_ptr, out_len);
            s
        };

        // Parse the manifest and extract decorator export names
        let manifest: serde_json::Value = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        manifest
            .get("decorators")
            .and_then(|d| d.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|d| d.get("export").and_then(|e| e.as_str()).map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub(crate) fn run_macro(&self, ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        if let Some(result) = self.try_run_ffi(ctx)? {
            return Ok(result);
        }

        bail!(
            "External macro '{}' from '{}' not found via FFI. \
             Ensure the package exports FFI symbols (built with #[ts_macro_derive]).",
            ctx.macro_name,
            ctx.module_path
        )
    }

    /// Attempt to call the macro via FFI by loading the `.node` shared library directly.
    /// Returns `Ok(Some(result))` on success, `Ok(None)` if the FFI symbol wasn't found,
    /// or `Err` on a real failure.
    fn try_run_ffi(&self, ctx: &MacroContextIR) -> anyhow::Result<Option<MacroResult>> {
        use convert_case::{Case, Casing};

        let ffi_symbol = format!(
            "__macroforge_ffi_run_{}",
            ctx.macro_name.to_case(Case::Snake)
        );
        let ctx_json =
            serde_json::to_string(ctx).map_err(|e| anyhow!("Failed to serialize context: {e}"))?;

        // Find the .node/.dylib file for this module
        let lib_path = match self.find_native_lib(&ctx.module_path) {
            Some(p) => p,
            None => return Ok(None),
        };

        // Load (or reuse cached) library
        let mut libs = self
            .loaded_libs
            .lock()
            .map_err(|e| anyhow!("Lock poisoned: {e}"))?;
        let lib = if let Some(lib) = libs.get(&ctx.module_path) {
            lib
        } else {
            let loaded = unsafe { libloading::Library::new(&lib_path) }
                .map_err(|e| anyhow!("Failed to load {}: {e}", lib_path.display()))?;
            libs.insert(ctx.module_path.clone(), loaded);
            libs.get(&ctx.module_path).unwrap()
        };

        // Look up the FFI symbol
        let run_fn: libloading::Symbol<FfiRunFn> = match unsafe { lib.get(ffi_symbol.as_bytes()) } {
            Ok(f) => f,
            Err(_) => return Ok(None), // Symbol not found — package wasn't built with FFI exports
        };

        let free_fn: libloading::Symbol<FfiFreeFn> =
            match unsafe { lib.get(b"__macroforge_ffi_free") } {
                Ok(f) => f,
                Err(_) => return Ok(None),
            };

        // Call the FFI function
        let mut out_ptr: *mut u8 = std::ptr::null_mut();
        let mut out_len: usize = 0;

        let status = unsafe {
            run_fn(
                ctx_json.as_ptr(),
                ctx_json.len(),
                &mut out_ptr,
                &mut out_len,
            )
        };

        if out_ptr.is_null() || out_len == 0 {
            bail!("FFI macro returned null output");
        }

        // Read the output into a String, then free the FFI buffer
        let output = unsafe {
            let slice = std::slice::from_raw_parts(out_ptr, out_len);
            let s = String::from_utf8_lossy(slice).into_owned();
            free_fn(out_ptr, out_len);
            s
        };

        if status != 0 {
            bail!("External macro FFI error: {output}");
        }

        let result: MacroResult =
            serde_json::from_str(&output).context("Failed to parse FFI macro result")?;

        Ok(Some(result))
    }

    /// Finds the native shared library (.node, .dylib, .so) for a given module path.
    fn find_native_lib(&self, module_path: &str) -> Option<std::path::PathBuf> {
        // Walk up from root_dir looking for node_modules/<module_path>
        let node_modules = self.root_dir.join("node_modules");
        let pkg_dir = node_modules.join(module_path);
        if !pkg_dir.is_dir() {
            return None;
        }

        // Find the first .node file in the package directory
        let entries = std::fs::read_dir(&pkg_dir).ok()?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if ext == "node" || ext == "dylib" || ext == "so" {
                    return Some(path);
                }
            }
        }
        None
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use js_sys;
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
static EXTERNAL_CALLBACKS: std::sync::OnceLock<ExternalCallbacks> = std::sync::OnceLock::new();

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
struct ExternalCallbacks {
    resolve: js_sys::Function,
    run: js_sys::Function,
}

/// Register JS callbacks for resolving and running external (user-defined) macros.
///
/// Required for WASM builds. The native (NAPI) build resolves external macros
/// by spawning a Node subprocess, but WASM cannot spawn processes. Instead, the
/// host JS environment must provide two callbacks:
///
/// resolve: Given a package path, return an array of decorator names exported by that package.
///
/// run: Given a JSON-serialized MacroContextIR, execute the external macro and return a JSON-serialized MacroResult.
///
/// Must be called before expandSync if the source uses import macro comments.
/// Does not exist on NAPI builds, so guard the call.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen(js_name = "setupExternalMacros")]
pub fn setup_external_macros(resolve: js_sys::Function, run: js_sys::Function) {
    let _ = EXTERNAL_CALLBACKS.set(ExternalCallbacks { resolve, run });
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
impl ExternalMacroLoader {
    pub(crate) fn new(_root_dir: std::path::PathBuf) -> Self {
        Self {}
    }

    pub(crate) fn resolve_decorator_names(&self, package_path: &str) -> Vec<String> {
        let Some(callbacks) = EXTERNAL_CALLBACKS.get() else {
            return Vec::new();
        };

        let this = JsValue::NULL;
        let pkg = JsValue::from_str(package_path);
        match callbacks.resolve.call1(&this, &pkg) {
            Ok(result) => serde_wasm_bindgen::from_value(result).unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    }

    pub(crate) fn run_macro(&self, ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        let Some(callbacks) = EXTERNAL_CALLBACKS.get() else {
            bail!("External macros callbacks not initialized. Call setupExternalMacros() first.");
        };

        let ctx_json =
            serde_json::to_string(ctx).map_err(|e| anyhow!("Failed to serialize context: {e}"))?;

        let this = JsValue::NULL;
        let arg = JsValue::from_str(&ctx_json);
        let result = callbacks
            .run
            .call1(&this, &arg)
            .map_err(|e| anyhow!("JS callback failed: {:?}", e))?;

        if result.is_null() || result.is_undefined() {
            bail!("External macro returned null or undefined");
        }

        if let Some(s) = result.as_string() {
            if s.starts_with("Error:") {
                bail!("External macro error: {}", s);
            }
            let host_result: crate::ts_syn::abi::MacroResult =
                serde_json::from_str(&s).context("Failed to parse macro result from string")?;
            return Ok(host_result);
        }

        let host_result: crate::ts_syn::abi::MacroResult =
            serde_wasm_bindgen::from_value(result).context("Failed to parse macro result")?;

        Ok(host_result)
    }
}

/// Resolves decorator names from multiple external packages by parsing macro import comments in the source.
pub(crate) fn resolve_external_decorator_names(
    source: &str,
    loader: Option<&ExternalMacroLoader>,
) -> Vec<String> {
    let mut all_names = Vec::new();
    if let Some(loader) = loader {
        // Simple extraction of macro package paths from /** import macro { ... } from "pkg" */ comments
        let mut search_start = 0;
        while let Some(idx) = source[search_start..].find("import macro") {
            let abs_idx = search_start + idx;
            // Find "from" after "import macro"
            if let Some(from_idx) = source[abs_idx..].find("from") {
                let from_abs = abs_idx + from_idx;
                let rest = &source[from_abs + 4..].trim_start();
                if rest.starts_with('"') || rest.starts_with('\'') {
                    let q = rest.chars().next().unwrap();
                    if let Some(end_quote) = rest[1..].find(q) {
                        let pkg = &rest[1..end_quote + 1];
                        let names = loader.resolve_decorator_names(pkg);
                        all_names.extend(names);
                    }
                }
            }
            search_start = abs_idx + 12; // Skip "import macro"
        }
    }
    all_names
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
impl ExternalMacroLoader {
    pub(crate) fn new(_root_dir: std::path::PathBuf) -> Self {
        Self {}
    }

    pub(crate) fn resolve_decorator_names(&self, _package_path: &str) -> Vec<String> {
        Vec::new()
    }

    pub(crate) fn run_macro(&self, _ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        bail!("External macros are not supported in this WASM build (wasm feature disabled)")
    }
}
