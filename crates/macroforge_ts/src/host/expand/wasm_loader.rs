//! Running wasm macro packages from the native host.
//!
//! `macroforge build` produces a wasm artifact, and this is how the CLI runs
//! it. The contract is a plain C ABI over pointers and lengths, which is
//! exactly what a wasm export over linear memory is.
//!
//! ```text
//! __macroforge_ffi_run_<macro>(ctx_ptr, ctx_len, out_ptr, out_len) -> i32
//! __macroforge_ffi_get_manifest(out_ptr, out_len)                  -> i32
//! __macroforge_ffi_free(ptr, len)
//! ```
//!
//! Values marshal through the module's exported `memory`: the host writes the
//! context JSON in, reads the result out, then hands the buffer back to
//! `__macroforge_ffi_free`.
//!
//! wasmtime compiles each package to machine code and keeps the result in an
//! on-disk cache, so a package is compiled once per build of it rather than
//! once per run.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};

use anyhow::{Context, Result, anyhow, bail};

use crate::host::file_stamp::FileStamp;

/// One engine for every module, so compiled code is shared across instances.
static ENGINE: LazyLock<Result<wasmtime::Engine, String>> = LazyLock::new(build_engine);

/// Where compiled packages are cached, when not wasmtime's default: the
/// user's cache directory.
const CACHE_DIR_VAR: &str = "MACROFORGE_WASM_CACHE_DIR";

/// The engine, with its compiled-module cache when one can be set up.
fn build_engine() -> Result<wasmtime::Engine, String> {
    let mut config = wasmtime::Config::new();
    let mut cache_config = wasmtime::CacheConfig::new();
    if let Some(dir) = std::env::var_os(CACHE_DIR_VAR) {
        cache_config.with_directory(PathBuf::from(dir));
    }
    match wasmtime::Cache::new(cache_config) {
        Ok(cache) => {
            config.cache(Some(cache));
        }
        // Without a cache every run compiles each package again: slower, but
        // still correct, so the cause is reported and the run goes on.
        Err(error) => eprintln!(
            "[macroforge] compiled macro packages will not be cached (set {CACHE_DIR_VAR} \
             to a writable directory): {error:?}"
        ),
    }
    wasmtime::Engine::new(&config).map_err(|error| format!("{error:?}"))
}

fn engine() -> Result<&'static wasmtime::Engine> {
    ENGINE
        .as_ref()
        .map_err(|error| anyhow!("failed to start the wasm engine: {error}"))
}

/// A package's module, compiled once for as long as its file is unchanged.
struct CompiledModule {
    module: wasmtime::Module,
    stamp: FileStamp,
}

/// Compiled modules, and the idle instances of each ready for the next call.
///
/// Instances are checked out for a call and returned after it, so expansions
/// running at once each get their own and never queue behind one another.
#[derive(Default)]
struct ModuleCache {
    compiled: HashMap<PathBuf, Arc<CompiledModule>>,
    idle: HashMap<PathBuf, Vec<WasmMacroModule>>,
}

static MODULES: LazyLock<Mutex<ModuleCache>> = LazyLock::new(Mutex::default);

fn lock_modules() -> Result<std::sync::MutexGuard<'static, ModuleCache>> {
    MODULES
        .lock()
        .map_err(|err| anyhow!("wasm module cache lock poisoned: {err}"))
}

/// The module at `path`, compiled now if it is new or its file changed.
fn compiled_module(path: &Path) -> Result<Arc<CompiledModule>> {
    let stamp = FileStamp::of(path)
        .with_context(|| format!("failed to stat wasm module at {}", path.display()))?;
    let mut cache = lock_modules()?;
    if let Some(compiled) = cache.compiled.get(path)
        && compiled.stamp == stamp
    {
        return Ok(Arc::clone(compiled));
    }
    let bytes = std::fs::read(path)
        .with_context(|| format!("failed to read wasm module at {}", path.display()))?;
    let module = wasmtime::Module::new(engine()?, &bytes[..]).map_err(|error| {
        anyhow!(
            "failed to compile wasm module at {}: {error:?}",
            path.display()
        )
    })?;
    let compiled = Arc::new(CompiledModule { module, stamp });
    cache
        .compiled
        .insert(path.to_path_buf(), Arc::clone(&compiled));
    // Instances of the previous build must not serve another call.
    cache.idle.remove(path);
    Ok(compiled)
}

