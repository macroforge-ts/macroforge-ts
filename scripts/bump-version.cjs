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

// Update svelte-language-server
updatePackageJson("packages/svelte-language-server/package.json", {
  version,
  "dependencies.macroforge": `^${version}`,
  "dependencies.@macroforge/typescript-plugin": `^${version}`,
});

// Update macroforge Zed extension lib.rs
const macroforgeLibRsPath = path.join(root, "crates/extensions/macroforge/src/lib.rs");
let macroforgeLibRs = fs.readFileSync(macroforgeLibRsPath, "utf8");
macroforgeLibRs = macroforgeLibRs.replace(
  /const TS_PLUGIN_VERSION: &str = ".*";/,
  `const TS_PLUGIN_VERSION: &str = "${version}";`
);
macroforgeLibRs = macroforgeLibRs.replace(
  /const MACROFORGE_VERSION: &str = ".*";/,
  `const MACROFORGE_VERSION: &str = "${version}";`
);
fs.writeFileSync(macroforgeLibRsPath, macroforgeLibRs);
console.log(`  Updated crates/extensions/macroforge/src/lib.rs`);

// Update svelte-macroforge Zed extension lib.rs
const svelteLibRsPath = path.join(root, "crates/extensions/svelte-macroforge/src/lib.rs");
let svelteLibRs = fs.readFileSync(svelteLibRsPath, "utf8");
svelteLibRs = svelteLibRs.replace(
  /const SVELTE_LS_VERSION: &str = ".*";/,
  `const SVELTE_LS_VERSION: &str = "${version}";`
);
// Also update the test assertion
svelteLibRs = svelteLibRs.replace(
  /assert_eq!\(SVELTE_LS_VERSION, ".*"\);/,
  `assert_eq!(SVELTE_LS_VERSION, "${version}");`
);
fs.writeFileSync(svelteLibRsPath, svelteLibRs);
console.log(`  Updated crates/extensions/svelte-macroforge/src/lib.rs`);

// Update Rust workspace Cargo.toml
const workspaceCargoPath = path.join(root, "crates/Cargo.toml");
let workspaceCargo = fs.readFileSync(workspaceCargoPath, "utf8");
workspaceCargo = workspaceCargo.replace(
  /^version = ".*"$/m,
  `version = "${version}"`
);
fs.writeFileSync(workspaceCargoPath, workspaceCargo);
console.log(`  Updated crates/Cargo.toml`);

// Update macroforge_ts Cargo.toml
const macroforgeTsCargoPath = path.join(root, "crates/macroforge_ts/Cargo.toml");
let macroforgeTsCargo = fs.readFileSync(macroforgeTsCargoPath, "utf8");
macroforgeTsCargo = macroforgeTsCargo.replace(
  /^version = ".*"$/m,
  `version = "${version}"`
);
fs.writeFileSync(macroforgeTsCargoPath, macroforgeTsCargo);
console.log(`  Updated crates/macroforge_ts/Cargo.toml`);

console.log(`\nDone! All packages updated to ${version}`);
console.log(`
Next steps:
  git add -A
  git commit -m "Bump version to ${version}"
  git tag v${version}
  git push && git push --tags
`);
