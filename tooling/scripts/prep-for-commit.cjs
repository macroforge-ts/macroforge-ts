#!/usr/bin/env node

/**
 * Prepare for commit: bump version, update dependencies, build, test, and sync documentation.
 *
 * If no version is specified, the patch version is auto-incremented (e.g., 0.1.22 -> 0.1.23).
 *
 * This script:
 * 1. Bumps version for selected repo(s) and updates versions.json cache
 * 2. Updates dependencies to latest versions from cache
 * 3. Extracts API documentation (before build so website includes fresh docs)
 * 4. Clean builds selected packages
 * 5. Runs all tests
 * 6. Rebuilds the docs book (BOOK.md)
 * 7. Syncs MCP server docs from website
 *
 * If any step fails after the bump, the version is rolled back.
 */

const { program } = require('commander');
const { execSync } = require("child_process");
const path = require("path");
const fs = require("fs");
const { root, resolve, REPOS, ALL_REPO_NAMES, parseRepos } = require("./env.cjs");

const VERSIONS_CACHE_PATH = resolve("tooling", "versions.json");

program
    .name('prep-for-commit')
    .description('Prepare a release: bump version, build, test, and sync documentation')
    .argument('[version]', 'Version string (e.g., 0.1.4). If not provided, auto-increments patch version')
    .option('-r, --repos <repos>', `Repos to update (comma-separated, or 'all', 'rust', 'ts'). Available: ${ALL_REPO_NAMES.join(', ')}`, 'all')
    .option('--dry-run', 'Run build and tests without bumping version')
    .option('--skip-build', 'Skip build and test steps')
    .option('--skip-docs', 'Skip documentation steps');

const websiteDir = resolve("website");

// Dependency map: which repos depend on which
const DEPENDENCY_MAP = {
    'core': [],
    'macros': [],
    'syn': [],
    'template': [],
    'shared': ['core'],
    'vite-plugin': ['core', 'shared'],
    'typescript-plugin': ['core', 'shared'],
    'svelte-language-server': ['core', 'typescript-plugin'],
    'svelte-preprocessor': ['core'],
    'mcp-server': ['core'],
    'website': ['core'],
    'zed-extensions': ['typescript-plugin', 'svelte-language-server'],
};

/**
 * Load versions from cache
 */
function loadVersionsCache() {
    if (fs.existsSync(VERSIONS_CACHE_PATH)) {
        return JSON.parse(fs.readFileSync(VERSIONS_CACHE_PATH, "utf8"));
    }
    // Default versions if cache doesn't exist
    return {};
}

/**
 * Save versions to cache
 */
function saveVersionsCache(versions) {
    fs.writeFileSync(VERSIONS_CACHE_PATH, JSON.stringify(versions, null, 2) + "\n");
    console.log(`  Updated ${VERSIONS_CACHE_PATH}`);
}

/**
 * Get current version for a repo from its package.json or Cargo.toml
 */
function getRepoVersion(repoName) {
    const repo = REPOS[repoName];
    if (!repo) return null;

    if (repo.packageJson) {
        const pkgPath = resolve(repo.packageJson);
        if (fs.existsSync(pkgPath)) {
            const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
            return pkg.version;
        }
    }

    if (repo.cargoToml) {
        const cargoPath = resolve(repo.cargoToml);
        if (fs.existsSync(cargoPath)) {
            const content = fs.readFileSync(cargoPath, "utf8");
            const match = content.match(/^version = "([^"]+)"/m);
            return match ? match[1] : null;
        }
    }

    return null;
}

/**
 * Increment patch version
 */
function incrementPatch(version) {
    const parts = version.split(".");
    parts[2] = String(Number(parts[2]) + 1);
    return parts.join(".");
}

/**
 * Update package.json with new version and dependency versions
 */