/// Runs `call` against an instance of the module at `path`: an idle one when
/// there is one of the current build, a new one otherwise.
///
/// An instance goes back to the pool only after a call that succeeded; one
/// whose call failed may have been left mid-way and is dropped.
pub(crate) fn with_instance<R>(
    path: &Path,
    call: impl FnOnce(&mut WasmMacroModule) -> Result<R>,
) -> Result<R> {
    let compiled = compiled_module(path)?;
    let idle = lock_modules()?
        .idle
        .get_mut(path)
        .and_then(Vec::pop)
        .filter(|instance| instance.stamp == compiled.stamp);
    let mut instance = match idle {
        Some(instance) => instance,
        None => WasmMacroModule::instantiate(path, &compiled)?,
    };
    let result = call(&mut instance)?;
    let mut cache = lock_modules()?;
    if cache
        .compiled
        .get(path)
        .is_some_and(|current| current.stamp == instance.stamp)
    {
        cache
            .idle
            .entry(path.to_path_buf())
            .or_default()
            .push(instance);
    }
    Ok(result)
}

/// The module's allocator export, and the name it was found under.
#[derive(Clone)]
struct Allocator {
    func: wasmtime::TypedFunc<(i32, i32), i32>,
    name: &'static str,
}

/// A macro's `__macroforge_ffi_run_*` export.
type RunFunc = wasmtime::TypedFunc<(i32, i32, i32, i32), i32>;

/// An instantiated wasm macro package, with its exports resolved once.
pub(crate) struct WasmMacroModule {
    store: wasmtime::Store<()>,
    instance: wasmtime::Instance,
    memory: Option<wasmtime::Memory>,
    malloc: Option<Allocator>,
    free: Option<wasmtime::TypedFunc<(i32, i32), ()>>,
    runs: HashMap<String, Option<RunFunc>>,
    /// `__macroforge_ffi_set_registry`, which a guest built before resident
    /// registries lacks.
    set_registry: Option<RunFunc>,
    /// The registry generation last installed in this instance.
    resident: Option<crate::ts_syn::abi::ir::type_registry::RegistryGeneration>,
    stamp: FileStamp,
}

impl WasmMacroModule {
    /// Instantiates `compiled`, the module at `path`.
    fn instantiate(path: &Path, compiled: &CompiledModule) -> Result<Self> {
        let engine = engine()?;
        let mut store = wasmtime::Store::new(engine, ());

        // A package built with the `wasm` feature also carries wasm-bindgen's
        // JS glue imports (`__wbg_*`, `__wbindgen_*`). They exist only for the
        // JsValue-returning entry points and are unreachable from the C ABI,
        // but instantiation still requires every import to resolve, so they
        // are satisfied with stubs that trap if anything ever does call them.
        // Trapping rather than returning a dummy value matters: a silent
        // wrong answer here would surface as corrupt macro output.
        let mut linker = wasmtime::Linker::new(engine);
        for import in compiled.module.imports() {
            let wasmtime::ExternType::Func(func_type) = import.ty() else {
                bail!(
                    "wasm macro package imports a non-function `{}::{}`, which the \
                     native host cannot provide",
                    import.module(),
                    import.name(),
                );
            };

            let module_name = import.module().to_string();
            let field_name = import.name().to_string();
            let func = wasmtime::Func::new(&mut store, func_type.clone(), move |_, _, _| {
                Err(wasmtime::Error::new(JsGlueCalled {
                    module: module_name.clone(),
                    name: field_name.clone(),
                }))
            });

            linker
                .define(&store, import.module(), import.name(), func)
                .map_err(|error| {
                    anyhow!(
                        "failed to stub import `{}::{}`: {error:?}",
                        import.module(),
                        import.name()
                    )
                })?;
        }

        // Runs the module's `start` function, which wasm-bindgen output uses to
        // initialise its externref table before any export is callable.
        let instance = linker
            .instantiate(&mut store, &compiled.module)
            .map_err(|error| anyhow!("failed to instantiate {}: {error:?}", path.display()))?;

        let memory = instance.get_memory(&mut store, "memory");
        let malloc = Self::ALLOCATORS.iter().find_map(|name| {
            instance
                .get_typed_func::<(i32, i32), i32>(&mut store, name)
                .ok()
                .map(|func| Allocator { func, name })
        });
        let free = instance
            .get_typed_func::<(i32, i32), ()>(&mut store, "__macroforge_ffi_free")
            .ok();
        let set_registry = instance
            .get_typed_func::<(i32, i32, i32, i32), i32>(
                &mut store,
                "__macroforge_ffi_set_registry",
            )
            .ok();

        Ok(Self {
            store,
            instance,
            memory,
            malloc,
            free,
            runs: HashMap::new(),
            set_registry,
            resident: None,
            stamp: compiled.stamp,
        })
    }

