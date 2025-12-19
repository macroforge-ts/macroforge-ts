#!/usr/bin/env node

const { program } = require('commander');
const { execSync } = require("child_process");
const path = require("path");
const fs = require("fs");
const { root, resolve, REPOS, ALL_REPO_NAMES, parseRepos } = require("./env.cjs");

program
    .name('cleanbuild-all')
    .description('Complete clean rebuild of selected packages')
    .option('-r, --repos <repos>', `Repos to build (comma-separated, or 'all', 'rust', 'ts'). Available: ${ALL_REPO_NAMES.join(', ')}`, 'all');

const websiteDir = resolve("website");

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

function runStep(step) {
    console.log(`\n==> ${step.label}`);
    if (step.fn) {
        step.fn();
    } else {
        execSync(step.cmd, { stdio: "inherit", cwd: step.cwd });
    }
}

function main(options) {
    const repos = parseRepos(options.repos);
    const repoNames = repos.map(r => r.name);

    console.log(`Clean building repos: ${repoNames.join(', ')}\n`);

    const steps = [];

    // Always clean root node_modules if building everything
    if (options.repos === 'all') {
        steps.push({
            label: "remove root node_modules",
            cmd: "rm -rf node_modules",
            cwd: root,
        });
    }

    // Core rust crate
    if (repoNames.includes('core')) {
        steps.push({
            label: "cleanbuild core (macroforge)",
            cmd: "npm run cleanbuild",
            cwd: resolve("crates", "macroforge_ts"),
        });
    }

    // TypeScript packages
    const tsPackages = ['shared', 'vite-plugin', 'typescript-plugin', 'svelte-language-server', 'svelte-preprocessor', 'mcp-server'];
    for (const pkg of tsPackages) {
        if (repoNames.includes(pkg)) {
            steps.push({
                label: `cleanbuild ${pkg}`,
                cmd: "rm -rf node_modules dist && npm install && npm run build",
                cwd: resolve("packages", pkg),
            });
        }
    }

    // Tooling/playground
    if (repoNames.includes('tooling')) {
        steps.push({
            label: "cleanbuild playground/macro",
            cmd: "npm run cleanbuild",
            cwd: resolve("tooling", "playground", "macro"),
        });
        steps.push({
            label: "cleanbuild playground/svelte",
            cmd: "npm run cleanbuild",
            cwd: resolve("tooling", "playground", "svelte"),
        });
        steps.push({
            label: "cleanbuild playground/vanilla",
            cmd: "npm run cleanbuild",
            cwd: resolve("tooling", "playground", "vanilla"),
        });
    }

    // Website
    if (repoNames.includes('website')) {
        steps.push({
            label: "git pull website from origin",
            cmd: "git pull origin",
            cwd: resolve("website"),
        });
        steps.push({
            label: "cleanbuild website",
            cmd: "rm -rf node_modules .svelte-kit && npm install",
            cwd: resolve("website"),
        });
        steps.push({
            label: "configure website for local build",
            fn: addExternalConfig,
        });
        steps.push({
            label: "build website",
            cmd: "npm run build",
            cwd: resolve("website"),
        });
        steps.push({
            label: "restore website config for production",
            fn: removeExternalConfig,
        });
    }

    try {
        steps.forEach(runStep);
        console.log("\nAll builds finished successfully.");
    } catch (err) {
        console.error(`\nFailed during step: ${err?.message ?? err}`);
        process.exit(err?.status || 1);
    }
}

program.action(main);
program.parse();