function updatePackageJson(pkgPath, version, depVersions) {
    const fullPath = resolve(pkgPath);
    if (!fs.existsSync(fullPath)) return;

    const pkg = JSON.parse(fs.readFileSync(fullPath, "utf8"));
    pkg.version = version;

    // Update dependencies to latest versions
    if (pkg.dependencies) {
        if (pkg.dependencies.macroforge && depVersions.core) {
            pkg.dependencies.macroforge = `^${depVersions.core}`;
        }
        if (pkg.dependencies['@macroforge/shared'] && depVersions.shared) {
            pkg.dependencies['@macroforge/shared'] = `^${depVersions.shared}`;
        }
        if (pkg.dependencies['@macroforge/typescript-plugin'] && depVersions['typescript-plugin']) {
            pkg.dependencies['@macroforge/typescript-plugin'] = `^${depVersions['typescript-plugin']}`;
        }
    }

    if (pkg.peerDependencies) {
        if (pkg.peerDependencies.macroforge && depVersions.core) {
            pkg.peerDependencies.macroforge = `^${depVersions.core}`;
        }
    }

    if (pkg.optionalDependencies) {
        // Update platform package versions for core
        const platforms = ['darwin-x64', 'darwin-arm64', 'linux-x64-gnu', 'linux-arm64-gnu', 'win32-x64-msvc', 'win32-arm64-msvc'];
        for (const platform of platforms) {
            const key = `@macroforge/bin-${platform}`;
            if (pkg.optionalDependencies[key]) {
                pkg.optionalDependencies[key] = version;
            }
        }
    }

    fs.writeFileSync(fullPath, JSON.stringify(pkg, null, 2) + "\n");
    console.log(`  Updated ${pkgPath}`);
}

/**
 * Update Cargo.toml with new version and dependency versions
 */
function updateCargoToml(cargoPath, version, depVersions) {
    const fullPath = resolve(cargoPath);
    if (!fs.existsSync(fullPath)) return;

    let content = fs.readFileSync(fullPath, "utf8");
    content = content.replace(/^version = ".*"$/m, `version = "${version}"`);

    // Update crate dependencies for core
    if (depVersions.macros) {
        content = content.replace(/macroforge_ts_macros = "[^"]+"/g, `macroforge_ts_macros = "${depVersions.macros}"`);
    }
    if (depVersions.syn) {
        content = content.replace(/macroforge_ts_syn = "[^"]+"/g, `macroforge_ts_syn = "${depVersions.syn}"`);
    }
    if (depVersions.template) {
        content = content.replace(/macroforge_ts_quote = "[^"]+"/g, `macroforge_ts_quote = "${depVersions.template}"`);
    }

    fs.writeFileSync(fullPath, content);
    console.log(`  Updated ${cargoPath}`);
}

/**
 * Update a single repo to a new version
 */
