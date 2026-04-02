use anyhow::{Context, Result};
use std::{fs, path::PathBuf, sync::Mutex};

/// Cached path to the type registry JSON file (built once per process via scanProjectSync).
pub(crate) static TYPE_REGISTRY_CACHE_PATH: Mutex<Option<String>> = Mutex::new(None);

/// Builds the type registry via a one-shot Node.js scanProjectSync call and
/// caches the result to a temp file. Subsequent calls are no-ops.
pub(crate) fn ensure_type_registry_cache() {
    let mut cached = TYPE_REGISTRY_CACHE_PATH.lock().unwrap();
    if cached.is_some() {
        return;
    }

    let scan_script = r#"
const { createRequire } = require('module');
const fs = require('fs');
const cwdRequire = createRequire(process.cwd() + '/package.json');

try {
  const macroforge = cwdRequire('macroforge');
  if (!macroforge.scanProjectSync) {
    process.exit(0);
  }
  const result = macroforge.scanProjectSync(process.cwd(), { exportedOnly: false });
  const outPath = process.argv[2];
  fs.writeFileSync(outPath, result.registryJson);
  console.log(JSON.stringify({ ok: true, types: result.typesFound, files: result.filesScanned }));
} catch (err) {
  console.error('Type scan failed:', err.message);
  process.exit(0);
}
"#;

    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("macroforge-cli");
    let _ = fs::create_dir_all(&temp_dir);
    let script_path = temp_dir.join("scan-project-wrapper.js");
    let registry_path = temp_dir.join("type-registry.json");
    let _ = fs::write(&script_path, scan_script);

    let cwd = std::env::current_dir().unwrap_or_default();
    if let Ok(output) = std::process::Command::new("node")
        .arg(&script_path)
        .arg(&registry_path)
        .current_dir(&cwd)
        .output()
        && output.status.success()
        && registry_path.exists()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(info) = serde_json::from_str::<serde_json::Value>(&stdout)
            && info.get("ok").and_then(|v| v.as_bool()).unwrap_or(false)
        {
            let types = info.get("types").and_then(|v| v.as_u64()).unwrap_or(0);
            let files = info.get("files").and_then(|v| v.as_u64()).unwrap_or(0);
            eprintln!(
                "[macroforge] Type scan: {} types from {} files",
                types, files
            );
            *cached = Some(registry_path.to_string_lossy().to_string());
            return;
        }
    }

    // Scan failed or not available — continue without type registry
    *cached = None;
}

/// Runs TypeScript type checking with macro expansion baked into file reads.
///
/// This function creates a Node.js script that wraps `tsc --noEmit` behavior
/// while transparently expanding macros when reading `.ts` and `.tsx` files.
/// Files containing `@derive` are expanded before being passed to the TypeScript
/// compiler.
///
/// The wrapper intercepts `CompilerHost.getSourceFile` to:
/// 1. Read the file contents
/// 2. Check for `@derive` decorators
/// 3. Expand macros if found
/// 4. Return the expanded source to TypeScript
///
/// # Arguments
///
/// * `project` - Optional path to `tsconfig.json` (defaults to `tsconfig.json` in cwd)
///
/// # Returns
///
/// Returns `Ok(())` if type checking passes, or an error with diagnostic details.
pub fn run_tsc_wrapper(project: Option<PathBuf>) -> Result<()> {
    // Build the type registry before launching tsc so that expandSync
    // can resolve cross-module type references.
    ensure_type_registry_cache();
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();

    // Write a temporary Node.js script that wraps tsc and expands macros on file load
    let script = r#"
const { createRequire } = require('module');
const fs = require('fs');
const cwdRequire = createRequire(process.cwd() + '/package.json');
const ts = cwdRequire('typescript');
const macros = cwdRequire('macroforge');
const path = require('path');

const projectArg = process.argv[2] || 'tsconfig.json';
const configPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, projectArg);
if (!configPath) {
  console.error(`[macroforge] tsconfig not found: ${projectArg}`);
  process.exit(1);
}

// Find and load macroforge config for foreign types
const CONFIG_FILES = [
  'macroforge.config.ts',
  'macroforge.config.mts',
  'macroforge.config.js',
  'macroforge.config.mjs',
  'macroforge.config.cjs',
];
let macroConfigPath = null;
let currentDir = process.cwd();
while (true) {
  for (const filename of CONFIG_FILES) {
    const candidate = path.join(currentDir, filename);
    if (fs.existsSync(candidate)) {
      macroConfigPath = candidate;
      break;
    }
  }
  if (macroConfigPath) break;
  // Stop at package.json boundary
  if (fs.existsSync(path.join(currentDir, 'package.json'))) break;
  const parent = path.dirname(currentDir);
  if (parent === currentDir) break;
  currentDir = parent;
}

