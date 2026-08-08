//! Running wasm macro packages from the native host.
//!
//! `macroforge build` produces a wasm artifact, so without this the CLI could
//! not load the packages its own build command emits — only a JS host could.
//! This makes wasm a peer of `.node`/`.dylib` rather than a JS-only format.
//!
//! The contract is the same C ABI the native path uses, because that ABI was
//! never native-specific: pointers and lengths over linear memory is exactly
//! what a wasm export is.
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

use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};

/// A loaded wasm macro package.
pub(crate) struct WasmMacroModule {
    store: wasmi::Store<()>,
    instance: wasmi::Instance,
}

impl WasmMacroModule {
    /// Instantiates the module at `path`.
    pub(crate) fn load(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read wasm module at {}", path.display()))?;

        let engine = wasmi::Engine::default();
        let module = wasmi::Module::new(&engine, &bytes[..])
            .with_context(|| format!("failed to parse wasm module at {}", path.display()))?;
        let mut store = wasmi::Store::new(&engine, ());

        // A package built with the `wasm` feature also carries wasm-bindgen's
        // JS glue imports (`__wbg_*`, `__wbindgen_*`). They exist only for the
        // JsValue-returning entry points and are unreachable from the C ABI,
        // but instantiation still requires every import to resolve — so they
        // are satisfied with stubs that trap if anything ever does call them.
        // Trapping rather than returning a dummy value matters: a silent
        // wrong answer here would surface as corrupt macro output.
        let mut linker = wasmi::Linker::new(&engine);
        for import in module.imports() {
            let wasmi::ExternType::Func(func_type) = import.ty() else {
                bail!(
                    "wasm macro package imports a non-function `{}::{}`, which the \
                     native host cannot provide",
                    import.module(),
                    import.name(),
                );
            };

            let module_name = import.module().to_string();
            let field_name = import.name().to_string();
            let func = wasmi::Func::new(
                &mut store,
                func_type.clone(),
                move |_caller, _params, _results| {
                    Err(wasmi::Error::host(JsGlueCalled {
                        module: module_name.clone(),
                        name: field_name.clone(),
                    }))
                },
            );

            linker
                .define(import.module(), import.name(), func)
                .with_context(|| {
                    format!("failed to stub import `{}::{}`", import.module(), import.name())
                })?;
        }

        // Runs the module's `start` function, which wasm-bindgen output uses to
        // initialise its externref table before any export is callable.
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .with_context(|| format!("failed to instantiate {}", path.display()))?;

        Ok(Self { store, instance })
    }

    /// Calls a macro's `__macroforge_ffi_run_*` export with a JSON context.
    ///
    /// Returns `Ok(None)` when the export is absent, so the caller can fall
    /// through to another loading strategy rather than treating a
    /// package-without-this-macro as a hard failure.
    pub(crate) fn run(&mut self, symbol: &str, ctx_json: &str) -> Result<Option<String>> {
        let Some(run) = self
            .instance
            .get_typed_func::<(i32, i32, i32, i32), i32>(&self.store, symbol)
            .ok()
        else {
            return Ok(None);
        };

        // Two i32 slots for the module to write its result pointer and length
        // into, plus the context bytes themselves.
        let ctx_bytes = ctx_json.as_bytes();
        let out_ptr_addr = self.alloc(8)?;
        let out_len_addr = out_ptr_addr + 4;
        let ctx_addr = self.alloc(ctx_bytes.len() as i32)?;

        self.memory()?
            .write(&mut self.store, ctx_addr as usize, ctx_bytes)
            .map_err(|e| anyhow!("failed to write macro context into wasm memory: {e}"))?;

        let status = run
            .call(
                &mut self.store,
                (ctx_addr, ctx_bytes.len() as i32, out_ptr_addr, out_len_addr),
            )
            .map_err(|e| anyhow!("wasm macro `{symbol}` trapped: {e}"))?;

        let result_ptr = self.read_i32(out_ptr_addr)?;
        let result_len = self.read_i32(out_len_addr)?;
        let payload = self.read_string(result_ptr, result_len)?;

        self.free(result_ptr, result_len)?;
        self.free(ctx_addr, ctx_bytes.len() as i32)?;
        self.free(out_ptr_addr, 8)?;

        // Non-zero means the payload is an error message, not a MacroResult.
        if status != 0 {
            bail!("{payload}");
        }
        Ok(Some(payload))
    }

    /// Reads the package manifest via `__macroforge_ffi_get_manifest`.
    pub(crate) fn manifest(&mut self) -> Result<String> {
        let manifest = self
            .instance
            .get_typed_func::<(i32, i32), i32>(&self.store, "__macroforge_ffi_get_manifest")
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

    fn memory(&self) -> Result<wasmi::Memory> {
        self.instance
            .get_memory(&self.store, "memory")
            .ok_or_else(|| anyhow!("wasm macro package exports no `memory`"))
    }

    /// Allocator exports, in the order they are tried.
    ///
    /// The name depends on how the module was built. wasm-bindgen emits
    /// `__wbindgen_malloc`, but appends `_command_export` under its command
    /// model — which is what `macroforge build` produces. A bare `cdylib`
    /// compiled without wasm-bindgen exposes `__rust_alloc` instead.
    const ALLOCATORS: &'static [&'static str] = &[
        "__wbindgen_malloc",
        "__wbindgen_malloc_command_export",
        "__rust_alloc",
    ];

    /// Allocates `len` bytes inside the module.
    fn alloc(&mut self, len: i32) -> Result<i32> {
        for name in Self::ALLOCATORS {
            let Ok(malloc) = self
                .instance
                .get_typed_func::<(i32, i32), i32>(&self.store, name)
            else {
                continue;
            };
            // Second argument is the alignment; 1 is valid for byte buffers.
            return malloc
                .call(&mut self.store, (len, 1))
                .map_err(|e| anyhow!("wasm allocation via `{name}` failed: {e}"));
        }

        bail!(
            "wasm macro package exports no recognised allocator (tried {})",
            Self::ALLOCATORS.join(", "),
        )
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
            .instance
            .get_typed_func::<(i32, i32), ()>(&self.store, "__macroforge_ffi_free")
            .map_err(|_| anyhow!("wasm macro package exports no `__macroforge_ffi_free`"))?;
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
impl wasmi::errors::HostError for JsGlueCalled {}

/// Finds the wasm artifact for a package, if it ships one.
///
/// `macroforge build` writes to `<pkg>/pkg/<name>_bg.wasm`; a bare
/// `cargo build --target wasm32-*` leaves `<pkg>/<name>.wasm`. Both are checked
/// so a package built either way is loadable.
pub(crate) fn find_wasm_module(package_dir: &Path) -> Option<PathBuf> {
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

    // Deterministic across platforms, where readdir order is not.
    candidates.sort();
    candidates.into_iter().next()
}
