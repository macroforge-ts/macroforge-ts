#!/usr/bin/env node
/**
 * Shared environment loader for all scripts.
 * Loads .env from tooling directory and exports MACROFORGE_ROOT.
 */

const path = require("path");
const fs = require("fs");

// Load .env file from tooling directory
const envPath = path.resolve(__dirname, "..", ".env");
if (fs.existsSync(envPath)) {
  const envContent = fs.readFileSync(envPath, "utf8");
  for (const line of envContent.split("\n")) {
    const trimmed = line.trim();
    if (trimmed && !trimmed.startsWith("#")) {
      const [key, ...valueParts] = trimmed.split("=");
      const value = valueParts.join("=");
      if (key && value && !process.env[key]) {
        process.env[key] = value;
      }
    }
  }
}

const MACROFORGE_ROOT = process.env.MACROFORGE_ROOT || path.resolve(__dirname, "..", "..");

/**
 * Repository definitions for all macroforge packages.
 * Each repo has:
 * - name: Short name used for CLI arguments
 * - path: Relative path from MACROFORGE_ROOT
 * - type: 'rust' | 'ts' | 'website' | 'tooling' | 'extension'
 * - packageJson: Path to package.json (if applicable)
 * - cargoToml: Path to Cargo.toml (if applicable)
 */
const REPOS = {
  // Rust crates
  core: {
    name: 'core',
    path: 'crates/macroforge_ts',
    type: 'rust',
    packageJson: 'crates/macroforge_ts/package.json',
    cargoToml: 'crates/macroforge_ts/Cargo.toml',
  },
  macros: {
    name: 'macros',
    path: 'crates/macroforge_ts_macros',
    type: 'rust',
    cargoToml: 'crates/macroforge_ts_macros/Cargo.toml',
  },
  syn: {
    name: 'syn',
    path: 'crates/macroforge_ts_syn',
    type: 'rust',
    cargoToml: 'crates/macroforge_ts_syn/Cargo.toml',
  },
  template: {
    name: 'template',
    path: 'crates/macroforge_ts_quote',
    type: 'rust',
    cargoToml: 'crates/macroforge_ts_quote/Cargo.toml',
  },
  // TypeScript packages
  shared: {
    name: 'shared',
    path: 'packages/shared',
    type: 'ts',
    packageJson: 'packages/shared/package.json',
  },
  'vite-plugin': {
    name: 'vite-plugin',
    path: 'packages/vite-plugin',
    type: 'ts',
    packageJson: 'packages/vite-plugin/package.json',
  },
  'typescript-plugin': {
    name: 'typescript-plugin',
    path: 'packages/typescript-plugin',
    type: 'ts',
    packageJson: 'packages/typescript-plugin/package.json',
  },
  'svelte-language-server': {
    name: 'svelte-language-server',
    path: 'packages/svelte-language-server',
    type: 'ts',
    packageJson: 'packages/svelte-language-server/package.json',
  },
  'svelte-preprocessor': {
    name: 'svelte-preprocessor',
    path: 'packages/svelte-preprocessor',
    type: 'ts',
    packageJson: 'packages/svelte-preprocessor/package.json',
  },
  'mcp-server': {
    name: 'mcp-server',
    path: 'packages/mcp-server',
    type: 'ts',
    packageJson: 'packages/mcp-server/package.json',
  },
  // Other
  website: {
    name: 'website',
    path: 'website',
    type: 'website',
    packageJson: 'website/package.json',
  },
  tooling: {
    name: 'tooling',
    path: 'tooling',
    type: 'tooling',
  },
  'zed-extensions': {
    name: 'zed-extensions',
    path: 'crates/extensions',
    type: 'extension',
  },
};

// Group repos by type for convenience
const RUST_REPOS = Object.values(REPOS).filter(r => r.type === 'rust');
const TS_REPOS = Object.values(REPOS).filter(r => r.type === 'ts');
const ALL_REPO_NAMES = Object.keys(REPOS);

/**
 * Parse repo names from CLI argument.
 * Accepts comma-separated list or 'all', 'rust', 'ts'.
 * @param {string} repoArg - The repo argument string
 * @returns {Array} Array of repo objects
 */
function parseRepos(repoArg) {
  if (!repoArg || repoArg === 'all') {
    return Object.values(REPOS);
  }
  if (repoArg === 'rust') {
    return RUST_REPOS;
  }
  if (repoArg === 'ts') {
    return TS_REPOS;
  }

  const names = repoArg.split(',').map(s => s.trim());
  const repos = [];
  for (const name of names) {
    if (REPOS[name]) {
      repos.push(REPOS[name]);
    } else {
      console.error(`Unknown repo: ${name}`);
      console.error(`Available repos: ${ALL_REPO_NAMES.join(', ')}`);
      process.exit(1);
    }
  }
  return repos;
}

module.exports = {
  MACROFORGE_ROOT,
  root: MACROFORGE_ROOT,
  resolve: (...segments) => path.resolve(MACROFORGE_ROOT, ...segments),
  REPOS,
  RUST_REPOS,
  TS_REPOS,
  ALL_REPO_NAMES,
  parseRepos,
};
