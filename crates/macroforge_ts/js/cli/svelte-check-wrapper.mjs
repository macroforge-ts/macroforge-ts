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
 *   MACROFORGE_DECLARATIVE_REGISTRY_PATH — path to pre-built declarative macro registry JSON
 */
import {
  createExpandOptions,
  expandSource,
  isMacroSourceFile,
  loadProjectModule,
} from "./wrapper-common.mjs";

const ts = loadProjectModule("typescript");
const macros = loadProjectModule("@macroforge/core");

const plugin = new macros.NativePlugin();
const expandOptions = createExpandOptions(macros);
const originalReadFile = ts.sys.readFile.bind(ts.sys);
ts.sys.readFile = (filePath, encoding) => {
  const content = originalReadFile(filePath, encoding);
  if (content === undefined) return content;
  if (isMacroSourceFile(filePath)) {
    return expandSource(plugin, filePath, content, expandOptions);
  }
  return content;
};

process.argv = [process.argv[0], "svelte-check", ...process.argv.slice(2)];
loadProjectModule("svelte-check");
