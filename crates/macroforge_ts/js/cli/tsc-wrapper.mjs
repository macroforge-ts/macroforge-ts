/**
 * TSC wrapper — spawned by the CLI's `run_tsc_wrapper`.
 *
 * Wraps `tsc --noEmit` with macro expansion baked into file reads.
 * Files containing macros are expanded before being passed to the
 * TypeScript compiler.
 *
 * Arguments:
 *   argv[2] — tsconfig path (default: "tsconfig.json")
 *
 * Environment:
 *   MACROFORGE_TYPE_REGISTRY_PATH — path to pre-built type registry JSON
 *   MACROFORGE_DECLARATIVE_REGISTRY_PATH — path to pre-built declarative macro registry JSON
 */
import path from "node:path";
import {
  createExpandOptions,
  expandSource,
  isMacroSourceFile,
  loadProjectModule,
} from "./wrapper-common.mjs";

const ts = loadProjectModule("typescript");
const macros = loadProjectModule("@macroforge/core");

const formatHost = {
  getCanonicalFileName: (fileName) => fileName,
  getCurrentDirectory: ts.sys.getCurrentDirectory,
  getNewLine: () => ts.sys.newLine,
};

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
const configFile = ts.readConfigFile(configPath, ts.sys.readFile);
if (configFile.error) {
  console.error(ts.formatDiagnostic(configFile.error, formatHost));
  process.exit(1);
}
const parsed = ts.parseJsonConfigFileContent(
  configFile.config,
  ts.sys,
  path.dirname(configPath),
);
const options = { ...parsed.options, noEmit: true };

const plugin = new macros.NativePlugin();
const expandOptions = createExpandOptions(macros);
const host = ts.createCompilerHost(options);
const originalGetSourceFile = host.getSourceFile.bind(host);
host.getSourceFile = (fileName, languageVersion, ...rest) => {
  if (isMacroSourceFile(fileName)) {
    const sourceText = ts.sys.readFile(fileName);
    if (sourceText !== undefined) {
      return ts.createSourceFile(
        fileName,
        expandSource(plugin, fileName, sourceText, expandOptions),
        languageVersion,
        true,
      );
    }
  }
  return originalGetSourceFile(fileName, languageVersion, ...rest);
};

const program = ts.createProgram(parsed.fileNames, options, host);
const diagnostics = ts.getPreEmitDiagnostics(program);
for (const diagnostic of diagnostics) {
  console.error(ts.formatDiagnostic(diagnostic, formatHost).trimEnd());
}
const hasError = diagnostics.some(
  (diagnostic) => diagnostic.category === ts.DiagnosticCategory.Error,
);
process.exit(hasError ? 1 : 0);
