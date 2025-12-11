#!/usr/bin/env node

/**
 * Prepare for commit: bump version, build, test, and sync documentation.
 *
 * Usage: node scripts/prep-for-commit.cjs [version]
 *
 * If no version is specified, the patch version is auto-incremented (e.g., 0.1.22 -> 0.1.23).
 *
 * This script:
 * 1. Bumps version across all packages
 * 2. Clean builds all packages (pixi run cleanbuild:all) - uses workspace symlinks
 * 3. Runs all tests (pixi run test:all)
 * 4. Syncs MCP server docs from website
 * 5. Rebuilds the docs book (BOOK.md)
 * 6. Updates website package-lock.json for deployment
 *
 * If any step fails after the bump, the version is rolled back.
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

const currentVersion = getCurrentVersion();

console.log("=".repeat(60));
console.log(`Preparing release ${version}`);
console.log("=".repeat(60));

let bumped = false;

function rollback() {
  if (!bumped) return;
  console.log("\n" + "!".repeat(60));
  console.log("Rolling back version bump...");
  console.log("!".repeat(60));
  try {
    execSync(`node scripts/bump-version.cjs ${currentVersion}`, { cwd: root, stdio: "inherit" });
    console.log(`Rolled back to ${currentVersion}`);
    bumped = false;
  } catch (e) {
    console.error("Failed to rollback. You may need to manually run:");
    console.error(`  git checkout -- .`);
  }
}

process.on("SIGINT", () => {
  console.log("\n\nInterrupted!");
  rollback();
  process.exit(1);
});

process.on("SIGTERM", () => {
  console.log("\n\nTerminated!");
  rollback();
  process.exit(1);
});

// Step 1: Bump version
console.log("\n[1/6] Bumping version...");
run(`node scripts/bump-version.cjs ${version}`);
bumped = true;

try {
  // Step 2: Clean build all packages (workspace uses local macroforge via symlink)
  console.log("\n[2/6] Clean building all packages...");
  run("pixi run cleanbuild:all");

  // Step 3: Run all tests
  console.log("\n[3/6] Running all tests...");
  run("pixi run test:all");

  // Step 4: Sync MCP docs from website
  console.log("\n[4/6] Syncing MCP server docs...");
  run("npm run build:docs", path.join(root, "packages/mcp-server"));

  // Step 5: Rebuild docs book
  console.log("\n[5/6] Rebuilding docs book...");
  run("node scripts/build-docs-book.cjs");
} catch (err) {
  rollback();
  process.exit(1);
}

// Step 6: Update website for deployment (switch from local to registry)
console.log("\n[6/6] Preparing website for deployment...");

// Update package.json to registry version
const websitePkgPath = path.join(root, "website/package.json");
const websitePkg = JSON.parse(fs.readFileSync(websitePkgPath, "utf8"));
websitePkg.dependencies.macroforge = `^${version}`;
fs.writeFileSync(websitePkgPath, JSON.stringify(websitePkg, null, 2) + "\n");
console.log(`  Updated website/package.json: macroforge -> ^${version}`);

// Update package-lock.json to registry version
const websiteLockPath = path.join(root, "website/package-lock.json");
const websiteLock = JSON.parse(fs.readFileSync(websiteLockPath, "utf8"));
websiteLock.version = version;
if (websiteLock.packages?.["node_modules/macroforge"]) {
  const pkg = websiteLock.packages["node_modules/macroforge"];
  delete pkg.link;
  pkg.version = version;
  pkg.resolved = `https://registry.npmjs.org/macroforge/-/macroforge-${version}.tgz`;
  delete pkg.integrity;
}
if (websiteLock.packages?.[""]?.dependencies?.macroforge) {
  websiteLock.packages[""].dependencies.macroforge = `^${version}`;
}
fs.writeFileSync(websiteLockPath, JSON.stringify(websiteLock, null, 2) + "\n");
console.log(`  Updated website/package-lock.json`);

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
