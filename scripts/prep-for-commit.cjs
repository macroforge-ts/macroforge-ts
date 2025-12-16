#!/usr/bin/env node

/**
 * Prepare for commit: bump version, build, test, and sync documentation.
 *
 * If no version is specified, the patch version is auto-incremented (e.g., 0.1.22 -> 0.1.23).
 *
 * This script:
 * 1. Bumps version across all packages
 * 2. Extracts API documentation (before build so website includes fresh docs)
 * 3. Clean builds all packages (pixi run cleanbuild:all) - includes website with docs
 * 4. Runs all tests (pixi run test:all)
 * 5. Rebuilds the docs book (BOOK.md)
 * 6. Syncs MCP server docs from website
 *
 * If any step fails after the bump, the version is rolled back.
 */

const { program } = require('commander');
const { execSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const root = path.resolve(__dirname, "..");

program
  .name('prep-for-commit')
  .description('Prepare a release: bump version, build, test, and sync documentation')
  .argument('[version]', 'Version string (e.g., 0.1.4). If not provided, auto-increments patch version');
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

function run(cmd, cwd = root) {
  console.log(`\n> ${cmd}\n`);
  execSync(cmd, { cwd, stdio: "inherit" });
}

function main(versionArg) {
  let version = versionArg;

  if (!version) {
    const current = getCurrentVersion();
    version = incrementPatch(current);
    console.log(`No version specified, incrementing ${current} -> ${version}`);
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
console.log("\n[1/7] Bumping version...");
run(`node scripts/bump-version.cjs ${version}`);
bumped = true;

try {
  // Step 2: Extract API documentation from source files
  // This must happen BEFORE cleanbuild so the website includes fresh docs
  console.log("\n[2/7] Extracting API documentation...");

  // Pull latest website from origin before making any config changes
  console.log("  Pulling latest website from origin...");
  execSync("git pull origin", { cwd: websiteDir, stdio: "inherit" });

  // Add external config for local file: dependency build
  console.log("  Configuring website for local build...");
  addExternalConfig();
  externalConfigAdded = true;

  // Extract docs (writes to website/static/api-data/)
  run("node scripts/extract-rust-docs.cjs");
  run("node scripts/extract-ts-docs.cjs");

  // Step 3: Clean build all packages (website will include fresh docs)
  console.log("\n[3/7] Clean building all packages...");
  run("pixi run cleanbuild:all");

  // Remove external config so production builds work correctly
  console.log("  Restoring website config for production...");
  removeExternalConfig();
  externalConfigAdded = false;

  // Step 4: Run all tests
  console.log("\n[4/7] Running all tests...");
  run("pixi run test:all");

  // Step 5: Rebuild docs book from rendered HTML
  console.log("\n[5/7] Rebuilding docs book...");
  run("node scripts/build-docs-book.cjs");

  // Step 6: Sync MCP docs from rendered website
  console.log("\n[6/7] Syncing MCP server docs...");
  run("npm run build:docs", path.join(root, "packages/mcp-server"));
} catch (err) {
  rollback();
  process.exit(1);
}

// Step 7: Final notes
// The website uses file:../crates/macroforge_ts during local builds (set by bump-version.cjs)
// CI will update it to registry version and regenerate package-lock.json after npm publish
console.log("\n[7/7] Done!");
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
}

program.action(main);
program.parse();
