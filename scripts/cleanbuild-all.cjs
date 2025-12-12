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
  // Clean root node_modules for a complete reset
  {
    label: "remove root node_modules",
    cmd: "rm -rf node_modules",
    cwd: root,
  },
  // Clean and build packages workspace (includes crates/macroforge_ts)
  {
    label: "cleanbuild packages workspace",
    cmd: "npm run cleanbuild",
    cwd: path.join(root, "packages"),
  },
  // Clean and build playground/macro
  {
    label: "cleanbuild playground/macro",
    cmd: "npm run cleanbuild",
    cwd: path.join(root, "playground", "macro"),
  },
  // Clean and build playground/svelte
  {
    label: "cleanbuild playground/svelte",
    cmd: "npm run cleanbuild",
    cwd: path.join(root, "playground", "svelte"),
  },
  // Clean and build playground/vanilla
  {
    label: "cleanbuild playground/vanilla",
    cmd: "npm run cleanbuild",
    cwd: path.join(root, "playground", "vanilla"),
  },
  // Website handling
  {
    label: "git pull website from origin",
    cmd: "git pull origin",
    cwd: path.join(root, "website"),
  },
  {
    label: "cleanbuild website",
    cmd: "rm -rf node_modules .svelte-kit && npm install",
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
