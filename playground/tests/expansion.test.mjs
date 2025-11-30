import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
const { expandSync } = require('@ts-macros/swc-napi');

const repoRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..', '..');

function expandFile(relPath) {
  const filePath = path.join(repoRoot, relPath);
  const code = fs.readFileSync(filePath, 'utf8');
  return expandSync(code, filePath, { keepDecorators: false });
}

function assertDecoratorsStripped(output, fileLabel) {
  assert.ok(
    !output.includes('@Derive'),
    `${fileLabel}: @Derive should be stripped from expanded output`,
  );
}

function assertMethodsGenerated(output, fileLabel) {
  assert.ok(
    /toString\s*\(/.test(output),
    `${fileLabel}: expected generated toString()`,
  );
  assert.ok(
    /toJSON\s*\(/.test(output),
    `${fileLabel}: expected generated toJSON()`,
  );
}

test('vanilla: decorators stripped and methods generated', () => {
  const { code } = expandFile('playground/vanilla/src/user.ts');
  assertDecoratorsStripped(code, 'vanilla/user.ts');
  assertMethodsGenerated(code, 'vanilla/user.ts');
});

test('svelte: decorators stripped and methods generated', () => {
  const { code } = expandFile('playground/svelte/src/lib/demo/macro-user.ts');
  assertDecoratorsStripped(code, 'svelte/macro-user.ts');
  assertMethodsGenerated(code, 'svelte/macro-user.ts');
});
