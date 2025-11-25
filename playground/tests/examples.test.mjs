import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createServer } from 'vite';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const playgroundRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(playgroundRoot, '..');
const vanillaRoot = path.join(playgroundRoot, 'vanilla');
const svelteRoot = path.join(playgroundRoot, 'svelte');
const rootConfigPath = path.join(repoRoot, 'ts-macros.json');

async function withViteServer(rootDir, optionsOrRunner, maybeRunner) {
  const options = typeof optionsOrRunner === 'function' ? {} : optionsOrRunner;
  const runner = typeof optionsOrRunner === 'function' ? optionsOrRunner : maybeRunner;
  const { useProjectCwd = false } = options ?? {};

  const configFile = path.join(rootDir, 'vite.config.ts');
  const previousCwd = process.cwd();
  let server;
  const createdLinks = [];
  let copiedConfig = false;
  const localConfigPath = path.join(rootDir, 'ts-macros.json');
  // Ensure workspace-local servers can discover the dynamic macro manifests that live in the repo root.
  const linkTargets = ['crates'];

  try {
    if (useProjectCwd) {
      process.chdir(rootDir);
    }

    // Copy the workspace-level config so the macro host loads @dynamic/json for the vanilla app.
    if (!fs.existsSync(localConfigPath) && fs.existsSync(rootConfigPath)) {
      fs.copyFileSync(rootConfigPath, localConfigPath);
      copiedConfig = true;
    }

    for (const dir of linkTargets) {
      const source = path.join(repoRoot, dir);
      const destination = path.join(rootDir, dir);
      if (!fs.existsSync(destination) && fs.existsSync(source)) {
        fs.symlinkSync(source, destination, 'dir');
        createdLinks.push(destination);
      }
    }

    server = await createServer({
      root: rootDir,
      configFile,
      logLevel: 'error',
      appType: 'custom',
      server: {
        middlewareMode: true,
        hmr: false
      },
      optimizeDeps: {
        disabled: true
      }
    });

    return await runner(server);
  } finally {
    if (server) {
      await server.close();
    }
    if (copiedConfig && fs.existsSync(localConfigPath)) {
      fs.rmSync(localConfigPath);
    }
    for (const link of createdLinks) {
      try {
        fs.unlinkSync(link);
      } catch {
        fs.rmSync(link, { recursive: true, force: true });
      }
    }
    if (useProjectCwd) {
      process.chdir(previousCwd);
    }
  }
}

test('vanilla playground macros emit runtime helpers', { timeout: 30000 }, async () => {
  await withViteServer(vanillaRoot, async (server) => {
    const mod = await server.ssrLoadModule('/src/user.ts');
    assert.ok(
      mod && typeof mod.User === 'function',
      'User class should be exported from vanilla playground'
    );

    const vanillaUser = new mod.User(9, 'Integration Tester', 'qa@example.com', 'tok_live_secret');

    assert.equal(typeof vanillaUser.toString, 'function', 'Debug derive should add toString');
    const summary = vanillaUser.toString();
    assert.ok(summary.startsWith('User {'), 'Derived toString() should include class label');
    assert.ok(
      summary.includes('identifier: 9'),
      'Derived toString() should respect rename option'
    );
    assert.ok(
      !summary.includes('authToken'),
      'Derived toString() should skip sensitive fields'
    );
  });
});

test(
  'svelte playground macros inline markdown and derive helpers',
  { timeout: 30000 },
  async () => {
    await withViteServer(
      svelteRoot,
      { useProjectCwd: true },
      async (server) => {
        const mod = await server.ssrLoadModule('/src/lib/demo/macro-user.ts');
        const { MacroUser, showcaseUserJson, showcaseUserSummary } = mod;

    assert.ok(MacroUser, 'MacroUser export should exist');
        const svelteUser = new MacroUser(
          'usr_55',
          'Rin Tester',
          'Macro QA',
          'Derive',
          '2025-02-01',
          'token_qa'
        );
        assert.deepEqual(svelteUser.toJSON(), {
          id: 'usr_55',
          name: 'Rin Tester',
          role: 'Macro QA',
          favoriteMacro: 'Derive',
          since: '2025-02-01',
          apiToken: 'token_qa'
        });

        assert.equal(typeof showcaseUserSummary, 'string', 'Derived summary should be a string');
        assert.equal(
          showcaseUserJson.favoriteMacro,
          'Derive',
          'Showcase JSON should include derived helpers'
        );
        assert.ok(
          showcaseUserSummary.includes('userId'),
          'Showcase summary should use renamed field label'
        );

        const transformed = await server.transformRequest('/src/lib/demo/macro-user.ts');
        assert.ok(transformed?.code.includes('toJSON()'), 'Derived methods should appear in transformed code');
      }
    );
  }
);
