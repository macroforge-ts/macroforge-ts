use anyhow::{Context, anyhow, bail};

use crate::host::derived::MacroManifest;
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

/// Extract every JSDoc annotation name a macro package's manifest contributes.
///
/// `decorators` holds helper attributes declared via `attributes((serde, "…"))`
/// and is keyed by `export`. `macros` holds the macros themselves; only
/// attribute macros are annotations, because an attribute macro is invoked as
/// `@traced` and its name *is* the annotation. Derive macros are invoked as
/// `@derive(Name)`, and `derive` is already seeded unconditionally.
pub(crate) fn annotation_names_from_manifest(manifest: &MacroManifest) -> Vec<String> {
    let decorators = manifest
        .decorators
        .iter()
        .map(|decorator| decorator.export.clone());
    let attributes = manifest
        .macros
        .iter()
        .filter(|entry| entry.kind.eq_ignore_ascii_case("attribute"))
        .map(|entry| entry.name.clone());
    let mut names: Vec<String> = decorators.chain(attributes).collect();
    names.sort_unstable();
    names.dedup();
    names
}

/// Parse the manifest JSON a macro package's manifest export returned.
fn parse_manifest(json: &str, package_path: &str) -> anyhow::Result<MacroManifest> {
    serde_json::from_str(json)
        .with_context(|| format!("macro package {package_path} returned a malformed manifest"))
}