function updateRepoVersion(repoName, version, allVersions) {
    const repo = REPOS[repoName];
    if (!repo) return;

    console.log(`\n[${repoName}] -> ${version}`);

    if (repo.packageJson) {
        updatePackageJson(repo.packageJson, version, allVersions);
    }

    if (repo.cargoToml) {
        updateCargoToml(repo.cargoToml, version, allVersions);
    }

    // Special handling for core - update platform packages
    if (repoName === 'core') {
        const platforms = ['darwin-x64', 'darwin-arm64', 'linux-x64-gnu', 'linux-arm64-gnu', 'win32-x64-msvc', 'win32-arm64-msvc'];
        for (const platform of platforms) {
            const platformPkgPath = `crates/macroforge_ts/npm/${platform}/package.json`;
            if (fs.existsSync(resolve(platformPkgPath))) {
                const pkg = JSON.parse(fs.readFileSync(resolve(platformPkgPath), "utf8"));
                pkg.version = version;
                fs.writeFileSync(resolve(platformPkgPath), JSON.stringify(pkg, null, 2) + "\n");
                console.log(`  Updated ${platformPkgPath}`);
            }
        }
    }

    // Special handling for zed-extensions
    if (repoName === 'zed-extensions') {
        // Update vtsls-macroforge
        const vtslsLibRsPath = resolve("crates/extensions/vtsls-macroforge/src/lib.rs");
        if (fs.existsSync(vtslsLibRsPath)) {
            let content = fs.readFileSync(vtslsLibRsPath, "utf8");
            if (allVersions['typescript-plugin']) {
                content = content.replace(
                    /const TS_PLUGIN_VERSION: &str = ".*";/,
                    `const TS_PLUGIN_VERSION: &str = "${allVersions['typescript-plugin']}";`
                );
            }
            if (allVersions.core) {
                content = content.replace(
                    /const MACROFORGE_VERSION: &str = ".*";/,
                    `const MACROFORGE_VERSION: &str = "${allVersions.core}";`
                );
            }
            fs.writeFileSync(vtslsLibRsPath, content);
            console.log(`  Updated crates/extensions/vtsls-macroforge/src/lib.rs`);
        }

        // Update svelte-macroforge
        const svelteLibRsPath = resolve("crates/extensions/svelte-macroforge/src/lib.rs");
        if (fs.existsSync(svelteLibRsPath)) {
            let content = fs.readFileSync(svelteLibRsPath, "utf8");
            if (allVersions['svelte-language-server']) {
                content = content.replace(
                    /const SVELTE_LS_VERSION: &str = ".*";/,
                    `const SVELTE_LS_VERSION: &str = "${allVersions['svelte-language-server']}";`
                );
                content = content.replace(
                    /assert_eq!\(SVELTE_LS_VERSION, ".*"\);/,
                    `assert_eq!(SVELTE_LS_VERSION, "${allVersions['svelte-language-server']}");`
                );
            }
            fs.writeFileSync(svelteLibRsPath, content);
            console.log(`  Updated crates/extensions/svelte-macroforge/src/lib.rs`);
        }
    }

    // Special handling for website - use local path for builds
    if (repoName === 'website') {
        const websitePkgPath = resolve("website/package.json");
        if (fs.existsSync(websitePkgPath)) {
            const pkg = JSON.parse(fs.readFileSync(websitePkgPath, "utf8"));
            pkg.version = version;
            pkg.dependencies.macroforge = "file:../crates/macroforge_ts";
            fs.writeFileSync(websitePkgPath, JSON.stringify(pkg, null, 2) + "\n");
            console.log(`  Updated website/package.json (using local macroforge)`);
        }
    }
}

/**
 * Add ssr.external for macroforge to vite.config.ts and svelte.config.js
 */
