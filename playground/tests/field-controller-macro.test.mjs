import { test } from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { expandSync } = require("macroforge");

const repoRoot = path.resolve(
  path.dirname(new URL(import.meta.url).pathname),
  "..",
  "..",
);
const svelteRoot = path.join(repoRoot, "playground/svelte");

function expandFile(filePath) {
  const code = fs.readFileSync(filePath, "utf8");
  return expandSync(code, filePath, { keepDecorators: false });
}

function normalize(source) {
  return source
    .replace(/\s+/g, " ")
    .replace(/\s*([\[\]\(\)\{\}<>,;:])\s*/g, "$1")
    .trim();
}

test("FieldController macro generates correct spacing", () => {
  const { code } = expandFile(
    path.join(svelteRoot, "src/lib/demo/field-controller.ts"),
  );
  const normalized = normalize(code);

  // Expected concatenations:
  // makeFormModelBaseProps
  assert.ok(
    normalized.includes("makeFormModelBaseProps<"),
    'Expected "makeFormModelBaseProps<" to be concatenated',
  );

  // memoFieldPath
  assert.ok(
    normalized.includes(normalize('export const memoFieldPath = ["memo"] as const;')),
    'Expected "export const memoFieldPath" to be concatenated',
  );

  // memoFieldController
  assert.ok(
    normalized.includes("memoFieldController(superForm:"),
    'Expected "memoFieldController" to be concatenated',
  );

  // makeFormModelBaseProps call
  assert.ok(
    normalized.includes("makeFormModelBaseProps("),
    'Expected "makeFormModelBaseProps(" call to be concatenated',
  );

  // Expected spacing around keywords:
  // return baseProps
  assert.ok(
    normalized.includes("return baseProps;"),
    'Expected "return baseProps;" with space',
  );

  // const proxy = formFieldProxy
  assert.ok(
    normalized.includes("const proxy = formFieldProxy"),
    'Expected "const proxy = formFieldProxy" with space',
  );
});
