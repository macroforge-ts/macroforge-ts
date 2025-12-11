#!/usr/bin/env node

/**
 * Sync website for deployment without bumping versions.
 *
 * Usage: node scripts/sync-website.cjs
 *
 * This script updates the website's package-lock.json to reference
 * the current published version of macroforge (removing workspace symlink).
 * Use this when you've made website-only changes and don't need a version bump.
 */

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

console.log(`Syncing website for deployment (macroforge@${version})...\n`);

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

console.log(`\nDone! Website is ready for deployment.`);
console.log(`
Next steps:
  git add -A
  git commit -m "Update website"
  git push
`);