function addExternalConfig() {
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
 * Remove ssr.external config
 */
function removeExternalConfig() {
    const viteConfigPath = path.join(websiteDir, "vite.config.ts");
    let viteConfig = fs.readFileSync(viteConfigPath, "utf8");

    viteConfig = viteConfig.replace(
        /,\n\tssr: \{\n\t\t\/\/ Temporary: needed for local file: dependency build\n\t\texternal: \['macroforge'\]\n\t\}/,
        ""
    );
    fs.writeFileSync(viteConfigPath, viteConfig);
    console.log("  Removed ssr.external from vite.config.ts");

    const svelteConfigPath = path.join(websiteDir, "svelte.config.js");
    let svelteConfig = fs.readFileSync(svelteConfigPath, "utf8");

    svelteConfig = svelteConfig.replace(
        "adapter: adapter({\n\t\t\tout: 'build',\n\t\t\texternal: ['macroforge']\n\t\t})",
        "adapter: adapter({\n\t\t\tout: 'build'\n\t\t})"
    );
    fs.writeFileSync(svelteConfigPath, svelteConfig);
    console.log("  Removed external from svelte.config.js");
}

function run(cmd, cwd = root) {
    console.log(`\n> ${cmd}\n`);
    execSync(cmd, { cwd, stdio: "inherit" });
}

function main(versionArg, options) {
    const dryRun = options.dryRun || false;
    const skipBuild = options.skipBuild || false;
    const skipDocs = options.skipDocs || false;
    const repos = parseRepos(options.repos);
    const repoNames = repos.map(r => r.name);

    // Load current versions from cache
    const versions = loadVersionsCache();
    const originalVersions = { ...versions };

    // Determine base version for incrementing
    const baseVersion = repoNames.length === 1
        ? (versions[repoNames[0]] || getRepoVersion(repoNames[0]) || '0.1.0')
        : (versions.core || getRepoVersion('core') || '0.1.0');

    let version = versionArg;

    if (dryRun) {
        version = baseVersion;
        console.log("=".repeat(60));
        console.log(`DRY RUN: Testing with current versions`);
        console.log(`Repos: ${repoNames.join(', ')}`);
        console.log("=".repeat(60));
    } else {
        if (!version) {
            version = incrementPatch(baseVersion);
            console.log(`No version specified, incrementing ${baseVersion} -> ${version}`);
        }
        console.log("=".repeat(60));
        console.log(`Preparing release ${version} for: ${repoNames.join(', ')}`);
        console.log("=".repeat(60));
    }

    let bumped = false;
    let externalConfigAdded = false;

    function rollback() {
        console.log("\n" + "!".repeat(60));
        console.log("Rolling back changes...");
        console.log("!".repeat(60));

        if (externalConfigAdded) {
            try {
                removeExternalConfig();
                externalConfigAdded = false;
            } catch (e) {
                console.error("Failed to remove external config");
            }
        }

        if (bumped) {
            try {
                // Restore original versions
                for (const repoName of repoNames) {
                    if (originalVersions[repoName]) {
                        updateRepoVersion(repoName, originalVersions[repoName], originalVersions);
                    }
                }
                saveVersionsCache(originalVersions);
                console.log(`Rolled back versions`);
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

    // Step 1: Bump versions
    if (dryRun) {
        console.log("\n[1/7] Skipping version bump (dry-run)...");
    } else {
        console.log("\n[1/7] Bumping versions...");

        // Update versions cache with new version for selected repos
        for (const repoName of repoNames) {
            versions[repoName] = version;
        }

        // Update each selected repo
        for (const repoName of repoNames) {
            updateRepoVersion(repoName, version, versions);
        }

        // Save updated versions cache
        saveVersionsCache(versions);
        bumped = true;
    }

    try {
        if (skipDocs) {
            console.log("\n[2/7] Skipping documentation extraction (--skip-docs)...");
        } else {
            // Step 2: Extract API documentation
            console.log("\n[2/7] Extracting API documentation...");

            console.log("  Pulling latest website from origin...");
            execSync("git pull origin", { cwd: websiteDir, stdio: "inherit" });

            console.log("  Configuring website for local build...");
            addExternalConfig();
            externalConfigAdded = true;

            run("rust-script scripts/extract-rust-docs.rs");
            run("rust-script scripts/extract-ts-docs.rs");
        }

        if (skipBuild) {
            console.log("\n[3/7] Skipping build (--skip-build)...");
            console.log("\n[4/7] Skipping tests (--skip-build)...");
        } else {
            // Step 3: Clean build
            console.log("\n[3/7] Clean building packages...");
            run("pixi run cleanbuild:all");

            if (externalConfigAdded) {
                console.log("  Restoring website config for production...");
                removeExternalConfig();
                externalConfigAdded = false;
            }

            // Step 4: Run tests
            console.log("\n[4/7] Running all tests...");
            run("pixi run test:all");
        }

        if (skipDocs) {
            console.log("\n[5/7] Skipping docs book (--skip-docs)...");
            console.log("\n[6/7] Skipping MCP docs sync (--skip-docs)...");
        } else {
            // Step 5: Rebuild docs book
            console.log("\n[5/7] Rebuilding docs book...");
            run("rust-script scripts/build-docs-book.rs");

            // Step 6: Sync MCP docs
            console.log("\n[6/7] Syncing MCP server docs...");
            run("npm run build:docs", resolve("packages/mcp-server"));
        }
    } catch (err) {
        rollback();
        process.exit(1);
    }

    // Step 7: Final notes
    console.log("\n[7/7] Done!");

    if (dryRun) {
        console.log("\n" + "=".repeat(60));
        console.log("DRY RUN COMPLETE - no version changes were made");
        console.log("=".repeat(60));
        console.log(`
All builds and tests passed. To do a real release, run:
  node tooling/scripts/prep-for-commit.cjs --repos ${options.repos}
`);
    } else {
        console.log("\n" + "=".repeat(60));
        console.log(`Done! Ready to commit version ${version}`);
        console.log(`Updated repos: ${repoNames.join(', ')}`);
        console.log("=".repeat(60));
        console.log(`
Next steps:
  git add -A
  git commit -m "Bump ${repoNames.join(', ')} to ${version}"
  git tag v${version}
  git push && git push --tags
`);
    }
}

program.action(main);
program.parse();
