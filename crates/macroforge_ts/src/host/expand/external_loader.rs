use anyhow::{Context, anyhow, bail};
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;

use crate::ts_syn::abi::{MacroContextIR, MacroResult};

// ============================================================================
// External Macro Loader
// ============================================================================

pub(crate) struct ExternalMacroLoader {
    #[cfg(not(target_arch = "wasm32"))]
    root_dir: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl ExternalMacroLoader {
    pub(crate) fn new(root_dir: std::path::PathBuf) -> Self {
        Self { root_dir }
    }

    /// Resolve decorator export names from an external macro package's manifest.
    pub(crate) fn resolve_decorator_names(&self, package_path: &str) -> Vec<String> {
        let script = format!(
            r#"
const {{ createRequire }} = require('module');
const path = require('path');
const fs = require('fs');
const rootDir = process.cwd();
const req = createRequire(path.join(rootDir, 'package.json'));
const candidates = [];
const seen = new Set();

const addCandidate = (id) => {{
  if (!id) return;
  const key = id.startsWith('.') || id.startsWith('/') ? path.resolve(rootDir, id) : id;
  if (seen.has(key)) return;
  seen.add(key);
  candidates.push(id);
}};

const addPackageDir = (dir) => {{
  try {{
    const pkgJsonPath = path.join(dir, 'package.json');
    if (!fs.existsSync(pkgJsonPath)) return;
    const pkgJson = JSON.parse(fs.readFileSync(pkgJsonPath, 'utf8'));
    addCandidate(pkgJson.name || dir);
    addCandidate(dir);
  }} catch {{}}
}};

addCandidate({pkg});
addCandidate(path.resolve(rootDir, {pkg}));
addPackageDir(path.join(rootDir, 'tooling', 'playground', 'macro'));
addPackageDir(path.join(rootDir, 'playground', 'macro'));

const packagesDir = path.join(rootDir, 'packages');
if (fs.existsSync(packagesDir)) {{
  for (const entry of fs.readdirSync(packagesDir, {{ withFileTypes: true }})) {{
    if (entry.isDirectory()) {{
      addPackageDir(path.join(packagesDir, entry.name));
    }}
  }}
}}

try {{
  for (const candidate of candidates) {{
    try {{
      const m = req(candidate);
      let found = false;
      const names = [];
      if (m.__macroforgeGetManifest) {{
        const manifest = m.__macroforgeGetManifest();
        names.push(...(manifest.decorators || []).map(d => d.export));
        found = true;
      }}
      for (const key of Object.keys(m)) {{
        if (key.startsWith('__macroforgeGetManifest_') && typeof m[key] === 'function') {{
          const manifest = m[key]();
          names.push(...(manifest.decorators || []).map(d => d.export));
          found = true;
        }}
      }}
      if (found) {{
        console.log(JSON.stringify([...new Set(names)]));
        process.exit(0);
      }}
    }} catch {{}}
  }}
  console.log("[]");
}} catch (e) {{
  console.error('[macroforge] Failed to load manifest from ' + {pkg} + ':', e.message);
  console.log("[]");
}}
"#,
            pkg = serde_json::to_string(package_path).unwrap_or_default()
        );

        let output = Command::new("node")
            .arg("-e")
            .arg(&script)
            .current_dir(&self.root_dir)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                serde_json::from_str::<Vec<String>>(stdout.trim()).unwrap_or_default()
            }
            _ => Vec::new(),
        }
    }

    pub(crate) fn run_macro(&self, ctx: &MacroContextIR) -> anyhow::Result<MacroResult> {
        let fn_name = format!("__macroforgeRun{}", ctx.macro_name);
        let ctx_json =
            serde_json::to_string(ctx).map_err(|e| anyhow!("Failed to serialize context: {e}"))?;

        // Write context JSON to a temp file to avoid OS argument length limits
        // (E2BIG / "Argument list too long" when the serialized context is large).
        let ctx_file =
            std::env::temp_dir().join(format!("macroforge_ctx_{}.json", std::process::id()));
        std::fs::write(&ctx_file, &ctx_json)
            .context("Failed to write macro context to temp file")?;

        let script = r#"
const [modulePath, fnName, ctxFile, rootDir] = process.argv.slice(1);
const path = require('path');
const fs = require('fs');
const { pathToFileURL } = require('url');

const ctxJson = fs.readFileSync(ctxFile, 'utf8');

const normalizeWorkspaces = (val) =>
  Array.isArray(val) ? val : (val && Array.isArray(val.packages) ? val.packages : []);

const toImportSpecifier = (id) => {
  if (id.startsWith('.') || id.startsWith('/')) {
    return pathToFileURL(path.resolve(rootDir, id)).href;
  }
  return id;
};

const expandWorkspace = (pattern) => {
  if (typeof pattern !== 'string') return [];
  const absolute = path.resolve(rootDir, pattern);
  if (!pattern.includes('*')) {
    return [absolute];
  }

  const starIdx = pattern.indexOf('*');
  const baseDir = path.resolve(rootDir, pattern.slice(0, starIdx));
  const suffix = pattern.slice(starIdx + 1);
  if (!fs.existsSync(baseDir)) return [];

  return fs
    .readdirSync(baseDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(baseDir, entry.name + suffix));
};

const candidates = [];
const seen = new Set();
const addCandidate = (id) => {
  if (!id) return;
  const key = id.startsWith('.') || id.startsWith('/') ? path.resolve(rootDir, id) : id;
  if (seen.has(key)) return;
  seen.add(key);
  candidates.push(id);
};

const addPackageDir = (dir) => {
  try {
    const pkgJsonPath = path.join(dir, 'package.json');
    if (!fs.existsSync(pkgJsonPath)) return;
    const pkgJson = JSON.parse(fs.readFileSync(pkgJsonPath, 'utf8'));
    addCandidate(pkgJson.name || dir);
    addCandidate(dir);
  } catch {}
};

let ctx;
try {
  ctx = JSON.parse(ctxJson);
} catch {}

// Prefer node_modules near the file being processed (walk upward toward rootDir)
if (ctx?.file_name) {
  let current = path.resolve(rootDir, path.dirname(ctx.file_name));
  const rootResolved = path.resolve(rootDir);
  while (true) {
    addCandidate(path.join(current, 'node_modules', modulePath));
    const parent = path.dirname(current);
    if (parent === current || !path.resolve(parent).startsWith(rootResolved)) break;
    current = parent;
  }
}

// Heuristic: check monorepo subpaths even without a root package.json
addPackageDir(path.join(rootDir, 'tooling', 'playground', 'macro'));
addPackageDir(path.join(rootDir, 'playground', 'macro'));
const packagesDir = path.join(rootDir, 'packages');
if (fs.existsSync(packagesDir)) {
  for (const entry of fs.readdirSync(packagesDir, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    addPackageDir(path.join(packagesDir, entry.name));
  }
}

try {
  const rootPkg = JSON.parse(fs.readFileSync(path.join(rootDir, 'package.json'), 'utf8'));
  const workspaces = normalizeWorkspaces(rootPkg.workspaces);

  for (const ws of workspaces) {
    for (const pkgDir of expandWorkspace(ws)) {
      try {
        const pkgJsonPath = path.join(pkgDir, 'package.json');
        if (!fs.existsSync(pkgJsonPath)) continue;

        const pkgJson = JSON.parse(fs.readFileSync(pkgJsonPath, 'utf8'));
        addCandidate(pkgJson.name || pkgDir);
        addCandidate(pkgDir);
      } catch {}
    }
  }
} catch {}

// Fallbacks: requested specifier and its absolute form
addCandidate(modulePath);
addCandidate(path.resolve(rootDir, modulePath));

const tryRequire = (id) => {
  try {
    return { module: require(id), loader: 'require' };
  } catch (error) {
    return { error };
  }
};

const tryImport = async (id) => {
  try {
    return { module: await import(toImportSpecifier(id)), loader: 'import' };
  } catch (error) {
    return { error };
  }
};

(async () => {
  const errors = [];

  for (const id of candidates) {
    let loaded = tryRequire(id);

    if (!loaded.module) {
      const imported = await tryImport(id);
      if (imported.module) {
        loaded = imported;
      } else {
        errors.push(
          `Failed to load '${id}' via require/import: ${
            imported.error?.message || loaded.error?.message || 'unknown error'
          }`
        );
        continue;
      }
    }

    const mod = loaded.module;
    const fn =
      mod?.[fnName] ||
      mod?.default?.[fnName] ||
      (typeof mod?.default === 'object' ? mod.default[fnName] : undefined);

    if (typeof fn !== 'function') {
      errors.push(`Module '${id}' loaded via ${loaded.loader} but missing export '${fnName}'`);
      continue;
    }

    const out = await fn(ctxJson);
    if (typeof out === 'string') {
      await new Promise((resolve, reject) => {
        process.stdout.write(out, (err) => err ? reject(err) : resolve());
      });
      process.exit(0);
    }

    errors.push(`Macro '${fnName}' in '${id}' returned ${typeof out}, expected string`);
  }

  if (errors.length === 0) {
    errors.push('Macro not found in any workspace candidate');
  }

  console.error(errors.join('\n'));
  process.exit(2);
})().catch((err) => {
  console.error(err?.stack || String(err));
  process.exit(1);
});
"#;

        let child = Command::new("node")
            .current_dir(&self.root_dir)
            .arg("-e")
            .arg(script)
            .arg(&ctx.module_path)
            .arg(&fn_name)
            .arg(ctx_file.to_string_lossy().as_ref())
            .arg(self.root_dir.to_string_lossy().as_ref())
            .output()
            .context("Failed to spawn node for external macro")?;

        // Clean up the temp file
        let _ = std::fs::remove_file(&ctx_file);

        if !child.status.success() {
            let stderr = String::from_utf8_lossy(&child.stderr);
            bail!("External macro runner failed: {stderr}");
        }

        let result_json =
            String::from_utf8(child.stdout).context("Macro runner returned non-UTF8 output")?;

        let host_result: crate::ts_syn::abi::MacroResult =
            serde_json::from_str(&result_json).context("Failed to parse macro result")?;

        Ok(MacroResult {
            runtime_patches: host_result.runtime_patches,
            type_patches: host_result.type_patches,
            diagnostics: host_result.diagnostics,
            tokens: host_result.tokens,
            insert_pos: host_result.insert_pos,
            debug: host_result.debug,
            cross_module_suffixes: host_result.cross_module_suffixes,
            cross_module_type_suffixes: host_result.cross_module_type_suffixes,
            imports: host_result.imports,
        })
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
