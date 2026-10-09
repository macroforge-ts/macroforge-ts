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
  const options = { emitMetadata: false };
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
  // The engine keeps each registry for the process, so every file names it
  // by id rather than sending its JSON again.
  try {
    const typeRegistryJson = readRegistry("MACROFORGE_TYPE_REGISTRY_PATH");
    if (typeRegistryJson) {
      options.typeRegistryId = macros.setTypeRegistry(typeRegistryJson);
    }
    const declarativeRegistryJson = readRegistry(
      "MACROFORGE_DECLARATIVE_REGISTRY_PATH",
    );
    if (declarativeRegistryJson) {
      options.declarativeRegistryId = macros.setDeclarativeRegistry(
        declarativeRegistryJson,
      );
    }
  } catch (error) {
    console.error("[macroforge] error: could not load the project registries");
    console.error(error);
    process.exit(1);
  }
  return options;
}

export function isMacroSourceFile(fileName) {
  return (fileName.endsWith(".ts") || fileName.endsWith(".tsx")) &&
    !fileName.endsWith(".d.ts");
}

/**
 * Expands `sourceText`, printing macro diagnostics and exiting on an error,
 * and returns the whole result: the code and its `sourceMapping`. A failed
 * expansion ends the check: type-checking the unexpanded text would report on
 * code the build never produces.
 */
export function expandSourceResult(plugin, fileName, sourceText, options) {
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
        diagnostic.start === undefined ? "" : position(sourceText, diagnostic.start)
      }: ${diagnostic.message}`,
    );
  }
  if (result.diagnostics.some((diagnostic) => diagnostic.level === "error")) {
    process.exit(1);
  }
  return result;
}

/** `:line:column`, 1-based, of a 0-based offset in `text`. */
function position(text, offset) {
  const before = text.slice(0, offset);
  const line = before.split("\n").length;
  return `:${line}:${offset - before.lastIndexOf("\n")}`;
}

/** Expands `sourceText` like `expandSourceResult`, returning only the code. */
export function expandSource(plugin, fileName, sourceText, options) {
  return expandSourceResult(plugin, fileName, sourceText, options).code;
}

/**
 * Maps a span of a file's expansion back onto the file as written. Code a
 * macro generated has no source position, so its span is anchored, empty,
 * where that code was inserted, and `generatedBy` names the macro.
 */
export function mapSpanToSource(macros, mapping, start, length) {
  const mapped = new macros.PositionMapper(mapping).mapSpanToOriginal(start, length);
  if (mapped) return { start: mapped.start, length: mapped.length, generatedBy: undefined };
  const region = mapping.generatedRegions.find(
    (candidate) => start >= candidate.start && start < candidate.end,
  );
  const anchor = mapping.segments
    .filter((segment) => segment.expandedEnd <= start)
    .reduce((position, segment) => Math.max(position, segment.originalEnd), 0);
  return { start: anchor, length: 0, generatedBy: region?.sourceMacro ?? "a macro" };
}
