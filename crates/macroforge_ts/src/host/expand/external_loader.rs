use std::process::Command;

use napi::Status;

use crate::ts_syn::abi::{MacroContextIR, MacroResult};

// ============================================================================
// External Decorator Name Resolution
// ============================================================================

/// Cache for resolved external decorator names per package path.
static EXTERNAL_DECORATOR_CACHE: std::sync::LazyLock<dashmap::DashMap<String, Vec<String>>> =
    std::sync::LazyLock::new(dashmap::DashMap::new);

/// Resolve decorator annotation names from external macro packages imported in the source.
///
/// Parses `/** import macro { ... } from "package" */` comments, then spawns Node.js
/// to load each package's manifest and extract decorator export names. Results are cached.
pub(crate) fn resolve_external_decorator_names(
    source: &str,
    loader: Option<&ExternalMacroLoader>,
) -> Vec<String> {
    use crate::ts_syn::import_registry::collect_macro_import_comments_pub;

    let imports = collect_macro_import_comments_pub(source);
    if imports.is_empty() {
        return Vec::new();
    }

    // Deduplicate package paths
    let packages: Vec<String> = imports
        .values()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .cloned()
        .collect();

    let mut all_names = Vec::new();
    for pkg in &packages {
        // Check cache first
        if let Some(cached) = EXTERNAL_DECORATOR_CACHE.get(pkg) {
            all_names.extend(cached.value().clone());
            continue;
        }

        // Resolve via Node.js
        let Some(loader) = loader else {
            continue;
        };

        let names = loader.resolve_decorator_names(pkg);
        EXTERNAL_DECORATOR_CACHE.insert(pkg.clone(), names.clone());
        all_names.extend(names);
    }

    all_names
}

// ============================================================================
// External Macro Loader
// ============================================================================

pub(crate) struct ExternalMacroLoader {
    root_dir: std::path::PathBuf,
}

impl ExternalMacroLoader {
    pub(crate) fn new(root_dir: std::path::PathBuf) -> Self {
        Self { root_dir }
    }

    /// Resolve decorator export names from an external macro package's manifest.
    pub(crate) fn resolve_decorator_names(&self, package_path: &str) -> Vec<String> {
        let script = format!(
            r#"
const {{ createRequire }} = require('module');
const req = createRequire(process.cwd() + '/package.json');
try {{
  const m = req({pkg});
  if (m.__macroforgeGetManifest) {{
    const manifest = m.__macroforgeGetManifest();
    const names = (manifest.decorators || []).map(d => d.export);
    console.log(JSON.stringify(names));
  }} else {{
    console.log("[]");
  }}
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

    pub(crate) fn run_macro(&self, ctx: &MacroContextIR) -> napi::Result<MacroResult> {
        let fn_name = format!("__macroforgeRun{}", ctx.macro_name);
        let ctx_json =
            serde_json::to_string(ctx).map_err(|e| napi::Error::new(Status::InvalidArg, e))?;

        // Write context JSON to a temp file to avoid OS argument length limits
        // (E2BIG / "Argument list too long" when the serialized context is large).
        let ctx_file =
            std::env::temp_dir().join(format!("macroforge_ctx_{}.json", std::process::id()));
        std::fs::write(&ctx_file, &ctx_json).map_err(|e| {
            napi::Error::new(
                Status::GenericFailure,
                format!("Failed to write macro context to temp file: {e}"),
            )
        })?;

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

// Fallbacks: requested specifier and its absolute form
addCandidate(modulePath);
addCandidate(path.resolve(rootDir, modulePath));

// Heuristic: check monorepo subpaths even without a root package.json
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
            .map_err(|e| {
                napi::Error::new(
                    Status::GenericFailure,
                    format!("Failed to spawn node for external macro: {e}"),
                )
            })?;

        // Clean up the temp file
        let _ = std::fs::remove_file(&ctx_file);

        if !child.status.success() {
            let stderr = String::from_utf8_lossy(&child.stderr);
            return Err(napi::Error::new(
                Status::GenericFailure,
                format!("External macro runner failed: {stderr}"),
            ));
        }

        let result_json = String::from_utf8(child.stdout).map_err(|e| {
            napi::Error::new(
                Status::InvalidArg,
                format!("Macro runner returned non-UTF8 output: {e}"),
            )
        })?;

        let host_result: crate::ts_syn::abi::MacroResult = serde_json::from_str(&result_json)
            .map_err(|e| {
                napi::Error::new(
                    Status::InvalidArg,
                    format!("Failed to parse macro result: {e}"),
                )
            })?;

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
