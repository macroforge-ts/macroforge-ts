#!/usr/bin/env node

/**
 * Prepare for commit: bump version, build, test, and sync documentation.
 *
 * Usage: node scripts/prep-for-commit.cjs [version]
 *
 * If no version is specified, the patch version is auto-incremented (e.g., 0.1.22 -> 0.1.23).
 *
 * This script:
 * 1. Clean builds all packages (pixi run cleanbuild:all)
 * 2. Runs all tests (pixi run test:all)
 * 3. Syncs MCP server docs from website
 * 4. Rebuilds the docs book (BOOK.md)
 * 5. Bumps version across all packages
 */

const { execSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const root = path.resolve(__dirname, "..");

function getCurrentVersion() {
  const pkgPath = path.join(root, "packages/vite-plugin/package.json");
  const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
  return pkg.version;
}

function incrementPatch(version) {
  const parts = version.split(".");
  parts[2] = String(Number(parts[2]) + 1);
  return parts.join(".");
}

let version = process.argv[2];

if (!version) {
  const current = getCurrentVersion();
  version = incrementPatch(current);
  console.log(`No version specified, incrementing ${current} -> ${version}`);
}

function run(cmd, cwd = root) {
  console.log(`\n> ${cmd}\n`);
  execSync(cmd, { cwd, stdio: "inherit" });
}

console.log("=".repeat(60));
console.log(`Preparing release ${version}`);
console.log("=".repeat(60));

// Step 1: Clean build all packages
console.log("\n[1/5] Clean building all packages...");
run("pixi run cleanbuild:all");

// Step 2: Run all tests
console.log("\n[2/5] Running all tests...");
run("pixi run test:all");

// Step 3: Sync MCP docs from website
console.log("\n[3/5] Syncing MCP server docs...");
run("npm run build:docs", path.join(root, "packages/mcp-server"));

// Step 4: Rebuild docs book
console.log("\n[4/5] Rebuilding docs book...");
run("node scripts/build-docs-book.cjs");

// Step 5: Bump version
console.log("\n[5/5] Bumping version...");
run(`node scripts/bump-version.cjs ${version}`);

console.log("\n" + "=".repeat(60));
console.log(`Done! Ready to commit version ${version}`);
console.log("=".repeat(60));
console.log(`
Next steps:
  git add -A
  git commit -m "Bump version to ${version}"
  git tag v${version}
  git push && git push --tags
`);