// Load the config if found (caches foreign types in native plugin)
if (macroConfigPath) {
  try {
    const configContent = fs.readFileSync(macroConfigPath, 'utf8');
    macros.loadConfig(configContent, macroConfigPath);
  } catch (e) {
    // Config load failed, continue without foreign types
  }
}

// Load pre-built type registry if available
const typeRegistryPath = process.env.MACROFORGE_TYPE_REGISTRY_PATH;
let typeRegistryJson = undefined;
if (typeRegistryPath) {
  try {
    typeRegistryJson = fs.readFileSync(typeRegistryPath, 'utf8');
  } catch {}
}

const configFile = ts.readConfigFile(configPath, ts.sys.readFile);
if (configFile.error) {
  console.error(ts.formatDiagnostic(configFile.error, {
    getCanonicalFileName: (f) => f,
    getCurrentDirectory: ts.sys.getCurrentDirectory,
    getNewLine: () => ts.sys.newLine
  }));
  process.exit(1);
}

const parsed = ts.parseJsonConfigFileContent(
  configFile.config,
  ts.sys,
  path.dirname(configPath)
);

const options = { ...parsed.options, noEmit: true };
const formatHost = {
  getCanonicalFileName: (f) => f,
  getCurrentDirectory: ts.sys.getCurrentDirectory,
  getNewLine: () => ts.sys.newLine,
};

const host = ts.createCompilerHost(options);
const origGetSourceFile = host.getSourceFile.bind(host);
host.getSourceFile = (fileName, languageVersion, ...rest) => {
  try {
    if (
      (fileName.endsWith('.ts') || fileName.endsWith('.tsx')) &&
      !fileName.endsWith('.d.ts')
    ) {
      const sourceText = ts.sys.readFile(fileName);
      if (sourceText && sourceText.includes('@derive')) {
        const expandOpts = {};
        if (macroConfigPath) expandOpts.configPath = macroConfigPath;
        if (typeRegistryJson) expandOpts.typeRegistryJson = typeRegistryJson;
        const expanded = macros.expandSync(sourceText, fileName, expandOpts);
        const text = expanded.code || sourceText;
        return ts.createSourceFile(fileName, text, languageVersion, true);
      }
    }
  } catch (e) {
    // fall through to original host
  }
  return origGetSourceFile(fileName, languageVersion, ...rest);
};

const program = ts.createProgram(parsed.fileNames, options, host);
const diagnostics = ts.getPreEmitDiagnostics(program);
if (diagnostics.length) {
  diagnostics.forEach((d) => {
    const msg = ts.formatDiagnostic(d, formatHost);
    console.error(msg.trimEnd());
  });
}
const hasError = diagnostics.some((d) => d.category === ts.DiagnosticCategory.Error);
process.exit(hasError ? 1 : 0);
"#;

    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("macroforge-cli");
    fs::create_dir_all(&temp_dir)?;
    let script_path = temp_dir.join("tsc-wrapper.js");
    fs::write(&script_path, script)?;

    let project_arg = project
        .unwrap_or_else(|| PathBuf::from("tsconfig.json"))
        .to_string_lossy()
        .to_string();

    let mut cmd = std::process::Command::new("node");
    cmd.arg(script_path).arg(project_arg);
    if let Some(ref rp) = registry_path {
        cmd.env("MACROFORGE_TYPE_REGISTRY_PATH", rp);
    }
    let status = cmd.status().context("failed to run node tsc wrapper")?;

    if !status.success() {
        anyhow::bail!("tsc wrapper exited with status {}", status);
    }

    Ok(())
}