pub(crate) struct ExternalMacroLoader {
    /// The project root macro packages resolve from.
    #[cfg(any(not(target_arch = "wasm32"), feature = "wasm"))]
    root_dir: std::path::PathBuf,
    /// Cache of loaded dynamic libraries, keyed by module path.
    /// Libraries must be kept alive for the symbols to remain valid.
    #[cfg(not(target_arch = "wasm32"))]
    loaded_libs: std::sync::Mutex<std::collections::HashMap<String, libloading::Library>>,
    /// Cache of instantiated wasm packages, keyed by module path.
    ///
    /// Instantiation parses and validates the whole module, which is far from
    /// free, and a watch session expands thousands of files — so a package is
    /// instantiated once and reused. Held behind a mutex because calling into a
    /// `wasmi::Store` needs `&mut`.
    #[cfg(not(target_arch = "wasm32"))]
    loaded_wasm:
        std::sync::Mutex<std::collections::HashMap<String, super::wasm_loader::WasmMacroModule>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ExternalMacroLoader {
    pub(crate) fn new(root_dir: std::path::PathBuf) -> Self {
        Self {
            root_dir,
            loaded_libs: std::sync::Mutex::new(std::collections::HashMap::new()),
            loaded_wasm: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// Resolve the annotation names an external macro package contributes.
    ///
    /// Loads the `.node`/`.dylib` shared library and calls the
    /// `__macroforge_ffi_get_manifest` symbol to get the full manifest, then
    /// extracts every name that may legally appear after `@` in a JSDoc
    /// annotation. Two manifest sections contribute:
    ///
    /// - `decorators` — helper attributes a derive macro declares via
    ///   `attributes((serde, "…"))`, used as `@serde`.
    /// - `macros` with `kind == "attribute"` — attribute macros, where the
    ///   macro's own name *is* the annotation, used as `@traced`.
    ///
    /// Derive macros are deliberately excluded: they are invoked as
    /// `@derive(Name)`, and `derive` is seeded unconditionally by
    /// `valid_annotation_names`.
    pub(crate) fn resolve_decorator_names(
        &self,
        package_path: &str,
    ) -> anyhow::Result<Vec<String>> {
        let Some(lib_path) = self.find_native_lib(package_path) else {
            // No native library: the package may still ship wasm, which is
            // what `macroforge build` produces.
            return self.resolve_decorator_names_wasm(package_path);
        };

        let mut libs = self
            .loaded_libs
            .lock()
            .map_err(|err| anyhow!("macro library cache lock poisoned: {err}"))?;
        let lib = match libs.entry(package_path.to_string()) {
            std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::hash_map::Entry::Vacant(entry) => entry.insert(
                unsafe { libloading::Library::new(&lib_path) }
                    .with_context(|| format!("failed to load {}", lib_path.display()))?,
            ),
        };

        let manifest_fn: libloading::Symbol<FfiManifestFn> =
            unsafe { lib.get(b"__macroforge_ffi_get_manifest") }.with_context(|| {
                format!(
                    "{} exports no __macroforge_ffi_get_manifest",
                    lib_path.display()
                )
            })?;
        let free_fn: libloading::Symbol<FfiFreeFn> = unsafe { lib.get(b"__macroforge_ffi_free") }
            .with_context(|| {
            format!("{} exports no __macroforge_ffi_free", lib_path.display())
        })?;

        let mut out_ptr: *mut u8 = std::ptr::null_mut();
        let mut out_len: usize = 0;
        let status = unsafe { manifest_fn(&mut out_ptr, &mut out_len) };
        if status != 0 || out_ptr.is_null() || out_len == 0 {
            bail!("{package_path}'s manifest export failed with status {status}");
        }

        let json = unsafe {
            let slice = std::slice::from_raw_parts(out_ptr, out_len);
            let json = String::from_utf8_lossy(slice).into_owned();
            free_fn(out_ptr, out_len);
            json
        };

        Ok(annotation_names_from_manifest(&parse_manifest(
            &json,
            package_path,
        )?))
    }

    /// Reads annotation names from a package's wasm manifest export.
    fn resolve_decorator_names_wasm(&self, package_path: &str) -> anyhow::Result<Vec<String>> {
        let Some(module_path) = self.find_wasm_for(package_path) else {
            bail!(
                "macro package {package_path} was not found, or ships neither a native \
                 library nor a wasm module"
            );
        };

        let mut cache = self
            .loaded_wasm
            .lock()
            .map_err(|err| anyhow!("macro module cache lock poisoned: {err}"))?;
        let module = match cache.entry(package_path.to_string()) {
            std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(super::wasm_loader::WasmMacroModule::load(&module_path)?)
            }
        };
        let json = module
            .manifest()
            .with_context(|| format!("{package_path}'s wasm manifest export failed"))?;

        Ok(annotation_names_from_manifest(&parse_manifest(
            &json,
            package_path,
        )?))
    }

    pub(crate) fn run_macro(&self, ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        if let Some(result) = self.try_run_ffi(ctx)? {
            return Ok(result);
        }

        // A package built by `macroforge build` ships wasm and no native
        // library, so this is the path that makes the CLI able to load what
        // that command produces.
        if let Some(result) = self.try_run_wasm(ctx)? {
            return Ok(result);
        }

        bail!(
            "External macro '{}' from '{}' could not be loaded. The package must \
             ship either a native library (.node/.dylib/.so) or a wasm module \
             exporting `__macroforge_ffi_run_{}`.",
            ctx.macro_name,
            ctx.module_path,
            {
                use convert_case::{Case, Casing};
                ctx.macro_name.to_case(Case::Snake)
            },
        )
    }

    /// Runs a macro from the package's wasm module.
    ///
    /// `Ok(None)` when the package ships no wasm, or ships one without this
    /// macro's export — both mean "try something else", not "fail".
    fn try_run_wasm(&self, ctx: &MacroContextIR) -> anyhow::Result<Option<MacroResult>> {
        use convert_case::{Case, Casing};

        let Some(module_path) = self.find_wasm_for(&ctx.module_path) else {
            return Ok(None);
        };

        let ctx_json =
            serde_json::to_string(ctx).map_err(|e| anyhow!("Failed to serialize context: {e}"))?;
        let symbol = format!(
            "__macroforge_ffi_run_{}",
            ctx.macro_name.to_case(Case::Snake)
        );

        let mut cache = self
            .loaded_wasm
            .lock()
            .map_err(|e| anyhow!("Lock poisoned: {e}"))?;

        let module = match cache.entry(ctx.module_path.clone()) {
            std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(super::wasm_loader::WasmMacroModule::load(&module_path)?)
            }
        };

        let Some(json) = module.run(&symbol, &ctx_json)? else {
            return Ok(None);
        };

        serde_json::from_str(&json)
            .map(Some)
            .map_err(|e| anyhow!("wasm macro returned malformed MacroResult: {e}"))
    }

    /// Locates an installed package directory, the way Node resolves it.
    ///
    /// Node checks `node_modules` in the starting directory and then in every
    /// ancestor, which is what makes a workspace work at all: a package in
    /// `apps/web` finds its dependencies in the repository root's
    /// `node_modules`. Looking only in `root_dir/node_modules` finds nothing
    /// there, and a macro package that cannot be found is a macro that silently
    /// contributes no output.
    fn find_package_dir(&self, module_path: &str) -> Option<std::path::PathBuf> {
        let mut dir = Some(self.root_dir.as_path());
        while let Some(current) = dir {
            let candidate = current.join("node_modules").join(module_path);
            if candidate.is_dir() {
                return Some(candidate);
            }
            dir = current.parent();
        }
        None
    }

    /// Locates a package's wasm artifact under `node_modules`.
    fn find_wasm_for(&self, module_path: &str) -> Option<std::path::PathBuf> {
        let pkg_dir = self.find_package_dir(module_path)?;
        super::wasm_loader::find_wasm_module(&pkg_dir)
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
        let lib = match libs.entry(ctx.module_path.clone()) {
            std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::hash_map::Entry::Vacant(entry) => entry.insert(
                unsafe { libloading::Library::new(&lib_path) }
                    .with_context(|| format!("failed to load {}", lib_path.display()))?,
            ),
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
        let pkg_dir = self.find_package_dir(module_path)?;

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

/// Loads macro packages from JS the way `require` does from the project root,
/// so the wasm build resolves them exactly as the project's own code would.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
mod node_packages {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = r#"
import { createRequire } from 'node:module';
import { resolve } from 'node:path';
const loaders = new Map();
function load(root, packagePath) {
    let projectRequire = loaders.get(root);
    if (projectRequire === undefined) {
        projectRequire = createRequire(resolve(root, 'package.json'));
        loaders.set(root, projectRequire);
    }
    return projectRequire(packagePath);
}
export function macroPackageManifests(root, packagePath) {
    const pkg = load(root, packagePath);
    return Object.entries(pkg)
        .filter(([name, value]) =>
            (name === '__macroforgeGetManifest' || name.startsWith('__macroforgeGetManifest_')) &&
            typeof value === 'function')
        .map(([, getManifest]) => JSON.stringify(getManifest()));
}
export function runMacroPackage(root, packagePath, functionName, contextJson) {
    const pkg = load(root, packagePath);
    const run = pkg[functionName] ?? pkg.default?.[functionName];
    if (typeof run !== 'function') {
        throw new Error(`${packagePath} exports no ${functionName}`);
    }
    return run(contextJson);
}
"#)]
    extern "C" {
        #[wasm_bindgen(catch, js_name = macroPackageManifests)]
        pub(super) fn macro_package_manifests(
            root: &str,
            package_path: &str,
        ) -> Result<js_sys::Array, JsValue>;
        #[wasm_bindgen(catch, js_name = runMacroPackage)]
        pub(super) fn run_macro_package(
            root: &str,
            package_path: &str,
            function_name: &str,
            context_json: &str,
        ) -> Result<JsValue, JsValue>;
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
impl ExternalMacroLoader {
    pub(crate) fn new(root_dir: std::path::PathBuf) -> Self {
        Self { root_dir }
    }

    pub(crate) fn resolve_decorator_names(
        &self,
        package_path: &str,
    ) -> anyhow::Result<Vec<String>> {
        let manifests =
            node_packages::macro_package_manifests(&self.root_dir.to_string_lossy(), package_path)
                .map_err(|error| {
                    anyhow!("could not load the macro package {package_path}: {error:?}")
                })?;
        let mut names = Vec::new();
        for manifest in manifests.iter() {
            let json = manifest.as_string().ok_or_else(|| {
                anyhow!("{package_path}'s manifest export returned a non-string manifest")
            })?;
            names.extend(annotation_names_from_manifest(&parse_manifest(
                &json,
                package_path,
            )?));
        }
        names.sort_unstable();
        names.dedup();
        Ok(names)
    }

    pub(crate) fn run_macro(&self, ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        let ctx_json =
            serde_json::to_string(ctx).context("failed to serialize the macro context")?;
        let function_name = format!("__macroforgeRun{}", ctx.macro_name);
        let output = node_packages::run_macro_package(
            &self.root_dir.to_string_lossy(),
            &ctx.module_path,
            &function_name,
            &ctx_json,
        )
        .map_err(|error| anyhow!("external macro {} failed: {error:?}", ctx.macro_name))?;
        let output = output.as_string().ok_or_else(|| {
            anyhow!(
                "{} from {} returned a non-string result",
                function_name,
                ctx.module_path
            )
        })?;
        if let Some(message) = output.strip_prefix("Error:") {
            bail!("external macro {} failed:{message}", ctx.macro_name);
        }
        serde_json::from_str(&output).context("failed to parse the external macro's result")
    }
}

/// Resolves the decorator names of every macro package the file imports.
pub(crate) fn resolve_external_decorator_names(
    macro_imports: &std::collections::HashMap<String, String>,
    loader: Option<&ExternalMacroLoader>,
) -> anyhow::Result<Vec<String>> {
    let Some(loader) = loader else {
        return Ok(Vec::new());
    };
    // A relative specifier names a local declarative-macro file, which the
    // declarative registry resolves; only package specifiers are macro packages.
    let packages: std::collections::BTreeSet<&String> = macro_imports
        .values()
        .filter(|module| !module.starts_with('.') && !module.starts_with('/'))
        .collect();
    let mut names = Vec::new();
    for package in packages {
        names.extend(loader.resolve_decorator_names(package)?);
    }
    Ok(names)
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
impl ExternalMacroLoader {
    pub(crate) fn new(_root_dir: std::path::PathBuf) -> Self {
        Self {}
    }

    pub(crate) fn resolve_decorator_names(
        &self,
        package_path: &str,
    ) -> anyhow::Result<Vec<String>> {
        bail!("cannot load the macro package {package_path}: this WASM build has no `wasm` feature")
    }

    pub(crate) fn run_macro(&self, _ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        bail!("External macros are not supported in this WASM build (wasm feature disabled)")
    }
}

#[cfg(test)]
mod tests {
    use super::annotation_names_from_manifest;
    use crate::host::derived::{DecoratorManifestEntry, MacroManifest, MacroManifestEntry};

    fn entry(name: &str, kind: &str) -> MacroManifestEntry {
        MacroManifestEntry {
            name: name.to_string(),
            kind: kind.to_string(),
            description: String::new(),
            package: "playground_macros".to_string(),
        }
    }

    /// A package that declares one attribute macro, one derive macro, one call
    /// macro and a helper decorator.
    fn manifest() -> MacroManifest {
        MacroManifest {
            version: 1,
            macros: vec![
                entry("Serialize", "derive"),
                entry("traced", "attribute"),
                entry("stringify", "call"),
            ],
            decorators: vec![DecoratorManifestEntry {
                module: "macroforge_ts".to_string(),
                export: "serde".to_string(),
                kind: "property".to_string(),
                docs: String::new(),
            }],
        }
    }

    #[test]
    fn attribute_macros_are_annotation_names() {
        // Attribute macros register under `macros`, not `decorators`; leaving
        // them out stripped `@traced` during lowering.
        assert!(annotation_names_from_manifest(&manifest()).contains(&"traced".to_string()));
    }

    #[test]
    fn helper_decorators_are_annotation_names() {
        assert!(annotation_names_from_manifest(&manifest()).contains(&"serde".to_string()));
    }

    #[test]
    fn derive_and_call_macros_are_not_annotation_names() {
        let names = annotation_names_from_manifest(&manifest());
        assert!(!names.contains(&"Serialize".to_string()));
        assert!(!names.contains(&"stringify".to_string()));
    }

    #[test]
    fn an_empty_manifest_yields_no_names() {
        assert!(annotation_names_from_manifest(&MacroManifest::default()).is_empty());
    }
}
