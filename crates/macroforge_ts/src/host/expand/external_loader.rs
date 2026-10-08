use anyhow::{Context, anyhow, bail};

use crate::host::derived::MacroManifest;
use crate::ts_syn::abi::{MacroContextIR, MacroResult};

// ============================================================================
// External Macro Loader
// ============================================================================

/// Extract every JSDoc annotation name a macro package's manifest contributes.
///
/// `decorators` holds helper attributes declared via `attributes((endec, "…"))`
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
    root_dir: std::path::PathBuf,
    /// For each package, the registry generation installed in it, or `None`
    /// once it turned out not to take resident registries.
    #[cfg(all(target_arch = "wasm32", feature = "wasm"))]
    resident: std::sync::Mutex<
        std::collections::HashMap<
            String,
            Option<crate::ts_syn::abi::ir::type_registry::RegistryGeneration>,
        >,
    >,
}

/// The installation payload for `registry`, serialized once per generation
/// and shared by every guest it is sent to.
#[cfg(any(not(target_arch = "wasm32"), feature = "wasm"))]
fn resident_payload(
    registry: &crate::ts_syn::abi::ir::type_registry::TypeRegistry,
) -> anyhow::Result<std::sync::Arc<str>> {
    use crate::ts_syn::abi::ir::type_registry::{RegistryGeneration, ResidentRegistryPayload};

    static LAST: std::sync::Mutex<Option<(RegistryGeneration, std::sync::Arc<str>)>> =
        std::sync::Mutex::new(None);

    let generation = registry.generation();
    let mut last = LAST
        .lock()
        .map_err(|err| anyhow!("registry payload cache lock poisoned: {err}"))?;
    if let Some((cached, payload)) = last.as_ref()
        && *cached == generation
    {
        return Ok(std::sync::Arc::clone(payload));
    }
    let payload: std::sync::Arc<str> = serde_json::to_string(&ResidentRegistryPayload {
        generation,
        registry: registry.clone(),
    })
    .context("failed to serialize the type registry for a macro package")?
    .into();
    *last = Some((generation, std::sync::Arc::clone(&payload)));
    Ok(payload)
}

/// `ctx` as sent to a guest holding its registry: the registry named by
/// generation instead of carried.
#[cfg(any(not(target_arch = "wasm32"), feature = "wasm"))]
fn resident_context_json(ctx: &MacroContextIR) -> anyhow::Result<String> {
    let mut wire = ctx.clone();
    wire.type_registry = ctx.type_registry.resident_reference();
    serde_json::to_string(&wire).context("failed to serialize the macro context")
}

/// Where each package's wasm module was found, by project root and package.
/// Only packages that were found are kept, and each is checked on use, so a
/// package installed or removed while the process runs is noticed.
#[cfg(not(target_arch = "wasm32"))]
static PACKAGE_MODULES: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<(std::path::PathBuf, String), std::path::PathBuf>>,
> = std::sync::LazyLock::new(std::sync::Mutex::default);

/// Each module's annotation names, with the stamp of the file they came from.
#[cfg(not(target_arch = "wasm32"))]
type ModuleAnnotations = std::collections::HashMap<
    std::path::PathBuf,
    (crate::host::file_stamp::FileStamp, Vec<String>),
>;

/// Each module's annotation names, for as long as its file is unchanged.
#[cfg(not(target_arch = "wasm32"))]
static MODULE_ANNOTATIONS: std::sync::LazyLock<std::sync::Mutex<ModuleAnnotations>> =
    std::sync::LazyLock::new(std::sync::Mutex::default);

#[cfg(not(target_arch = "wasm32"))]
impl ExternalMacroLoader {
    pub(crate) fn new(root_dir: std::path::PathBuf) -> Self {
        Self { root_dir }
    }

    /// Resolve the annotation names an external macro package contributes,
    /// from its wasm manifest export.
    ///
    /// Two manifest sections contribute: `decorators`, the helper attributes a
    /// derive macro declares (used as `@endec`), and attribute macros, whose own
    /// name is the annotation (used as `@traced`). Derive macros are invoked as
    /// `@derive(Name)`, and `derive` is seeded unconditionally.
    pub(crate) fn resolve_decorator_names(
        &self,
        package_path: &str,
    ) -> anyhow::Result<Vec<String>> {
        let Some(module_path) = self.find_wasm_for(package_path)? else {
            bail!("macro package {package_path} was not found, or ships no wasm module");
        };
        let stamp = crate::host::file_stamp::FileStamp::of(&module_path)
            .with_context(|| format!("failed to stat {}", module_path.display()))?;
        {
            let cached = MODULE_ANNOTATIONS
                .lock()
                .map_err(|err| anyhow!("macro manifest cache lock poisoned: {err}"))?;
            if let Some((cached_stamp, names)) = cached.get(&module_path)
                && *cached_stamp == stamp
            {
                return Ok(names.clone());
            }
        }

        let json = super::wasm_loader::with_instance(&module_path, |module| module.manifest())
            .with_context(|| format!("{package_path}'s wasm manifest export failed"))?;
        let names = annotation_names_from_manifest(&parse_manifest(&json, package_path)?);
        MODULE_ANNOTATIONS
            .lock()
            .map_err(|err| anyhow!("macro manifest cache lock poisoned: {err}"))?
            .insert(module_path, (stamp, names.clone()));
        Ok(names)
    }

    pub(crate) fn run_macro(&self, ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        let result = self.load_and_run(ctx)?;
        ctx.type_registry
            .record_macro_reads(result.registry_reads.as_ref());
        Ok(result)
    }

