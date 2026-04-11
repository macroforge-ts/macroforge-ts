/**
 * Svelte-check wrapper — spawned by the CLI's `run_svelte_check_wrapper`.
 *
 * Patches `ts.sys.readFile` to expand macros before svelte-check sees the files.
 * Since svelte-check uses TypeScript as a peer dependency and Node.js caches
 * modules, the patched `ts.sys.readFile` is shared with svelte-check's internal
 * TypeScript language service.
 *
 * Arguments:
 *   argv[2..] — forwarded to svelte-check CLI
 *
 * Environment:
 *   MACROFORGE_TYPE_REGISTRY_PATH — path to pre-built type registry JSON
 */
const { createRequire } = require("module");
const fs = require("fs");
const path = require("path");
const cwdRequire = createRequire(process.cwd() + "/package.json");
let ts;
let macros;
try {
  ts = cwdRequire("typescript");
} catch {
  console.error(
    "[macroforge] error: typescript is not installed in this project",
  );
  process.exit(1);
}
try {
  macros = cwdRequire("macroforge");
} catch {
  console.error(
    "[macroforge] error: macroforge is not installed in this project",
  );
  process.exit(1);
}
if (macros.setupExternalMacros) {
  let resolveDecoratorNames = function (packagePath) {
      const candidates = [packagePath];
      for (const id of candidates) {
        try {
          const pkg = req(id);
          const names = [];
          if (pkg.__macroforgeGetManifest) {
            names.push(
              ...(pkg.__macroforgeGetManifest().decorators || []).map(
                (d) => d.export,
              ),
            );
          }
          for (const key of Object.keys(pkg)) {
            if (
              key.startsWith("__macroforgeGetManifest_") &&
              typeof pkg[key] === "function"
            ) {
              names.push(...(pkg[key]().decorators || []).map((d) => d.export));
            }
          }
          if (names.length > 0) return [...new Set(names)];
        } catch {}
      }
      return [];
    },
    runMacro = function (ctxJson) {
      const ctx = JSON.parse(ctxJson);
      const fnName = `__macroforgeRun${ctx.macro_name}`;
      const candidates = [ctx.module_path];
      for (const id of candidates) {
        try {
          const pkg = req(id);
          const fn_ = pkg?.[fnName] || pkg?.default?.[fnName];
          if (typeof fn_ === "function") return fn_(ctxJson);
        } catch {}
      }
      throw new Error(`Macro ${fnName} not found in ${ctx.module_path}`);
    };
  var resolveDecoratorNames2 = resolveDecoratorNames,
    runMacro2 = runMacro;
  const req = createRequire(process.cwd() + "/package.json");
  macros.setupExternalMacros(resolveDecoratorNames, runMacro);
}
const CONFIG_FILES = [
  "macroforge.config.ts",
  "macroforge.config.mts",
  "macroforge.config.js",
  "macroforge.config.mjs",
  "macroforge.config.cjs",
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
  if (fs.existsSync(path.join(currentDir, "package.json"))) break;
  const parent = path.dirname(currentDir);
  if (parent === currentDir) break;
  currentDir = parent;
}
if (macroConfigPath) {
  try {
    const configContent = fs.readFileSync(macroConfigPath, "utf8");
    macros.loadConfig(configContent, macroConfigPath);
  } catch {}
}
const typeRegistryPath = process.env.MACROFORGE_TYPE_REGISTRY_PATH;
let typeRegistryJson = void 0;
if (typeRegistryPath) {
  try {
    typeRegistryJson = fs.readFileSync(typeRegistryPath, "utf8");
  } catch {}
}
const declarativeRegistryPath =
  process.env.MACROFORGE_DECLARATIVE_REGISTRY_PATH;
let declarativeRegistryJson = void 0;
if (declarativeRegistryPath) {
  try {
    declarativeRegistryJson = fs.readFileSync(declarativeRegistryPath, "utf8");
  } catch {}
}
const plugin = new macros.NativePlugin();
const expandOpts = {};
if (macroConfigPath) expandOpts.configPath = macroConfigPath;
if (typeRegistryJson) expandOpts.typeRegistryJson = typeRegistryJson;
if (declarativeRegistryJson) {
  expandOpts.declarativeRegistryJson = declarativeRegistryJson;
}
const tsSys = ts.sys;
const origReadFile = tsSys.readFile.bind(tsSys);
// Text-level fast path matching the tsc wrapper — covers derive macros
// plus both sides of the declarative macro system (defining and consuming).
function hasMacroMarkers(sourceText) {
  if (!sourceText) return false;
  if (sourceText.includes("@derive")) return true;
  if (sourceText.includes("macroforge/rules")) return true;
  if (sourceText.includes("import macro")) return true;
  return false;
}
tsSys.readFile = (filePath, encoding) => {
  const content = origReadFile(filePath, encoding);
  if (content == null) return content;
  try {
    if (
      (filePath.endsWith(".ts") || filePath.endsWith(".tsx")) &&
      !filePath.endsWith(".d.ts") &&
      hasMacroMarkers(content)
    ) {
      const result = plugin.processFile(filePath, content, expandOpts);
      return result.code || content;
    }
  } catch {}
  return content;
};
const args = ["svelte-check"];
for (let i = 2; i < process.argv.length; i++) {
  args.push(process.argv[i]);
}
process.argv = [process.argv[0], ...args];
try {
  cwdRequire("svelte-check");
} catch (e) {
  if (e.code === "MODULE_NOT_FOUND") {
    console.error(
      "[macroforge] error: svelte-check is not installed in this project",
    );
    console.error(
      "[macroforge] install it with: npm install --save-dev svelte-check",
    );
    process.exit(1);
  }
  throw e;
}
