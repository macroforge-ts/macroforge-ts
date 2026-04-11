/**
 * TSC wrapper — spawned by the CLI's `run_tsc_wrapper`.
 *
 * Wraps `tsc --noEmit` with macro expansion baked into file reads.
 * Files containing `@derive` are expanded before being passed to the
 * TypeScript compiler.
 *
 * Arguments:
 *   argv[2] — tsconfig path (default: "tsconfig.json")
 *
 * Environment:
 *   MACROFORGE_TYPE_REGISTRY_PATH — path to pre-built type registry JSON
 */
const { createRequire } = require("module");
const fs = require("fs");
const path = require("path");
const cwdRequire = createRequire(process.cwd() + "/package.json");
const ts = cwdRequire("typescript");
const macros = cwdRequire("macroforge");
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
const projectArg = process.argv[2] || "tsconfig.json";
const configPath = ts.findConfigFile(
  process.cwd(),
  ts.sys.fileExists,
  projectArg,
);
if (!configPath) {
  console.error(`[macroforge] tsconfig not found: ${projectArg}`);
  process.exit(1);
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
const configFile = ts.readConfigFile(configPath, ts.sys.readFile);
if (configFile.error) {
  console.error(
    ts.formatDiagnostic(configFile.error, {
      getCanonicalFileName: (f) => f,
      getCurrentDirectory: ts.sys.getCurrentDirectory,
      getNewLine: () => ts.sys.newLine,
    }),
  );
  process.exit(1);
}
const parsed = ts.parseJsonConfigFileContent(
  configFile.config,
  ts.sys,
  path.dirname(configPath),
);
const options = { ...parsed.options, noEmit: true };
const formatHost = {
  getCanonicalFileName: (f) => f,
  getCurrentDirectory: ts.sys.getCurrentDirectory,
  getNewLine: () => ts.sys.newLine,
};
const plugin = new macros.NativePlugin();
const tscExpandOpts = {};
if (macroConfigPath) tscExpandOpts.configPath = macroConfigPath;
if (typeRegistryJson) tscExpandOpts.typeRegistryJson = typeRegistryJson;
if (declarativeRegistryJson) {
  tscExpandOpts.declarativeRegistryJson = declarativeRegistryJson;
}
const host = ts.createCompilerHost(options);
const origGetSourceFile = host.getSourceFile.bind(host);
// Text-level fast path: skip files that don't contain any macro markers.
// Matches:
//   - `@derive` (derive macros)
//   - `macroforge/rules` (declarative-macro-defining files that
//     `import { macroRules } from "macroforge/rules"`)
//   - `import macro` inside a JSDoc comment (declarative-macro-consuming
//     files that use `/** import macro { $name } from "./file" */`)
function hasMacroMarkers(sourceText) {
  if (!sourceText) return false;
  if (sourceText.includes("@derive")) return true;
  if (sourceText.includes("macroforge/rules")) return true;
  if (sourceText.includes("import macro")) return true;
  return false;
}
host.getSourceFile = (fileName, languageVersion, ...rest) => {
  try {
    if (
      (fileName.endsWith(".ts") || fileName.endsWith(".tsx")) &&
      !fileName.endsWith(".d.ts")
    ) {
      const sourceText = ts.sys.readFile(fileName);
      if (hasMacroMarkers(sourceText)) {
        const result = plugin.processFile(fileName, sourceText, tscExpandOpts);
        const text = result.code || sourceText;
        return ts.createSourceFile(fileName, text, languageVersion, true);
      }
    }
  } catch (e) {
    if (process.env.MACROFORGE_DEBUG_WRAPPER) {
      console.error(
        `[macroforge tsc wrapper] expand failed for ${fileName}:`,
        e,
      );
    }
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
const hasError = diagnostics.some(
  (d) => d.category === ts.DiagnosticCategory.Error,
);
process.exit(hasError ? 1 : 0);
