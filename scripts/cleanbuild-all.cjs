#!/usr/bin/env node

const { execSync } = require('child_process');
const path = require('path');

const root = path.resolve(__dirname, '..');

const steps = [
  {
    label: 'remove root node_modules',
    cmd: 'rm -rf node_modules',
    cwd: root,
  },
  {
    label: 'remove packages node_modules',
    cmd: 'rm -rf node_modules',
    cwd: path.join(root, 'packages'),
  },
  {
    label: 'remove packages/vite-plugin-macroforge node_modules',
    cmd: 'rm -rf node_modules',
    cwd: path.join(root, 'packages', 'vite-plugin-macroforge'),
  },
  {
    label: 'remove packages/tsserver-plugin-macroforge node_modules',
    cmd: 'rm -rf node_modules',
    cwd: path.join(root, 'packages', 'tsserver-plugin-macroforge'),
  },
  {
    label: 'remove packages/svelte-language-server-macroforge node_modules',
    cmd: 'rm -rf node_modules',
    cwd: path.join(root, 'packages', 'svelte-language-server-macroforge'),
  },
  {
    label: 'remove playground/svelte node_modules',
    cmd: 'rm -rf node_modules',
    cwd: path.join(root, 'playground', 'svelte'),
  },
  {
    label: 'remove playground/svelte .svelte-kit',
    cmd: 'rm -rf .svelte-kit',
    cwd: path.join(root, 'playground', 'svelte'),
  },
  {
    label: 'remove playground/vanilla node_modules',
    cmd: 'rm -rf node_modules',
    cwd: path.join(root, 'playground', 'vanilla'),
  },
  {
    label: 'remove playground/macro node_modules',
    cmd: 'rm -rf node_modules',
    cwd: path.join(root, 'playground', 'macro'),
  },
  {
    label: 'remove tests/e2e/fixtures/ts-project node_modules',
    cmd: 'rm -rf node_modules',
    cwd: path.join(root, 'tests', 'e2e', 'fixtures', 'ts-project'),
  },
  {
    label: 'install packages workspace deps',
    cmd: 'bun install',
    cwd: path.join(root, 'packages'),
  },
  {
    label: 'clean extension.wasm (macroforge)',
    cmd: 'rm -f extension.wasm',
    cwd: path.join(root, 'crates', 'extensions', 'macroforge'),
  },
  {
    label: 'clean extension.wasm (svelte-macroforge)',
    cmd: 'rm -f extension.wasm',
    cwd: path.join(root, 'crates', 'extensions', 'svelte-macroforge'),
  },
  {
    label: 'clean crates/macroforge_ts',
    cmd: 'rm -f macroforge.*.node swc-napi-macros.*.node pkg/*.node',
    cwd: path.join(root, 'crates', 'macroforge_ts'),
  },
  {
    label: 'build crates/macroforge_ts',
    cmd: 'bun x @napi-rs/cli build --platform --release',
    cwd: path.join(root, 'crates', 'macroforge_ts'),
  },
  {
    label: 'clean packages/vite-plugin-macroforge',
    cmd: 'bun run clean',
    cwd: path.join(root, 'packages', 'vite-plugin-macroforge'),
  },
  {
    label: 'build packages/vite-plugin-macroforge',
    cmd: 'bun run build',
    cwd: path.join(root, 'packages', 'vite-plugin-macroforge'),
  },
  {
    label: 'clean packages/tsserver-plugin-macroforge',
    cmd: 'bun run clean',
    cwd: path.join(root, 'packages', 'tsserver-plugin-macroforge'),
  },
  {
    label: 'build packages/tsserver-plugin-macroforge',
    cmd: 'bun run build',
    cwd: path.join(root, 'packages', 'tsserver-plugin-macroforge'),
  },
  {
    label: 'clean packages/svelte-language-server-macroforge',
    cmd: 'bun run clean',
    cwd: path.join(root, 'packages', 'svelte-language-server-macroforge'),
  },
  {
    label: 'build packages/svelte-language-server-macroforge',
    cmd: 'bun run build',
    cwd: path.join(root, 'packages', 'svelte-language-server-macroforge'),
  },
  {
    label: 'install playground/macro deps',
    cmd: 'bun install',
    cwd: path.join(root, 'playground', 'macro'),
  },
  {
    label: 'cleanbuild playground/macro',
    cmd: 'bun run cleanbuild',
    cwd: path.join(root, 'playground', 'macro'),
  },
  {
    label: 'install playground/svelte deps',
    cmd: 'bun install',
    cwd: path.join(root, 'playground', 'svelte'),
  },
  {
    label: 'build playground/svelte',
    cmd: 'bun run build',
    cwd: path.join(root, 'playground', 'svelte'),
  },
  {
    label: 'install playground/vanilla deps',
    cmd: 'bun install',
    cwd: path.join(root, 'playground', 'vanilla'),
  },
  {
    label: 'build playground/vanilla',
    cmd: 'bun run build',
    cwd: path.join(root, 'playground', 'vanilla'),
  },
];

function runStep(step) {
  console.log(`\n==> ${step.label}`);
  execSync(step.cmd, { stdio: 'inherit', cwd: step.cwd });
}

try {
  steps.forEach(runStep);
  console.log('\nAll builds finished successfully.');
} catch (err) {
  console.error(`\nFailed during step: ${err?.message ?? err}`);
  process.exit(err?.status || 1);
}