    /// Calls a macro's `__macroforge_ffi_run_*` export with a JSON context.
    ///
    /// Returns `Ok(None)` when the export is absent, so the caller can fall
    /// through to another loading strategy rather than treating a
    /// package-without-this-macro as a hard failure.
    pub(crate) fn run(&mut self, symbol: &str, ctx_json: &str) -> Result<Option<String>> {
        let run = match self.runs.get(symbol) {
            Some(resolved) => resolved.clone(),
            None => {
                let resolved = self
                    .instance
                    .get_typed_func::<(i32, i32, i32, i32), i32>(&mut self.store, symbol)
                    .ok();
                self.runs.insert(symbol.to_string(), resolved.clone());
                resolved
            }
        };
        let Some(run) = run else {
            return Ok(None);
        };

        let (status, payload) =
            self.call_with_input(run, ctx_json, &format!("macro `{symbol}`"), "macro context")?;
        // Non-zero means the payload is an error message, not a MacroResult.
        if status != 0 {
            bail!("{payload}");
        }
        Ok(Some(payload))
    }

    /// Whether this guest takes a registry once and contexts that refer to it.
    pub(crate) fn accepts_resident_registry(&self) -> bool {
        self.set_registry.is_some()
    }

    /// Installs the registry of `generation` in this instance unless it holds
    /// it already. `payload` builds the JSON to send, only when it is sent.
    pub(crate) fn ensure_resident(
        &mut self,
        generation: crate::ts_syn::abi::ir::type_registry::RegistryGeneration,
        payload: impl FnOnce() -> Result<Arc<str>>,
    ) -> Result<()> {
        if self.resident == Some(generation) {
            return Ok(());
        }
        let Some(set_registry) = self.set_registry.clone() else {
            bail!("this macro package exports no `__macroforge_ffi_set_registry`");
        };
        let payload = payload()?;
        let (status, message) = self.call_with_input(
            set_registry,
            &payload,
            "export `__macroforge_ffi_set_registry`",
            "type registry",
        )?;
        if status != 0 {
            bail!("installing the type registry failed: {message}");
        }
        self.resident = Some(generation);
        Ok(())
    }

    /// Calls `export`, described as `export_label`, with `input` (an
    /// `input_label`) written into the module's memory, and returns its
    /// status and the string it wrote back.
    fn call_with_input(
        &mut self,
        export: RunFunc,
        input: &str,
        export_label: &str,
        input_label: &str,
    ) -> Result<(i32, String)> {
        // Two i32 slots for the module to write its result pointer and length
        // into, plus the input bytes themselves.
        let input_bytes = input.as_bytes();
        let out_ptr_addr = self.alloc(8)?;
        let out_len_addr = out_ptr_addr + 4;
        let input_addr = self.alloc(input_bytes.len() as i32)?;

        // Freshly allocated memory holds whatever was freed there last; an
        // export that writes no result must read back as an empty one.
        self.memory()?
            .write(&mut self.store, out_ptr_addr as usize, &[0u8; 8])
            .map_err(|e| anyhow!("failed to clear the result slots in wasm memory: {e}"))?;

        self.memory()?
            .write(&mut self.store, input_addr as usize, input_bytes)
            .map_err(|e| anyhow!("failed to write {input_label} into wasm memory: {e}"))?;

        let status = export
            .call(
                &mut self.store,
                (
                    input_addr,
                    input_bytes.len() as i32,
                    out_ptr_addr,
                    out_len_addr,
                ),
            )
            .map_err(|e| anyhow!("wasm {export_label} trapped: {e}"))?;

        let result_ptr = self.read_i32(out_ptr_addr)?;
        let result_len = self.read_i32(out_len_addr)?;
        let payload = self.read_string(result_ptr, result_len)?;

        self.free(result_ptr, result_len)?;
        self.free(input_addr, input_bytes.len() as i32)?;
        self.free(out_ptr_addr, 8)?;
        Ok((status, payload))
    }

