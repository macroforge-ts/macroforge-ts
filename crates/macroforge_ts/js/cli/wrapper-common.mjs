/**
 * Setup shared by the tsc and svelte-check wrappers. Materialized next to
 * each wrapper by the CLI's `materialize_scripts`.
 */
import fs from "node:fs";
import { createRequire } from "node:module";
import path from "node:path";

const projectRequire = createRequire(
  path.join(process.cwd(), "package.json"),
);

const CONFIG_FILES = [
  "macroforge.config.ts",
  "macroforge.config.mts",
  "macroforge.config.js",
  "macroforge.config.mjs",
  "macroforge.config.cjs",
];

/** Loads a dependency of the project being checked, exiting when it cannot. */
export function loadProjectModule(specifier) {
  try {
    return projectRequire(specifier);
  } catch (error) {
    console.error(
      `[macroforge] error: could not load ${specifier} from this project`,
    );
    console.error(error);
    process.exit(1);
  }
}

/** Walks up from the cwd to the package root looking for a macroforge config. */
function findMacroConfig() {
  let currentDir = process.cwd();
  while (true) {
    for (const filename of CONFIG_FILES) {
      const candidate = path.join(currentDir, filename);
      if (fs.existsSync(candidate)) return candidate;
    }
    if (fs.existsSync(path.join(currentDir, "package.json"))) return null;
    const parent = path.dirname(currentDir);
    if (parent === currentDir) return null;
    currentDir = parent;
  }
}

function readRegistry(variable) {
  const registryPath = process.env[variable];
  if (!registryPath) return undefined;
  try {
    return fs.readFileSync(registryPath, "utf8");
  } catch (error) {
    console.error(`[macroforge] could not read ${variable} (${registryPath})`);
    console.error(error);
    return undefined;
  }
}

/** Loads the project's macro config and builds the options for `processFile`. */
export function createExpandOptions(macros) {
  const options = {};
  const configPath = findMacroConfig();
  if (configPath) {
    try {
      macros.loadConfig(fs.readFileSync(configPath, "utf8"), configPath);
    } catch (error) {
      console.error(`[macroforge] error: could not load ${configPath}`);
      console.error(error);
      process.exit(1);
    }
    options.configPath = configPath;
  }
  const typeRegistryJson = readRegistry("MACROFORGE_TYPE_REGISTRY_PATH");
  if (typeRegistryJson) options.typeRegistryJson = typeRegistryJson;
  const declarativeRegistryJson = readRegistry(
    "MACROFORGE_DECLARATIVE_REGISTRY_PATH",
  );
  if (declarativeRegistryJson) {
    options.declarativeRegistryJson = declarativeRegistryJson;
  }
  return options;
}

export function isMacroSourceFile(fileName) {
  return (fileName.endsWith(".ts") || fileName.endsWith(".tsx")) &&
    !fileName.endsWith(".d.ts");
}

/**
 * Expands a file's macros. A failed expansion ends the check: type-checking
 * the unexpanded text would report on code the build never produces.
 */
export function expandSource(plugin, fileName, sourceText, options) {
  let result;
  try {
    result = plugin.processFile(fileName, sourceText, options);
  } catch (error) {
    console.error(`[macroforge] error: expansion failed for ${fileName}`);
    console.error(error);
    process.exit(1);
  }
  for (const diagnostic of result.diagnostics) {
    if (diagnostic.level === "info") continue;
    console.error(
      `[macroforge] ${diagnostic.level}: ${fileName}${
        diagnostic.start === undefined ? "" : `@${diagnostic.start}`
      }: ${diagnostic.message}`,
    );
  }
  if (result.diagnostics.some((diagnostic) => diagnostic.level === "error")) {
    process.exit(1);
  }
  return result.code;
}