    fn load_and_run(&self, ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        if let Some(result) = self.try_run_wasm(ctx)? {
            return Ok(result);
        }

        bail!(
            "External macro '{}' from '{}' could not be loaded. The package must \
             ship a wasm module exporting `__macroforge_ffi_run_{}`, which \
             `macroforge build` produces.",
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
    /// macro's export.
    fn try_run_wasm(&self, ctx: &MacroContextIR) -> anyhow::Result<Option<MacroResult>> {
        use convert_case::{Case, Casing};

        let Some(module_path) = self.find_wasm_for(&ctx.module_path)? else {
            return Ok(None);
        };

        let symbol = format!(
            "__macroforge_ffi_run_{}",
            ctx.macro_name.to_case(Case::Snake)
        );

        // A guest that takes resident registries gets the registry once per
        // generation and contexts that refer to it; any other gets it whole.
        let Some(json) = super::wasm_loader::with_instance(&module_path, |module| {
            let ctx_json = if module.accepts_resident_registry() {
                module.ensure_resident(ctx.type_registry.generation(), || {
                    resident_payload(&ctx.type_registry)
                })?;
                resident_context_json(ctx)?
            } else {
                serde_json::to_string(ctx)
                    .map_err(|e| anyhow!("Failed to serialize context: {e}"))?
            };
            module.run(&symbol, &ctx_json)
        })?
        else {
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
    fn find_wasm_for(&self, module_path: &str) -> anyhow::Result<Option<std::path::PathBuf>> {
        let key = (self.root_dir.clone(), module_path.to_string());
        let mut found = PACKAGE_MODULES
            .lock()
            .map_err(|err| anyhow!("macro package cache lock poisoned: {err}"))?;
        if let Some(wasm) = found.get(&key)
            && wasm.is_file()
        {
            return Ok(Some(wasm.clone()));
        }
        let wasm = self
            .find_package_dir(module_path)
            .and_then(|package_dir| super::wasm_loader::find_wasm_module(&package_dir));
        match &wasm {
            Some(path) => {
                found.insert(key, path.clone());
            }
            None => {
                found.remove(&key);
            }
        }
        Ok(wasm)
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
// Installs a type registry in the package, returning false for a package
// built before resident registries, which takes the registry in every context.
export function setMacroPackageRegistry(root, packagePath, payloadJson) {
    const pkg = load(root, packagePath);
    const install = pkg.__macroforgeSetRegistry ?? pkg.default?.__macroforgeSetRegistry;
    if (typeof install !== 'function') {
        return false;
    }
    install(payloadJson);
    return true;
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
        #[wasm_bindgen(catch, js_name = setMacroPackageRegistry)]
        pub(super) fn set_macro_package_registry(
            root: &str,
            package_path: &str,
            payload_json: &str,
        ) -> Result<bool, JsValue>;
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
        Self {
            root_dir,
            resident: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// Whether the package holds `ctx`'s registry, installing it first when
    /// the package takes resident registries and holds another generation.
    fn ensure_resident(&self, ctx: &MacroContextIR) -> anyhow::Result<bool> {
        let generation = ctx.type_registry.generation();
        let mut resident = self
            .resident
            .lock()
            .map_err(|err| anyhow!("resident registry table lock poisoned: {err}"))?;
        match resident.get(&ctx.module_path) {
            Some(None) => return Ok(false),
            Some(Some(installed)) if *installed == generation => return Ok(true),
            _ => {}
        }
        let payload = resident_payload(&ctx.type_registry)?;
        let installed = node_packages::set_macro_package_registry(
            &self.root_dir.to_string_lossy(),
            &ctx.module_path,
            &payload,
        )
        .map_err(|error| {
            anyhow!(
                "installing the type registry in {} failed: {error:?}",
                ctx.module_path
            )
        })?;
        resident.insert(ctx.module_path.clone(), installed.then_some(generation));
        Ok(installed)
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
        let ctx_json = if self.ensure_resident(ctx)? {
            resident_context_json(ctx)?
        } else {
            serde_json::to_string(ctx).context("failed to serialize the macro context")?
        };
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
        let result: MacroResult =
            serde_json::from_str(&output).context("failed to parse the external macro's result")?;
        ctx.type_registry
            .record_macro_reads(result.registry_reads.as_ref());
        Ok(result)
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
    pub(crate) fn new(root_dir: std::path::PathBuf) -> Self {
        Self { root_dir }
    }

    pub(crate) fn resolve_decorator_names(
        &self,
        package_path: &str,
    ) -> anyhow::Result<Vec<String>> {
        bail!(
            "cannot load the macro package {package_path} for {}: this WASM build has no \
             `wasm` feature",
            self.root_dir.display()
        )
    }

    pub(crate) fn run_macro(&self, ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        bail!(
            "cannot run the external macro {} from {}: this WASM build has no `wasm` feature",
            ctx.macro_name,
            ctx.module_path
        )
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
                entry("Encode", "derive"),
                entry("traced", "attribute"),
                entry("stringify", "call"),
            ],
            decorators: vec![DecoratorManifestEntry {
                module: "macroforge_ts".to_string(),
                export: "endec".to_string(),
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
        assert!(annotation_names_from_manifest(&manifest()).contains(&"endec".to_string()));
    }

    #[test]
    fn derive_and_call_macros_are_not_annotation_names() {
        let names = annotation_names_from_manifest(&manifest());
        assert!(!names.contains(&"Encode".to_string()));
        assert!(!names.contains(&"stringify".to_string()));
    }

    #[test]
    fn an_empty_manifest_yields_no_names() {
        assert!(annotation_names_from_manifest(&MacroManifest::default()).is_empty());
    }
}
