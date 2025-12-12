#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");
const { expandSync } = require("../crates/macroforge_ts");
const Module = require("node:module");

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

function isSourceFile(file) {
  // Must end with .ts and not contain .expanded. anywhere in the filename
  return file.endsWith(".ts") && !file.includes(".expanded.");
}

function getExpandedPath(file) {
  // Handle files with multiple extensions like .svelte.ts
  // Insert .expanded as the first extension: foo.svelte.ts -> foo.expanded.svelte.ts
  const dir = path.dirname(file);
  const basename = path.basename(file);

  // Find where extensions start (first dot after the base name)
  const firstDotIndex = basename.indexOf(".");
  if (firstDotIndex === -1) {
    // No extension, just append .expanded
    return path.join(dir, basename + ".expanded");
  }

  const nameWithoutExt = basename.slice(0, firstDotIndex);
  const extensions = basename.slice(firstDotIndex);
  return path.join(dir, nameWithoutExt + ".expanded" + extensions);
}

function expandFile(file) {
  const code = fs.readFileSync(file, "utf8");
  const outPath = getExpandedPath(file);
  try {
    const res = expandSync(code, file, { keepDecorators: false });
    fs.writeFileSync(outPath, res.code);
    console.log(
      `expanded ${path.relative(process.cwd(), file)} -> ${path.relative(process.cwd(), outPath)}`,
    );
  } catch (err) {
    console.error(`failed to expand ${file}: ${err.message || err}`);
    process.exitCode = 1;
  }
}

for (const root of roots) {
  if (!fs.existsSync(root)) continue;
  for (const entry of fs.readdirSync(root)) {
    const full = path.join(root, entry);
    if (fs.statSync(full).isFile() && isSourceFile(entry)) {
      expandFile(full);
    }
  }
}