    /// Reads the package manifest via `__macroforge_ffi_get_manifest`.
    pub(crate) fn manifest(&mut self) -> Result<String> {
        let manifest = self
            .instance
            .get_typed_func::<(i32, i32), i32>(&mut self.store, "__macroforge_ffi_get_manifest")
            .map_err(|_| anyhow!("wasm package exports no `__macroforge_ffi_get_manifest`"))?;

        let out_ptr_addr = self.alloc(8)?;
        let out_len_addr = out_ptr_addr + 4;

        let status = manifest
            .call(&mut self.store, (out_ptr_addr, out_len_addr))
            .map_err(|e| anyhow!("manifest export trapped: {e}"))?;

        let result_ptr = self.read_i32(out_ptr_addr)?;
        let result_len = self.read_i32(out_len_addr)?;
        let payload = self.read_string(result_ptr, result_len)?;

        self.free(result_ptr, result_len)?;
        self.free(out_ptr_addr, 8)?;

        if status != 0 {
            bail!("{payload}");
        }
        Ok(payload)
    }

    fn memory(&self) -> Result<wasmtime::Memory> {
        self.memory
            .ok_or_else(|| anyhow!("wasm macro package exports no `memory`"))
    }

    /// Allocator exports, in the order they are tried.
    ///
    /// The name depends on how the module was built. wasm-bindgen emits
    /// `__wbindgen_malloc`, but appends `_command_export` under its command
    /// model, which is what `macroforge build` produces. A bare `cdylib`
    /// compiled without wasm-bindgen exposes `__rust_alloc` instead.
    const ALLOCATORS: &'static [&'static str] = &[
        "__wbindgen_malloc",
        "__wbindgen_malloc_command_export",
        "__rust_alloc",
    ];

    /// Allocates `len` bytes inside the module.
    fn alloc(&mut self, len: i32) -> Result<i32> {
        let Some(Allocator { func: malloc, name }) = self.malloc.clone() else {
            bail!(
                "wasm macro package exports no recognised allocator (tried {})",
                Self::ALLOCATORS.join(", "),
            );
        };
        // Second argument is the alignment; 1 is valid for byte buffers.
        malloc
            .call(&mut self.store, (len, 1))
            .map_err(|e| anyhow!("wasm allocation via `{name}` failed: {e}"))
    }

    /// Releases a buffer through the module's own deallocator.
    ///
    /// Failures are surfaced rather than swallowed: a leak here is unbounded
    /// across a watch session, which expands thousands of files.
    fn free(&mut self, ptr: i32, len: i32) -> Result<()> {
        if ptr == 0 || len == 0 {
            return Ok(());
        }
        let free = self
            .free
            .clone()
            .ok_or_else(|| anyhow!("wasm macro package exports no `__macroforge_ffi_free`"))?;
        free.call(&mut self.store, (ptr, len))
            .map_err(|e| anyhow!("wasm deallocation failed: {e}"))
    }

    fn read_i32(&self, addr: i32) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.memory()?
            .read(&self.store, addr as usize, &mut buf)
            .map_err(|e| anyhow!("failed to read from wasm memory: {e}"))?;
        Ok(i32::from_le_bytes(buf))
    }

    fn read_string(&self, ptr: i32, len: i32) -> Result<String> {
        if ptr == 0 || len == 0 {
            return Ok(String::new());
        }
        let mut buf = vec![0u8; len as usize];
        self.memory()?
            .read(&self.store, ptr as usize, &mut buf)
            .map_err(|e| anyhow!("failed to read from wasm memory: {e}"))?;
        String::from_utf8(buf).map_err(|e| anyhow!("wasm macro returned invalid UTF-8: {e}"))
    }
}

/// Raised when a stubbed wasm-bindgen import is actually invoked.
#[derive(Debug)]
struct JsGlueCalled {
    module: String,
    name: String,
}

impl std::fmt::Display for JsGlueCalled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "wasm macro called the JavaScript binding `{}::{}`, which the native \
             host cannot provide. The C-ABI entry points must not depend on \
             JsValue.",
            self.module, self.name,
        )
    }
}

impl std::error::Error for JsGlueCalled {}

/// The part of a package's `package.json` that names its entry module.
#[derive(serde::Deserialize)]
struct PackageManifest {
    main: Option<String>,
}

