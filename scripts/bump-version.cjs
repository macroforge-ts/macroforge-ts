#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

const root = path.resolve(__dirname, "..");
const version = process.argv[2];

if (!version) {
  console.error("Usage: node scripts/bump-version.cjs <version>");
  console.error("Example: node scripts/bump-version.cjs 0.1.4");
  process.exit(1);
}

console.log(`Bumping all packages to version ${version}...\n`);

// Helper to update package.json
function updatePackageJson(pkgPath, updates) {
  const fullPath = path.join(root, pkgPath);
  const pkg = JSON.parse(fs.readFileSync(fullPath, "utf8"));

  for (const [key, value] of Object.entries(updates)) {
    if (key.includes(".")) {
      // Handle nested keys like "dependencies.macroforge"
      const parts = key.split(".");
      let obj = pkg;
      for (let i = 0; i < parts.length - 1; i++) {
        obj = obj[parts[i]];
      }
      obj[parts[parts.length - 1]] = value;
    } else {
      pkg[key] = value;
    }
  }

  fs.writeFileSync(fullPath, JSON.stringify(pkg, null, 2) + "\n");
  console.log(`  Updated ${pkgPath}`);
}

// Platform packages
const platforms = [
  "darwin-x64",
  "darwin-arm64",
  "linux-x64-gnu",
  "linux-arm64-gnu",
  "win32-x64-msvc",
  "win32-arm64-msvc",
];

// Update main macroforge package
updatePackageJson("crates/macroforge_ts/package.json", {
  version,
  "optionalDependencies.@macroforge/bin-darwin-x64": version,
  "optionalDependencies.@macroforge/bin-darwin-arm64": version,
  "optionalDependencies.@macroforge/bin-linux-x64-gnu": version,
  "optionalDependencies.@macroforge/bin-linux-arm64-gnu": version,
  "optionalDependencies.@macroforge/bin-win32-x64-msvc": version,
  "optionalDependencies.@macroforge/bin-win32-arm64-msvc": version,
});

// Update all platform packages
for (const platform of platforms) {
  updatePackageJson(`crates/macroforge_ts/npm/${platform}/package.json`, {
    version,
  });
}

// Update typescript-plugin
updatePackageJson("packages/typescript-plugin/package.json", {
  version,
  "dependencies.macroforge": `^${version}`,
});

// Update Zed extension lib.rs
const libRsPath = path.join(root, "crates/extensions/macroforge/src/lib.rs");
let libRs = fs.readFileSync(libRsPath, "utf8");
libRs = libRs.replace(
  /const TS_PLUGIN_VERSION: &str = ".*";/,
  `const TS_PLUGIN_VERSION: &str = "${version}";`
);
libRs = libRs.replace(
  /const MACROFORGE_VERSION: &str = ".*";/,
  `const MACROFORGE_VERSION: &str = "${version}";`
);
fs.writeFileSync(libRsPath, libRs);
console.log(`  Updated crates/extensions/macroforge/src/lib.rs`);

console.log(`\nDone! All packages updated to ${version}`);
console.log(`
Next steps:
  git add -A
  git commit -m "Bump version to ${version}"
  git tag v${version}
  git push && git push --tags
`);
