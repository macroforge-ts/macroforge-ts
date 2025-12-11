#!/usr/bin/env node

/**
 * Sync website for deployment without bumping versions.
 *
 * Usage: node scripts/sync-website.cjs
 *
 * This script updates the website's package.json and package-lock.json to reference
 * the current published version of macroforge (removing workspace symlink).
 * Use this when you've made website-only changes and don't need a version bump.
 */

const { execSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const root = path.resolve(__dirname, "..");

// Get current version from an existing package
function getCurrentVersion() {
  const pkgPath = path.join(root, "packages/vite-plugin/package.json");
  const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
  return pkg.version;
}

const version = getCurrentVersion();
const websiteDir = path.join(root, "website");

console.log(`Syncing website for deployment (macroforge@${version})...\n`);

// Pull latest from origin before making changes
console.log(`  Pulling latest website from origin...`);
execSync("git pull origin", { cwd: websiteDir, stdio: "inherit" });

// Update package.json to registry version
const websitePkgPath = path.join(websiteDir, "package.json");
const websitePkg = JSON.parse(fs.readFileSync(websitePkgPath, "utf8"));
websitePkg.dependencies.macroforge = `^${version}`;
fs.writeFileSync(websitePkgPath, JSON.stringify(websitePkg, null, 2) + "\n");
console.log(`  Updated website/package.json: macroforge -> ^${version}`);

// Delete old lock file and regenerate with npm install
// This ensures proper integrity hashes for all dependencies
const websiteLockPath = path.join(websiteDir, "package-lock.json");
fs.unlinkSync(websiteLockPath);
console.log(`  Deleted website/package-lock.json`);

console.log(`  Running npm install to regenerate lock file...`);
execSync("npm install", { cwd: websiteDir, stdio: "inherit" });
console.log(`  Regenerated website/package-lock.json`);

console.log(`\nDone! Website is ready for deployment.`);
console.log(`
Next steps:
  git add -A
  git commit -m "Update website"
  git push
`);