/// Finds the wasm artifact for a package, if it ships one.
///
/// `macroforge build` writes `<pkg>/pkg/<name>.js` with `<name>_bg.wasm` beside
/// it, and the package's `main` names the glue, so that is the wasm the
/// package means even when the directory holds others left by an earlier
/// build under another name. Without a `main`, a lone wasm in `<pkg>/pkg` or
/// `<pkg>` (a bare `cargo build --target wasm32-*`) is used.
pub(crate) fn find_wasm_module(package_dir: &Path) -> Result<Option<PathBuf>> {
    let manifest_path = package_dir.join("package.json");
    if manifest_path.is_file() {
        let text = std::fs::read_to_string(&manifest_path)
            .with_context(|| format!("failed to read {}", manifest_path.display()))?;
        let manifest: PackageManifest = serde_json::from_str(&text)
            .with_context(|| format!("failed to parse {}", manifest_path.display()))?;
        if let Some(main) = manifest.main {
            let glue = package_dir.join(main);
            if let (Some(dir), Some(stem)) = (glue.parent(), glue.file_stem()) {
                let stem = stem.to_string_lossy();
                for name in [format!("{stem}_bg.wasm"), format!("{stem}.wasm")] {
                    let wasm = dir.join(name);
                    if wasm.is_file() {
                        return Ok(Some(wasm));
                    }
                }
            }
        }
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    for dir in [package_dir.join("pkg"), package_dir.to_path_buf()] {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("wasm") {
                candidates.push(path);
            }
        }
    }
    match candidates.as_slice() {
        [] => Ok(None),
        [only] => Ok(Some(only.clone())),
        _ => {
            candidates.sort();
            bail!(
                "{} has several wasm modules ({}) and no package.json `main` naming one",
                package_dir.display(),
                candidates
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{find_wasm_module, with_instance};
    use std::path::PathBuf;

    /// The testground's macro package, built from this checkout's guest code
    /// by `mf test` before the Rust tests run.
    fn testground_macro() -> PathBuf {
        let package =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tooling/testground/macro");
        match find_wasm_module(&package) {
            Ok(Some(wasm)) => wasm,
            Ok(None) => panic!(
                "no wasm in {}; build it with `target/debug/mf test testground` or \
                 `pixi run build:testground:macro`",
                package.display()
            ),
            Err(error) => panic!("finding the testground wasm failed: {error:#}"),
        }
    }

    fn package_with(files: &[(&str, &str)]) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "mf-wasm-lookup-{}-{}",
            std::process::id(),
            files.len()
        ));
        for (name, contents) in files {
            let path = dir.join(name);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .unwrap_or_else(|error| panic!("creating {}: {error}", parent.display()));
            }
            std::fs::write(&path, contents)
                .unwrap_or_else(|error| panic!("writing {}: {error}", path.display()));
        }
        dir
    }

    /// Removes a scratch package, reporting a failure rather than failing the
    /// test that already ran.
    fn remove_package(dir: &std::path::Path) {
        if let Err(error) = std::fs::remove_dir_all(dir) {
            eprintln!("could not remove {}: {error}", dir.display());
        }
    }

    #[test]
    fn the_wasm_beside_the_main_glue_wins_over_a_stale_one() {
        let dir = package_with(&[
            ("package.json", r#"{ "main": "pkg/current.js" }"#),
            ("pkg/current.js", ""),
            ("pkg/current_bg.wasm", ""),
            ("pkg/aaa_stale_bg.wasm", ""),
        ]);
        let found = find_wasm_module(&dir);
        remove_package(&dir);
        assert_eq!(found.ok().flatten(), Some(dir.join("pkg/current_bg.wasm")));
    }

    #[test]
    fn several_wasm_modules_without_a_main_are_an_error() {
        let dir = package_with(&[("pkg/one_bg.wasm", ""), ("pkg/two_bg.wasm", ""), ("x", "")]);
        let found = find_wasm_module(&dir);
        remove_package(&dir);
        assert!(found.is_err(), "{found:?}");
    }

    #[test]
    fn a_guest_built_from_this_checkout_takes_resident_registries() {
        let accepts = with_instance(&testground_macro(), |module| {
            Ok(module.accepts_resident_registry())
        })
        .expect("the testground macro package instantiates");
        assert!(
            accepts,
            "a guest built from this checkout must export `__macroforge_ffi_set_registry`"
        );
    }
}
