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
const websiteDir = path.join(root, "website");

/**
 * Add ssr.external for macroforge to vite.config.ts and svelte.config.js
 * This is needed when building with local file: dependency
 */
function addExternalConfig() {
  // Update vite.config.ts
  const viteConfigPath = path.join(websiteDir, "vite.config.ts");
  let viteConfig = fs.readFileSync(viteConfigPath, "utf8");

  if (!viteConfig.includes("external: ['macroforge']")) {
    viteConfig = viteConfig.replace(
      "plugins: [tailwindcss(), sveltekit()]",
      `plugins: [tailwindcss(), sveltekit()],
\tssr: {
\t\t// Temporary: needed for local file: dependency build
\t\texternal: ['macroforge']
\t}`
    );
    fs.writeFileSync(viteConfigPath, viteConfig);
    console.log("  Added ssr.external to vite.config.ts");
  }

  // Update svelte.config.js
  const svelteConfigPath = path.join(websiteDir, "svelte.config.js");
  let svelteConfig = fs.readFileSync(svelteConfigPath, "utf8");

  if (!svelteConfig.includes("external: ['macroforge']")) {
    svelteConfig = svelteConfig.replace(
      "adapter: adapter({\n\t\t\tout: 'build'\n\t\t})",
      "adapter: adapter({\n\t\t\tout: 'build',\n\t\t\texternal: ['macroforge']\n\t\t})"
    );
    fs.writeFileSync(svelteConfigPath, svelteConfig);
    console.log("  Added external to svelte.config.js");
  }
}

/**
 * Remove ssr.external config from vite.config.ts and svelte.config.js
 * This ensures production builds (using npm registry) work correctly
 */
function removeExternalConfig() {
  // Update vite.config.ts
  const viteConfigPath = path.join(websiteDir, "vite.config.ts");
  let viteConfig = fs.readFileSync(viteConfigPath, "utf8");

  viteConfig = viteConfig.replace(
    /,\n\tssr: \{\n\t\t\/\/ Temporary: needed for local file: dependency build\n\t\texternal: \['macroforge'\]\n\t\}/,
    ""
  );
  fs.writeFileSync(viteConfigPath, viteConfig);
  console.log("  Removed ssr.external from vite.config.ts");

  // Update svelte.config.js
  const svelteConfigPath = path.join(websiteDir, "svelte.config.js");
  let svelteConfig = fs.readFileSync(svelteConfigPath, "utf8");

  svelteConfig = svelteConfig.replace(
    "adapter: adapter({\n\t\t\tout: 'build',\n\t\t\texternal: ['macroforge']\n\t\t})",
    "adapter: adapter({\n\t\t\tout: 'build'\n\t\t})"
  );
  fs.writeFileSync(svelteConfigPath, svelteConfig);
  console.log("  Removed external from svelte.config.js");
}

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
let externalConfigAdded = false;

function rollback() {
  console.log("\n" + "!".repeat(60));
  console.log("Rolling back changes...");
  console.log("!".repeat(60));

  // Remove external config if it was added
  if (externalConfigAdded) {
    try {
      removeExternalConfig();
      externalConfigAdded = false;
    } catch (e) {
      console.error("Failed to remove external config");
    }
  }

  // Rollback version bump
  if (bumped) {
    try {
      execSync(`node scripts/bump-version.cjs ${currentVersion}`, { cwd: root, stdio: "inherit" });
      console.log(`Rolled back to ${currentVersion}`);
      bumped = false;
    } catch (e) {
      console.error("Failed to rollback. You may need to manually run:");
      console.error(`  git checkout -- .`);
    }
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

  // Pull latest website from origin before making any config changes
  // This must happen before addExternalConfig() so our changes aren't overwritten
  console.log("  Pulling latest website from origin...");
  execSync("git pull origin", { cwd: websiteDir, stdio: "inherit" });

  // Add external config for local file: dependency build
  console.log("  Configuring website for local build...");
  addExternalConfig();
  externalConfigAdded = true;

  run("pixi run cleanbuild:all");

  // Remove external config so production builds work correctly
  console.log("  Restoring website config for production...");
  removeExternalConfig();
  externalConfigAdded = false;

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

// Step 6: Note about website deployment
// The website uses file:../crates/macroforge_ts during local builds (set by bump-version.cjs)
// CI will update it to registry version and regenerate package-lock.json after npm publish
console.log("\n[6/6] Website preparation...");
console.log("  Website uses local macroforge for build (file:../crates/macroforge_ts)");
console.log("  CI will update to registry version after npm publish");

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
