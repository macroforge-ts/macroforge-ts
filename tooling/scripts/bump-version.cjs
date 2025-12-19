#!/usr/bin/env node

const { program } = require('commander');
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");
const { root, resolve, REPOS, ALL_REPO_NAMES, parseRepos } = require("./env.cjs");

program
    .name('bump-version')
    .description('Update version numbers across packages, crates, and config files')
    .argument('<version>', 'Version string (e.g., 0.1.4)')
    .option('-r, --repos <repos>', `Repos to update (comma-separated, or 'all', 'rust', 'ts'). Available: ${ALL_REPO_NAMES.join(', ')}`, 'all');

// Helper to update package.json
function updatePackageJson(pkgPath, updates) {
    const fullPath = path.join(root, pkgPath);
    if (!fs.existsSync(fullPath)) {
        console.log(`  Skipping ${pkgPath} (not found)`);
        return;
    }
    const pkg = JSON.parse(fs.readFileSync(fullPath, "utf8"));

    for (const [key, value] of Object.entries(updates)) {
        if (key.includes(".")) {
            // Handle nested keys like "dependencies.macroforge"
            const parts = key.split(".");
            let obj = pkg;
            for (let i = 0; i < parts.length - 1; i++) {
                if (!obj[parts[i]]) obj[parts[i]] = {};
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

// Helper to update Cargo.toml version
function updateCargoToml(cargoPath, version) {
    const fullPath = path.join(root, cargoPath);
    if (!fs.existsSync(fullPath)) {
        console.log(`  Skipping ${cargoPath} (not found)`);
        return;
    }
    let content = fs.readFileSync(fullPath, "utf8");
    content = content.replace(/^version = ".*"$/m, `version = "${version}"`);
    fs.writeFileSync(fullPath, content);
    console.log(`  Updated ${cargoPath}`);
}

function main(version, options) {
    const repos = parseRepos(options.repos);
    const repoNames = repos.map(r => r.name);

    console.log(`Bumping version to ${version} for repos: ${repoNames.join(', ')}\n`);

    // Platform packages (for core)
    const platforms = [
        "darwin-x64",
        "darwin-arm64",
        "linux-x64-gnu",
        "linux-arm64-gnu",
        "win32-x64-msvc",
        "win32-arm64-msvc",
    ];

    // Update core (macroforge_ts)
    if (repoNames.includes('core')) {
        console.log('\n[core]');
        updatePackageJson("crates/macroforge_ts/package.json", {
            version,
            "optionalDependencies.@macroforge/bin-darwin-x64": version,
            "optionalDependencies.@macroforge/bin-darwin-arm64": version,
            "optionalDependencies.@macroforge/bin-linux-x64-gnu": version,
            "optionalDependencies.@macroforge/bin-linux-arm64-gnu": version,
            "optionalDependencies.@macroforge/bin-win32-x64-msvc": version,
            "optionalDependencies.@macroforge/bin-win32-arm64-msvc": version,
        });
        updateCargoToml("crates/macroforge_ts/Cargo.toml", version);

        // Update platform packages
        for (const platform of platforms) {
            updatePackageJson(`crates/macroforge_ts/npm/${platform}/package.json`, { version });
        }

        // Update internal crate dependencies in core
        const macroforgeTsCargoPath = path.join(root, "crates/macroforge_ts/Cargo.toml");
        let macroforgeTsCargo = fs.readFileSync(macroforgeTsCargoPath, "utf8");
        macroforgeTsCargo = macroforgeTsCargo.replace(
            /macroforge_ts_syn = "[^"]+"/g,
            `macroforge_ts_syn = "${version}"`
        );
        macroforgeTsCargo = macroforgeTsCargo.replace(
            /macroforge_ts_quote = "[^"]+"/g,
            `macroforge_ts_quote = "${version}"`
        );
        macroforgeTsCargo = macroforgeTsCargo.replace(
            /macroforge_ts_macros = "[^"]+"/g,
            `macroforge_ts_macros = "${version}"`
        );
        fs.writeFileSync(macroforgeTsCargoPath, macroforgeTsCargo);
        console.log(`  Updated crate dependencies in crates/macroforge_ts/Cargo.toml`);
    }

    // Update macros
    if (repoNames.includes('macros')) {
        console.log('\n[macros]');
        updateCargoToml("crates/macroforge_ts_macros/Cargo.toml", version);
    }

    // Update syn
    if (repoNames.includes('syn')) {
        console.log('\n[syn]');
        updateCargoToml("crates/macroforge_ts_syn/Cargo.toml", version);
    }

    // Update template (quote)
    if (repoNames.includes('template')) {
        console.log('\n[template]');
        updateCargoToml("crates/macroforge_ts_quote/Cargo.toml", version);
    }

    // Update shared
    if (repoNames.includes('shared')) {
        console.log('\n[shared]');
        updatePackageJson("packages/shared/package.json", {
            version,
            "dependencies.macroforge": `^${version}`,
        });
    }

    // Update typescript-plugin
    if (repoNames.includes('typescript-plugin')) {
        console.log('\n[typescript-plugin]');
        updatePackageJson("packages/typescript-plugin/package.json", {
            version,
            "dependencies.@macroforge/shared": `^${version}`,
            "dependencies.macroforge": `^${version}`,
        });
    }

    // Update svelte-language-server
    if (repoNames.includes('svelte-language-server')) {
        console.log('\n[svelte-language-server]');
        updatePackageJson("packages/svelte-language-server/package.json", {
            version,
            "dependencies.macroforge": `^${version}`,
            "dependencies.@macroforge/typescript-plugin": `^${version}`,
        });
    }

    // Update vite-plugin
    if (repoNames.includes('vite-plugin')) {
        console.log('\n[vite-plugin]');
        updatePackageJson("packages/vite-plugin/package.json", {
            version,
            "dependencies.@macroforge/shared": `^${version}`,
            "dependencies.macroforge": `^${version}`,
        });
    }

    // Update mcp-server
    if (repoNames.includes('mcp-server')) {
        console.log('\n[mcp-server]');
        updatePackageJson("packages/mcp-server/package.json", {
            version,
            "peerDependencies.macroforge": `^${version}`,
        });
    }

    // Update svelte-preprocessor
    if (repoNames.includes('svelte-preprocessor')) {
        console.log('\n[svelte-preprocessor]');
        updatePackageJson("packages/svelte-preprocessor/package.json", {
            version,
            "dependencies.macroforge": `^${version}`,
        });
    }

    // Update website
    if (repoNames.includes('website')) {
        console.log('\n[website]');
        updatePackageJson("website/package.json", {
            version,
            "dependencies.macroforge": `file:../crates/macroforge_ts`,
        });
    }

    // Update zed-extensions
    if (repoNames.includes('zed-extensions')) {
        console.log('\n[zed-extensions]');

        // Update vtsls-macroforge Zed extension lib.rs
        const vtslsLibRsPath = path.join(root, "crates/extensions/vtsls-macroforge/src/lib.rs");
        if (fs.existsSync(vtslsLibRsPath)) {
            let vtslsLibRs = fs.readFileSync(vtslsLibRsPath, "utf8");
            vtslsLibRs = vtslsLibRs.replace(
                /const TS_PLUGIN_VERSION: &str = ".*";/,
                `const TS_PLUGIN_VERSION: &str = "${version}";`
            );
            vtslsLibRs = vtslsLibRs.replace(
                /const MACROFORGE_VERSION: &str = ".*";/,
                `const MACROFORGE_VERSION: &str = "${version}";`
            );
            fs.writeFileSync(vtslsLibRsPath, vtslsLibRs);
            console.log(`  Updated crates/extensions/vtsls-macroforge/src/lib.rs`);
        }

        // Update svelte-macroforge Zed extension lib.rs
        const svelteLibRsPath = path.join(root, "crates/extensions/svelte-macroforge/src/lib.rs");
        if (fs.existsSync(svelteLibRsPath)) {
            let svelteLibRs = fs.readFileSync(svelteLibRsPath, "utf8");
            svelteLibRs = svelteLibRs.replace(
                /const SVELTE_LS_VERSION: &str = ".*";/,
                `const SVELTE_LS_VERSION: &str = "${version}";`
            );
            svelteLibRs = svelteLibRs.replace(
                /assert_eq!\(SVELTE_LS_VERSION, ".*"\);/,
                `assert_eq!(SVELTE_LS_VERSION, "${version}");`
            );
            fs.writeFileSync(svelteLibRsPath, svelteLibRs);
            console.log(`  Updated crates/extensions/svelte-macroforge/src/lib.rs`);
        }
    }

    console.log(`\nDone! Updated version to ${version}`);
}

program.action(main);
program.parse();
