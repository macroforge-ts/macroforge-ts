import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
const { expandSync } = require('@macroforge/swc-napi');

const repoRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..', '..');
const svelteRoot = path.join(repoRoot, 'playground/svelte');

function expandFile(filePath) {
  const code = fs.readFileSync(filePath, 'utf8');
  return expandSync(code, filePath, { keepDecorators: false });
}

test('FieldController macro generates correct spacing', () => {
  const { code } = expandFile(path.join(svelteRoot, 'src/lib/demo/field-controller-test.ts'));

  // Expected concatenations:
  // makeFormModelBaseProps
  assert.ok(
    code.includes('makeFormModelBaseProps<'),
    'Expected "makeFormModelBaseProps<" to be concatenated'
  );

  // this.prototype.memoFieldPath
  assert.ok(
    code.includes('this.prototype.memoFieldPath = ["memo"];'),
    'Expected "this.prototype.memoFieldPath" to be concatenated'
  );

  // memoFieldController
  assert.ok(
    code.includes('memoFieldController(superForm:'),
    'Expected "memoFieldController" to be concatenated'
  );

  // this.makeFormModelBaseProps
  assert.ok(
    code.includes('this.makeFormModelBaseProps('),
    'Expected "this.makeFormModelBaseProps" to be concatenated'
  );

  // Expected spacing around keywords:
  // return baseProps
  assert.ok(
    code.includes('return baseProps;'),
    'Expected "return baseProps;" with space'
  );

  // const proxy = formFieldProxy
  assert.ok(
    code.includes('const proxy = formFieldProxy'),
    'Expected "const proxy = formFieldProxy" with space'
  );
});