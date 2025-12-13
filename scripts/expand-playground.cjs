#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");
const { execSync } = require("node:child_process");
const Module = require("node:module");

// Parse --use-cli flag
const useCli = process.argv.includes("--use-cli");

// Only load the Node.js API if not using CLI
const expandSync = useCli ? null : require("../crates/macroforge_ts").expandSync;

// Path to the CLI binary (debug or release)
const cliBinary = (() => {
  const release = path.join(__dirname, "..", "crates", "target", "release", "macroforge");
  const debug = path.join(__dirname, "..", "crates", "target", "debug", "macroforge");
  if (fs.existsSync(release)) return release;
  if (fs.existsSync(debug)) return debug;
  return "macroforge"; // Fall back to PATH
})();

// Ensure external macros resolve (e.g. @playground/macro used by demos)
const extraNodePaths = [
  path.join(__dirname, "..", "playground", "tests", "node_modules"),
  path.join(__dirname, "..", "playground", "macro", "node_modules"),
].filter(fs.existsSync);

if (extraNodePaths.length > 0) {
  const existing = process.env.NODE_PATH ? process.env.NODE_PATH.split(path.delimiter) : [];
  process.env.NODE_PATH = [...extraNodePaths, ...existing].join(path.delimiter);
  Module._initPaths();
}

const roots = [
  path.join(__dirname, "..", "playground", "svelte", "src", "lib", "demo"),
  path.join(__dirname, "..", "playground", "vanilla", "src"),
];

// Format expanded files with biome (run from each playground root to respect local biome.json)
const playgroundRoots = [
  path.join(__dirname, "..", "playground", "svelte"),
  path.join(__dirname, "..", "playground", "vanilla"),
];

if (useCli) {
  // Use CLI's --scan feature for each root
  console.log(`Using CLI binary: ${cliBinary}`);
  for (const root of roots) {
    if (!fs.existsSync(root)) continue;
    try {
      execSync(`"${cliBinary}" expand --scan "${root}"`, {
        stdio: "inherit",
      });
    } catch (err) {
      console.error(`failed to scan ${root}: ${err.message || err}`);
      process.exitCode = 1;
    }
  }
} else {
  // Use Node.js API
  function isSourceFile(file) {
    return file.endsWith(".ts") && !file.includes(".expanded.");
  }

  function getExpandedPath(file) {
    const dir = path.dirname(file);
    const basename = path.basename(file);
    const firstDotIndex = basename.indexOf(".");
    if (firstDotIndex === -1) {
      return path.join(dir, basename + ".expanded");
    }
    const nameWithoutExt = basename.slice(0, firstDotIndex);
    const extensions = basename.slice(firstDotIndex);
    return path.join(dir, nameWithoutExt + ".expanded" + extensions);
  }

  for (const root of roots) {
    if (!fs.existsSync(root)) continue;
    for (const entry of fs.readdirSync(root)) {
      const full = path.join(root, entry);
      if (fs.statSync(full).isFile() && isSourceFile(entry)) {
        const code = fs.readFileSync(full, "utf8");
        const outPath = getExpandedPath(full);
        try {
          const res = expandSync(code, full, { keepDecorators: false });
          fs.writeFileSync(outPath, res.code);
          console.log(
            `expanded ${path.relative(process.cwd(), full)} -> ${path.relative(process.cwd(), outPath)}`,
          );
        } catch (err) {
          console.error(`failed to expand ${full}: ${err.message || err}`);
          process.exitCode = 1;
        }
      }
    }
  }
}

// Format expanded files with biome
for (const playgroundRoot of playgroundRoots) {
  if (!fs.existsSync(playgroundRoot)) continue;
  try {
    execSync("npx biome format --write src", {
      stdio: "inherit",
      cwd: playgroundRoot,
    });
  } catch {
    // Formatting is best-effort, don't fail if biome isn't available
  }
}
