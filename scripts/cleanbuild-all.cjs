#!/usr/bin/env node

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

const steps = [
  {
    label: "remove root node_modules",
    cmd: "rm -rf node_modules",
    cwd: root,
  },
  {
    label: "remove packages node_modules",
    cmd: "rm -rf node_modules",
    cwd: path.join(root, "packages"),
  },
  {
    label: "remove packages/vite-plugin node_modules",
    cmd: "rm -rf node_modules",
    cwd: path.join(root, "packages", "vite-plugin"),
  },
  {
    label: "remove packages/typescript-plugin node_modules",
    cmd: "rm -rf node_modules",
    cwd: path.join(root, "packages", "typescript-plugin"),
  },
  {
    label: "remove packages/svelte-language-server node_modules",
    cmd: "rm -rf node_modules",
    cwd: path.join(root, "packages", "svelte-language-server"),
  },
  {
    label: "remove playground/svelte node_modules",
    cmd: "rm -rf node_modules",
    cwd: path.join(root, "playground", "svelte"),
  },
  {
    label: "remove playground/svelte .svelte-kit",
    cmd: "rm -rf .svelte-kit",
    cwd: path.join(root, "playground", "svelte"),
  },
  {
    label: "remove playground/vanilla node_modules",
    cmd: "rm -rf node_modules",
    cwd: path.join(root, "playground", "vanilla"),
  },
  {
    label: "remove playground/macro node_modules",
    cmd: "rm -rf node_modules",
    cwd: path.join(root, "playground", "macro"),
  },
  {
    label: "remove tests/e2e/fixtures/ts-project node_modules",
    cmd: "rm -rf node_modules",
    cwd: path.join(root, "tests", "e2e", "fixtures", "ts-project"),
  },
  // Build crates BEFORE installing packages so they get the fresh native module
  {
    label: "clean extension.wasm (vtsls-macroforge)",
    cmd: "rm -f extension.wasm",
    cwd: path.join(root, "crates", "extensions", "vtsls-macroforge"),
  },
  {
    label: "clean extension.wasm (svelte-macroforge)",
    cmd: "rm -f extension.wasm",
    cwd: path.join(root, "crates", "extensions", "svelte-macroforge"),
  },
  {
    label: "clean crates/macroforge_ts",
    cmd: "rm -f macroforge.*.node pkg/*.node",
    cwd: path.join(root, "crates", "macroforge_ts"),
  },
  {
    label: "build crates/macroforge_ts",
    cmd: "npx -y -p @napi-rs/cli napi build --platform --release",
    cwd: path.join(root, "crates", "macroforge_ts"),
  },
  // Now install packages after crates are built
  {
    label: "install packages workspace deps",
    cmd: "npm install",
    cwd: path.join(root, "packages"),
  },
  {
    label: "clean packages/vite-plugin",
    cmd: "npm run clean",
    cwd: path.join(root, "packages", "vite-plugin"),
  },
  {
    label: "build packages/vite-plugin",
    cmd: "npm run build",
    cwd: path.join(root, "packages", "vite-plugin"),
  },
  {
    label: "clean packages/typescript-plugin",
    cmd: "npm run clean",
    cwd: path.join(root, "packages", "typescript-plugin"),
  },
  {
    label: "build packages/typescript-plugin",
    cmd: "npm run build",
    cwd: path.join(root, "packages", "typescript-plugin"),
  },
  {
    label: "clean packages/svelte-language-server",
    cmd: "npm run clean",
    cwd: path.join(root, "packages", "svelte-language-server"),
  },
  {
    label: "build packages/svelte-language-server",
    cmd: "npm run build",
    cwd: path.join(root, "packages", "svelte-language-server"),
  },
  {
    label: "install playground/macro deps",
    cmd: "npm install",
    cwd: path.join(root, "playground", "macro"),
  },
  {
    label: "cleanbuild playground/macro",
    cmd: "npm run cleanbuild",
    cwd: path.join(root, "playground", "macro"),
  },
  {
    label: "install playground/svelte deps",
    cmd: "npm install",
    cwd: path.join(root, "playground", "svelte"),
  },
  {
    label: "build playground/svelte",
    cmd: "npm run build",
    cwd: path.join(root, "playground", "svelte"),
  },
  {
    label: "install playground/vanilla deps",
    cmd: "npm install",
    cwd: path.join(root, "playground", "vanilla"),
  },
  {
    label: "build playground/vanilla",
    cmd: "npm run build",
    cwd: path.join(root, "playground", "vanilla"),
  },
  {
    label: "remove website node_modules",
    cmd: "rm -rf node_modules",
    cwd: path.join(root, "website"),
  },
  {
    label: "remove website .svelte-kit",
    cmd: "rm -rf .svelte-kit",
    cwd: path.join(root, "website"),
  },
  {
    label: "install website deps",
    cmd: "npm install",
    cwd: path.join(root, "website"),
  },
  {
    label: "configure website for local build",
    fn: addExternalConfig,
  },
  {
    label: "build website",
    cmd: "npm run build",
    cwd: path.join(root, "website"),
  },
  {
    label: "restore website config for production",
    fn: removeExternalConfig,
  },
];

function runStep(step) {
  console.log(`\n==> ${step.label}`);
  if (step.fn) {
    step.fn();
  } else {
    execSync(step.cmd, { stdio: "inherit", cwd: step.cwd });
  }
}

try {
  steps.forEach(runStep);
  console.log("\nAll builds finished successfully.");
} catch (err) {
  console.error(`\nFailed during step: ${err?.message ?? err}`);
  process.exit(err?.status || 1);
}
