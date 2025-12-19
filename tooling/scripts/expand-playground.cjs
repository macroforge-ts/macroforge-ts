#!/usr/bin/env node

const { program } = require('commander');
const fs = require("node:fs");
const path = require("node:path");
const { execSync } = require("node:child_process");
const Module = require("node:module");
const { root, resolve } = require("./env.cjs");

program
  .name('expand-playground')
  .description('Expand macros in playground TypeScript files')
  .option('--use-cli', 'Use CLI binary instead of Node.js API');

// Path to the CLI binary (debug or release)
const cliBinary = (() => {
  const release = resolve("crates", "target", "release", "macroforge");
  const debug = resolve("crates", "target", "debug", "macroforge");
  if (fs.existsSync(release)) return release;
  if (fs.existsSync(debug)) return debug;
  return "macroforge"; // Fall back to PATH
})();

// Ensure external macros resolve (e.g. @playground/macro used by demos)
const extraNodePaths = [
  resolve("tooling", "playground", "tests", "node_modules"),
  resolve("tooling", "playground", "macro", "node_modules"),
].filter(fs.existsSync);

if (extraNodePaths.length > 0) {
  const existing = process.env.NODE_PATH ? process.env.NODE_PATH.split(path.delimiter) : [];
  process.env.NODE_PATH = [...extraNodePaths, ...existing].join(path.delimiter);
  Module._initPaths();
}

const roots = [
  resolve("tooling", "playground", "svelte", "src", "lib", "demo"),
  resolve("tooling", "playground", "svelte", "src", "lib", "demo", "types"),
  resolve("tooling", "playground", "vanilla", "src"),
];

// Format expanded files with biome (run from each playground root to respect local biome.json)
const playgroundRoots = [
  resolve("tooling", "playground", "svelte"),
  resolve("tooling", "playground", "vanilla"),
];

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

function main(options) {
  const useCli = options.useCli;

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
    const { expandSync, loadConfig, clearConfigCache } = require(resolve("crates", "macroforge_ts"));

    // Config file names in order of precedence
    const CONFIG_FILES = [
      "macroforge.config.ts",
      "macroforge.config.mts",
      "macroforge.config.js",
      "macroforge.config.mjs",
      "macroforge.config.cjs",
    ];

    // Find config file starting from a directory, walking up to package.json
    function findConfigFile(startDir) {
      let dir = startDir;
      while (dir) {
        for (const configName of CONFIG_FILES) {
          const configPath = path.join(dir, configName);
          if (fs.existsSync(configPath)) {
            return configPath;
          }
        }
        // Stop at package.json (project root)
        if (fs.existsSync(path.join(dir, "package.json"))) {
          break;
        }
        const parent = path.dirname(dir);
        if (parent === dir) break;
        dir = parent;
      }
      return null;
    }

    // Cache loaded configs by path
    const loadedConfigs = new Set();

    for (const root of roots) {
      if (!fs.existsSync(root)) continue;

      // Find and load config for this root
      const configPath = findConfigFile(root);
      if (configPath && !loadedConfigs.has(configPath)) {
        const configContent = fs.readFileSync(configPath, "utf8");
        loadConfig(configContent, configPath);
        loadedConfigs.add(configPath);
        console.log(`loaded config: ${path.relative(process.cwd(), configPath)}`);
      }

      for (const entry of fs.readdirSync(root)) {
        const full = path.join(root, entry);
        if (fs.statSync(full).isFile() && isSourceFile(entry)) {
          const code = fs.readFileSync(full, "utf8");
          const outPath = getExpandedPath(full);
          try {
            const options = { keepDecorators: false };
            if (configPath) {
              options.configPath = configPath;
            }
            const res = expandSync(code, full, options);
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
    const playgroundName = path.basename(playgroundRoot);
    try {
      execSync("npx biome format --write src", {
        stdio: "pipe",
        cwd: playgroundRoot,
      });
      console.log(`formatted playground/${playgroundName}/src`);
    } catch (err) {
      const stderr = err.stderr?.toString() || "";
      const stdout = err.stdout?.toString() || "";
      const output = stderr || stdout || err.message || "unknown error";

      // Try to extract which files failed from biome output
      const failedFiles = [];
      const filePattern = /(?:^|\s)(src\/[^\s:]+)/gm;
      let match;
      while ((match = filePattern.exec(output)) !== null) {
        failedFiles.push(match[1]);
      }

      console.error(`\n❌ Formatting failed in playground/${playgroundName}`);
      if (failedFiles.length > 0) {
        console.error(`   Failed files:`);
        for (const file of [...new Set(failedFiles)]) {
          console.error(`     - ${file}`);
        }
      }
      console.error(`\n   Biome output:\n${output.split('\n').map(l => '   ' + l).join('\n')}`);
      console.error(`\n   To debug, run: cd playground/${playgroundName} && npx biome format --write src\n`);
      process.exitCode = 1;
    }
  }
}

program.action(main);
program.parse();