/// Runs svelte-check with macro expansion baked into file reads.
///
/// This function creates a Node.js script that patches `ts.sys.readFile` before
/// loading svelte-check. Since svelte-check uses TypeScript as a peer dependency
/// and Node.js caches modules, the patched `ts.sys.readFile` is shared with
/// svelte-check's internal TypeScript language service.
///
/// Files containing `@derive` are expanded before being passed to svelte-check.
///
/// # Arguments
///
/// * `workspace` - Optional workspace directory (defaults to cwd)
/// * `tsconfig` - Optional path to `tsconfig.json`
/// * `output` - Optional output format (human, human-verbose, machine, machine-verbose)
/// * `fail_on_warnings` - If true, exit with error on warnings
///
/// # Returns
///
/// Returns `Ok(())` if svelte-check passes, or an error with diagnostic details.
pub fn run_svelte_check_wrapper(
    workspace: Option<PathBuf>,
    tsconfig: Option<PathBuf>,
    output: Option<String>,
    fail_on_warnings: bool,
) -> Result<()> {
    // Build the type registry before launching svelte-check so that
    // expandSync can resolve cross-module type references (e.g., enum
    // fieldset variants defined in separate files).
    ensure_type_registry_cache();
    let registry_path = TYPE_REGISTRY_CACHE_PATH.lock().unwrap().clone();

    let script = r#"
const { createRequire } = require('module');
const fs = require('fs');
const path = require('path');
const cwdRequire = createRequire(process.cwd() + '/package.json');

// --- 1. Load TypeScript and macroforge from the project ---
let ts, macros;
try {
  ts = cwdRequire('typescript');
} catch {
  console.error('[macroforge] error: typescript is not installed in this project');
  process.exit(1);
}
try {
  macros = cwdRequire('macroforge');
} catch {
  console.error('[macroforge] error: macroforge is not installed in this project');
  process.exit(1);
}

// --- 2. Find and load macroforge config ---
const CONFIG_FILES = [
  'macroforge.config.ts',
  'macroforge.config.mts',
  'macroforge.config.js',
  'macroforge.config.mjs',
  'macroforge.config.cjs',
];
let macroConfigPath = null;
let currentDir = process.cwd();
while (true) {
  for (const filename of CONFIG_FILES) {
    const candidate = path.join(currentDir, filename);
    if (fs.existsSync(candidate)) {
      macroConfigPath = candidate;
      break;
    }
  }
  if (macroConfigPath) break;
  if (fs.existsSync(path.join(currentDir, 'package.json'))) break;
  const parent = path.dirname(currentDir);
  if (parent === currentDir) break;
  currentDir = parent;
}

if (macroConfigPath) {
  try {
    const configContent = fs.readFileSync(macroConfigPath, 'utf8');
    macros.loadConfig(configContent, macroConfigPath);
  } catch (e) {
    // Config load failed, continue without foreign types
  }
}

// --- 2b. Load pre-built type registry if available ---
const typeRegistryPath = process.env.MACROFORGE_TYPE_REGISTRY_PATH;
let typeRegistryJson = undefined;
if (typeRegistryPath) {
  try {
    typeRegistryJson = fs.readFileSync(typeRegistryPath, 'utf8');
  } catch {}
}

// --- 3. Patch ts.sys.readFile to expand macros ---
const origReadFile = ts.sys.readFile.bind(ts.sys);
ts.sys.readFile = (filePath, encoding) => {
  const content = origReadFile(filePath, encoding);
  if (content == null) return content;
  try {
    if (
      (filePath.endsWith('.ts') || filePath.endsWith('.tsx')) &&
      !filePath.endsWith('.d.ts') &&
      content.includes('@derive')
    ) {
      const expandOpts = {};
      if (macroConfigPath) expandOpts.configPath = macroConfigPath;
      if (typeRegistryJson) expandOpts.typeRegistryJson = typeRegistryJson;
      const expanded = macros.expandSync(content, filePath, expandOpts);
      return expanded.code || content;
    }
  } catch (e) {
    // Expansion failed, fall through to original content
  }
  return content;
};

// --- 4. Forward to svelte-check ---
// Rewrite process.argv so sade (svelte-check's CLI parser) sees the right command.
// argv[0] = node, argv[1] = 'svelte-check', rest = flags
const args = ['svelte-check'];
// The Rust side passes extra CLI args starting at process.argv[2]
for (let i = 2; i < process.argv.length; i++) {
  args.push(process.argv[i]);
}
process.argv = [process.argv[0], ...args];

try {
  cwdRequire('svelte-check');
} catch (e) {
  if (e.code === 'MODULE_NOT_FOUND') {
    console.error('[macroforge] error: svelte-check is not installed in this project');
    console.error('[macroforge] install it with: npm install --save-dev svelte-check');
    process.exit(1);
  }
  throw e;
}
"#;

    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("macroforge-cli");
    fs::create_dir_all(&temp_dir)?;
    let script_path = temp_dir.join("svelte-check-wrapper.js");
    fs::write(&script_path, script)?;

    let mut cmd = std::process::Command::new("node");
    cmd.arg(&script_path);

    // Pass the type registry path via environment variable so the JS
    // script can load it and feed it to expandSync.
    if let Some(ref rp) = registry_path {
        cmd.env("MACROFORGE_TYPE_REGISTRY_PATH", rp);
    }

    // Pass through CLI args for svelte-check to pick up
    if let Some(ref ws) = workspace {
        cmd.arg("--workspace").arg(ws);
    }
    if let Some(ref ts) = tsconfig {
        cmd.arg("--tsconfig").arg(ts);
    }
    if let Some(ref out) = output {
        cmd.arg("--output").arg(out);
    }
    if fail_on_warnings {
        cmd.arg("--fail-on-warnings");
    }

    let status = cmd
        .status()
        .context("failed to run node svelte-check wrapper")?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
